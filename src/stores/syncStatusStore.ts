/**
 * 全局云同步运行状态
 *
 * 云同步有多个 UI 入口（设置页 SyncSettingsSection、数据治理面板 SyncTab 等），
 * 此前各入口用组件内 useState 各自维护 isSyncing，彼此不可见，用户可以在
 * 两个页面同时触发同步。后端已用 try_acquire 全局锁兜底（第二个请求立即
 * 失败），本 store 让所有入口共享同一份"正在同步"状态：
 * - 同步进行中时所有入口的按钮统一禁用；
 * - 重复触发在前端即被拦截，无需等后端报错。
 *
 * 自动同步配置与状态也在此 store 集中管理，便于生命周期绑定。
 */
import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface GlobalSyncState {
  /** 是否有同步正在进行（任意入口触发的都算） */
  isSyncing: boolean;
  /** 触发当前同步的入口标识（用于调试与提示） */
  source: string | null;
  /**
   * 尝试开始一次同步。
   * @returns true 表示成功占用；false 表示已有同步在进行，调用方应放弃本次触发
   */
  beginSync: (source: string) => boolean;
  /** 同步结束（无论成功失败）时调用，释放占用 */
  endSync: () => void;

  // ─── 自动同步配置 ────────────────────────────────────────
  /** 是否启用自动同步 */
  isAutoSyncEnabled: boolean;
  /** 自动同步间隔（毫秒），0 表示关闭 */
  autoSyncInterval: number;
  /** 启动时是否自动同步 */
  syncOnStartup: boolean;
  /** 窗口失焦时是否自动同步 */
  syncOnBlur: boolean;

  /** 加载自动同步配置 */
  loadAutoSyncConfig: () => Promise<void>;
  /** 保存自动同步配置 */
  saveAutoSyncConfig: (config: {
    isAutoSyncEnabled: boolean;
    autoSyncInterval: number;
    syncOnStartup: boolean;
    syncOnBlur: boolean;
  }) => Promise<void>;
  /** 触发自动同步 */
  triggerAutoSync: (direction: 'upload' | 'download' | 'bidirectional') => Promise<unknown>;
}

export const useGlobalSyncStore = create<GlobalSyncState>((set, get) => ({
  isSyncing: false,
  source: null,
  beginSync: (source) => {
    if (get().isSyncing) {
      return false;
    }
    set({ isSyncing: true, source });
    return true;
  },
  endSync: () => set({ isSyncing: false, source: null }),

  // 自动同步配置默认值
  isAutoSyncEnabled: false,
  autoSyncInterval: 0,
  syncOnStartup: true,
  syncOnBlur: true,

  loadAutoSyncConfig: async () => {
    try {
      const config = await invoke<{
        is_auto_sync_enabled: boolean;
        auto_sync_interval: number;
        sync_on_startup: boolean;
        sync_on_blur: boolean;
      }>('get_auto_sync_config');
      set({
        isAutoSyncEnabled: config.is_auto_sync_enabled,
        autoSyncInterval: config.auto_sync_interval,
        syncOnStartup: config.sync_on_startup,
        syncOnBlur: config.sync_on_blur,
      });
    } catch (err) {
      console.warn('[syncStatusStore] loadAutoSyncConfig failed:', err);
    }
  },

  saveAutoSyncConfig: async (config) => {
    try {
      await invoke('set_auto_sync_config', {
        config: {
          is_auto_sync_enabled: config.isAutoSyncEnabled,
          auto_sync_interval: config.autoSyncInterval,
          sync_on_startup: config.syncOnStartup,
          sync_on_blur: config.syncOnBlur,
        },
      });
      set({
        isAutoSyncEnabled: config.isAutoSyncEnabled,
        autoSyncInterval: config.autoSyncInterval,
        syncOnStartup: config.syncOnStartup,
        syncOnBlur: config.syncOnBlur,
      });
    } catch (err) {
      console.warn('[syncStatusStore] saveAutoSyncConfig failed:', err);
      throw err;
    }
  },

  triggerAutoSync: async (direction) => {
    try {
      const result = await invoke('trigger_auto_sync', { direction });
      return result;
    } catch (err) {
      console.warn('[syncStatusStore] triggerAutoSync failed:', err);
      throw err;
    }
  },
}));
