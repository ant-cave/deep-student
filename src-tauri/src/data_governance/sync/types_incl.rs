

/// 公开的时间戳解析函数（供 conflict_resolver 等子模块复用）
pub fn parse_flexible_timestamp_public(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    use chrono::{DateTime, NaiveDateTime, Utc};
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Some(naive.and_utc());
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
        return Some(naive.and_utc());
    }
    // 纯数字串：尝试作为毫秒时间戳解析
    // （resources / chat_v2_todo_lists 等表用 INTEGER ms 存储 updated_at）
    if let Ok(ms) = s.parse::<i64>() {
        // 秒级 (1e9 ~ 1e10) vs 毫秒级 (1e12 ~ 1e13) 用阈值区分，避免
        // 2038 前后年份的数值被误当毫秒
        const MS_THRESHOLD: i64 = 100_000_000_000; // 1e11
        if ms >= MS_THRESHOLD {
            return DateTime::<Utc>::from_timestamp_millis(ms);
        } else if ms >= 1_000_000_000 {
            return DateTime::<Utc>::from_timestamp(ms, 0);
        }
    }
    None
}

use super::schema_registry::DatabaseId;
use classification::SyncCategory;
use rusqlite::{params, types::Type, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// 记录并跳过迭代中的错误，避免静默丢弃
fn log_and_skip_err<T, E: std::fmt::Display>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(v) => Some(v),
        Err(e) => {
            tracing::warn!("[Sync] Row parse error (skipped): {}", e);
            None
        }
    }
}

type IdAliasMap = HashMap<(String, String), String>;
type ForeignKeyViolationSet = HashSet<String>;

#[derive(Debug, Clone)]
struct ForeignKeyColumn {
    child_column: String,
    parent_table: String,
    parent_column: String,
}

/// 带指数退避的异步重试工具
///
/// 对可重试的网络操作（如上传/下载清单和变更）进行最多 `max_retries` 次尝试，
/// 每次失败后以指数退避等待（500ms, 1s, 2s, ...）。
///
/// [P3 Fix] 注意：底层传输层（WebDAV/S3）可能有自己的重试机制（通常 3 次）。
/// 调用方应使用较低的 max_retries（建议 2）以避免叠加过多重试。
#[cfg(feature = "data_governance")]
async fn retry_async<F, Fut, T>(op_name: &str, max_retries: u32, f: F) -> Result<T, SyncError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, SyncError>>,
{
    let base_ms: u64 = 500;
    let mut last_err = SyncError::Network(format!("{}: 未知错误", op_name));
    for attempt in 0..max_retries {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) => {
                last_err = e;
                if attempt + 1 < max_retries {
                    let delay = base_ms * (1u64 << attempt);
                    tracing::warn!(
                        "[Sync] {} 重试 {}/{}: {}（等待 {}ms）",
                        op_name,
                        attempt + 1,
                        max_retries,
                        last_err,
                        delay
                    );
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                }
            }
        }
    }
    Err(last_err)
}

#[cfg(feature = "data_governance")]
// 云存储集成
use crate::cloud_storage::CloudStorage;

/// 同步清单
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncManifest {
    /// 同步事务 ID
    pub sync_transaction_id: String,
    /// 各数据库状态
    pub databases: HashMap<String, DatabaseSyncState>,
    /// 状态
    pub status: SyncTransactionStatus,
    /// 创建时间
    pub created_at: String,
    /// 设备 ID
    pub device_id: String,
    /// 云同步协议格式版本（3 = per-device seq/cursor）
    #[serde(default = "default_manifest_format_version")]
    pub format_version: u32,
    /// 本设备已经成功发布到云端的最大序号
    #[serde(default)]
    pub published_max_seq: u64,
    /// 本设备已经安全消费到的其他设备序号
    #[serde(default)]
    pub cursors: HashMap<String, u64>,
    /// restore 后旧设备清单可指向新设备 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superseded_by: Option<String>,
    /// 本设备最近见过的快照（Phase 2 快照引导占位）
    #[serde(default)]
    pub snapshot_seen: HashMap<String, String>,
}

fn default_manifest_format_version() -> u32 {
    2
}

/// 数据库同步状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSyncState {
    /// Schema 版本
    pub schema_version: u32,
    /// 数据版本（最大 local_version）
    pub data_version: u64,
    /// Checksum
    pub checksum: String,
    /// 最后更新时间
    #[serde(default)]
    pub last_updated_at: Option<String>,
}

/// 同步事务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncTransactionStatus {
    /// 完成
    Complete,
    /// 部分完成（需要修复）
    Partial,
    /// 失败
    Failed,
}

/// 数据库级冲突
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConflict {
    /// 数据库名称
    pub database_name: String,
    /// 冲突类型
    pub conflict_type: DatabaseConflictType,
    /// 本地状态
    pub local_state: Option<DatabaseSyncState>,
    /// 云端状态
    pub cloud_state: Option<DatabaseSyncState>,
}

/// 数据库冲突类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatabaseConflictType {
    /// Schema 版本不匹配（需要迁移）
    SchemaMismatch,
    /// 数据版本冲突（双方都有修改）
    DataConflict,
    /// Checksum 不匹配（数据内容不同）
    ChecksumMismatch,
    /// 本地有，云端没有
    LocalOnly,
    /// 云端有，本地没有
    CloudOnly,
}

/// 冲突记录（记录级别）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictRecord {
    /// 数据库名称
    pub database_name: String,
    /// 表名
    pub table_name: String,
    /// 记录 ID
    pub record_id: String,
    /// 本地版本
    pub local_version: u64,
    /// 云端版本
    pub cloud_version: u64,
    /// 本地更新时间
    pub local_updated_at: String,
    /// 云端更新时间
    pub cloud_updated_at: String,
    /// 本地数据（JSON）
    pub local_data: serde_json::Value,
    /// 云端数据（JSON）
    pub cloud_data: serde_json::Value,
}

/// 冲突检测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictDetectionResult {
    /// 数据库级冲突
    pub database_conflicts: Vec<DatabaseConflict>,
    /// 记录级冲突（需要进一步查询数据库）
    pub record_conflicts: Vec<ConflictRecord>,
    /// 是否有冲突
    pub has_conflicts: bool,
    /// 是否需要迁移
    pub needs_migration: bool,
}

impl ConflictDetectionResult {
    /// 创建空的检测结果（无冲突）
    pub fn empty() -> Self {
        Self {
            database_conflicts: Vec::new(),
            record_conflicts: Vec::new(),
            has_conflicts: false,
            needs_migration: false,
        }
    }

    /// 冲突总数
    pub fn total_conflicts(&self) -> usize {
        self.database_conflicts.len() + self.record_conflicts.len()
    }
}

/// 合并策略
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum MergeStrategy {
    /// 保留本地
    KeepLocal,
    /// 使用云端
    UseCloud,
    /// 保留最新（按 updated_at）
    KeepLatest,
    /// 手动合并（用户选择）
    Manual,
}

/// 同步结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    /// 是否成功
    pub success: bool,
    /// 同步的数据库数量
    pub synced_databases: usize,
    /// 解决的冲突数量
    pub resolved_conflicts: usize,
    /// 需要手动处理的冲突
    pub pending_manual_conflicts: Vec<ConflictRecord>,
    /// 错误信息（如果有）
    pub errors: Vec<String>,
}

impl SyncResult {
    /// 创建成功结果
    pub fn success(synced_databases: usize, resolved_conflicts: usize) -> Self {
        Self {
            success: true,
            synced_databases,
            resolved_conflicts,
            pending_manual_conflicts: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// 创建需要手动处理的结果
    pub fn needs_manual(conflicts: Vec<ConflictRecord>) -> Self {
        Self {
            success: false,
            synced_databases: 0,
            resolved_conflicts: 0,
            pending_manual_conflicts: conflicts,
            errors: Vec::new(),
        }
    }

    /// 创建失败结果
    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            success: false,
            synced_databases: 0,
            resolved_conflicts: 0,
            pending_manual_conflicts: Vec::new(),
            errors,
        }
    }
}

/// 同步错误
#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Conflict detected: {count} records")]
    Conflict { count: usize },

    #[error("Schema mismatch: local={local}, cloud={cloud}")]
    SchemaMismatch { local: u32, cloud: u32 },

    #[error("Partial sync: {completed}/{total} databases")]
    PartialSync { completed: usize, total: usize },

    #[error("Manual resolution required: {count} conflicts")]
    ManualResolutionRequired { count: usize },

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// 云端变更时间戳超出本地 wall clock 未来容忍窗口（疑似时钟漂移/篡改）。
    /// 此类变更必须进入隔离区（可见、可重放），绝不允许静默丢弃——
    /// 否则一台时钟超前的设备的全部写入会被其他设备永久忽略（违反 INV-1）。
    #[error("Clock drift suspected: {table}.{record_id} timestamp is {drift_ms}ms in the future")]
    ClockDriftSuspected {
        table: String,
        record_id: String,
        drift_ms: i64,
    },
}

/// 云端 UPSERT 新鲜度评估结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UpsertFreshness {
    /// 正常应用
    Proceed,
    /// 本地严格更新（LWW），跳过这条云端变更
    SkipStale,
    /// 云端时间戳超前本地 wall clock 过多，疑似漂移 → 调用方必须隔离而非丢弃
    SuspectDrift { drift_ms: i64 },
}

/// 同步字段 SQL（用于需要同步的表）
pub const SYNC_FIELDS_SQL: &str = r#"
    -- 添加同步字段
    ALTER TABLE {table} ADD COLUMN device_id TEXT;
    ALTER TABLE {table} ADD COLUMN local_version INTEGER DEFAULT 0;
    ALTER TABLE {table} ADD COLUMN sync_version INTEGER DEFAULT 0;
    ALTER TABLE {table} ADD COLUMN updated_at TEXT DEFAULT (datetime('now'));
    ALTER TABLE {table} ADD COLUMN deleted_at TEXT;  -- tombstone，非 NULL 表示已删除

    -- 创建索引
    CREATE INDEX IF NOT EXISTS idx_{table}_local_version ON {table}(local_version);
    CREATE INDEX IF NOT EXISTS idx_{table}_sync_version ON {table}(sync_version);
    CREATE INDEX IF NOT EXISTS idx_{table}_deleted_at ON {table}(deleted_at);
"#;

/// 工作区数据库云同步清单（ws_*.db 文件级同步）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspacesManifest {
    /// ws_id → 条目
    pub entries: HashMap<String, WorkspaceEntry>,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub archive: Option<ArchiveEntry>,
}

/// 单个工作区数据库的同步条目
///
/// [P1 churn 修复] 两个哈希字段承担不同职责，不可混用：
/// - `sha256`：**传输对象**（VACUUM INTO 快照）的哈希，用于下载完整性校验；
/// - `source_sha256`：上传时**本地源文件**的哈希，用于变更检测。
///   VACUUM 会重写页布局，快照哈希与活动文件哈希几乎永不相等——
///   旧实现拿 `sha256` 与本地活动文件哈希比较，导致每次同步都误判
///   "已变更" 而把所有工作区 DB 原样重传。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceEntry {
    pub sha256: String,
    pub size: u64,
    pub updated_at: String,
    /// 上传时本地源文件（活动 .db）的哈希；旧清单无此字段（None 时退化为旧行为）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_sha256: Option<String>,
    /// 上传者设备 ID（审计/调试）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

/// VFS blob 云同步清单（内容寻址）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlobsManifest {
    /// content_hash → 条目
    pub entries: HashMap<String, BlobEntry>,
    /// 归档列表，按 seq 递增
    #[serde(default)]
    pub archives: Vec<ArchiveEntry>,
    #[serde(default)]
    pub updated_at: String,
}

/// 归档文件元数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArchiveEntry {
    /// 归档序号（从 1 开始递增）
    pub seq: u32,
    /// 归档文件的 sha256
    pub sha256: String,
    /// 归档文件大小（字节）
    pub size: u64,
    /// 归档包含的 blob 数量
    pub blob_count: usize,
    /// 创建时间（RFC3339）
    pub created_at: String,
}

/// 单个 blob 的同步条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlobEntry {
    /// 相对路径（相对于 vfs_blobs/），如 "ab/abc123....pdf"
    pub relative_path: String,
    pub size: u64,
    #[serde(default)]
    pub updated_at: String,
    /// 所属归档序号，0 表示逐个上传模式（旧版兼容）
    #[serde(default)]
    pub archive_seq: u32,
}

/// VFS Blob 同步结果，区分完全成功与部分失败
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlobSyncOutcome {
    pub uploaded: usize,
    pub downloaded: usize,
    pub upload_failures: Vec<String>,
    pub download_failures: Vec<String>,
    /// 上传的归档数量
    #[serde(default)]
    pub archives_uploaded: usize,
    /// 下载的归档数量
    #[serde(default)]
    pub archives_downloaded: usize,
}

impl BlobSyncOutcome {
    pub fn has_failures(&self) -> bool {
        !self.upload_failures.is_empty() || !self.download_failures.is_empty()
    }

    pub fn failure_summary(&self) -> Option<String> {
        if !self.has_failures() {
            return None;
        }
        Some(format!(
            "附件同步部分失败：{} 个上传失败，{} 个下载失败",
            self.upload_failures.len(),
            self.download_failures.len()
        ))
    }
}

/// 通用资产目录云同步清单（images/documents/...）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssetDirsManifest {
    /// key -> 条目，key 形如 "active/images/a.png" 或 "app_data/pdf_ocr_sessions/x.json"
    pub entries: HashMap<String, AssetFileEntry>,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub archives: Vec<ArchiveEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AssetFileEntry {
    pub sha256: String,
    pub size: u64,
    #[serde(default)]
    pub updated_at: String,
    #[serde(default)]
    pub archive_seq: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetSyncOutcome {
    pub uploaded: usize,
    pub downloaded: usize,
    pub upload_failures: Vec<String>,
    pub download_failures: Vec<String>,
}

pub(crate) type FileTransferProgressCallback =
    std::sync::Arc<dyn Fn(String, u64, u64) + Send + Sync>;

impl AssetSyncOutcome {
    pub fn has_failures(&self) -> bool {
        !self.upload_failures.is_empty() || !self.download_failures.is_empty()
    }

    pub fn failure_summary(&self) -> Option<String> {
        if !self.has_failures() {
            return None;
        }
        Some(format!(
            "资产目录同步部分失败：{} 个上传失败，{} 个下载失败",
            self.upload_failures.len(),
            self.download_failures.len()
        ))
    }
}

/// 下载变更结果（包含非致命解析告警）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DownloadChangesResult {
    pub changes: Vec<SyncChangeWithData>,
    pub decode_failures: Vec<String>,
    pub cursor_advancements: HashMap<String, u64>,
    pub legacy_processed_keys: Vec<String>,
}

impl DownloadChangesResult {
    pub fn len(&self) -> usize {
        self.changes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, SyncChangeWithData> {
        self.changes.iter()
    }
}

impl AsRef<[SyncChangeWithData]> for DownloadChangesResult {
    fn as_ref(&self) -> &[SyncChangeWithData] {
        &self.changes
    }
}

impl IntoIterator for DownloadChangesResult {
    type Item = SyncChangeWithData;
    type IntoIter = std::vec::IntoIter<SyncChangeWithData>;

    fn into_iter(self) -> Self::IntoIter {
        self.changes.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsedChangeKey {
    V3 {
        device_id: String,
        seq: u64,
        version: u64,
    },
    Legacy {
        device_id: String,
        version: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SyncDatabaseSnapshot {
    format_version: u32,
    database_name: String,
    device_id: String,
    created_at: String,
    schema_version: u32,
    data_version: u64,
    checksum: String,
    /// Per uploader seq covered by this full database snapshot.
    covered_cursors: HashMap<String, u64>,
    /// table_name -> complete RowSync rows for this database.
    rows: HashMap<String, Vec<serde_json::Value>>,
}

#[derive(Debug, Clone)]
struct SnapshotCoverage {
    covered_cursors: HashMap<String, u64>,
}

/// 同步管理器
pub struct SyncManager {
    /// 本地设备 ID
    device_id: String,
    /// 可选的端到端加密密码（对文本 payload 生效，批判报告 P0-2 修复）
    ///
    /// 覆盖范围：
    /// - ✅ 加密：`SyncManifest`、`SyncChangesPayload`、`*Tombstones`、
    ///   各种 metadata manifest（workspaces/blobs/assets）
    /// - ❌ **不**加密：VFS blob 的 raw bytes、workspace `.db` 文件。
    ///   原因：blob 走内容寻址（sha256 作 key），加密会破坏去重语义；
    ///   workspace DB 的完整性校验依赖明文 sha256。这两类的加密需要
    ///   额外的密文-明文 hash 双校验，作为后续 P1 任务单独处理。
    ///
    /// 语义：
    /// - `None` 或空字符串：所有 payload 明文上传（向后兼容旧数据）
    /// - `Some(pw)` 非空：文本 payload 使用 `DSBK` 容器加密（AES-256-GCM + Argon2id）
    ///
    /// 解密端自动探测：遇到 `DSBK` 魔数走解密，否则当明文处理。这让加密可以
    /// 平滑启用，不破坏已存在的明文云端数据。
    #[cfg(feature = "data_governance")]
    encryption_password: Option<String>,
}


// ====================================================================
// Phase 1：统一归档同步 Manifest
// ====================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSyncManifest {
    pub format_version: u32,
    pub device_id: String,
    pub created_at: String,
    pub workspaces: WorkspacesManifest,
    pub blobs: BlobsManifest,
    pub assets: AssetDirsManifest,
    pub changes_meta: std::collections::HashMap<String, DeviceChangesMeta>,
    #[serde(default)]
    pub blob_tombstones_raw: Option<serde_json::Value>,
    #[serde(default)]
    pub asset_tombstones_raw: Option<serde_json::Value>,
    #[serde(default)]
    pub snapshot_seen: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceChangesMeta {
    pub published_max_seq: u64,
    pub changes_archive: Option<ArchiveEntry>,
    pub updated_at: String,
}

impl UnifiedSyncManifest {
    pub fn empty(device_id: &str) -> Self {
        Self {
            format_version: 3,
            device_id: device_id.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            workspaces: WorkspacesManifest::default(),
            blobs: BlobsManifest::default(),
            assets: AssetDirsManifest::default(),
            changes_meta: std::collections::HashMap::new(),
            blob_tombstones_raw: None,
            asset_tombstones_raw: None,
            snapshot_seen: std::collections::HashMap::new(),
        }
    }
}
