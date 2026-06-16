//! # 自动云同步服务
//!
//! 在后台自动触发云同步，无需用户手动操作。
//!
//! 触发时机：
//! 1. App 启动后延迟 30 秒，执行下载优先同步（拉取其他设备的变更）
//! 2. 窗口失焦时执行上传优先同步（推送本地变更）
//! 3. 周期性兜底：用户可配置间隔（关/15分钟/30分钟/1小时）
//!
//! 使用快照上传机制，同步期间不阻塞用户操作。

use crate::data_governance::commands_sync::{
    data_governance_run_sync_internal, SyncExecutionResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter, Manager};
use tokio::time::{sleep, Duration};
use tracing::{debug, info, warn};

/// 自动同步配置存储键
const AUTO_SYNC_CONFIG_KEY: &str = "sync.auto_sync_config";

/// 上次自动同步时间存储键
const LAST_AUTO_SYNC_KEY: &str = "sync.last_auto_sync_time";

/// 防止自动同步重入的标志
static AUTO_SYNC_RUNNING: AtomicBool = AtomicBool::new(false);

/// 自动同步配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AutoSyncConfig {
    /// 是否启用自动同步
    #[serde(default)]
    pub enabled: bool,

    /// 同步间隔（分钟），0 表示关闭
    /// 可选值: 0(关), 15, 30, 60
    #[serde(default)]
    pub interval_minutes: u32,

    /// 启动时自动下载
    #[serde(default = "default_true")]
    pub sync_on_startup: bool,

    /// 窗口失焦时自动上传
    #[serde(default = "default_true")]
    pub sync_on_blur: bool,
}

fn default_true() -> bool {
    true
}

impl AutoSyncConfig {
    /// 从数据库加载自动同步配置
    pub fn load(database: &crate::database::Database) -> Result<Self, String> {
        match database.get_setting(AUTO_SYNC_CONFIG_KEY).map_err(|e| e.to_string())? {
            Some(json_str) => {
                let config: AutoSyncConfig = serde_json::from_str(&json_str)
                    .map_err(|e| format!("解析自动同步配置失败: {}", e))?;
                Ok(config)
            }
            None => Ok(Self::default()),
        }
    }

    /// 保存自动同步配置到数据库
    pub fn save(&self, database: &crate::database::Database) -> Result<(), String> {
        let json_str = serde_json::to_string(self)
            .map_err(|e| format!("序列化自动同步配置失败: {}", e))?;
        database
            .save_setting(AUTO_SYNC_CONFIG_KEY, &json_str)
            .map_err(|e| format!("保存自动同步配置失败: {}", e))?;
        Ok(())
    }
}

/// 获取应用数据库（主库）
fn get_app_database(app: &AppHandle) -> Option<std::sync::Arc<crate::database::Database>> {
    app.try_state::<crate::commands::AppState>()
        .map(|s| s.database.clone())
}

/// 加载云存储配置（带凭据补全）
async fn load_cloud_config(app: &AppHandle) -> Option<crate::cloud_storage::CloudStorageConfig> {
    let database = get_app_database(app)?;

    // 先尝试新 key，再尝试旧 key
    let config_json = database.get_setting("cloud_storage.config").ok().flatten()
        .or_else(|| database.get_setting("cloud_storage_config").ok().flatten())?;

    let mut config: crate::cloud_storage::CloudStorageConfig =
        serde_json::from_str(&config_json).ok()?;

    // 补全安全存储中的凭据
    crate::secure_store::hydrate_cloud_config(app, &mut config);
    Some(config)
}

/// 执行一次自动同步（内部函数）
async fn perform_auto_sync(app: &AppHandle, direction: &str) -> Result<(), String> {
    // 防止重入
    if AUTO_SYNC_RUNNING
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        debug!("[AutoSync] 上一次同步仍在运行，跳过本次");
        return Ok(());
    }

    let _guard = AutoSyncRunningGuard;

    info!("[AutoSync] 开始自动同步: direction={}", direction);

    // 通知前端同步开始
    let _ = app.emit("auto-sync-started", direction);

    let config = load_cloud_config(app)
        .await
        .ok_or("未配置云存储，跳过自动同步")?;

    let result =
        data_governance_run_sync_internal(app.clone(), direction.to_string(), Some(config), None)
            .await;

    // 通知前端同步结果
    match &result {
        Ok(resp) => {
            info!(
                "[AutoSync] 自动同步完成: success={}, uploaded={}, downloaded={}",
                resp.success, resp.changes_uploaded, resp.changes_downloaded
            );
            let _ = app.emit("auto-sync-completed", &resp);
        }
        Err(e) => {
            warn!("[AutoSync] 自动同步失败: {}", e);
            let _ = app.emit("auto-sync-error", e);
        }
    }

    // 记录上次同步时间
    if let Some(db) = get_app_database(app) {
        let now = chrono::Utc::now().to_rfc3339();
        let _ = db.save_setting(LAST_AUTO_SYNC_KEY, &now);
    }

    result.map(|_| ())
}

struct AutoSyncRunningGuard;

impl Drop for AutoSyncRunningGuard {
    fn drop(&mut self) {
        AUTO_SYNC_RUNNING.store(false, Ordering::SeqCst);
    }
}

/// 启动自动同步调度器
/// 在应用初始化完成后调用
pub async fn start_auto_sync_scheduler(app: AppHandle) {
    info!("[AutoSync] 自动同步调度器已启动");

    // 首次延迟 30 秒，避免与应用启动争用资源
    sleep(Duration::from_secs(30)).await;

    // 启动时下载同步
    if let Some(db) = get_app_database(&app) {
        match AutoSyncConfig::load(&db) {
            Ok(config) => {
                if config.enabled && config.sync_on_startup {
                    info!("[AutoSync] 执行启动时下载同步");
                    let _ = perform_auto_sync(&app, "download").await;
                }
            }
            Err(e) => {
                warn!("[AutoSync] 加载自动同步配置失败: {}", e);
            }
        }
    }

    // 周期性检查
    loop {
        sleep(Duration::from_secs(60)).await; // 每分钟检查一次

        let Some(db) = get_app_database(&app) else {
            continue;
        };
        let Ok(config) = AutoSyncConfig::load(&db) else {
            continue;
        };

        if !config.enabled || config.interval_minutes == 0 {
            continue;
        }

        // 检查是否到了同步间隔
        let last_sync = get_last_auto_sync_time(&db).ok().flatten();
        let now = chrono::Utc::now();
        let should_sync = match last_sync {
            Some(last_time) => {
                let elapsed = now.signed_duration_since(last_time);
                elapsed.num_minutes() >= config.interval_minutes as i64
            }
            None => true,
        };

        if should_sync {
            // 周期性同步使用双向
            let _ = perform_auto_sync(&app, "bidirectional").await;
        }
    }
}

/// 获取上次自动同步时间
fn get_last_auto_sync_time(
    database: &crate::database::Database,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, String> {
    use chrono::DateTime;
    match database.get_setting(LAST_AUTO_SYNC_KEY).map_err(|e| e.to_string())? {
        Some(time_str) => match DateTime::parse_from_rfc3339(&time_str) {
            Ok(dt) => Ok(Some(dt.with_timezone(&chrono::Utc))),
            Err(e) => {
                warn!("[AutoSync] 解析上次同步时间失败: {}", e);
                Ok(None)
            }
        },
        None => Ok(None),
    }
}

// ============================================================================
// Tauri 命令
// ============================================================================

use crate::commands::AppState;
use tauri::State;

/// 获取自动同步配置
#[tauri::command]
pub async fn get_auto_sync_config(state: State<'_, AppState>) -> Result<AutoSyncConfig, String> {
    AutoSyncConfig::load(&state.database)
}

/// 保存自动同步配置
#[tauri::command]
pub async fn set_auto_sync_config(
    config: AutoSyncConfig,
    state: State<'_, AppState>,
) -> Result<(), String> {
    config.save(&state.database)?;
    info!(
        "[AutoSync] 配置已更新: enabled={}, interval={}min, startup={}, blur={}",
        config.enabled,
        config.interval_minutes,
        config.sync_on_startup,
        config.sync_on_blur
    );
    Ok(())
}

/// 手动触发一次自动同步（供前端调用）
#[tauri::command]
pub async fn trigger_auto_sync(
    app: AppHandle,
    direction: String,
) -> Result<SyncExecutionResponse, String> {
    info!("[AutoSync] 手动触发自动同步: direction={}", direction);

    let config = load_cloud_config(&app)
        .await
        .ok_or("未配置云存储")?;

    data_governance_run_sync_internal(app, direction, Some(config), Some("keep_latest".to_string()))
        .await
}

/// 窗口失焦时触发上传同步
#[tauri::command]
pub async fn on_window_blur_sync(app: AppHandle) -> Result<(), String> {
    let Some(db) = get_app_database(&app) else {
        return Ok(());
    };
    let config = AutoSyncConfig::load(&db)?;
    if !config.enabled || !config.sync_on_blur {
        return Ok(());
    }

    info!("[AutoSync] 窗口失焦触发上传同步");
    // 使用 spawn 在后台执行，不阻塞前端
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        let _ = perform_auto_sync(&app_clone, "upload").await;
    });
    Ok(())
}
