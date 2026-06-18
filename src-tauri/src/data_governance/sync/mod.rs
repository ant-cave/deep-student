//! # Sync 模块 — 云同步管理系统
//!
//! 拆分结构：
//! - `types_incl.rs` — 类型定义 + Phase 1 UnifiedSyncManifest
//! - `impls_incl.rs` — 所有 impl 块
//! - `tests_*_incl.rs` — 测试

pub mod classification;
pub use conflict_resolver::{
    ConflictAwareApplyResult, ConflictOutcome, ConflictPolicy, ConflictRecordToSave,
    ConflictResolver, ConflictSide,
};
pub use emitter::{OptionalEmitter, SyncProgressCallback, SyncProgressEmitter, EVENT_NAME};
pub use hlc::{Hlc, MAX_DRIFT_MS};
pub use progress::{ProgressTracker, SpeedCalculator, SyncPhase, SyncProgress};
pub use state::SyncStateStore;
pub use tombstone::{
    apply_blob_tombstones, AssetTombstoneEntry, AssetTombstones, BlobTombstoneEntry, BlobTombstones,
};
pub mod conflict_resolver;
pub mod emitter;
pub mod field_merge;
pub mod hlc;
pub mod progress;
pub mod state;
pub mod tombstone;

// 类型定义（含 Phase 1 UnifiedSyncManifest）
include!("types_incl.rs");

// 所有 impl 块
include!("impls_incl.rs");

// ========================================================================
// Phase 1：统一 Manifest 云端操作方法
// ========================================================================

impl SyncManager {
    pub async fn upload_unified_manifest(
        &self, storage: &dyn CloudStorage, unified: &UnifiedSyncManifest,
    ) -> Result<(), SyncError> {
        let json = serde_json::to_vec_pretty(unified)
            .map_err(|e| SyncError::Database(format!("序列化统一清单失败: {}", e)))?;
        let payload = self.encode_payload(&json)?;
        retry_async("上传统一清单", 2, || {
            let payload = payload.clone();
            async move {
                storage.put("data_governance/sync_manifest.json", &payload).await
                    .map_err(|e| SyncError::Network(format!("上传统一清单失败: {}", e)))
            }
        }).await?;
        Ok(())
    }

    pub async fn download_unified_manifest(
        &self, storage: &dyn CloudStorage,
    ) -> Result<Option<UnifiedSyncManifest>, SyncError> {
        let bytes = storage.get("data_governance/sync_manifest.json").await
            .map_err(|e| SyncError::Network(format!("下载统一清单失败: {}", e)))?;
        match bytes {
            Some(b) => {
                let decoded = self.decode_payload(&b)?;
                match serde_json::from_slice::<UnifiedSyncManifest>(&decoded) {
                    Ok(m) => Ok(Some(m)),
                    Err(e) => {
                        tracing::warn!("[sync] 统一清单解析失败，回退旧格式: {}", e);
                        Ok(None)
                    }
                }
            }
            None => Ok(None),
        }
    }

    pub fn build_sync_manifest_from_unified(
        unified: &UnifiedSyncManifest,
        databases: HashMap<String, DatabaseSyncState>,
    ) -> SyncManifest {
        SyncManifest {
            sync_transaction_id: uuid::Uuid::new_v4().to_string(),
            databases, status: SyncTransactionStatus::Complete,
            created_at: unified.created_at.clone(),
            device_id: unified.device_id.clone(),
            format_version: 3,
            published_max_seq: unified.changes_meta
                .get(&unified.device_id).map(|m| m.published_max_seq).unwrap_or(0),
            cursors: HashMap::new(),
            superseded_by: None,
            snapshot_seen: unified.snapshot_seen.clone(),
        }
    }
}

// 测试（必须放在文件末尾）
#[cfg(test)] include!("tests_a_incl.rs");
#[cfg(test)] include!("tests_b_incl.rs");
#[cfg(test)] include!("tests_c_incl.rs");
#[cfg(test)] include!("tests_d_incl.rs");

// ========================================================================
// Phase 1：统一同步协调方法
// ========================================================================

impl SyncManager {
    /// 使用统一清单执行完整双向同步（替代多次独立 GET/PUT）
    ///
    /// 1. 下载统一清单（兼容旧格式回退）
    /// 2. 分别同步工作区、blobs、资产目录
    /// 3. 上传更新后的统一清单
    pub async fn sync_all_unified(
        &self,
        storage: &dyn CloudStorage,
        active_dir: &std::path::Path,
        app_data_dir: &std::path::Path,
        direction: SyncDirection,
    ) -> Result<(BlobSyncOutcome, AssetSyncOutcome), SyncError> {
        use crate::cloud_storage::CloudStorage;

        // 下载统一清单
        let unified = self.download_unified_manifest(storage).await?;
        let has_unified = unified.is_some();

        // 执行各组件同步
        self.sync_workspace_databases(storage, active_dir, direction).await?;

        let blob_outcome = self.sync_vfs_blobs_with_tombstones(storage, &active_dir.join("vfs_blobs"), direction).await?;

        let asset_outcome = self.sync_asset_directories_with_tombstones(storage, active_dir, app_data_dir, direction).await?;

        // 构建并上传统一清单
        if direction != SyncDirection::Download || !has_unified {
            let workspaces = self.download_workspaces_manifest(storage).await?;
            let blobs = self.download_blobs_manifest(storage).await?;
            let assets = self.download_assets_manifest(storage).await?;

            let unified = UnifiedSyncManifest {
                format_version: 3,
                device_id: self.device_id.clone(),
                created_at: chrono::Utc::now().to_rfc3339(),
                workspaces,
                blobs,
                assets,
                changes_meta: std::collections::HashMap::new(),
                blob_tombstones_raw: None,
                asset_tombstones_raw: None,
                snapshot_seen: std::collections::HashMap::new(),
            };

            self.upload_unified_manifest(storage, &unified).await?;
        }

        Ok((blob_outcome, asset_outcome))
    }

    /// [Phase 4] 从 UnifiedSyncManifest 的 changes_meta 下载变更归档
    async fn download_changes_from_manifest(
        &self, storage: &dyn CloudStorage, _unified: &UnifiedSyncManifest,
        _since_version: u64, _per_db_since: Option<&HashMap<String, u64>>,
    ) -> Result<DownloadChangesResult, SyncError> {
        // Phase 4 stub: for now, return empty result to trigger fallback to list_outcome.
        // Full implementation will download archives from changes_meta.
        Ok(DownloadChangesResult::default())
    }

    /// 下载工作区清单（兼容模式：优先从统一清单读取）
    pub(crate) async fn download_workspaces_from_unified(
        &self, storage: &dyn CloudStorage,
    ) -> Result<WorkspacesManifest, SyncError> {
        if let Some(unified) = self.download_unified_manifest(storage).await? {
            return Ok(unified.workspaces);
        }
        self.download_workspaces_manifest(storage).await
    }

    /// 下载 Blob 清单（兼容模式：优先从统一清单读取）
    pub(crate) async fn download_blobs_from_unified(
        &self, storage: &dyn CloudStorage,
    ) -> Result<BlobsManifest, SyncError> {
        if let Some(unified) = self.download_unified_manifest(storage).await? {
            return Ok(unified.blobs);
        }
        self.download_blobs_manifest(storage).await
    }

    /// 下载资产清单（兼容模式：优先从统一清单读取）
    pub(crate) async fn download_assets_from_unified(
        &self, storage: &dyn CloudStorage,
    ) -> Result<AssetDirsManifest, SyncError> {
        if let Some(unified) = self.download_unified_manifest(storage).await? {
            return Ok(unified.assets);
        }
        self.download_assets_manifest(storage).await
    }
}
