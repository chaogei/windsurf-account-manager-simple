<template>
  <el-dialog
    v-model="dialogVisible"
    :title="$t('dialog.teamSettings.title')"
    width="700px"
    :close-on-click-modal="false"
    destroy-on-close
    class="team-settings-dialog"
  >
    <div v-loading="loading" class="settings-container">
      <!-- Windsurf Settings -->
      <div class="settings-section">
        <h3 class="section-title">{{ $t('dialog.teamSettings.windsurfSettings') }}</h3>
        
        <!-- Models -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.modelsConfig') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.modelsConfigDesc') }}</span>
          </div>
          <el-button size="small" @click="openModelsConfig">{{ $t('dialog.teamSettings.configure') }}</el-button>
        </div>
        
        <!-- Enable Web Search -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.webSearch') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.webSearchDesc') }}</span>
          </div>
          <el-switch v-model="settings.enableWebSearch" @change="handleSettingChange" />
        </div>
        
        <!-- Auto Run Terminal Commands -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.autoRunCommands') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.autoRunCommandsDesc') }}</span>
          </div>
          <el-switch v-model="settings.allowAutoRunCommands" @change="handleSettingChange" />
        </div>
        
        <!-- MCP Servers -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.mcpServers') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.mcpServersDesc') }}</span>
          </div>
          <el-switch v-model="settings.allowMcpServers" @change="handleSettingChange" />
        </div>
        
        <!-- Whitelisted MCP Servers -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.mcpWhitelist') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.mcpWhitelistDesc') }}</span>
          </div>
          <el-button size="small" @click="openMcpWhitelist">{{ $t('dialog.teamSettings.addServer') }}</el-button>
        </div>
        
        <!-- App Deploys -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.appDeploys') }} <el-tag size="small" type="warning">BETA</el-tag></span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.appDeploysDesc') }}</span>
          </div>
          <el-select v-model="settings.appDeploysMode" size="small" style="width: 160px" @change="handleSettingChange">
            <el-option :label="$t('dialog.teamSettings.fullDeployPermission')" value="full" />
            <el-option :label="$t('dialog.teamSettings.internalTeamsOnly')" value="teams" />
            <el-option :label="$t('dialog.teamSettings.disableDeployment')" value="disabled" />
          </el-select>
        </div>
        
        <!-- Conversation Sharing -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.conversationSharing') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.conversationSharingDesc') }}</span>
          </div>
          <el-switch v-model="settings.allowConversationSharing" @change="handleSettingChange" />
        </div>
        
        <!-- DeepWiki -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.deepwiki') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.deepwikiDesc') }}</span>
          </div>
          <el-switch v-model="settings.enableDeepwiki" @change="handleSettingChange" />
        </div>
        
        <!-- Fast Context -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.fastContext') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.fastContextDesc') }}</span>
          </div>
          <el-switch v-model="settings.enableFastContext" @change="handleSettingChange" />
        </div>
        
        <!-- Codemaps -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.codemaps') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.codemapsDesc') }}</span>
          </div>
          <el-button size="small" @click="codemapsDialogVisible = true">配置</el-button>
        </div>
        
        <!-- Vibe and Replace -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.vibeAndReplace') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.vibeAndReplaceDesc') }}</span>
          </div>
          <el-switch v-model="settings.allowVibeAndReplace" @change="handleSettingChange" />
        </div>
        
        <!-- Github Integration -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.githubIntegration') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.githubIntegrationDesc') }}</span>
          </div>
          <el-switch v-model="settings.allowGithubReviews" @change="handleSettingChange" />
        </div>
      </div>
      
      <!-- Other Settings -->
      <div class="settings-section">
        <h3 class="section-title">{{ $t('dialog.teamSettings.otherSettings') }}</h3>
        
        <!-- Domain Management -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.domainManagement') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.domainManagementDesc') }}</span>
          </div>
          <el-button size="small" disabled>配置</el-button>
        </div>
        
        <!-- SSO and SAML Configuration -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.ssoSamlConfig') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.ssoSamlConfigDesc') }}</span>
          </div>
          <el-button size="small" disabled>配置</el-button>
        </div>
        
        <!-- Service Key Configuration -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.serviceKeyConfig') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.serviceKeyConfigDesc') }}</span>
          </div>
          <el-button size="small" disabled>配置</el-button>
        </div>
        
        <!-- Role Management -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.roleManagement') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.roleManagementDesc') }}</span>
          </div>
          <el-button size="small" disabled>配置</el-button>
        </div>
        
        <!-- Individual Level Analytics -->
        <div class="setting-item">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.individualAnalytics') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.individualAnalyticsDesc') }}</span>
          </div>
          <el-switch v-model="settings.allowIndividualAnalytics" @change="handleSettingChange" />
        </div>
      </div>
    </div>
    
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="dialogVisible = false">{{ $t('dialog.teamSettings.cancel') }}</el-button>
        <el-button type="primary" :loading="saving" @click="saveSettings">
          <el-icon><Check /></el-icon> {{ $t('dialog.teamSettings.saveSettings') }}
        </el-button>
      </div>
    </template>
    
    <!-- Models Configuration Dialog -->
    <el-dialog
      v-model="modelsDialogVisible"
      title="模型配置 (Models)"
      width="700px"
      append-to-body
      @open="loadModelsConfig"
    >
      <div class="models-config" v-loading="loadingModels">
        <p class="models-desc">{{ $t('dialog.teamSettings.modelsConfigDesc') }}</p>
        
        <!-- Cascade Models -->
        <div class="models-section">
          <h4>{{ $t('dialog.teamSettings.cascadeModels') }}</h4>
          <el-select
            v-model="selectedCascadeModels"
            multiple
            filterable
            collapse-tags
            collapse-tags-tooltip
            :max-collapse-tags="5"
            placeholder="{{ $t('dialog.teamSettings.selectCascadeModels') }}"
            style="width: 100%"
            popper-class="model-select-dropdown"
          >
            <el-option
              v-for="model in availableCascadeModels"
              :key="model"
              :label="model"
              :value="model"
            >
              <div class="model-option">
                <span class="model-check" :class="{ checked: selectedCascadeModels.includes(model) }">✓</span>
                <span class="model-name">{{ model }}</span>
                <span class="model-multiplier" :class="getMultiplierClass(getModelMultiplier(model, 'cascade'))">
                  {{ formatMultiplier(getModelMultiplier(model, 'cascade')) }}
                </span>
              </div>
            </el-option>
          </el-select>
        </div>
        
        <!-- Command Models -->
        <div class="models-section">
          <h4>{{ $t('dialog.teamSettings.commandModels') }}</h4>
          <el-select
            v-model="selectedCommandModels"
            multiple
            filterable
            collapse-tags
            collapse-tags-tooltip
            :max-collapse-tags="5"
            placeholder="{{ $t('dialog.teamSettings.selectCommandModels') }}"
            style="width: 100%"
            popper-class="model-select-dropdown"
          >
            <el-option
              v-for="model in availableCommandModels"
              :key="model"
              :label="model"
              :value="model"
            >
              <div class="model-option">
                <span class="model-check" :class="{ checked: selectedCommandModels.includes(model) }">✓</span>
                <span class="model-name">{{ model }}</span>
                <span class="model-multiplier" :class="getMultiplierClass(getModelMultiplier(model, 'command'))">
                  {{ formatMultiplier(getModelMultiplier(model, 'command')) }}
                </span>
              </div>
            </el-option>
          </el-select>
        </div>
        
        <!-- Extension Models -->
        <div class="models-section">
          <h4>{{ $t('dialog.teamSettings.extensionModels') }}</h4>
          <el-select
            v-model="selectedExtensionModels"
            multiple
            filterable
            collapse-tags
            collapse-tags-tooltip
            :max-collapse-tags="5"
            placeholder="{{ $t('dialog.teamSettings.selectExtensionModels') }}"
            style="width: 100%"
            popper-class="model-select-dropdown"
          >
            <el-option
              v-for="model in availableExtensionModels"
              :key="model"
              :label="model"
              :value="model"
            >
              <div class="model-option">
                <span class="model-check" :class="{ checked: selectedExtensionModels.includes(model) }">✓</span>
                <span class="model-name">{{ model }}</span>
              </div>
            </el-option>
          </el-select>
        </div>
      </div>
      <template #footer>
        <el-button @click="modelsDialogVisible = false">{{ $t('dialog.teamSettings.cancel') }}</el-button>
        <el-button type="primary" :loading="savingModels" @click="saveModelsConfig">{{ $t('dialog.teamSettings.save') }}</el-button>
      </template>
    </el-dialog>
    
    <!-- MCP Whitelist Dialog -->
    <el-dialog
      v-model="mcpDialogVisible"
      title="{{ $t('dialog.teamSettings.addMcpServer') }}"
      width="500px"
      append-to-body
    >
      <div class="mcp-add-dialog">
        <div class="mcp-header">
          <span class="mcp-label">{{ $t('dialog.teamSettings.mcpServerId') }}</span>
          <el-link type="primary" @click="mcpManualMode = !mcpManualMode">
            {{ mcpManualMode ? $t('dialog.teamSettings.selectFromList') : $t('dialog.teamSettings.manualInput') }}
          </el-link>
        </div>
        
        <!-- 下拉选择模式 -->
        <el-select
          v-if="!mcpManualMode"
          v-model="selectedMcpPlugin"
          placeholder="{{ $t('dialog.teamSettings.selectMcpServer') }}"
          filterable
          style="width: 100%; margin-bottom: 16px"
          :loading="loadingPlugins"
          @visible-change="onMcpSelectOpen"
        >
          <el-option
            v-for="plugin in availableMcpPlugins"
            :key="plugin.id"
            :label="plugin.title"
            :value="plugin.id"
          >
            <div style="display: flex; flex-direction: column;">
              <span style="font-weight: 500;">{{ plugin.title }}</span>
              <span style="font-size: 12px; color: #999;">{{ plugin.id }}</span>
            </div>
          </el-option>
        </el-select>
        
        <!-- 手动输入模式 -->
        <el-input
          v-else
          v-model="newMcpServer"
          placeholder="{{ $t('dialog.teamSettings.inputMcpServerId') }}"
          style="margin-bottom: 16px"
        />
        
        <div class="mcp-config-section">
          <span class="mcp-label">{{ $t('dialog.teamSettings.mcpServerConfig') }}</span>
          <el-input
            v-model="mcpServerConfig"
            type="textarea"
            :rows="4"
            placeholder="{{ $t('dialog.teamSettings.mcpServerConfigDesc') }}"
          />
        </div>
        
        <div class="mcp-help-text">
          {{ $t('dialog.teamSettings.mcpHelpText') }} <el-link type="primary" href="https://docs.windsurf.com/mcp" target="_blank">{{ $t('dialog.teamSettings.mcpDocs') }}</el-link>
        </div>
        
        <!-- 已添加的服务器列表 -->
        <div class="mcp-added-list" v-if="mcpServers.length">
          <div class="mcp-added-header">{{ $t('dialog.teamSettings.addedMcpServers') }}</div>
          <div class="mcp-list">
            <el-tag
              v-for="(server, index) in mcpServers"
              :key="index"
              closable
              @close="removeMcpServer(index)"
              class="mcp-tag"
            >
              {{ server }}
            </el-tag>
          </div>
        </div>
      </div>
      <template #footer>
        <el-button @click="mcpDialogVisible = false">{{ $t('dialog.teamSettings.cancel') }}</el-button>
        <el-button type="primary" @click="addSelectedMcpServer">{{ $t('dialog.teamSettings.add') }}</el-button>
      </template>
    </el-dialog>
    
    <!-- Codemaps Configuration Dialog -->
    <el-dialog
      v-model="codemapsDialogVisible"
      title="{{ $t('dialog.teamSettings.codemaps') }}"
      width="500px"
      append-to-body
    >
      <div class="codemaps-config">
        <p class="codemaps-desc">{{ $t('dialog.teamSettings.codemapsDesc') }}</p>
        
        <div class="codemaps-setting">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.enableCodemaps') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.enableCodemapsDesc') }}</span>
          </div>
          <el-switch v-model="settings.enableCodemaps" />
        </div>
        
        <div class="codemaps-setting">
          <div class="setting-info">
            <span class="setting-name">{{ $t('dialog.teamSettings.codemapSharing') }}</span>
            <span class="setting-desc">{{ $t('dialog.teamSettings.codemapSharingDesc') }}</span>
          </div>
          <el-select v-model="settings.codemapSharing" size="small" style="width: 130px">
            <el-option label="{{ $t('dialog.teamSettings.enabled') }}" value="enabled" />
            <el-option label="{{ $t('dialog.teamSettings.teamOnly') }}" value="team" />
            <el-option label="{{ $t('dialog.teamSettings.disabled') }}" value="disabled" />
          </el-select>
        </div>
      </div>
      <template #footer>
        <el-button @click="codemapsDialogVisible = false">取消</el-button>
        <el-button type="primary" @click="saveCodemapsConfig">保存</el-button>
      </template>
    </el-dialog>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch, reactive } from 'vue';
import { ElMessage } from 'element-plus';
import { Check } from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/core';

interface Props {
  modelValue: boolean;
  accountId: string;
}

const props = defineProps<Props>();
const emit = defineEmits(['update:modelValue']);

const dialogVisible = ref(false);
const loading = ref(false);
const saving = ref(false);
const modelsDialogVisible = ref(false);
const mcpDialogVisible = ref(false);
const codemapsDialogVisible = ref(false);
const newMcpServer = ref('');
const mcpServers = ref<string[]>([]);

// MCP Plugin 相关
const mcpManualMode = ref(false);
const selectedMcpPlugin = ref('');
const loadingPlugins = ref(false);
const mcpServerConfig = ref('');
const availableMcpPlugins = ref<Array<{id: string; title: string; description?: string; trustLevel?: string}>>([]);

// Settings state
const settings = reactive({
  enableWebSearch: false,
  canEditWebSearch: false,
  allowAutoRunCommands: true,
  allowMcpServers: true,
  appDeploysMode: 'disabled',
  allowConversationSharing: false,
  enableDeepwiki: false,
  enableCodemaps: false,
  codemapSharing: 'enabled',
  enableFastContext: false,
  allowVibeAndReplace: false,
  allowGithubReviews: false,
  allowGithubDescriptionEdits: false,
  allowIndividualAnalytics: false,
  allowAttribution: false,
  allowBrowserFeatures: false,
});

// Models configuration
interface ModelInfo {
  name: string;
  multiplier: number; // 倍率: 0=Free, 0.5=0.5x, 1=1x, 2=2x, 3=3x
}

const loadingModels = ref(false);
const savingModels = ref(false);
const teamId = ref('');
const cascadeModelInfos = ref<ModelInfo[]>([]);
const commandModelInfos = ref<ModelInfo[]>([]);
const extensionModelInfos = ref<ModelInfo[]>([]);
const availableCascadeModels = ref<string[]>([]);
const availableCommandModels = ref<string[]>([]);
const availableExtensionModels = ref<string[]>([]);
const selectedCascadeModels = ref<string[]>([]);
const selectedCommandModels = ref<string[]>([]);
const selectedExtensionModels = ref<string[]>([]);

// 获取模型倍率
function getModelMultiplier(modelName: string, type: 'cascade' | 'command' | 'extension'): number {
  let infos: ModelInfo[] = [];
  if (type === 'cascade') infos = cascadeModelInfos.value;
  else if (type === 'command') infos = commandModelInfos.value;
  else infos = extensionModelInfos.value;
  
  const model = infos.find(m => m.name === modelName);
  return model?.multiplier ?? 1;
}

// 格式化倍率显示
function formatMultiplier(multiplier: number): string {
  if (multiplier === 0) return '(Free)';
  if (multiplier === 1) return '(1x credits)';
  if (multiplier < 1) return `(${multiplier}x credits)`;
  return `(${multiplier}x credits)`;
}

// 获取倍率样式类
function getMultiplierClass(multiplier: number): string {
  if (multiplier === 0) return 'free';
  if (multiplier <= 0.5) return 'low';
  if (multiplier <= 1) return 'normal';
  if (multiplier <= 2) return 'high';
  return 'very-high';
}

watch(() => props.modelValue, (val) => {
  dialogVisible.value = val;
  if (val) {
    loadSettings();
  }
});

watch(dialogVisible, (val) => {
  emit('update:modelValue', val);
});

async function loadSettings() {
  loading.value = true;
  try {
    // 调用 GetTeamConfigRecord API 获取当前设置
    console.log('Loading team config for account:', props.accountId);
    const result = await invoke('get_team_config', { id: props.accountId }) as any;
    console.log('Team config result:', JSON.stringify(result, null, 2));
    
    if (result.success && result.data) {
      const config = result.data;
      // 解析嵌套的团队配置数据
      // 响应格式: { "subMesssage_1": { "int_5": 1, ... } } 或 { "1": { ... } }
      const teamConfig = config["subMesssage_1"] || config["1"] || config;
      console.log('Parsed teamConfig:', JSON.stringify(teamConfig, null, 2));
      
      // 辅助函数：获取字段值（支持 int_X 和 X 两种格式）
      const getField = (fieldNum: number): any => {
        return teamConfig[`int_${fieldNum}`] ?? teamConfig[`string_${fieldNum}`] ?? teamConfig[`${fieldNum}`] ?? teamConfig[fieldNum];
      };
      
      // 辅助函数：解析 protobuf 布尔值 (1=true, 0/undefined=false)
      const parseBool = (fieldNum: number, defaultVal = false): boolean => {
        const val = getField(fieldNum);
        if (val === 1 || val === true || val === "1") return true;
        if (val === 0 || val === false || val === "0") return false;
        return defaultVal;
      };
      
      // TeamConfig proto 字段映射:
      // field 5: allow_mcp_servers, field 7: allow_auto_run_commands
      // field 10: allow_app_deployments, field 12: allow_github_reviews
      // field 13: allow_github_description_edits, field 17: allow_individual_level_analytics
      // field 18: allow_conversation_sharing, field 19: allow_sandbox_app_deployments
      // field 20: allow_teams_app_deployments, field 22: allow_attribution
      // field 25: allow_browser_experimental_features, field 27: allow_vibe_and_replace
      // field 28: disable_deepwiki, field 31: disable_codemaps, field 33: disable_fast_context
      
      // 允许类字段 (allow_xxx): 如果存在且为1则为 true
      settings.allowAutoRunCommands = parseBool(7, true);
      settings.allowMcpServers = parseBool(5, true);
      settings.allowConversationSharing = parseBool(18, false);
      settings.allowVibeAndReplace = parseBool(27, false);
      settings.allowGithubReviews = parseBool(12, false);
      settings.allowGithubDescriptionEdits = parseBool(13, false);
      settings.allowIndividualAnalytics = parseBool(17, false);
      settings.allowAttribution = parseBool(22, false);
      settings.allowBrowserFeatures = parseBool(25, false);
      
      // 禁用类字段 (disable_xxx): 如果为1则功能禁用，UI显示为关闭
      // 如果字段不存在，说明没有禁用，功能是启用的
      settings.enableDeepwiki = !parseBool(28, false);
      settings.enableCodemaps = !parseBool(31, false);
      settings.enableFastContext = !parseBool(33, false);
      
      // Codemap sharing (field 32, string: "enabled" or "disabled")
      const codemapSharingValue = getField(32);
      settings.codemapSharing = codemapSharingValue || 'enabled';
      
      // App deploys mode
      // field 10: allow_app_deployments, field 19: allow_sandbox_app_deployments, field 20: allow_teams_app_deployments
      const allowAppDeploys = parseBool(10, false);
      const allowSandboxDeploys = parseBool(19, false);
      const allowTeamsDeploys = parseBool(20, false);
      
      if (allowAppDeploys && allowSandboxDeploys && allowTeamsDeploys) {
        settings.appDeploysMode = 'full';  // Full deploy access
      } else if (allowTeamsDeploys) {
        settings.appDeploysMode = 'teams'; // Internal teams only
      } else {
        settings.appDeploysMode = 'disabled'; // Disable deploys
      }
      
      // MCP servers whitelist (field 23)
      const mcpList = teamConfig["subMesssage_23"] || teamConfig["23"];
      if (mcpList && Array.isArray(mcpList)) {
        mcpServers.value = mcpList;
      }
      
      console.log('Parsed settings:', {
        allowAutoRunCommands: settings.allowAutoRunCommands,
        allowMcpServers: settings.allowMcpServers,
        enableDeepwiki: settings.enableDeepwiki,
        enableCodemaps: settings.enableCodemaps,
        enableFastContext: settings.enableFastContext,
        allowVibeAndReplace: settings.allowVibeAndReplace,
        allowConversationSharing: settings.allowConversationSharing,
        allowIndividualAnalytics: settings.allowIndividualAnalytics,
      });
    } else if (result.error) {
      console.warn('Failed to load team config:', result.error);
      // 如果是权限问题，静默处理
      ElMessage.warning('该账号可能没有团队管理权限');
    }
  } catch (error: any) {
    console.error('Failed to load team settings:', error);
    const errorMsg = error?.message || error?.toString() || '未知错误';
    ElMessage.warning(`加载失败: ${errorMsg}`);
  } finally {
    loading.value = false;
  }
}

function handleSettingChange() {
  // 可以在这里添加即时保存逻辑
}

async function saveSettings() {
  saving.value = true;
  try {
    const updateData = {
      allow_auto_run_commands: settings.allowAutoRunCommands,
      allow_mcp_servers: settings.allowMcpServers,
      allow_conversation_sharing: settings.allowConversationSharing,
      disable_deepwiki: !settings.enableDeepwiki,
      disable_codemaps: !settings.enableCodemaps,
      disable_fast_context: !settings.enableFastContext,
      allow_vibe_and_replace: settings.allowVibeAndReplace,
      allow_github_reviews: settings.allowGithubReviews,
      allow_github_description_edits: settings.allowGithubDescriptionEdits,
      allow_individual_level_analytics: settings.allowIndividualAnalytics,
      allow_attribution: settings.allowAttribution,
      allow_browser_experimental_features: settings.allowBrowserFeatures,
      // App deploys: full = all three, teams = only teams, disabled = none
      allow_app_deployments: settings.appDeploysMode === 'full',
      allow_sandbox_app_deployments: settings.appDeploysMode === 'full',
      allow_teams_app_deployments: settings.appDeploysMode === 'full' || settings.appDeploysMode === 'teams',
      allowed_mcp_servers: mcpServers.value.join(','),
      // Codemaps
      allow_codemap_sharing: settings.codemapSharing,
    };
    
    const result = await invoke('update_team_config', { id: props.accountId, config: updateData }) as any;
    if (result.success) {
      ElMessage.success('团队设置已保存');
    } else {
      ElMessage.error(result.error || '保存失败');
    }
  } catch (error) {
    console.error('Failed to save team settings:', error);
    ElMessage.error('保存团队设置失败');
  } finally {
    saving.value = false;
  }
}

function openModelsConfig() {
  modelsDialogVisible.value = true;
}

async function loadModelsConfig() {
  loadingModels.value = true;
  try {
    // 获取可用模型列表
    const modelConfigResult = await invoke('get_cascade_model_configs', { id: props.accountId }) as any;
    console.log('Model configs:', modelConfigResult);
    
    if (modelConfigResult.success && modelConfigResult.data) {
      const data = modelConfigResult.data;
      console.log('Model data keys:', Object.keys(data));
      console.log('Model data full:', JSON.stringify(data, null, 2).substring(0, 2000));
      console.log('subMesssage_2 (sorts):', JSON.stringify(data.subMesssage_2, null, 2));
      console.log('subMesssage_3 (default):', JSON.stringify(data.subMesssage_3, null, 2));
      
      // 响应格式: CascadeModelConfigData
      // - subMesssage_1 = client_model_configs (repeated ClientModelConfig)
      // - 每个 ClientModelConfig 的 string_1 = label (模型名称)
      let modelConfigs: any[] = [];
      
      // 尝试多种可能的字段名和嵌套层级
      const field1 = data.subMesssage_1 || data.subMessage_1 || data.repeated_1 || data["1"];
      console.log('Field1 type:', typeof field1, 'isArray:', Array.isArray(field1));
      
      if (Array.isArray(field1)) {
        modelConfigs = field1;
      } else if (field1 && typeof field1 === 'object') {
        // 检查是否有嵌套的 repeated 字段
        const nested = field1.repeated_1 || field1.subMesssage_1 || field1["1"];
        if (Array.isArray(nested)) {
          modelConfigs = nested;
        } else if (field1.string_1) {
          // 如果是单个对象，检查它是否有 string_1 字段
          modelConfigs = [field1];
        }
      }
      
      console.log('Model configs count:', modelConfigs.length);
      
      // 提取模型名称和倍率 - 从每个元素的 string_1 和 float_3 字段获取
      // ClientModelConfig: label=1(string), multiplier=3(float), disabled=4(bool)
      const allModels: string[] = [];
      const enabledModels: string[] = [];
      const modelInfos: ModelInfo[] = [];
      
      if (Array.isArray(modelConfigs)) {
        modelConfigs.forEach((config: any, idx: number) => {
          const modelName = config.string_1 || config.label || config["1"];
          // multiplier 字段 (field 3): 倍率，没有该字段表示免费模型
          const rawMultiplier = config.float_3 ?? config.multiplier ?? config["3"];
          const multiplier = rawMultiplier !== undefined ? rawMultiplier : 0; // 无 float_3 = Free
          // disabled 字段 (field 4): true=未选中, false=已选中
          const isDisabled = config.bool_4 === true || config.disabled === true || config["4"] === true;
          
          console.log(`Config ${idx}:`, modelName, 'multiplier:', multiplier, 'disabled:', isDisabled);
          if (modelName && typeof modelName === 'string') {
            allModels.push(modelName);
            modelInfos.push({ name: modelName, multiplier: typeof multiplier === 'number' ? multiplier : 1 });
            if (!isDisabled) {
              enabledModels.push(modelName);
            }
          }
        });
      }
      
      console.log('All cascade models:', allModels.length, 'Enabled:', enabledModels.length);
      
      // 可用模型 = 所有模型，已选模型 = disabled=false 的模型
      cascadeModelInfos.value = modelInfos;
      availableCascadeModels.value = allModels.sort();
      selectedCascadeModels.value = enabledModels;
      console.log('Selected cascade models:', enabledModels);
      
      availableExtensionModels.value = [
        'Base Model ⚡️',
        'GPT-4o',
        'o1-preview',
        'o1-mini',
        'Codeium Premier 🔥',
        'Claude 3.7 Sonnet',
        'Claude 3.5 Sonnet',
      ];
    }
    
    // 获取 Command 模型配置
    const commandConfigResult = await invoke('get_command_model_configs', { id: props.accountId }) as any;
    console.log('Command configs:', commandConfigResult);
    
    if (commandConfigResult.success && commandConfigResult.data) {
      const cmdData = commandConfigResult.data;
      const cmdField1 = cmdData.subMesssage_1 || cmdData.subMessage_1 || cmdData.repeated_1 || cmdData["1"];
      
      if (Array.isArray(cmdField1)) {
        // 从 Command API 获取模型列表并设置为已选
        const commandModels: string[] = [];
        const cmdModelInfos: ModelInfo[] = [];
        cmdField1.forEach((config: any) => {
          const modelName = config.string_1 || config.label || config["1"];
          // multiplier 字段 (field 3): 倍率，没有该字段表示免费模型
          const rawMultiplier = config.float_3 ?? config.multiplier ?? config["3"];
          const multiplier = rawMultiplier !== undefined ? rawMultiplier : 0; // 无 float_3 = Free
          if (modelName) {
            commandModels.push(modelName);
            cmdModelInfos.push({ name: modelName, multiplier: typeof multiplier === 'number' ? multiplier : 1 });
          }
        });
        
        console.log('Command models from API:', commandModels);
        commandModelInfos.value = cmdModelInfos;
        availableCommandModels.value = commandModels;
        selectedCommandModels.value = commandModels; // Command API 返回的就是已选模型
      }
    }
    
    // 获取当前团队的模型配置
    const controlsResult = await invoke('get_team_organizational_controls', { id: props.accountId }) as any;
    console.log('Team controls:', controlsResult);
    console.log('Team controls data keys:', controlsResult.data ? Object.keys(controlsResult.data) : 'no data');
    
    if (controlsResult.success && controlsResult.data) {
      const data = controlsResult.data;
      console.log('Full data:', JSON.stringify(data, null, 2));
      
      // 响应格式: { subMessage_1: { string_1: team_id, repeated_2: [...], repeated_3: [...], repeated_6: [...] } }
      const controls = data.subMessage_1 || data.subMesssage_1 || data["1"] || data;
      console.log('Controls keys:', Object.keys(controls));
      console.log('Controls:', controls);
      
      // 解析 team_id，可能包含 protobuf 前缀需要清理
      let rawTeamId = controls.string_1 || controls["1"] || '';
      // 移除可能的 protobuf 嵌套消息前缀 (如 "\n$")
      if (typeof rawTeamId === 'string' && rawTeamId.includes('$')) {
        rawTeamId = rawTeamId.substring(rawTeamId.indexOf('$') + 1);
      }
      teamId.value = rawTeamId;
      
      // 解析已选模型 - Rust parser 使用 string_X 格式
      const parseRepeatedString = (field: any): string[] => {
        if (Array.isArray(field)) return field;
        if (typeof field === 'string') return [field];
        return [];
      };
      
      // string_2 = Cascade, string_3 = Command, string_6 = Extension
      // 只有当 API 返回了已选模型时才覆盖，否则保留从 Recommended 列表获取的默认值
      const cascadeFromApi = parseRepeatedString(controls.string_2 || controls.repeated_2 || controls["2"]);
      const commandFromApi = parseRepeatedString(controls.string_3 || controls.repeated_3 || controls["3"]);
      const extensionFromApi = parseRepeatedString(controls.string_6 || controls.repeated_6 || controls["6"]);
      
      if (cascadeFromApi.length > 0) {
        selectedCascadeModels.value = cascadeFromApi;
      }
      if (commandFromApi.length > 0) {
        selectedCommandModels.value = commandFromApi;
      }
      if (extensionFromApi.length > 0) {
        selectedExtensionModels.value = extensionFromApi;
      }
      
      // 同时从已选模型中更新可用列表（如果有新的模型）
      selectedCascadeModels.value.forEach(m => {
        if (!availableCascadeModels.value.includes(m)) {
          availableCascadeModels.value.push(m);
        }
      });
      selectedCommandModels.value.forEach(m => {
        if (!availableCommandModels.value.includes(m)) {
          availableCommandModels.value.push(m);
        }
      });
      selectedExtensionModels.value.forEach(m => {
        if (!availableExtensionModels.value.includes(m)) {
          availableExtensionModels.value.push(m);
        }
      });
      
      console.log('Parsed models:', {
        teamId: teamId.value,
        cascade: selectedCascadeModels.value,
        command: selectedCommandModels.value,
        extension: selectedExtensionModels.value,
      });
    }
  } catch (error) {
    console.error('Failed to load models config:', error);
    ElMessage.error('加载模型配置失败');
  } finally {
    loadingModels.value = false;
  }
}

async function saveModelsConfig() {
  if (!teamId.value) {
    ElMessage.warning('该账号不是团队账号，无法保存模型配置');
    return;
  }
  
  savingModels.value = true;
  try {
    const result = await invoke('upsert_team_organizational_controls', {
      id: props.accountId,
      teamId: teamId.value,
      cascadeModels: selectedCascadeModels.value,
      commandModels: selectedCommandModels.value,
      extensionModels: selectedExtensionModels.value,
    }) as any;
    
    if (result.success) {
      ElMessage.success('模型配置已保存');
      modelsDialogVisible.value = false;
    } else {
      ElMessage.error(result.error || '保存失败');
    }
  } catch (error) {
    console.error('Failed to save models config:', error);
    ElMessage.error('保存模型配置失败');
  } finally {
    savingModels.value = false;
  }
}

function saveCodemapsConfig() {
  // Codemaps 设置会在主保存时一起保存
  codemapsDialogVisible.value = false;
  ElMessage.success('Codemaps 配置已更新');
}

function openMcpWhitelist() {
  mcpDialogVisible.value = true;
  // 重置状态
  selectedMcpPlugin.value = '';
  newMcpServer.value = '';
  mcpServerConfig.value = '';
  mcpManualMode.value = false;
}

async function onMcpSelectOpen(visible: boolean) {
  if (visible && availableMcpPlugins.value.length === 0) {
    await loadMcpPlugins();
  }
}

async function loadMcpPlugins() {
  loadingPlugins.value = true;
  try {
    const result = await invoke('get_available_mcp_plugins', { id: props.accountId }) as any;
    if (result.success && result.data?.plugins) {
      availableMcpPlugins.value = result.data.plugins.map((p: any) => ({
        id: p.id,
        title: p.title,
        description: p.description,
        trustLevel: p.trustLevel
      }));
      // 按标题排序
      availableMcpPlugins.value.sort((a, b) => a.title.localeCompare(b.title));
    }
  } catch (error) {
    console.error('Failed to load MCP plugins:', error);
  } finally {
    loadingPlugins.value = false;
  }
}

function addSelectedMcpServer() {
  const serverId = mcpManualMode.value ? newMcpServer.value.trim() : selectedMcpPlugin.value;
  if (serverId && !mcpServers.value.includes(serverId)) {
    mcpServers.value.push(serverId);
    // 重置输入
    selectedMcpPlugin.value = '';
    newMcpServer.value = '';
    mcpServerConfig.value = '';
    ElMessage.success(`已添加 ${serverId}`);
  } else if (mcpServers.value.includes(serverId)) {
    ElMessage.warning('该服务器已在白名单中');
  } else {
    ElMessage.warning('请选择或输入服务器 ID');
  }
}

function removeMcpServer(index: number) {
  mcpServers.value.splice(index, 1);
}
</script>

<style lang="scss" scoped>
.team-settings-dialog {
  :deep(.el-dialog__body) {
    padding: 16px 24px;
    max-height: 70vh;
    overflow-y: auto;
  }
}

.settings-container {
  min-height: 300px;
}

.settings-section {
  margin-bottom: 24px;
  
  .section-title {
    font-size: 16px;
    font-weight: 600;
    color: #303133;
    margin-bottom: 16px;
    padding-bottom: 8px;
    border-bottom: 2px solid #e4e7ed;
  }
}

.setting-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid #f0f2f5;
  
  &:last-child {
    border-bottom: none;
  }
  
  .setting-info {
    flex: 1;
    margin-right: 16px;
    
    .setting-name {
      display: block;
      font-size: 14px;
      font-weight: 500;
      color: #303133;
      margin-bottom: 4px;
      
      .el-tag {
        margin-left: 8px;
        vertical-align: middle;
      }
    }
    
    .setting-desc {
      display: block;
      font-size: 12px;
      color: #909399;
      line-height: 1.4;
    }
  }
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.models-config {
  .models-desc {
    font-size: 14px;
    color: #666;
    margin-bottom: 20px;
  }
  
  .models-section {
    margin-bottom: 20px;
    
    h4 {
      font-size: 14px;
      font-weight: 600;
      margin-bottom: 12px;
      color: #303133;
    }
  }
}

// 下拉选项样式 (全局样式)
:global(.model-select-dropdown) {
  .el-select-dropdown__item {
    padding: 8px 12px;
    height: auto;
  }
}

.model-option {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  
  .model-check {
    width: 16px;
    color: transparent;
    font-weight: bold;
    
    &.checked {
      color: #10b981;
    }
  }
  
  .model-name {
    flex: 1;
  }
  
  .model-multiplier {
    font-size: 12px;
    margin-left: auto;
    padding: 2px 6px;
    border-radius: 4px;
    
    &.free {
      color: #10b981;
    }
    
    &.low {
      color: #3b82f6;
    }
    
    &.normal {
      color: #6b7280;
    }
    
    &.high {
      color: #f59e0b;
    }
    
    &.very-high {
      color: #ef4444;
    }
  }
}

.codemaps-config {
  .codemaps-desc {
    font-size: 14px;
    color: #666;
    margin-bottom: 20px;
  }
  
  .codemaps-setting {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 0;
    border-bottom: 1px solid #f0f0f0;
    
    &:last-child {
      border-bottom: none;
    }
    
    .setting-info {
      flex: 1;
      
      .setting-name {
        display: block;
        font-weight: 500;
        color: #333;
        margin-bottom: 4px;
      }
      
      .setting-desc {
        font-size: 12px;
        color: #999;
      }
    }
  }
}

.mcp-whitelist {
  .mcp-list {
    margin-top: 16px;
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  
  .mcp-tag {
    font-size: 13px;
  }
}

.mcp-add-dialog {
  .mcp-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
  }
  
  .mcp-label {
    font-weight: 500;
    color: #333;
    display: block;
    margin-bottom: 8px;
  }
  
  .mcp-config-section {
    margin-bottom: 16px;
  }
  
  .mcp-help-text {
    font-size: 12px;
    color: #666;
    margin-bottom: 16px;
  }
  
  .mcp-added-list {
    margin-top: 20px;
    padding-top: 16px;
    border-top: 1px solid #eee;
    
    .mcp-added-header {
      font-weight: 500;
      color: #333;
      margin-bottom: 12px;
    }
    
    .mcp-list {
      display: flex;
      flex-wrap: wrap;
      gap: 8px;
    }
    
    .mcp-tag {
      font-size: 13px;
    }
  }
}
</style>
