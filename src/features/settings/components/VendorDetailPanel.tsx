/**
 * VendorDetailPanel - 供应商详情面板
 * 从 ApisTab 拆分，负责渲染选中供应商的配置和模型列表
 */

import React, { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { ArrowSquareOut, CaretDown, CaretUp, Check, DownloadSimple, LinkSimple, PencilSimple, Plus, Pulse, Spinner, Star, Trash, X } from '@phosphor-icons/react';
import { NotionButton } from '@/components/ui/NotionButton';
import { Badge } from '@/components/ui/shad/Badge';
import { Switch } from '@/components/ui/shad/Switch';
import { Sheet, SheetContent, SheetHeader, SheetTitle, SheetDescription } from '@/components/ui/shad/Sheet';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from '@/components/ui/shad/Dialog';
import { CustomScrollArea } from '@/components/custom-scroll-area';
import { Z_INDEX } from '@/config/zIndex';
import { cn } from '@/lib/utils';
import { showGlobalNotification } from '@/components/UnifiedNotification';
import { ProviderIcon } from '@/components/ui/ProviderIcon';
import { openUrl } from '@/utils/urlOpener';
import { SiliconFlowLogo } from '@/components/ui/SiliconFlowLogo';
import { ModelCapabilityIcons } from '@/components/shared/ModelCapabilityIcons';
import {
  settingsQuietActiveSurfaceClassName,
  settingsQuietInteractiveRowClassName,
  settingsQuietRowBaseClassName,
} from './SettingsCommon';
import { SiliconFlowSection } from './SiliconFlowSection';
import { VendorModelFetcher, supportsModelFetching } from './VendorModelFetcher';
import { ShadApiEditModal } from './ShadApiEditModal';
import { useVendorSettings } from './VendorSettingsContext';
import { convertProfileToApiConfig } from './modelConverters';
import { groupByModelFamily } from './modelFamily';
import type { VendorConfig } from '@/types';

// --- Helpers ---

type TranslateFn = (key: string, options?: { defaultValue?: string }) => string;

const getProviderDisplayName = (providerType?: string | null, t?: TranslateFn) => {
  if (!providerType) return 'OpenAI';
  const normalizedProviderType = providerType.toLowerCase();
  const map: Record<string, string> = {
    openai: 'OpenAI',
    anthropic: 'Anthropic',
    google: 'Google',
    siliconflow: 'SiliconFlow',
    deepseek: 'DeepSeek',
    ollama: 'Ollama',
    nvidia: 'NVIDIA',
    mimo: 'Xiaomi MiMo',
  };
  const fallback = map[normalizedProviderType] || providerType;
  return t?.(`settings:vendor_modal.providers.${normalizedProviderType}`, { defaultValue: fallback }) ?? fallback;
};

const getVendorDisplayName = (vendor: VendorConfig, providerLabel: string) => {
  if ((vendor.providerType ?? '').toLowerCase() === 'siliconflow') {
    return providerLabel;
  }
  return vendor.name || providerLabel;
};

const getProviderWebsiteUrl = (providerType?: string | null): string | null => {
  if (!providerType) return null;
  const map: Record<string, string> = {
    siliconflow: 'https://cloud.siliconflow.cn/i/deadXN1B',
    deepseek: 'https://deepseek.com',
    qwen: 'https://bailian.console.aliyun.com',
    zhipu: 'https://open.bigmodel.cn',
    doubao: 'https://www.volcengine.com/product/doubao',
    minimax: 'https://platform.minimaxi.com',
    moonshot: 'https://platform.moonshot.cn',
    openai: 'https://platform.openai.com',
    gemini: 'https://aistudio.google.com',
    anthropic: 'https://console.anthropic.com',
    google: 'https://aistudio.google.com',
    nvidia: 'https://build.nvidia.com/nim',
    mimo: 'https://platform.xiaomimimo.com',
  };
  return map[providerType.toLowerCase()] || null;
};

// --- Component ---

export const VendorDetailPanel: React.FC = () => {
  const { t } = useTranslation(['settings', 'common']);
  const {
    selectedVendor,
    selectedVendorModels,
    selectedVendorIsSiliconflow,
    vendorBusy,
    testingApi,
    handleOpenVendorModal,
    handleDeleteVendor,
    handleOpenModelEditor,
    inlineEditState,
    setInlineEditState,
    handleSaveInlineEdit,
    isAddingNewModel,
    handleAddModelInline,
    handleCancelAddModel,
    handleToggleModelProfile,
    handleDeleteModelProfile,
    handleToggleFavorite,
    testApiConnection,
    handleSiliconFlowConfig,
    handleBatchCreateConfigs,
    handleBatchConfigsCreated,
    onAddVendorModels,
  } = useVendorSettings();

  const [collapsedFamilies, setCollapsedFamilies] = useState<Set<string>>(new Set());
  const [isModelFetcherDialogOpen, setIsModelFetcherDialogOpen] = useState(false);

  // 模型按家族分组（GPT-4 / Claude Opus / Gemini 2.5 …）
  const familyGroups = useMemo(
    () => groupByModelFamily(selectedVendorModels, ({ api }) => api.model),
    [selectedVendorModels],
  );
  // 仅当存在 2 个及以上家族时才分组渲染；否则维持扁平避免噪声
  const shouldGroupByFamily = familyGroups.length >= 2;

  // 切换供应商时折叠状态归零（默认全展开）
  useEffect(() => {
    setCollapsedFamilies(new Set());
  }, [selectedVendor?.id]);

  if (!selectedVendor) {
    return (
      <div className="rounded-2xl border border-dashed border-border/60 p-10 text-center text-muted-foreground">
        {t('settings:vendor_panel.create_vendor_cta')}
      </div>
    );
  }

  const providerLabel = getProviderDisplayName(selectedVendor.providerType, t);
  const vendorDisplayName = getVendorDisplayName(selectedVendor, providerLabel);

  const renderModelCard = ({ profile, api }: (typeof selectedVendorModels)[number]) => {
    const isEditing = inlineEditState?.profileId === profile.id;

    const handleEditClick = () => {
      if (isEditing) {
        setInlineEditState(null);
      } else {
        if (isAddingNewModel) handleCancelAddModel();
        const editApi = convertProfileToApiConfig(profile, selectedVendor);
        setInlineEditState({ profileId: profile.id, api: editApi });
      }
    };

    const isReadOnly = !!(api.isBuiltin && api.isReadOnly);

    return (
      <div key={profile.id} className={cn(
        "group/card relative border border-transparent",
        isEditing
          ? cn(settingsQuietRowBaseClassName, settingsQuietActiveSurfaceClassName)
          : settingsQuietInteractiveRowClassName
      )}>
        {/* 卡片头部 */}
        <div className="p-3">
          <div className="flex items-center gap-3">
            <ProviderIcon modelId={api.model} size={20} showTooltip={false} />
            <div className="flex-1 min-w-0 space-y-0.5">
              <div className="flex items-center gap-2">
                <span className="text-sm font-medium text-foreground truncate">{profile.label || api.name}</span>
                {!profile.enabled && <span className="text-[10px] px-1.5 py-0.5 rounded-full bg-muted text-muted-foreground whitespace-nowrap shrink-0">{t('settings:status.disabled')}</span>}
                {isReadOnly && <span className="text-[10px] px-1.5 py-0.5 rounded-full bg-primary/10 text-primary whitespace-nowrap shrink-0">{t('settings:api_config.badge_builtin_free')}</span>}
              </div>
              <div className="flex items-center gap-1.5">
                <span className="font-mono text-xs text-muted-foreground truncate">{api.model}</span>
                <ModelCapabilityIcons
                  isMultimodal={profile.isMultimodal}
                  isReasoning={profile.isReasoning}
                  isEmbedding={profile.isEmbedding}
                  isReranker={profile.isReranker}
                  supportsTools={profile.supportsTools}
                  size="xs"
                />
              </div>
            </div>

            {/* 操作区域：次要操作 + 编辑 + 开关（开关在最右） */}
            <div className="flex items-center gap-1.5 shrink-0">
              {/* 次要操作：hover 时显示 */}
              <div className="flex items-center gap-0.5 opacity-0 group-hover/card:opacity-100 transition-opacity duration-150">
                <NotionButton
                  size="sm"
                  variant="ghost"
                  iconOnly
                  className={cn(profile.isFavorite && "text-yellow-500 opacity-100")}
                  onClick={() => handleToggleFavorite(profile)}
                  disabled={vendorBusy}
                  title={t('settings:api_config.toggle_favorite')}
                >
                  <Star className="h-3.5 w-3.5" weight={profile.isFavorite ? 'fill' : 'regular'} />
                </NotionButton>
                <NotionButton
                  size="sm"
                  variant="ghost"
                  iconOnly
                  onClick={() => void testApiConnection(api)}
                  disabled={testingApi === api.id || vendorBusy}
                  title={t('settings:api_config.test_button')}
                >
                  {testingApi === api.id ? <Spinner className="h-3.5 w-3.5 animate-spin" /> : <Pulse className="h-3.5 w-3.5" />}
                </NotionButton>

                {/* 删除：触发全局确认对话框 */}
                {!isReadOnly ? (
                  <NotionButton
                    size="sm"
                    variant="ghost"
                    iconOnly
                    disabled={vendorBusy}
                    title={t('common:actions.delete')}
                    className="text-muted-foreground hover:text-destructive"
                    onClick={() => handleDeleteModelProfile(profile)}
                  >
                    <Trash className="h-3.5 w-3.5" />
                  </NotionButton>
                ) : (
                  /* 占位：保持对齐 */
                  <div className="h-7 w-7 shrink-0" />
                )}
              </div>
              {/* 编辑按钮 */}
              <NotionButton
                size="sm"
                variant={isEditing ? "default" : "ghost"}
                iconOnly
                onClick={handleEditClick}
                disabled={vendorBusy}
                title={t('common:actions.edit')}
              >
                <PencilSimple className="h-3.5 w-3.5" />
              </NotionButton>
              {/* 开关：最右 */}
              <Switch
                checked={profile.enabled}
                onCheckedChange={value => handleToggleModelProfile(profile, value)}
                disabled={isReadOnly || vendorBusy}
              />
            </div>
          </div>
        </div>
      </div>
    );
  };

  return (
    <>
      {/* 供应商头部 */}
      <div className="w-full">
        <div className="mb-5 flex flex-col gap-2">
          <div className="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
            <div className="flex items-center gap-2 min-w-0">
              {selectedVendorIsSiliconflow && <SiliconFlowLogo className="h-5" />}
              <h3 className="text-lg font-medium text-foreground truncate">
                {vendorDisplayName}
              </h3>
              {selectedVendorIsSiliconflow && (
                <Badge variant="default" className="bg-primary/10 text-primary border-primary/20 text-[10px] px-1.5 py-0 shrink-0">
                  {t('settings:api.modal.capabilities.recommended')}
                </Badge>
              )}
              {(() => {
                const websiteUrl = selectedVendor.websiteUrl || getProviderWebsiteUrl(selectedVendor.providerType);
                return websiteUrl ? (
                  <NotionButton
                    size="sm"
                    variant="ghost"
                    iconOnly
                    className="opacity-60 hover:opacity-100"
                    onClick={() => void openUrl(websiteUrl)}
                    title={t('settings:vendor_panel.open_website')}
                  >
                    <ArrowSquareOut className="h-3.5 w-3.5" />
                  </NotionButton>
                ) : null;
              })()}
            </div>
            <div className="flex flex-wrap gap-2">
              <NotionButton size="sm" variant="ghost" onClick={() => handleOpenVendorModal(selectedVendor)}>{t('common:actions.edit')}</NotionButton>
              {!selectedVendorIsSiliconflow && !selectedVendor.isBuiltin && !selectedVendor.isReadOnly && (
                <NotionButton size="sm" variant="danger" onClick={() => handleDeleteVendor(selectedVendor)}>{t('common:actions.delete')}</NotionButton>
              )}
            </div>
          </div>
        </div>

        {/* 连接配置摘要 — 点击打开 VendorConfigModal */}
        <div className="rounded-lg border border-border/40 overflow-hidden">
          <button
            type="button"
            onClick={() => handleOpenVendorModal(selectedVendor)}
            className="w-full flex items-center justify-between gap-3 px-4 py-3 text-left hover:bg-muted/30 transition-colors"
          >
            <div className="flex items-center gap-2 min-w-0 text-sm">
              <LinkSimple className="h-4 w-4 text-muted-foreground shrink-0" aria-hidden="true" />
              {(() => {
                const hasUrl = !!(selectedVendor.baseUrl?.trim());
                const hasKey = !!(selectedVendor.apiKey?.trim());
                const configured = selectedVendor.noApiKey ? hasUrl : (hasUrl && hasKey);
                if (configured) {
                  return (
                    <span className="flex items-center gap-2 min-w-0 text-muted-foreground">
                      <Check className="h-3.5 w-3.5 text-green-500 shrink-0" />
                      <span className="truncate font-mono text-xs">{selectedVendor.baseUrl}</span>
                      <span className="text-muted-foreground/60 shrink-0">·</span>
                      {selectedVendor.noApiKey ? (
                        <span className="text-xs shrink-0 text-muted-foreground">
                          {t('settings:vendor_panel.no_api_key_short', { defaultValue: '无 Key' })}
                        </span>
                      ) : (
                        <span className="text-xs shrink-0">{t('settings:vendor_panel.api_key_configured_short', { defaultValue: 'Key ✓' })}</span>
                      )}
                    </span>
                  );
                }
                return (
                  <span className="text-muted-foreground text-xs">
                    {t('settings:vendor_panel.connection_not_configured', { defaultValue: '连接未配置' })}
                  </span>
                );
              })()}
            </div>
            <PencilSimple className="h-4 w-4 text-muted-foreground shrink-0" />
          </button>
        </div>
      </div>

      {/* 模型管理区 */}
      <div className="w-full pt-4">
        <div className="space-y-6">
          {!selectedVendorIsSiliconflow && (
            <div
              className={cn(
                'sticky top-0 md:top-4 z-10 -mx-1 px-1 py-3',
                'bg-[color:var(--shell-workspace-panel)]/85',
                'supports-[backdrop-filter]:bg-[color:var(--shell-workspace-panel)]/65 supports-[backdrop-filter]:backdrop-blur-md',
                'border-b border-border/40',
                'flex flex-wrap items-center justify-between gap-2'
              )}
            >
              <div className="min-w-0 flex-1 space-y-1">
                <h3 className="text-lg font-medium text-foreground">{t('settings:vendor_panel.model_list_title')}</h3>
                <p className="text-sm text-muted-foreground">{t('settings:vendor_panel.model_list_desc', { count: selectedVendorModels.length })}</p>
              </div>
              <div className="flex flex-shrink-0 items-center gap-2">
                {onAddVendorModels && supportsModelFetching(selectedVendor.providerType) && (
                  <NotionButton
                    size="sm"
                    variant="ghost"
                    onClick={() => setIsModelFetcherDialogOpen(true)}
                  >
                    <DownloadSimple className="h-3.5 w-3.5" />
                    {t('settings:vendor_panel.fetch_models_button')}
                  </NotionButton>
                )}
                <NotionButton size="sm" variant="primary" onClick={() => {
                  handleAddModelInline(selectedVendor);
                }}>
                  <Plus className="h-3.5 w-3.5" />{t('settings:vendor_panel.add_model_button')}
                </NotionButton>
              </div>
            </div>
          )}

          <div>
            {selectedVendorIsSiliconflow && (
              <div className="mb-6">
                <SiliconFlowSection variant="models" onCreateConfig={handleSiliconFlowConfig} onBatchCreateConfigs={handleBatchCreateConfigs} onBatchConfigsCreated={handleBatchConfigsCreated} showMessage={showGlobalNotification} />
              </div>
            )}
            <div className="space-y-3">
              {selectedVendorModels.length === 0 && !isAddingNewModel ? (
                <div className="rounded-lg border border-dashed border-border/60 p-8 text-center text-sm text-muted-foreground bg-muted/10">{t('settings:vendor_panel.model_empty')}</div>
              ) : shouldGroupByFamily ? (
                <div className="space-y-3">
                  {familyGroups.map((group) => {
                    const isCollapsed = collapsedFamilies.has(group.family.id);
                    const groupId = `vendor-family-${group.family.id}`;
                    return (
                      <section
                        key={group.family.id}
                        className="rounded-lg border border-border/40 overflow-hidden"
                      >
                        <button
                          type="button"
                          onClick={() => {
                            setCollapsedFamilies((prev) => {
                              const next = new Set(prev);
                              if (next.has(group.family.id)) next.delete(group.family.id);
                              else next.add(group.family.id);
                              return next;
                            });
                          }}
                          className="w-full flex items-center justify-between gap-3 px-4 py-3 text-left hover:bg-muted/30 transition-colors"
                          aria-expanded={!isCollapsed}
                          aria-controls={groupId}
                        >
                          <div className="flex items-baseline gap-2 min-w-0">
                            <span className="text-sm font-medium text-foreground truncate">{group.family.label}</span>
                            <span className="text-xs text-muted-foreground/60 shrink-0 tabular-nums">{group.items.length}</span>
                          </div>
                          <span className="text-muted-foreground shrink-0" aria-hidden="true">
                            {isCollapsed ? <CaretDown className="h-4 w-4" /> : <CaretUp className="h-4 w-4" />}
                          </span>
                        </button>
                        <div
                          id={groupId}
                          className={cn(
                            'grid transition-all duration-300 ease-in-out',
                            isCollapsed ? 'grid-rows-[0fr]' : 'grid-rows-[1fr]'
                          )}
                        >
                          <div className="overflow-hidden">
                            <div className="px-2 pb-2 pt-1 space-y-2">
                              {group.items.map(renderModelCard)}
                            </div>
                          </div>
                        </div>
                      </section>
                    );
                  })}
                </div>
              ) : (
                <div className="space-y-3">
                  {selectedVendorModels.map(renderModelCard)}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* 模型编辑器 Sheet */}
      <Sheet
        open={!!(inlineEditState || isAddingNewModel)}
        onOpenChange={(open) => {
          if (!open) {
            if (isAddingNewModel) handleCancelAddModel();
            setInlineEditState(null);
          }
        }}
      >
        <SheetContent side="right" className="w-[min(92vw,32rem)] p-0 flex flex-col" overlayClassName="backdrop-blur-sm !z-[10000]" style={{ zIndex: 10000 }}>
          <SheetHeader className="px-6 pt-6 pb-4 border-b border-border/40 shrink-0">
            <SheetTitle>
              {isAddingNewModel
                ? t('settings:vendor_panel.add_model_button')
                : t('common:actions.edit')
              }
            </SheetTitle>
            <SheetDescription>
              {selectedVendor.name || providerLabel}
            </SheetDescription>
          </SheetHeader>
          <CustomScrollArea className="flex-1 min-h-0">
            <div className="p-6">
              {inlineEditState && (
                <ShadApiEditModal
                  api={inlineEditState.api}
                  onSave={async (editedApi) => {
                    await handleSaveInlineEdit(editedApi);
                    if (!isAddingNewModel) setInlineEditState(null);
                  }}
                  onCancel={() => {
                    if (isAddingNewModel) handleCancelAddModel();
                    setInlineEditState(null);
                  }}
                  hideConnectionFields
                  lockedVendorInfo={{
                    name: selectedVendor.name,
                    baseUrl: selectedVendor.baseUrl,
                    providerType: selectedVendor.providerType,
                  }}
                  embeddedMode={true}
                />
              )}
            </div>
          </CustomScrollArea>
        </SheetContent>
      </Sheet>

      {/* 获取模型列表 Dialog */}
      {onAddVendorModels && supportsModelFetching(selectedVendor.providerType) && (
        <Dialog open={isModelFetcherDialogOpen} onOpenChange={setIsModelFetcherDialogOpen}>
          <DialogContent className="w-full max-w-2xl p-0 overflow-hidden" zIndex={Z_INDEX.sheetModal}>
            <DialogHeader className="relative px-5 pt-5 pb-4 border-b border-border/40">
              <button
                onClick={() => setIsModelFetcherDialogOpen(false)}
                className="absolute right-4 top-4 rounded-md p-1 text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
                aria-label="关闭"
              >
                <X size={18} weight="bold" />
              </button>
              <DialogTitle>{t('settings:vendor_model_fetcher.dialog_title')}</DialogTitle>
              <DialogDescription>
                {t('settings:vendor_model_fetcher.dialog_description', { vendor: selectedVendor.name || providerLabel })}
              </DialogDescription>
            </DialogHeader>
            <div className="p-4">
              <VendorModelFetcher
                key={selectedVendor.id}
                vendor={selectedVendor}
                existingModelIds={selectedVendorModels.map(({ profile }) => profile.model)}
                onAddModels={onAddVendorModels}
                embedded="dialog"
              />
            </div>
          </DialogContent>
        </Dialog>
      )}
    </>
  );
};
