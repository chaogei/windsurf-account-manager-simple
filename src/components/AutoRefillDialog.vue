<template>
  <el-dialog
    v-model="dialogVisible"
    :title="$t('dialog.autoRefill.title')"
    width="520px"
    :close-on-click-modal="false"
    @open="loadSettings"
  >
    <div class="auto-refill-container">
      <!-- 当前状态 -->
      <div class="status-section">
        <div class="status-header">
          <span class="status-title">{{ $t('dialog.autoRefill.statusTitle') }}</span>
          <el-button
            :type="settings.enabled ? 'danger' : 'success'"
            size="small"
            :loading="saving"
            @click="toggleEnabled"
          >
            {{ settings.enabled ? $t('dialog.autoRefill.disableAutoRefill') : $t('dialog.autoRefill.enableAutoRefillBtn') }}
          </el-button>
        </div>
        <div class="status-description" v-if="settings.enabled">
          <span v-html="$t('dialog.autoRefill.statusDescriptionEnabled', { spent: settings.topUpSpent, budget: settings.monthlyTopUpAmount, amount: settings.topUpIncrement })"></span>
        </div>
        <div class="status-description" v-else>
          {{ $t('dialog.autoRefill.statusDescriptionDisabled') }}
        </div>
      </div>

      <!-- 设置表单 -->
      <div class="settings-section" v-if="settings.enabled">
        <el-divider />
        
        <!-- 月度预算 -->
        <div class="setting-item">
          <div class="setting-label">
            <span class="label-title">{{ $t('dialog.autoRefill.monthlyBudgetLimit') }}</span>
            <span class="label-desc">{{ $t('dialog.autoRefill.monthlyBudgetDesc') }}</span>
          </div>
          <div class="setting-options">
            <span class="currency">$</span>
            <el-radio-group v-model="settings.monthlyTopUpAmount" size="small">
              <el-radio-button :value="120">120</el-radio-button>
              <el-radio-button :value="160">160</el-radio-button>
              <el-radio-button :value="200">200</el-radio-button>
            </el-radio-group>
            <el-input-number
              v-model="customMonthlyBudget"
              :min="40"
              :step="40"
              size="small"
              style="width: 160px; margin-left: 8px;"
              @change="onCustomMonthlyChange"
            />
          </div>
        </div>

        <!-- 充值增量 -->
        <div class="setting-item">
          <div class="setting-label">
            <span class="label-title">{{ $t('dialog.autoRefill.singleRefillAmount') }}</span>
            <span class="label-desc">{{ $t('dialog.autoRefill.singleRefillDesc') }}</span>
          </div>
          <div class="setting-options">
            <span class="currency">$</span>
            <el-radio-group v-model="settings.topUpIncrement" size="small">
              <el-radio-button :value="40">40</el-radio-button>
              <el-radio-button :value="120">120</el-radio-button>
              <el-radio-button :value="200">200</el-radio-button>
            </el-radio-group>
            <el-input-number
              v-model="customIncrement"
              :min="40"
              :step="40"
              size="small"
              style="width: 160px; margin-left: 8px;"
              @change="onCustomIncrementChange"
            />
          </div>
        </div>

        <!-- 使用情况 -->
        <el-divider />
        <div class="usage-section">
          <div class="usage-title">{{ $t('dialog.autoRefill.monthlyUsage') }}</div>
          <div class="usage-bar">
            <el-progress
              :percentage="usagePercentage"
              :stroke-width="12"
              :show-text="false"
              :color="usageColor"
            />
          </div>
          <div class="usage-text">
            {{ $t('dialog.autoRefill.usageText', { used: settings.topUpSpent, total: settings.monthlyTopUpAmount }) }}
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <el-button @click="dialogVisible = false">{{ $t('dialog.autoRefill.cancel') }}</el-button>
      <el-button type="primary" :loading="saving" @click="saveSettings">
        {{ $t('dialog.autoRefill.save') }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'

interface Props {
  modelValue: boolean
  accountId: string
}

const props = defineProps<Props>()
const emit = defineEmits(['update:modelValue'])

const dialogVisible = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

const loading = ref(false)
const saving = ref(false)

const settings = ref({
  enabled: false,
  monthlyTopUpAmount: 120, // 单位：美元
  topUpIncrement: 40,      // 单位：美元
  topUpSpent: 0            // 单位：美元
})

const customMonthlyBudget = ref(120)
const customIncrement = ref(40)

const usagePercentage = computed(() => {
  if (settings.value.monthlyTopUpAmount === 0) return 0
  return Math.min(100, (settings.value.topUpSpent / settings.value.monthlyTopUpAmount) * 100)
})

const usageColor = computed(() => {
  if (usagePercentage.value > 80) return '#f56c6c'
  if (usagePercentage.value > 50) return '#e6a23c'
  return '#67c23a'
})

function onCustomMonthlyChange(val: number) {
  // 确保是 $40 的倍数
  const rounded = Math.round(val / 40) * 40
  customMonthlyBudget.value = rounded
  settings.value.monthlyTopUpAmount = rounded
}

function onCustomIncrementChange(val: number) {
  // 确保是 $40 的倍数
  const rounded = Math.round(val / 40) * 40
  customIncrement.value = rounded
  settings.value.topUpIncrement = rounded
}

async function loadSettings() {
  if (!props.accountId) return
  
  loading.value = true
  try {
    const result = await invoke<any>('get_credit_top_up_settings', {
      id: props.accountId
    })
    
    console.log('[AutoRefill] Settings loaded:', result)
    
    if (result.success) {
      settings.value.enabled = result.top_up_enabled || false
      settings.value.monthlyTopUpAmount = result.monthly_top_up_amount || 120
      settings.value.topUpIncrement = result.top_up_increment || 40
      settings.value.topUpSpent = result.top_up_spent || 0
      customMonthlyBudget.value = settings.value.monthlyTopUpAmount
      customIncrement.value = settings.value.topUpIncrement
    }
  } catch (error: any) {
    console.error('Failed to load settings:', error)
    ElMessage.error($t('dialog.autoRefill.loadFailed') + ': ' + error.toString())
  } finally {
    loading.value = false
  }
}

async function toggleEnabled() {
  settings.value.enabled = !settings.value.enabled
  if (settings.value.enabled) {
    // 启用时使用默认值
    if (settings.value.monthlyTopUpAmount === 0) {
      settings.value.monthlyTopUpAmount = 120
    }
    if (settings.value.topUpIncrement === 0) {
      settings.value.topUpIncrement = 40
    }
  }
  await saveSettings()
}

async function saveSettings() {
  if (!props.accountId) return
  
  saving.value = true
  try {
    const result = await invoke<any>('update_credit_top_up_settings', {
      id: props.accountId,
      enabled: settings.value.enabled,
      monthlyTopUpAmount: settings.value.monthlyTopUpAmount,
      topUpIncrement: settings.value.topUpIncrement
    })
    
    console.log('[AutoRefill] Settings saved:', result)
    
    if (result.success) {
      ElMessage.success(result.message || $t('dialog.autoRefill.saveSuccess'))
    } else {
      ElMessage.error(result.error || $t('dialog.autoRefill.saveFailed'))
    }
  } catch (error: any) {
    console.error('Failed to save settings:', error)
    ElMessage.error($t('dialog.autoRefill.saveFailed') + ': ' + error.toString())
  } finally {
    saving.value = false
  }
}

watch(() => props.modelValue, (val) => {
  if (val) {
    loadSettings()
  }
})
</script>

<style scoped>
.auto-refill-container {
  padding: 10px;
}

.status-section {
  background: linear-gradient(135deg, #fef3e2 0%, #fde8d0 100%);
  border-radius: 12px;
  padding: 20px;
}

.status-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.status-title {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

.status-description {
  color: #606266;
  font-size: 14px;
  line-height: 1.6;
}

.highlight {
  color: #e6a23c;
  font-weight: 600;
}

.settings-section {
  margin-top: 20px;
}

.setting-item {
  margin-bottom: 24px;
}

.setting-label {
  margin-bottom: 12px;
}

.label-title {
  display: block;
  font-size: 15px;
  font-weight: 500;
  color: #303133;
  margin-bottom: 4px;
}

.label-desc {
  display: block;
  font-size: 12px;
  color: #909399;
}

.setting-options {
  display: flex;
  align-items: center;
  gap: 8px;
}

.currency {
  font-size: 14px;
  color: #606266;
  margin-right: 4px;
}

.usage-section {
  background: #f5f7fa;
  border-radius: 8px;
  padding: 16px;
}

.usage-title {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
  margin-bottom: 12px;
}

.usage-bar {
  margin-bottom: 8px;
}

.usage-text {
  font-size: 12px;
  color: #909399;
  text-align: right;
}
</style>
