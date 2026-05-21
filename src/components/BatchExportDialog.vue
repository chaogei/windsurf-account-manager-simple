<template>
  <el-dialog
    v-model="visible"
    title="批量导出账号"
    width="860px"
    class="batch-export-dialog"
    :close-on-click-modal="false"
    @open="handleOpen"
  >
    <div class="export-dialog-content">
      <el-alert
        type="warning"
        :closable="false"
        show-icon
        class="export-security-alert"
      >
        <template #title>即将导出 {{ accounts.length }} 个账号，导出内容可能包含密码、Token、API Key 等敏感凭证，请妥善保管。</template>
      </el-alert>

      <div class="export-section">
        <div class="section-header">
          <div>
            <div class="section-title">导出预设</div>
            <div class="section-desc">快速选择常用字段组合，也可以继续手动调整字段。</div>
          </div>
        </div>
        <div class="preset-grid">
          <button
            v-for="preset in exportPresets"
            :key="preset.key"
            type="button"
            class="preset-card"
            :class="{ active: activePreset === preset.key }"
            @click="applyPreset(preset.key)"
          >
            <span class="preset-title">{{ preset.label }}</span>
            <span class="preset-desc">{{ preset.description }}</span>
          </button>
        </div>
      </div>

      <div class="export-section">
        <div class="section-header">
          <div>
            <div class="section-title">导出字段</div>
            <div class="section-desc">自由组合账号、密码、Token、分组、标签、套餐等字段。</div>
          </div>
          <div class="field-actions">
            <el-button size="small" @click="selectAllFields">全选</el-button>
            <el-button size="small" @click="resetFields">重置</el-button>
          </div>
        </div>
        <div class="field-grid">
          <label
            v-for="field in exportFields"
            :key="field.key"
            class="field-card"
            :class="{ checked: selectedFields.includes(field.key) }"
          >
            <el-checkbox
              :model-value="selectedFields.includes(field.key)"
              @change="toggleField(field.key, $event as boolean)"
            >
              <span class="field-label">{{ field.label }}</span>
              <span class="field-desc">{{ field.description }}</span>
            </el-checkbox>
          </label>
        </div>
      </div>

      <div class="export-section" v-if="selectedFields.length > 0">
        <div class="section-header">
          <div>
            <div class="section-title">字段顺序</div>
            <div class="section-desc">拖拽或使用按钮调整字段在导出结果中的先后顺序。</div>
          </div>
          <el-tag type="info" effect="plain">{{ selectedFields.length }} 个字段</el-tag>
        </div>
        <draggable
          v-model="selectedFields"
          item-key="item"
          tag="div"
          class="order-grid"
          handle=".order-drag"
          :animation="180"
          @change="activePreset = 'custom'"
        >
          <template #item="{ element, index }">
            <div class="order-item">
              <span class="order-drag" title="拖拽排序">
                <el-icon><Rank /></el-icon>
              </span>
              <span class="order-index">{{ index + 1 }}</span>
              <span class="order-label">{{ getFieldLabel(element) }}</span>
              <div class="order-actions">
                <el-button text size="small" :disabled="index === 0" @click="moveField(index, -1)">
                  <el-icon><ArrowUp /></el-icon>
                </el-button>
                <el-button text size="small" :disabled="index === selectedFields.length - 1" @click="moveField(index, 1)">
                  <el-icon><ArrowDown /></el-icon>
                </el-button>
                <el-button text size="small" @click="toggleField(element, false)">
                  <el-icon><Close /></el-icon>
                </el-button>
              </div>
            </div>
          </template>
        </draggable>
      </div>

      <div class="export-options-grid">
        <div class="export-section">
          <div class="section-title">导出格式</div>
          <el-radio-group v-model="exportFormat" class="option-grid">
            <el-radio-button label="txt">TXT</el-radio-button>
            <el-radio-button label="csv">CSV</el-radio-button>
            <el-radio-button label="json">JSON</el-radio-button>
          </el-radio-group>
        </div>

        <div class="export-section" :class="{ muted: exportFormat !== 'txt' }">
          <div class="section-title">文本分隔符</div>
          <el-radio-group v-model="delimiterType" class="delimiter-grid" :disabled="exportFormat !== 'txt'">
            <el-radio-button label="space">空格</el-radio-button>
            <el-radio-button label="tab">Tab</el-radio-button>
            <el-radio-button label="pipe">|</el-radio-button>
            <el-radio-button label="comma">,</el-radio-button>
            <el-radio-button label="tripleDash">---</el-radio-button>
            <el-radio-button label="quadDash">----</el-radio-button>
            <el-radio-button label="custom">自定义</el-radio-button>
          </el-radio-group>
          <el-input
            v-if="delimiterType === 'custom'"
            v-model="customDelimiter"
            class="custom-delimiter-input"
            placeholder="输入自定义分隔符"
            :disabled="exportFormat !== 'txt'"
          />
        </div>

        <div class="export-section">
          <div class="section-title">导出方式</div>
          <el-radio-group v-model="exportTarget" class="option-grid">
            <el-radio-button label="clipboard">复制到剪贴板</el-radio-button>
            <el-radio-button label="file">下载文件</el-radio-button>
          </el-radio-group>
        </div>

        <div class="export-section">
          <div class="section-title">附加选项</div>
          <el-switch v-model="includeHeader" active-text="包含表头" inactive-text="不含表头" :disabled="exportFormat === 'json'" />
        </div>
      </div>

      <div class="export-section preview-section">
        <div class="section-header">
          <div>
            <div class="section-title">导出预览</div>
            <div class="section-desc">仅展示前 {{ previewLimit }} 条，实际导出 {{ accounts.length }} 条。</div>
          </div>
          <el-tag type="info" effect="plain">{{ selectedFields.length }} 个字段</el-tag>
        </div>
        <pre class="export-preview">{{ previewContent || '请选择至少一个导出字段' }}</pre>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="visible = false">取消</el-button>
        <el-button type="primary" :disabled="!canExport" @click="handleExport">
          {{ exportTarget === 'clipboard' ? '复制导出' : '下载导出' }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { ElMessage } from 'element-plus';
import { Rank, ArrowUp, ArrowDown, Close } from '@element-plus/icons-vue';
import draggable from 'vuedraggable';
import type { Account } from '@/types';

type ExportFormat = 'txt' | 'csv' | 'json';
type ExportTarget = 'clipboard' | 'file';
type DelimiterType = 'space' | 'tab' | 'pipe' | 'comma' | 'tripleDash' | 'quadDash' | 'custom';
type ExportFieldKey =
  | 'email'
  | 'password'
  | 'refresh_token'
  | 'access_token'
  | 'windsurf_api_key'
  | 'devin_auth1_token'
  | 'devin_session_token'
  | 'nickname'
  | 'group'
  | 'tags'
  | 'status'
  | 'plan_name'
  | 'auth_provider'
  | 'created_at'
  | 'token_expires_at'
  | 'subscription_expires_at';

type ExportPresetKey = 'email_password' | 'email_refresh_token' | 'devin_auth1' | 'devin_session' | 'full_backup' | 'custom';

interface ExportField {
  key: ExportFieldKey;
  label: string;
  description: string;
  sensitive: boolean;
  getValue: (account: Account) => string;
}

interface ExportPreset {
  key: ExportPresetKey;
  label: string;
  description: string;
  fields: ExportFieldKey[];
}

const props = defineProps<{
  modelValue: boolean;
  accounts: Account[];
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
}>();

const visible = computed({
  get: () => props.modelValue,
  set: value => emit('update:modelValue', value)
});

const previewLimit = 5;
const exportFormat = ref<ExportFormat>('txt');
const exportTarget = ref<ExportTarget>('clipboard');
const delimiterType = ref<DelimiterType>('space');
const customDelimiter = ref('----');
const includeHeader = ref(false);
const activePreset = ref<ExportPresetKey>('email_password');
const selectedFields = ref<ExportFieldKey[]>(['email', 'password']);

const exportFields: ExportField[] = [
  { key: 'email', label: '邮箱', description: '账号邮箱或用户名', sensitive: false, getValue: account => account.email || '' },
  { key: 'password', label: '密码', description: '传统登录密码', sensitive: true, getValue: account => account.password || '' },
  { key: 'refresh_token', label: 'Refresh Token', description: 'Firebase 刷新凭证', sensitive: true, getValue: account => account.refresh_token || '' },
  { key: 'access_token', label: 'Access / Session Token', description: '当前访问令牌或会话令牌', sensitive: true, getValue: account => account.token || '' },
  { key: 'windsurf_api_key', label: 'Windsurf API Key', description: 'Windsurf API Key', sensitive: true, getValue: account => account.windsurf_api_key || '' },
  { key: 'devin_auth1_token', label: 'Devin Auth1 Token', description: 'Devin 一级认证令牌', sensitive: true, getValue: account => account.devin_auth1_token || '' },
  { key: 'devin_session_token', label: 'Devin Session Token', description: '仅 Devin 账号导出 Session Token', sensitive: true, getValue: account => account.auth_provider === 'devin' ? (account.token || '') : '' },
  { key: 'nickname', label: '备注', description: '本地备注名称', sensitive: false, getValue: account => account.nickname || '' },
  { key: 'group', label: '分组', description: '账号所属分组', sensitive: false, getValue: account => account.group || '' },
  { key: 'tags', label: '标签', description: '多个标签用 ; 连接', sensitive: false, getValue: account => (account.tags || []).join(';') },
  { key: 'status', label: '状态', description: 'active / inactive / error', sensitive: false, getValue: account => account.status || '' },
  { key: 'plan_name', label: '套餐', description: 'Free / Pro / Teams / Enterprise', sensitive: false, getValue: account => account.plan_name || '' },
  { key: 'auth_provider', label: '认证提供方', description: 'Firebase 或 Devin', sensitive: false, getValue: account => account.auth_provider || 'firebase' },
  { key: 'created_at', label: '创建时间', description: '本地创建时间', sensitive: false, getValue: account => account.created_at || '' },
  { key: 'token_expires_at', label: 'Token 过期时间', description: '访问令牌过期时间', sensitive: false, getValue: account => account.token_expires_at || '' },
  { key: 'subscription_expires_at', label: '订阅到期时间', description: '订阅周期结束时间', sensitive: false, getValue: account => account.subscription_expires_at || '' }
];

const exportPresets: ExportPreset[] = [
  { key: 'email_password', label: '邮箱 + 密码', description: '传统账号迁移', fields: ['email', 'password'] },
  { key: 'email_refresh_token', label: '邮箱 + Refresh Token', description: '刷新账号信息', fields: ['email', 'refresh_token'] },
  { key: 'devin_auth1', label: '邮箱 + Devin Auth1', description: 'Devin 换机迁移', fields: ['email', 'devin_auth1_token'] },
  { key: 'devin_session', label: '邮箱 + Devin Session', description: '当前会话凭证', fields: ['email', 'devin_session_token'] },
  { key: 'full_backup', label: '完整备份字段', description: '账号、凭证和本地元数据', fields: ['email', 'password', 'refresh_token', 'access_token', 'windsurf_api_key', 'devin_auth1_token', 'devin_session_token', 'nickname', 'group', 'tags', 'status', 'plan_name', 'auth_provider', 'created_at', 'token_expires_at', 'subscription_expires_at'] },
  { key: 'custom', label: '自定义组合', description: '手动选择字段', fields: ['email', 'password'] }
];

const fieldMap = computed(() => {
  const map = new Map<ExportFieldKey, ExportField>();
  exportFields.forEach(field => map.set(field.key, field));
  return map;
});
const selectedFieldDefs = computed(() =>
  selectedFields.value
    .map(key => fieldMap.value.get(key))
    .filter((field): field is ExportField => !!field)
);
function getFieldLabel(key: ExportFieldKey) {
  return fieldMap.value.get(key)?.label || key;
}
function toggleField(key: ExportFieldKey, checked: boolean) {
  activePreset.value = 'custom';
  if (checked) {
    if (!selectedFields.value.includes(key)) {
      selectedFields.value = [...selectedFields.value, key];
    }
  } else {
    selectedFields.value = selectedFields.value.filter(item => item !== key);
  }
}
function moveField(index: number, direction: -1 | 1) {
  const target = index + direction;
  if (target < 0 || target >= selectedFields.value.length) return;
  const next = [...selectedFields.value];
  [next[index], next[target]] = [next[target], next[index]];
  selectedFields.value = next;
  activePreset.value = 'custom';
}
const canExport = computed(() => props.accounts.length > 0 && selectedFieldDefs.value.length > 0);
const delimiter = computed(() => {
  const delimiters: Record<DelimiterType, string> = {
    space: ' ',
    tab: '\t',
    pipe: '|',
    comma: ',',
    tripleDash: '---',
    quadDash: '----',
    custom: customDelimiter.value
  };
  return delimiters[delimiterType.value];
});
const previewContent = computed(() => buildExportContent(props.accounts.slice(0, previewLimit)));

function handleOpen() {
  if (selectedFields.value.length === 0) {
    applyPreset('email_password');
  }
}

function applyPreset(key: ExportPresetKey) {
  const preset = exportPresets.find(item => item.key === key);
  if (!preset) return;
  activePreset.value = key;
  selectedFields.value = [...preset.fields];
}

function selectAllFields() {
  activePreset.value = 'custom';
  selectedFields.value = exportFields.map(field => field.key);
}

function resetFields() {
  applyPreset('email_password');
}

function buildRows(accounts: Account[]) {
  return accounts.map(account => selectedFieldDefs.value.map(field => field.getValue(account)));
}

function buildExportContent(accounts: Account[]) {
  if (selectedFieldDefs.value.length === 0) return '';
  const labels = selectedFieldDefs.value.map(field => field.label);
  const rows = buildRows(accounts);

  if (exportFormat.value === 'json') {
    return JSON.stringify(accounts.map(account => {
      const item: Record<string, string> = {};
      selectedFieldDefs.value.forEach(field => {
        item[field.key] = field.getValue(account);
      });
      return item;
    }), null, 2);
  }

  if (exportFormat.value === 'csv') {
    const csvSourceRows = includeHeader.value ? [labels, ...rows] : rows;
    const csvRows = csvSourceRows.map(row => row.map(escapeCsvCell).join(','));
    return csvRows.join('\n');
  }

  const textRows = rows.map(row => row.join(delimiter.value));
  return includeHeader.value ? [labels.join(delimiter.value), ...textRows].join('\n') : textRows.join('\n');
}

function escapeCsvCell(value: string) {
  return `"${String(value).replace(/"/g, '""')}"`;
}

function buildFileName() {
  const timestamp = new Date().toISOString().replace(/[:.]/g, '-').substring(0, 19);
  const extension = exportFormat.value === 'json' ? 'json' : exportFormat.value === 'csv' ? 'csv' : 'txt';
  return `accounts_export_${timestamp}.${extension}`;
}

async function handleExport() {
  if (!canExport.value) {
    ElMessage.warning('请选择至少一个导出字段');
    return;
  }

  if (exportFormat.value === 'txt' && delimiterType.value === 'custom' && customDelimiter.value.length === 0) {
    ElMessage.warning('自定义分隔符不能为空');
    return;
  }

  const content = buildExportContent(props.accounts);
  if (exportTarget.value === 'clipboard') {
    await navigator.clipboard.writeText(content);
    ElMessage.success(`已复制 ${props.accounts.length} 个账号到剪贴板`);
    visible.value = false;
    return;
  }

  const bom = exportFormat.value === 'csv' ? '\uFEFF' : '';
  const blob = new Blob([bom + content], { type: 'text/plain;charset=utf-8' });
  const url = window.URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = buildFileName();
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  window.URL.revokeObjectURL(url);
  ElMessage.success(`已导出 ${props.accounts.length} 个账号`);
  visible.value = false;
}
</script>

<style scoped>
.export-dialog-content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.export-security-alert {
  border-radius: 14px;
}

.export-section {
  padding: 16px;
  border: 1px solid var(--theme-border, #e4e7ed);
  border-radius: 18px;
  background: color-mix(in srgb, var(--theme-surface, #ffffff) 86%, transparent);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.22), 0 10px 24px rgba(15, 23, 42, 0.06);
}

.export-section.muted {
  opacity: 0.68;
}

.section-header {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: flex-start;
  margin-bottom: 12px;
}

.section-title {
  color: var(--theme-text, #303133);
  font-size: 14px;
  font-weight: 800;
}

.section-desc {
  margin-top: 4px;
  color: var(--theme-text-muted, #909399);
  font-size: 12px;
  line-height: 1.5;
}

.preset-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px;
}

.preset-card {
  display: flex;
  flex-direction: column;
  gap: 5px;
  min-height: 72px;
  padding: 12px;
  border: 1px solid var(--theme-border, #e4e7ed);
  border-radius: 14px;
  background: color-mix(in srgb, var(--theme-surface-strong, #ffffff) 88%, transparent);
  color: var(--theme-text, #303133);
  text-align: left;
  cursor: pointer;
  transition: all 0.2s ease;
}

.preset-card:hover,
.preset-card.active {
  border-color: var(--theme-primary, #409eff);
  background: color-mix(in srgb, var(--theme-primary, #409eff) 12%, var(--theme-surface, #ffffff));
  box-shadow: 0 10px 22px color-mix(in srgb, var(--theme-primary, #409eff) 16%, transparent);
}

.preset-title {
  font-size: 13px;
  font-weight: 800;
}

.preset-desc {
  color: var(--theme-text-muted, #909399);
  font-size: 12px;
  line-height: 1.4;
}

.field-actions {
  display: flex;
  gap: 8px;
}

.field-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 10px;
}

.field-card {
  height: auto;
  margin: 0 !important;
  padding: 10px 12px;
  border: 1px solid var(--theme-border, #e4e7ed);
  border-radius: 14px;
  background: color-mix(in srgb, var(--theme-surface-strong, #ffffff) 86%, transparent);
}

.field-card :deep(.el-checkbox) {
  width: 100%;
  height: auto;
  margin: 0;
  align-items: flex-start;
}

.field-card :deep(.el-checkbox__label) {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
  white-space: normal;
}

.field-card.checked {
  border-color: var(--theme-primary, #409eff);
  background: color-mix(in srgb, var(--theme-primary, #409eff) 10%, var(--theme-surface, #ffffff));
}

.order-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.order-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 14px;
  border: 1px solid var(--theme-border, #e4e7ed);
  border-radius: 12px;
  background: color-mix(in srgb, var(--theme-surface-strong, #ffffff) 90%, transparent);
  transition: box-shadow 0.2s ease;
}

.order-item:hover {
  box-shadow: 0 6px 18px rgba(15, 23, 42, 0.08);
}

.order-drag {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  color: var(--theme-text-muted, #909399);
  cursor: grab;
}

.order-drag:active {
  cursor: grabbing;
}

.order-index {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 22px;
  height: 22px;
  border-radius: 999px;
  background: var(--theme-primary, #409eff);
  color: #fff;
  font-size: 12px;
  font-weight: 700;
}

.order-label {
  flex: 1;
  color: var(--theme-text, #303133);
  font-size: 13px;
  font-weight: 600;
}

.order-actions {
  display: flex;
  gap: 2px;
}

.field-label {
  color: var(--theme-text, #303133);
  font-size: 13px;
  font-weight: 800;
}

.field-desc {
  color: var(--theme-text-muted, #909399);
  font-size: 12px;
  line-height: 1.35;
  white-space: normal;
}

.export-options-grid {
  display: grid;
  grid-template-columns: 1fr 1.3fr;
  gap: 16px;
}

.option-grid,
.delimiter-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 10px;
}

.option-grid :deep(.el-radio-button__inner),
.delimiter-grid :deep(.el-radio-button__inner) {
  border-radius: 999px !important;
  border: 1px solid var(--theme-border, #dcdfe6) !important;
  background: var(--theme-surface, #ffffff) !important;
  color: var(--theme-text, #303133) !important;
}

.option-grid :deep(.el-radio-button__original-radio:checked + .el-radio-button__inner),
.delimiter-grid :deep(.el-radio-button__original-radio:checked + .el-radio-button__inner) {
  background: var(--theme-primary, #409eff) !important;
  border-color: var(--theme-primary, #409eff) !important;
  color: #ffffff !important;
  box-shadow: 0 8px 18px var(--theme-primary-glow, rgba(64, 158, 255, 0.2)) !important;
}

.custom-delimiter-input {
  margin-top: 10px;
}

.preview-section {
  padding-bottom: 12px;
}

.export-preview {
  max-height: 220px;
  min-height: 120px;
  margin: 0;
  padding: 14px;
  overflow: auto;
  border: 1px solid var(--theme-border, #e4e7ed);
  border-radius: 14px;
  background: color-mix(in srgb, var(--theme-bg, #f5f7fa) 88%, transparent);
  color: var(--theme-text, #303133);
  font-family: Consolas, Monaco, 'Courier New', monospace;
  font-size: 12px;
  line-height: 1.65;
  white-space: pre-wrap;
  word-break: break-all;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

@media (max-width: 900px) {
  .preset-grid,
  .field-grid,
  .export-options-grid {
    grid-template-columns: 1fr;
  }
}
</style>
