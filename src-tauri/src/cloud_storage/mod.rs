//! 云存储模块
//!
//! 提供统一的云存储访问层，支持 WebDAV 和 S3 兼容存储。
//!
//! ## 支持的存储后端
//! - **WebDAV**: 坚果云、Nextcloud、自建 WebDAV 等
//! - **S3**: AWS S3、Cloudflare R2、阿里云 OSS、MinIO 等
//!
//! ## 使用示例
//! ```rust,ignore
//! use cloud_storage::{create_storage, CloudStorageConfig, StorageProvider};
//!
//! let config = CloudStorageConfig {
//!     provider: StorageProvider::S3,
//!     s3: Some(S3Config { ... }),
//!     ..Default::default()
//! };
//!
//! let storage = create_storage(&config).await?;
//! storage.put("backups/data.zip", &data).await?;
//! ```

mod config;
mod ftp;
#[cfg(feature = "cloud_storage_s3")]
mod s3;
mod sync_manager;
mod traits;
mod webdav;

pub use config::{CloudStorageConfig, FtpConfig, S3Config, StorageProvider, WebDavConfig};
pub use sync_manager::{
    get_device_id, BackupVersion, CloudManifest, CloudSyncManager, DownloadResult, SyncStatus,
    UploadResult,
};
pub use traits::{CloudStorage, FileInfo, Result};

use serde::Serialize;
use tauri::{AppHandle, Emitter};

use crate::models::AppError;
use ftp::FtpStorage;
#[cfg(feature = "cloud_storage_s3")]
use s3::S3Storage;
use webdav::WebDavStorage;

/// 云同步操作进度事件（通过 `cloud-sync-progress` 事件发送到前端）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct CloudSyncProgressEvent {
    /// 操作类型: "upload" | "download"
    operation: &'static str,
    /// 阶段标识: "transferring" | "done"
    stage: &'static str,
    /// 阶段描述（面向用户的中文说明）
    stage_label: &'static str,
    /// 已传输字节数
    bytes_done: u64,
    /// 总字节数（0 = 未知）
    bytes_total: u64,
    /// 传输进度百分比 0.0–100.0（仅文件传输阶段有意义）
    percent: f32,
}

fn emit_sync_progress(app: &AppHandle, event: CloudSyncProgressEvent) {
    if let Err(e) = app.emit("cloud-sync-progress", &event) {
        tracing::warn!("[CloudSync] 进度事件发射失败: {}", e);
    }
}

/// 根据配置创建存储实例
///
/// # Arguments
/// * `config` - 云存储配置
///
/// # Returns
/// 实现了 CloudStorage trait 的存储实例
pub async fn create_storage(config: &CloudStorageConfig) -> Result<Box<dyn CloudStorage>> {
    // 验证配置
    config.validate().map_err(|e| AppError::validation(e))?;

    let root = config.root();

    tracing::info!(
        "[CloudStorage] 创建存储实例: provider={}, root={}, encrypted={}",
        config.provider,
        root,
        config
            .encryption_password
            .as_ref()
            .map(|p| !p.is_empty())
            .unwrap_or(false)
    );

    match config.provider {
        StorageProvider::WebDav => {
            let webdav_config = config
                .webdav
                .clone()
                .ok_or_else(|| AppError::validation("缺少 WebDAV 配置"))?;
            let storage = WebDavStorage::new(webdav_config, root)?;
            Ok(Box::new(storage))
        }
        #[cfg(feature = "cloud_storage_s3")]
        StorageProvider::S3 => {
            let s3_config = config
                .s3
                .clone()
                .ok_or_else(|| AppError::validation("缺少 S3 配置"))?;
            let storage = S3Storage::new(s3_config, root).await?;
            Ok(Box::new(storage))
        }
        #[cfg(not(feature = "cloud_storage_s3"))]
        StorageProvider::S3 => Err(AppError::configuration(
            "S3 存储支持未启用，请在编译时启用 cloud_storage_s3 feature".to_string(),
        )),
        StorageProvider::Ftp => {
            let ftp_config = config
                .ftp
                .clone()
                .ok_or_else(|| AppError::validation("缺少 FTP 配置"))?;
            let storage = FtpStorage::new(ftp_config, root)?;
            Ok(Box::new(storage))
        }
    }
}

// ============== Tauri Commands ==============

/// 检查云存储连接
#[tauri::command]
pub async fn cloud_storage_check_connection(config: CloudStorageConfig) -> Result<bool> {
    tracing::info!(
        "[CloudStorage] 开始检查云存储连接: provider={}",
        config.provider
    );
    let storage = create_storage(&config).await?;
    match storage.check_connection().await {
        Ok(()) => {
            tracing::info!("[CloudStorage] 云存储连接成功: provider={}", config.provider);
            Ok(true)
        }
        Err(e) => {
            tracing::error!("[CloudStorage] 云存储连接失败: provider={}, error={}", config.provider, e);
            Err(e)
        }
    }
}

/// 上传文件到云存储
#[tauri::command]
pub async fn cloud_storage_put(
    config: CloudStorageConfig,
    key: String,
    data: Vec<u8>,
) -> Result<()> {
    let size = data.len();
    tracing::info!(
        "[CloudStorage] 上传文件: key={}, size={}, provider={}",
        key,
        size,
        config.provider
    );
    let storage = create_storage(&config).await?;
    match storage.put(&key, &data).await {
        Ok(()) => {
            tracing::info!("[CloudStorage] 上传文件成功: key={}, size={}", key, size);
            Ok(())
        }
        Err(e) => {
            tracing::error!("[CloudStorage] 上传文件失败: key={}, error={}", key, e);
            Err(e)
        }
    }
}

/// 从云存储下载文件
#[tauri::command]
pub async fn cloud_storage_get(config: CloudStorageConfig, key: String) -> Result<Option<Vec<u8>>> {
    tracing::info!(
        "[CloudStorage] 下载文件: key={}, provider={}",
        key,
        config.provider
    );
    let storage = create_storage(&config).await?;
    match storage.get(&key).await {
        Ok(Some(data)) => {
            tracing::info!("[CloudStorage] 下载文件成功: key={}, size={}", key, data.len());
            Ok(Some(data))
        }
        Ok(None) => {
            tracing::info!("[CloudStorage] 下载文件不存在: key={}", key);
            Ok(None)
        }
        Err(e) => {
            tracing::error!("[CloudStorage] 下载文件失败: key={}, error={}", key, e);
            Err(e)
        }
    }
}

/// 列出云存储中的文件
#[tauri::command]
pub async fn cloud_storage_list(
    config: CloudStorageConfig,
    prefix: String,
) -> Result<Vec<FileInfo>> {
    tracing::info!(
        "[CloudStorage] 列出文件: prefix={}, provider={}",
        prefix,
        config.provider
    );
    let storage = create_storage(&config).await?;
    match storage.list(&prefix).await {
        Ok(files) => {
            tracing::info!(
                "[CloudStorage] 列出文件成功: prefix={}, count={}",
                prefix,
                files.len()
            );
            Ok(files)
        }
        Err(e) => {
            tracing::error!("[CloudStorage] 列出文件失败: prefix={}, error={}", prefix, e);
            Err(e)
        }
    }
}

/// 删除云存储中的文件
#[tauri::command]
pub async fn cloud_storage_delete(config: CloudStorageConfig, key: String) -> Result<()> {
    tracing::info!("[CloudStorage] 删除文件: key={}, provider={}", key, config.provider);
    let storage = create_storage(&config).await?;
    match storage.delete(&key).await {
        Ok(()) => {
            tracing::info!("[CloudStorage] 删除文件成功: key={}", key);
            Ok(())
        }
        Err(e) => {
            tracing::error!("[CloudStorage] 删除文件失败: key={}, error={}", key, e);
            Err(e)
        }
    }
}

/// 获取文件信息
#[tauri::command]
pub async fn cloud_storage_stat(
    config: CloudStorageConfig,
    key: String,
) -> Result<Option<FileInfo>> {
    tracing::info!("[CloudStorage] 查询文件信息: key={}, provider={}", key, config.provider);
    let storage = create_storage(&config).await?;
    storage.stat(&key).await
}

/// 检查文件是否存在
#[tauri::command]
pub async fn cloud_storage_exists(config: CloudStorageConfig, key: String) -> Result<bool> {
    tracing::info!("[CloudStorage] 检查文件是否存在: key={}, provider={}", key, config.provider);
    let storage = create_storage(&config).await?;
    let exists = storage.exists(&key).await?;
    tracing::info!("[CloudStorage] 检查文件是否存在: key={}, exists={}", key, exists);
    Ok(exists)
}

// ============== Sync Manager Commands ==============

/// 获取同步状态
#[tauri::command]
pub async fn cloud_sync_get_status(config: CloudStorageConfig) -> Result<SyncStatus> {
    tracing::info!("[CloudSync] 查询同步状态: provider={}", config.provider);
    let storage = create_storage(&config).await?;
    let manager = CloudSyncManager::new(storage, get_device_id());
    let status = manager.get_status().await;
    tracing::info!(
        "[CloudSync] 查询同步状态完成: connected={}, version_count={}",
        status.connected,
        status.cloud_version_count
    );
    Ok(status)
}

/// 列出云端所有备份版本
#[tauri::command]
pub async fn cloud_sync_list_versions(config: CloudStorageConfig) -> Result<Vec<BackupVersion>> {
    tracing::info!("[CloudSync] 列出云端版本: provider={}", config.provider);
    let storage = create_storage(&config).await?;
    let manager = CloudSyncManager::new(storage, get_device_id());
    let versions = manager.list_versions().await?;
    tracing::info!("[CloudSync] 列出云端版本完成: count={}", versions.len());
    Ok(versions)
}

/// 上传备份到云端（带实时进度事件）
///
/// 通过 `cloud-sync-progress` Tauri 事件向前端推送字节级传输进度。
#[tauri::command]
pub async fn cloud_sync_upload(
    app_handle: AppHandle,
    config: CloudStorageConfig,
    zip_path: String,
    app_version: Option<String>,
    note: Option<String>,
) -> Result<UploadResult> {
    let file_size = std::fs::metadata(&zip_path)
        .map(|m| m.len())
        .unwrap_or(0);
    let encrypted = config
        .encryption_password
        .as_deref()
        .filter(|s| !s.is_empty())
        .is_some();
    tracing::info!(
        "[CloudSync] 开始上传备份: zip_path={}, size={}, encrypted={}, provider={}",
        zip_path,
        file_size,
        encrypted,
        config.provider
    );

    // 如果配置了加密密码，先把 ZIP 加密到临时文件再上传
    // 临时文件在 ZIP 附近创建，上传成功后删除
    let mut encrypted_temp: Option<std::path::PathBuf> = None;
    let actual_upload_path: std::path::PathBuf = if let Some(pwd) = config
        .encryption_password
        .as_deref()
        .filter(|s| !s.is_empty())
    {
        tracing::info!("[CloudSync] 端到端加密已启用，加密上传...");
        let plaintext = std::fs::read(&zip_path)
            .map_err(|e| AppError::file_system(format!("读取 ZIP 失败: {}", e)))?;
        let ciphertext = crate::crypto::backup_crypto::encrypt_backup(&plaintext, pwd)
            .map_err(|e| AppError::internal(format!("加密备份失败: {}", e)))?;
        // 写临时加密文件（同目录，确保同一文件系统 → 快）
        let original = std::path::Path::new(&zip_path);
        let temp_path = original.with_extension("zip.dsbk");
        std::fs::write(&temp_path, &ciphertext)
            .map_err(|e| AppError::file_system(format!("写入加密文件失败: {}", e)))?;
        encrypted_temp = Some(temp_path.clone());
        temp_path
    } else {
        std::path::Path::new(&zip_path).to_path_buf()
    };

    let file_size = std::fs::metadata(&actual_upload_path)
        .map(|m| m.len())
        .unwrap_or(0);

    let storage = create_storage(&config).await?;
    let manager = CloudSyncManager::new(storage, get_device_id());

    emit_sync_progress(
        &app_handle,
        CloudSyncProgressEvent {
            operation: "upload",
            stage: "transferring",
            stage_label: "正在上传文件...",
            bytes_done: 0,
            bytes_total: file_size,
            percent: 0.0,
        },
    );

    let handle = app_handle.clone();
    let progress_cb: traits::UploadProgressCallback = Box::new(move |done, total| {
        let pct = if total > 0 {
            (done as f32 / total as f32 * 95.0).min(95.0)
        } else {
            0.0
        };
        emit_sync_progress(
            &handle,
            CloudSyncProgressEvent {
                operation: "upload",
                stage: "transferring",
                stage_label: "正在上传文件...",
                bytes_done: done,
                bytes_total: total,
                percent: pct,
            },
        );
    });

    let upload_result = manager
        .upload_with_progress(&actual_upload_path, app_version, note, Some(progress_cb))
        .await;

    // 无论成功失败都清理临时加密文件
    if let Some(temp) = encrypted_temp {
        let _ = std::fs::remove_file(&temp);
    }

    let result = upload_result?;

    emit_sync_progress(
        &app_handle,
        CloudSyncProgressEvent {
            operation: "upload",
            stage: "done",
            stage_label: "上传完成",
            bytes_done: file_size,
            bytes_total: file_size,
            percent: 100.0,
        },
    );

    tracing::info!(
        "[CloudSync] 上传备份完成: version={}, size={}, checksum={}",
        result.version.id,
        result.version.size,
        &result.version.checksum[..32.min(result.version.checksum.len())]
    );
    Ok(result)
}

/// 从云端下载备份（带实时进度事件）
///
/// 通过 `cloud-sync-progress` Tauri 事件向前端推送字节级下载进度。
#[tauri::command]
pub async fn cloud_sync_download(
    app_handle: AppHandle,
    config: CloudStorageConfig,
    version_id: Option<String>,
    local_dir: String,
) -> Result<DownloadResult> {
    tracing::info!(
        "[CloudSync] 开始下载备份: version={:?}, local_dir={}, provider={}",
        version_id,
        local_dir,
        config.provider
    );
    let storage = create_storage(&config).await?;
    let manager = CloudSyncManager::new(storage, get_device_id());

    emit_sync_progress(
        &app_handle,
        CloudSyncProgressEvent {
            operation: "download",
            stage: "transferring",
            stage_label: "正在下载备份...",
            bytes_done: 0,
            bytes_total: 0,
            percent: 0.0,
        },
    );

    let handle = app_handle.clone();
    let progress_cb: traits::DownloadProgressCallback = Box::new(move |done, total| {
        let pct = if total > 0 {
            (done as f32 / total as f32 * 95.0).min(95.0)
        } else {
            0.0
        };
        emit_sync_progress(
            &handle,
            CloudSyncProgressEvent {
                operation: "download",
                stage: "transferring",
                stage_label: "正在下载备份...",
                bytes_done: done,
                bytes_total: total,
                percent: pct,
            },
        );
    });

    let result = manager
        .download_with_progress(
            version_id.as_deref(),
            std::path::Path::new(&local_dir),
            Some(progress_cb),
        )
        .await?;

    // 如果文件被加密（DSBK 魔数）则解密；未加密则原样保留
    // 支持"用户上传时加密，下载设备未配置密码"的场景：返回明确错误
    let downloaded_path = std::path::Path::new(&result.local_path);
    let head = {
        let mut buf = vec![0u8; 4];
        if let Ok(mut f) = std::fs::File::open(downloaded_path) {
            use std::io::Read;
            let _ = f.read(&mut buf);
        }
        buf
    };
    let is_encrypted = crate::crypto::backup_crypto::is_encrypted_backup(&head);
    if is_encrypted {
        let pwd = config
            .encryption_password
            .as_deref()
            .filter(|s| !s.is_empty());
        let pwd = pwd.ok_or_else(|| {
            AppError::configuration(
                "云端备份已加密，但未提供解密密码。请在云存储配置里填写相同的加密密码后重试。"
                    .to_string(),
            )
        })?;
        tracing::info!("[CloudSync] 检测到加密备份，开始解密...");
        let ciphertext = std::fs::read(downloaded_path)
            .map_err(|e| AppError::file_system(format!("读取加密备份失败: {}", e)))?;
        let plaintext =
            crate::crypto::backup_crypto::decrypt_backup(&ciphertext, pwd).map_err(|e| {
                AppError::validation(format!("解密备份失败（密码错或数据损坏）: {}", e))
            })?;
        std::fs::write(downloaded_path, &plaintext)
            .map_err(|e| AppError::file_system(format!("写入解密后 ZIP 失败: {}", e)))?;
    }

    emit_sync_progress(
        &app_handle,
        CloudSyncProgressEvent {
            operation: "download",
            stage: "done",
            stage_label: "下载完成",
            bytes_done: result.version.size,
            bytes_total: result.version.size,
            percent: 100.0,
        },
    );

    tracing::info!(
        "[CloudSync] 下载备份完成: version={}, local_path={}, size={}",
        result.version.id,
        result.local_path,
        result.version.size
    );
    Ok(result)
}

/// 删除云端备份版本
#[tauri::command]
pub async fn cloud_sync_delete_version(
    config: CloudStorageConfig,
    version_id: String,
) -> Result<()> {
    tracing::info!(
        "[CloudSync] 删除云端版本: version={}, provider={}",
        version_id,
        config.provider
    );
    let storage = create_storage(&config).await?;
    let manager = CloudSyncManager::new(storage, get_device_id());
    manager.delete_version(&version_id).await?;
    tracing::info!("[CloudSync] 删除云端版本成功: version={}", version_id);
    Ok(())
}

/// 获取设备 ID
#[tauri::command]
pub fn cloud_sync_get_device_id() -> String {
    get_device_id()
}

/// 检查 S3 feature 是否启用
#[tauri::command]
pub fn cloud_storage_is_s3_enabled() -> bool {
    cfg!(feature = "cloud_storage_s3")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = CloudStorageConfig {
            provider: StorageProvider::WebDav,
            webdav: Some(WebDavConfig {
                endpoint: "https://dav.example.com".into(),
                username: "user".into(),
                password: "pass".into(),
            }),
            ..Default::default()
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_provider_display() {
        assert_eq!(format!("{}", StorageProvider::WebDav), "WebDAV");
        assert_eq!(format!("{}", StorageProvider::S3), "S3");
    }
}
