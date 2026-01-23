<template>
  <el-dialog
    v-model="visible"
    width="900px"
    class="batch-plan-dialog"
    :close-on-click-modal="false"
    :close-on-press-escape="!isRunning"
    :show-close="!isRunning"
    @close="handleClose"
  >
    <template #header>
      <div class="dialog-header">
        <div class="header-icon">
          <el-icon><Trophy /></el-icon>
        </div>
        <div class="header-text">
          <h3>{{ $t('dialog.batchUpdatePlan.title') }}</h3>
          <p>{{ $t('dialog.batchUpdatePlan.description') }}</p>
        </div>
      </div>
    </template>

    <div class="batch-plan-content">
      <!-- 选中账号信息 -->
      <div class="selected-accounts-card">
        <div class="card-icon">
          <el-icon><User /></el-icon>
        </div>
        <div class="card-info">
          <span class="label">{{ $t('dialog.batchUpdatePlan.selectedAccounts') }}</span>
          <span class="count">{{ selectedAccountIds.length }}</span>
        </div>
        <div class="card-badge" v-if="loopMode">
          <el-icon><Refresh /></el-icon>
          <span>{{ $t('dialog.batchUpdatePlan.loopMode') }}</span>
        </div>
      </div>
      
      <!-- 计划选择 -->
      <div class="plan-selection">
        <div class="section-header">
          <el-icon><Medal /></el-icon>
          <span>{{ $t('dialog.batchUpdatePlan.selectTargetPlan') }}</span>
        </div>
        <div class="plan-cards">
          <div 
            v-for="plan in planConfigs"
            :key="plan.key"
            class="plan-card" 
            :class="{ active: selectedPlan === plan.key, disabled: isRunning }"
            :style="{ '--plan-color': plan.color }"
            @click="!isRunning && (selectedPlan = plan.key as PlanType)"
          >
            <div class="tier-label">T{{ plan.tier }}</div>
            <div class="plan-icon" :style="{ background: `linear-gradient(135deg, ${plan.color} 0%, ${plan.color}dd 100%)`, boxShadow: `0 4px 12px ${plan.color}40` }">
              <span class="tier-num">{{ plan.tier }}</span>
            </div>
            <div class="plan-name">{{ plan.name }}</div>
            <div class="plan-desc">{{ plan.desc }}</div>
            <div class="plan-check" v-if="selectedPlan === plan.key" :style="{ background: plan.color }">
              <el-icon><Check /></el-icon>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 付款周期选择 -->
      <div class="payment-period-section">
        <div class="period-label">
          <el-icon><Calendar /></el-icon>
          <span>{{ $t('dialog.batchUpdatePlan.paymentPeriod') }}</span>
        </div>
        <el-radio-group v-model="paymentPeriod" :disabled="isRunning" size="small">
          <el-radio-button :value="1">
            <el-icon><Clock /></el-icon>
            {{ $t('dialog.batchUpdatePlan.monthly') }}
          </el-radio-button>
          <el-radio-button :value="2">
            <el-icon><Calendar /></el-icon>
            {{ $t('dialog.batchUpdatePlan.yearly') }}
          </el-radio-button>
        </el-radio-group>
      </div>

      <!-- 循环模式设置 -->
      <div class="loop-settings" :class="{ active: loopMode }">
        <div class="loop-content">
          <div class="loop-icon">
            <el-icon><Refresh /></el-icon>
          </div>
          <div class="loop-info">
            <div class="loop-title">{{ $t('dialog.batchUpdatePlan.loopModeTitle') }}</div>
            <div class="loop-desc">{{ $t('dialog.batchUpdatePlan.loopModeDesc') }}</div>
          </div>
        </div>
        <el-switch 
          v-model="loopMode" 
          :disabled="isRunning"
          active-color="#6366f1"
        />
      </div>
      
      <!-- 执行状态 -->
      <div v-if="isRunning || stats.totalAttempts > 0" class="execution-panel">
        <div class="panel-header">
          <div class="header-left">
            <el-icon v-if="isRunning" class="is-loading"><Loading /></el-icon>
            <el-icon v-else><SuccessFilled /></el-icon>
            <span>{{ isRunning ? $t('dialog.batchUpdatePlan.running') : $t('dialog.batchUpdatePlan.completed') }}</span>
          </div>
          <el-tag 
            :type="isRunning ? 'primary' : 'success'" 
            effect="dark" 
            size="small"
            round
          >
            {{ isRunning ? $t('dialog.batchUpdatePlan.running') : $t('dialog.batchUpdatePlan.completed') }}
          </el-tag>
        </div>
        
        <!-- 统计卡片 -->
        <div class="stats-cards">
          <div class="stat-card success">
            <el-icon><SuccessFilled /></el-icon>
            <div class="stat-value">{{ stats.successCount }}</div>
            <div class="stat-label">{{ $t('dialog.batchUpdatePlan.success') }}</div>
          </div>
          <div class="stat-card failed">
            <el-icon><CircleCloseFilled /></el-icon>
            <div class="stat-value">{{ stats.failedCount }}</div>
            <div class="stat-label">{{ $t('dialog.batchUpdatePlan.failed') }}</div>
          </div>
          <div class="stat-card total">
            <el-icon><DataLine /></el-icon>
            <div class="stat-value">{{ stats.totalAttempts }}</div>
            <div class="stat-label">{{ $t('dialog.batchUpdatePlan.total') }}</div>
          </div>
          <div class="stat-card progress">
            <el-icon><User /></el-icon>
            <div class="stat-value">{{ stats.processedAccounts }}/{{ selectedAccountIds.length }}</div>
            <div class="stat-label">{{ $t('dialog.batchUpdatePlan.progress') }}</div>
          </div>
        </div>
        
        <!-- 连续失败警告 -->
        <div v-if="stats.consecutiveFailures > 0" class="warning-alert">
          <el-icon><Warning /></el-icon>
          <span>{{ $t('dialog.batchUpdatePlan.consecutiveFailures', { count: stats.consecutiveFailures, total: 3 }) }}</span>
        </div>
        
        <!-- 最后错误 -->
        <div v-if="stats.lastError" class="error-alert">
          <el-icon><InfoFilled /></el-icon>
          <span>{{ stats.lastError }}</span>
        </div>
        
        <!-- 执行日志 -->
        <div v-if="executionLogs.length > 0" class="logs-section">
          <div class="logs-header">
            <div class="header-left">
              <el-icon><Document /></el-icon>
              <span>{{ $t('dialog.batchUpdatePlan.executionLogs') }}</span>
              <el-tag size="small" type="info" effect="plain">{{ executionLogs.length }}</el-tag>
            </div>
            <el-button link size="small" @click="executionLogs = []">
              <el-icon><Delete /></el-icon>
              {{ $t('dialog.batchUpdatePlan.clear') }}
            </el-button>
          </div>
          <div class="logs-container" ref="logsContainer">
            <div 
              v-for="(log, index) in executionLogs.slice(-100)" 
              :key="index" 
              :class="['log-item', log.type]"
            >
              <span class="log-time">{{ log.time }}</span>
              <span class="log-message">{{ log.message }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose" :disabled="isRunning" size="large">
          {{ $t('dialog.batchUpdatePlan.cancel') }}
        </el-button>
        <el-button
          v-if="isRunning"
          type="danger"
          size="large"
          @click="stopExecution"
        >
          <el-icon><VideoPause /></el-icon>
          {{ $t('dialog.batchUpdatePlan.stopExecution') }}
        </el-button>
        <el-button
          v-else
          type="primary"
          size="large"
          @click="startExecution"
          :disabled="!selectedPlan || selectedAccountIds.length === 0"
        >
          <el-icon><VideoPlay /></el-icon>
          {{ loopMode ? $t('dialog.batchUpdatePlan.startLoopUpdate') : $t('dialog.batchUpdatePlan.startBatchUpdate') }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, watch, nextTick } from 'vue';
import { ElMessage } from 'element-plus';
import { 
  User, Refresh, SuccessFilled, CircleCloseFilled, DataLine, Warning, 
  InfoFilled, Loading, VideoPause, VideoPlay, Trophy,
  Medal, Check, Document, Delete, Clock, Calendar
} from '@element-plus/icons-vue';
import { apiService } from '@/api';
import type { Account } from '@/types';

const props = defineProps<{
  modelValue: boolean;
  selectedAccountIds: string[];
  accounts: Account[];
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  'success': [];
}>();

const visible = ref(props.modelValue);

// 所有可用的订阅计划类型
type PlanType = 'teams' | 'pro' | 'enterprise_saas' | 'hybrid' | 'enterprise_self_hosted' | 'waitlist_pro' | 'teams_ultimate' | 'pro_ultimate' | 'trial' | 'enterprise_self_serve' | 'enterprise_saas_pooled' | '';
const selectedPlan = ref<PlanType>('');

// 订阅计划配置
const planConfigs = [
  { key: 'teams', name: 'Teams', tier: 1, color: '#10b981', desc: '团队版' },
  { key: 'pro', name: 'Pro', tier: 2, color: '#3b82f6', desc: '专业版' },
  { key: 'enterprise_saas', name: 'Ent SaaS', tier: 3, color: '#8b5cf6', desc: '企业SaaS' },
  { key: 'hybrid', name: 'Hybrid', tier: 4, color: '#f59e0b', desc: '混合部署' },
  { key: 'enterprise_self_hosted', name: 'Ent Self', tier: 5, color: '#ec4899', desc: '企业自托管' },
  { key: 'waitlist_pro', name: 'Wait Pro', tier: 6, color: '#6366f1', desc: '等待列表' },
  { key: 'teams_ultimate', name: 'Teams Ult', tier: 7, color: '#14b8a6', desc: '团队旗舰' },
  { key: 'pro_ultimate', name: 'Pro Ult', tier: 8, color: '#f97316', desc: 'Pro旗舰' },
  { key: 'trial', name: 'Trial', tier: 9, color: '#84cc16', desc: '试用版' },
  { key: 'enterprise_self_serve', name: 'Ent Self-Serve', tier: 10, color: '#a855f7', desc: '企业自助' },
  { key: 'enterprise_saas_pooled', name: 'Ent Pooled', tier: 11, color: '#0891b2', desc: 'SaaS池化' },
];

const loopMode = ref(false);
// 付款周期: 1=月付, 2=年付
const paymentPeriod = ref<number>(1);
const isRunning = ref(false);
const shouldStop = ref(false);
const currentAccount = ref<Account | null>(null);
const currentLoopCount = ref(1);
const logsContainer = ref<HTMLElement | null>(null);

const stats = reactive({
  successCount: 0,
  failedCount: 0,
  totalAttempts: 0,
  processedAccounts: 0,
  consecutiveFailures: 0,
  lastError: ''
});

interface LogEntry {
  time: string;
  message: string;
  type: 'success' | 'error' | 'info';
}

const executionLogs = ref<LogEntry[]>([]);

watch(() => props.modelValue, (val) => {
  visible.value = val;
  if (val) {
    resetState();
  }
});

watch(visible, (val) => {
  emit('update:modelValue', val);
});

function resetState() {
  selectedPlan.value = '';
  loopMode.value = false;
  isRunning.value = false;
  shouldStop.value = false;
  currentAccount.value = null;
  currentLoopCount.value = 1;
  stats.successCount = 0;
  stats.failedCount = 0;
  stats.totalAttempts = 0;
  stats.processedAccounts = 0;
  stats.consecutiveFailures = 0;
  stats.lastError = '';
  executionLogs.value = [];
}

function addLog(message: string, type: 'success' | 'error' | 'info') {
  const now = new Date();
  const time = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`;
  executionLogs.value.push({ time, message, type });
  
  // 自动滚动到底部
  nextTick(() => {
    if (logsContainer.value) {
      logsContainer.value.scrollTop = logsContainer.value.scrollHeight;
    }
  });
}

function getSelectedAccounts(): Account[] {
  return props.accounts.filter(a => props.selectedAccountIds.includes(a.id));
}

// 返回: success=是否成功, error=错误信息, hasReason=失败时是否有明确原因
async function executeSingleUpdate(account: Account): Promise<{ success: boolean; error?: string; hasReason: boolean }> {
  try {
    const result = await apiService.updatePlan(account.id, selectedPlan.value, paymentPeriod.value, false);
    if (result.success) {
      return { success: true, hasReason: false };
    } else {
      const reason = result.payment_failure_reason;
      return { 
        success: false, 
        error: reason || '更换计划失败',
        hasReason: !!reason  // 有明确原因（如卡号错误）不计入连续失败
      };
    }
  } catch (err: any) {
    return { success: false, error: err.toString(), hasReason: true }; // 异常也算有原因
  }
}

function delay(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// 单个账号独立循环执行（循环模式下使用）
async function executeAccountLoop(account: Account): Promise<void> {
  let consecutiveFailures = 0;  // 连续无原因失败次数
  let loopCount = 0;
  
  while (!shouldStop.value && consecutiveFailures < 3) {
    loopCount++;
    stats.totalAttempts++;
    
    const result = await executeSingleUpdate(account);
    
    if (result.success) {
      stats.successCount++;
      consecutiveFailures = 0;
      addLog(`[${account.email}] 第${loopCount}轮 更换成功`, 'success');
    } else {
      stats.failedCount++;
      stats.lastError = result.error || '未知错误';
      
      // 只有无明确原因的失败才计入连续失败次数
      if (!result.hasReason) {
        consecutiveFailures++;
        addLog(`[${account.email}] 第${loopCount}轮 更换失败(${consecutiveFailures}/3): ${result.error}`, 'error');
      } else {
        // 有明确原因（如卡号错误）的失败重置计数器
        consecutiveFailures = 0;
        addLog(`[${account.email}] 第${loopCount}轮 更换失败(有原因): ${result.error}`, 'error');
      }
    }
    
    if (consecutiveFailures >= 3) {
      addLog(`[${account.email}] 连续3次无原因失败，已停止`, 'error');
      stats.processedAccounts++;
      break;
    }
    
    // 每次执行之间短暂延迟
    await delay(300);
  }
  
  if (shouldStop.value) {
    addLog(`[${account.email}] 用户停止，共执行${loopCount}轮`, 'info');
    stats.processedAccounts++;
  }
}

// 单次并发执行所有账号
async function executeBatchOnce(accounts: Account[]): Promise<void> {
  const tasks = accounts.map(async (account) => {
    if (shouldStop.value) return;
    
    stats.totalAttempts++;
    const result = await executeSingleUpdate(account);
    
    if (result.success) {
      stats.successCount++;
      addLog(`[${account.email}] 更换成功`, 'success');
    } else {
      stats.failedCount++;
      stats.lastError = result.error || '未知错误';
      addLog(`[${account.email}] 更换失败: ${result.error}`, 'error');
    }
    
    stats.processedAccounts++;
  });
  
  await Promise.all(tasks);
}

async function startExecution() {
  if (!selectedPlan.value) {
    ElMessage.warning('请选择目标计划');
    return;
  }

  const selectedAccounts = getSelectedAccounts();
  if (selectedAccounts.length === 0) {
    ElMessage.warning('没有选中的账号');
    return;
  }

  isRunning.value = true;
  shouldStop.value = false;
  stats.successCount = 0;
  stats.failedCount = 0;
  stats.totalAttempts = 0;
  stats.processedAccounts = 0;
  stats.consecutiveFailures = 0;
  stats.lastError = '';
  currentLoopCount.value = 1;

  addLog(`开始并发批量更换订阅到 ${selectedPlan.value.toUpperCase()}（${selectedAccounts.length} 个账号）`, 'info');

  if (loopMode.value) {
    // 循环模式 - 每个账号独立循环，各自执行直到连续3次失败或手动停止
    addLog(`循环模式：每个账号独立循环执行，连续3次失败后停止`, 'info');
    
    // 所有账号并发启动各自的循环
    const tasks = selectedAccounts.map(account => executeAccountLoop(account));
    await Promise.all(tasks);
    
    addLog(`所有账号循环执行完成`, 'info');
  } else {
    // 单次模式 - 并发执行所有账号一次
    addLog(`单次模式：并发执行 ${selectedAccounts.length} 个账号...`, 'info');
    await executeBatchOnce(selectedAccounts);
  }

  isRunning.value = false;
  currentAccount.value = null;
  
  // 显示最终结果
  if (stats.successCount > 0) {
    ElMessage.success(`批量更换完成: 成功 ${stats.successCount} 次，失败 ${stats.failedCount} 次`);
    emit('success');
  } else if (stats.totalAttempts > 0) {
    ElMessage.error('批量更换失败，没有成功的操作');
  }
}

function stopExecution() {
  shouldStop.value = true;
  ElMessage.info('正在停止执行...');
}

function handleClose() {
  if (isRunning.value) {
    return;
  }
  visible.value = false;
}
</script>

<style scoped lang="scss">
.dialog-header {
  display: flex;
  align-items: center;
  gap: 16px;
  
  .header-icon {
    width: 48px;
    height: 48px;
    border-radius: 12px;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    
    .el-icon {
      font-size: 24px;
      color: #fff;
    }
  }
  
  .header-text {
    h3 {
      margin: 0 0 4px 0;
      font-size: 18px;
      font-weight: 600;
      color: #1e293b;
    }
    
    p {
      margin: 0;
      font-size: 13px;
      color: #64748b;
    }
  }
}

.batch-plan-content {
  padding: 4px 0;
}

.selected-accounts-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 20px;
  background: linear-gradient(135deg, #eff6ff 0%, #dbeafe 100%);
  border-radius: 12px;
  margin-bottom: 24px;
  position: relative;
  overflow: hidden;
  
  &::before {
    content: '';
    position: absolute;
    top: -50%;
    right: -20%;
    width: 200px;
    height: 200px;
    background: radial-gradient(circle, rgba(59, 130, 246, 0.15) 0%, transparent 70%);
    border-radius: 50%;
  }
  
  .card-icon {
    width: 44px;
    height: 44px;
    border-radius: 10px;
    background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
    
    .el-icon {
      font-size: 22px;
      color: #fff;
    }
  }
  
  .card-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    
    .label {
      font-size: 12px;
      color: #64748b;
    }
    
    .count {
      font-size: 28px;
      font-weight: 700;
      color: #1e40af;
      line-height: 1;
    }
  }
  
  .card-badge {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
    border-radius: 20px;
    color: #fff;
    font-size: 12px;
    font-weight: 500;
    
    .el-icon {
      font-size: 14px;
    }
  }
}

.plan-selection {
  margin-bottom: 24px;
  
  .section-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
    font-weight: 600;
    color: #374151;
    margin-bottom: 16px;
    
    .el-icon {
      font-size: 18px;
      color: #6366f1;
    }
  }
  
  .plan-cards {
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    gap: 10px;
  }
  
  .plan-card {
    position: relative;
    padding: 12px 8px;
    border: 2px solid #e5e7eb;
    border-radius: 10px;
    cursor: pointer;
    transition: all 0.2s ease;
    text-align: center;
    background: #fff;
    
    &:hover:not(.disabled) {
      border-color: var(--plan-color, #c7d2fe);
      background: #f8faff;
      transform: translateY(-2px);
    }
    
    &.active {
      border-color: var(--plan-color, #6366f1);
      background: linear-gradient(135deg, #eef2ff 0%, #e0e7ff 100%);
      
      .plan-name {
        color: var(--plan-color, #4f46e5);
      }
    }
    
    &.disabled {
      opacity: 0.6;
      cursor: not-allowed;
    }

    .tier-label {
      position: absolute;
      top: 4px;
      left: 4px;
      font-size: 9px;
      font-weight: 600;
      color: #94a3b8;
      background: #f1f5f9;
      padding: 1px 4px;
      border-radius: 3px;
    }
    
    .plan-icon {
      width: 36px;
      height: 36px;
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      margin: 0 auto 8px;
      
      .tier-num {
        font-size: 16px;
        font-weight: 700;
        color: #fff;
      }
    }
    
    .plan-name {
      font-size: 12px;
      font-weight: 600;
      color: #1e293b;
      margin-bottom: 2px;
      line-height: 1.2;
    }
    
    .plan-desc {
      font-size: 10px;
      color: #94a3b8;
    }
    
    .plan-check {
      position: absolute;
      top: 6px;
      right: 6px;
      width: 18px;
      height: 18px;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      
      .el-icon {
        font-size: 12px;
        color: #fff;
      }
    }
  }
}

/* 付款周期选择 */
.payment-period-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
  border: 1px solid #fcd34d;
  border-radius: 10px;
  margin-bottom: 16px;

  .period-label {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    font-weight: 600;
    color: #92400e;

    .el-icon {
      font-size: 16px;
      color: #d97706;
    }
  }

  :deep(.el-radio-group) {
    .el-radio-button__inner {
      display: flex;
      align-items: center;
      gap: 4px;
      padding: 6px 12px;
      font-size: 12px;
    }
  }
}

.loop-settings {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  margin-bottom: 24px;
  transition: all 0.2s ease;
  
  &.active {
    background: linear-gradient(135deg, #eef2ff 0%, #e0e7ff 100%);
    border-color: #c7d2fe;
    
    .loop-icon {
      background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
      
      .el-icon {
        color: #fff;
      }
    }
    
    .loop-title {
      color: #4f46e5;
    }
  }
  
  .loop-content {
    display: flex;
    align-items: center;
    gap: 14px;
  }
  
  .loop-icon {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    background: #e2e8f0;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    
    .el-icon {
      font-size: 20px;
      color: #64748b;
    }
  }
  
  .loop-info {
    .loop-title {
      font-size: 14px;
      font-weight: 600;
      color: #374151;
      margin-bottom: 2px;
    }
    
    .loop-desc {
      font-size: 12px;
      color: #94a3b8;
    }
  }
}

.execution-panel {
  background: #f8fafc;
  border: 1px solid #e2e8f0;
  border-radius: 12px;
  padding: 20px;
  
  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
    
    .header-left {
      display: flex;
      align-items: center;
      gap: 10px;
      font-size: 15px;
      font-weight: 600;
      color: #1e293b;
      
      .el-icon {
        font-size: 20px;
        color: #6366f1;
      }
      
      .is-loading {
        animation: rotating 1s linear infinite;
      }
    }
  }
  
  .stats-cards {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
    margin-bottom: 16px;
    
    .stat-card {
      padding: 14px;
      border-radius: 10px;
      text-align: center;
      
      .el-icon {
        font-size: 20px;
        margin-bottom: 6px;
      }
      
      .stat-value {
        font-size: 22px;
        font-weight: 700;
        line-height: 1.2;
      }
      
      .stat-label {
        font-size: 11px;
        margin-top: 2px;
      }
      
      &.success {
        background: linear-gradient(135deg, #dcfce7 0%, #bbf7d0 100%);
        .el-icon, .stat-value { color: #16a34a; }
        .stat-label { color: #15803d; }
      }
      
      &.failed {
        background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
        .el-icon, .stat-value { color: #dc2626; }
        .stat-label { color: #b91c1c; }
      }
      
      &.total {
        background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%);
        .el-icon, .stat-value { color: #2563eb; }
        .stat-label { color: #1d4ed8; }
      }
      
      &.progress {
        background: linear-gradient(135deg, #ede9fe 0%, #ddd6fe 100%);
        .el-icon, .stat-value { color: #7c3aed; }
        .stat-label { color: #6d28d9; }
      }
    }
  }
  
  .warning-alert {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 16px;
    background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
    border-radius: 8px;
    margin-bottom: 12px;
    font-size: 13px;
    color: #92400e;
    
    .el-icon {
      font-size: 18px;
      color: #d97706;
    }
  }
  
  .error-alert {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 12px 16px;
    background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
    border-radius: 8px;
    margin-bottom: 12px;
    font-size: 12px;
    color: #991b1b;
    word-break: break-all;
    
    .el-icon {
      font-size: 16px;
      color: #dc2626;
      flex-shrink: 0;
      margin-top: 1px;
    }
  }
  
  .logs-section {
    .logs-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 10px;
      
      .header-left {
        display: flex;
        align-items: center;
        gap: 8px;
        font-size: 13px;
        font-weight: 500;
        color: #64748b;
        
        .el-icon {
          font-size: 16px;
        }
      }
    }
    
    .logs-container {
      max-height: 180px;
      overflow-y: auto;
      background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
      border-radius: 10px;
      padding: 12px 16px;
      font-family: 'JetBrains Mono', 'Consolas', 'Monaco', monospace;
      font-size: 12px;
      
      &::-webkit-scrollbar {
        width: 6px;
      }
      
      &::-webkit-scrollbar-track {
        background: transparent;
      }
      
      &::-webkit-scrollbar-thumb {
        background: #475569;
        border-radius: 3px;
      }
      
      .log-item {
        display: flex;
        gap: 12px;
        padding: 4px 0;
        border-bottom: 1px solid rgba(255, 255, 255, 0.05);
        
        &:last-child {
          border-bottom: none;
        }
        
        .log-time {
          color: #64748b;
          flex-shrink: 0;
          font-size: 11px;
        }
        
        .log-message {
          word-break: break-all;
          line-height: 1.5;
        }
        
        &.success .log-message {
          color: #4ade80;
        }
        
        &.error .log-message {
          color: #f87171;
        }
        
        &.info .log-message {
          color: #60a5fa;
        }
      }
    }
  }
}

@keyframes rotating {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding-top: 8px;
}

/* 暗色主题 */
:global(.dark) {
  .dialog-header {
    .header-text {
      h3 { color: #f1f5f9; }
      p { color: #94a3b8; }
    }
  }
  
  .selected-accounts-card {
    background: linear-gradient(135deg, #1e3a5f 0%, #1e40af 100%);
    
    .card-info {
      .label { color: #94a3b8; }
      .count { color: #93c5fd; }
    }
  }
  
  .plan-selection {
    .section-header {
      color: #e2e8f0;
    }
    
    .plan-card {
      background: #1e293b;
      border-color: #334155;

      .tier-label {
        background: #334155;
        color: #64748b;
      }
      
      &:hover:not(.disabled) {
        background: #334155;
        border-color: var(--plan-color, #6366f1);
      }
      
      &.active {
        background: linear-gradient(135deg, #312e81 0%, #3730a3 100%);
        border-color: var(--plan-color, #6366f1);
      }
      
      .plan-name { color: #f1f5f9; }
      .plan-desc { color: #64748b; }
    }
  }
  
  .loop-settings {
    background: #1e293b;
    border-color: #334155;
    
    &.active {
      background: linear-gradient(135deg, #312e81 0%, #3730a3 100%);
      border-color: #6366f1;
    }
    
    .loop-icon {
      background: #334155;
    }
    
    .loop-info {
      .loop-title { color: #e2e8f0; }
      .loop-desc { color: #64748b; }
    }
  }
  
  .execution-panel {
    background: #1e293b;
    border-color: #334155;
    
    .panel-header .header-left {
      color: #f1f5f9;
    }
    
    .stats-cards .stat-card {
      &.success { background: linear-gradient(135deg, #14532d 0%, #166534 100%); }
      &.failed { background: linear-gradient(135deg, #7f1d1d 0%, #991b1b 100%); }
      &.total { background: linear-gradient(135deg, #1e3a8a 0%, #1d4ed8 100%); }
      &.progress { background: linear-gradient(135deg, #4c1d95 0%, #5b21b6 100%); }
    }
    
    .warning-alert {
      background: linear-gradient(135deg, #78350f 0%, #92400e 100%);
      color: #fef3c7;
    }
    
    .error-alert {
      background: linear-gradient(135deg, #7f1d1d 0%, #991b1b 100%);
      color: #fecaca;
    }
  }
}
</style>
