/**
 * 同步指示器徽章
 *
 * 轻量级组件，定期轮询后端 `count_record_conflicts` 并把未解决冲突的总数
 * 以醒目的方式展示出来。放在同步 Tab 概览区，用户不需要展开冲突面板也能
 * 立刻看到"有 N 条冲突待处理"。
 *
 * 为什么要单独做：
 * - __sync_conflicts 表由后端 `apply_downloaded_changes_with_conflict_guard`
 *   自动写入，前端不会主动知道新增。
 * - 轮询间隔保守（30s），在多数"什么都没发生"的时间窗里几乎没开销。
 * - 出错静默，不打扰用户；唯一会呈现的状态是"有冲突"。
 *
 * 2026-06 新增：非阻塞式自动同步进度指示
 * - 监听 Tauri 事件 'auto-sync-started', 'auto-sync-completed', 'auto-sync-error'
 * - 显示轻量进度条/toast，用户可 dismiss，同步在后台继续进行
 */

import React, { useEffect, useState, useCallback } from 'react';
import { Warning, CheckCircle, X } from '@phosphor-icons/react';
import { useTranslation } from 'react-i18next';
import * as DataGovernanceApi from '@/api/dataGovernance';
import { listen } from '@tauri-apps/api/event';
import { cn } from '@/lib/utils';

const POLL_INTERVAL_MS = 30_000;

export const SyncIndicator: React.FC<{ compact?: boolean; refreshSignal?: string | number }> = ({
  compact = false,
  refreshSignal,
}) => {
  const [counts, setCounts] = useState<Record<string, number> | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;

    const tick = async () => {
      try {
        const result = await DataGovernanceApi.countRecordConflicts();
        if (!cancelled) {
          setCounts(result);
        }
      } catch {
        // 静默：后端命令缺失或数据库未就绪时不骚扰用户
      } finally {
        if (!cancelled) setLoading(false);
      }
    };

    void tick();
    const timer = setInterval(() => void tick(), POLL_INTERVAL_MS);
    return () => {
      cancelled = true;
      clearInterval(timer);
    };
  }, [refreshSignal]);

  if (loading || !counts) return null;

  const total = Object.values(counts).reduce((a, b) => a + b, 0);

  if (total === 0) {
    if (compact) return null;
    return (
      <span className="inline-flex items-center gap-1 text-xs text-emerald-600 dark:text-emerald-400">
        <CheckCircle size={12} />
        无冲突
      </span>
    );
  }

  const perDb = Object.entries(counts)
    .filter(([, n]) => n > 0)
    .map(([db, n]) => `${db}: ${n}`)
    .join(', ');

  return (
    <span
      className="inline-flex items-center gap-1 rounded-full bg-amber-500/15 px-2 py-0.5 text-xs font-medium text-amber-700 dark:text-amber-300 ring-1 ring-inset ring-amber-500/30"
      title={`未解决冲突：${perDb}`}
    >
      <Warning size={12} />
      {total} 条冲突
    </span>
  );
};

/**
 * 非阻塞式自动同步进度指示器
 *
 * 监听自动同步 Tauri 事件，以轻量 toast 形式展示，用户可手动关闭。
 * 关闭不影响后台同步继续进行。
 */
export const AutoSyncIndicator: React.FC = () => {
  const { t } = useTranslation(['data']);
  const [status, setStatus] = useState<'idle' | 'running' | 'completed' | 'error'>('idle');
  const [message, setMessage] = useState('');
  const [dismissed, setDismissed] = useState(false);

  const handleDismiss = useCallback(() => {
    setDismissed(true);
  }, []);

  useEffect(() => {
    let unlisteners: Array<() => void> = [];

    const setup = async () => {
      try {
        const unlistenStarted = await listen('auto-sync-started', () => {
          setDismissed(false);
          setStatus('running');
          setMessage(t('data:governance.auto_sync_running'));
        });
        unlisteners.push(unlistenStarted);

        const unlistenCompleted = await listen('auto-sync-completed', () => {
          setDismissed(false);
          setStatus('completed');
          setMessage(t('data:governance.auto_sync_completed'));
          // 完成后 5 秒自动消失
          setTimeout(() => {
            setStatus('idle');
          }, 5000);
        });
        unlisteners.push(unlistenCompleted);

        const unlistenError = await listen<{ error?: string }>('auto-sync-error', (event) => {
          setDismissed(false);
          setStatus('error');
          setMessage(`${t('data:governance.auto_sync_error')}: ${event.payload?.error ?? ''}`);
        });
        unlisteners.push(unlistenError);
      } catch {
        // Tauri 事件监听注册失败，静默忽略（非 Tauri 环境下）
      }
    };

    void setup();
    return () => {
      unlisteners.forEach((unlisten) => unlisten());
    };
  }, [t]);

  if (status === 'idle' || dismissed) return null;

  const statusStyles: Record<string, string> = {
    running: 'bg-blue-500/10 border-blue-500/30 text-blue-700 dark:text-blue-300',
    completed: 'bg-emerald-500/10 border-emerald-500/30 text-emerald-700 dark:text-emerald-300',
    error: 'bg-red-500/10 border-red-500/30 text-red-700 dark:text-red-300',
  };

  const statusIcons: Record<string, React.ReactNode> = {
    running: <span className="inline-block w-2 h-2 rounded-full bg-blue-500 animate-pulse" />,
    completed: <CheckCircle size={14} className="text-emerald-500" />,
    error: <Warning size={14} className="text-red-500" />,
  };

  return (
    <div
      className={cn(
        'fixed bottom-4 right-4 z-[9999] flex items-center gap-2 rounded-lg border px-3 py-2 text-xs shadow-lg backdrop-blur-sm',
        statusStyles[status]
      )}
      role="status"
      aria-live="polite"
    >
      {statusIcons[status]}
      <span className="flex-1">{message}</span>
      <button
        type="button"
        onClick={handleDismiss}
        className="ml-1 opacity-60 hover:opacity-100 transition-opacity"
        aria-label="Dismiss"
      >
        <X size={14} />
      </button>
      {status === 'running' && (
        <div className="absolute bottom-0 left-0 right-0 h-0.5 rounded-b-lg overflow-hidden">
          <div className="h-full bg-blue-500/60 animate-pulse w-2/3" />
        </div>
      )}
    </div>
  );
};

export default SyncIndicator;
