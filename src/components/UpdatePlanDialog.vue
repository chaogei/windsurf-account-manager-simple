<template>
  <el-dialog
    v-model="visible"
    :title="$t('dialog.updatePlan.title')"
    width="1100px"
    class="plan-dialog"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <div class="plan-selection">
      <!-- 当前套餐信息 -->
      <div v-if="account?.plan_name" class="current-plan-info">
        <div class="info-left">
          <div class="info-label">{{ $t('dialog.updatePlan.currentPlan') }}</div>
          <div class="info-value">
            <el-tag :class="['plan-tag', `plan-${account.plan_name?.toLowerCase()}`]" effect="dark">
              <el-icon><Trophy /></el-icon>
              {{ account.plan_name }}
            </el-tag>
          </div>
        </div>
        <div class="info-right" v-if="account.total_quota">
          <div class="quota-label">{{ $t('dialog.updatePlan.quotaUsage') }}</div>
          <div class="quota-value">
            <span class="used">{{ formatQuota(account.used_quota) }}</span>
            <span class="separator">/</span>
            <span class="total">{{ formatQuota(account.total_quota) }}</span>
          </div>
          <el-progress 
            :percentage="Math.min(Math.round((account.used_quota || 0) / (account.total_quota || 1) * 100), 100)" 
            :status="getQuotaStatus((account.used_quota || 0) / (account.total_quota || 1))"
            :stroke-width="6"
            :show-text="false"
            class="quota-progress"
          />
        </div>
      </div>
      
      <div class="plans-container">
        <div 
          v-for="plan in planConfigs"
          :key="plan.key"
          class="plan-card"
          :class="{ 
            active: selectedPlan === plan.key,
            'is-current': isCurrentPlan(plan.key)
          }"
          :style="{ '--theme-color': plan.color, '--theme-bg': plan.color + '15', '--theme-shadow': plan.color + '25' }"
          @click="selectedPlan = plan.key as PlanType"
        >
          <div class="tier-badge">Tier {{ plan.tier }}</div>
          <div class="card-header">
            <div class="icon-wrapper" :style="{ background: plan.color + '20', color: plan.color }">
              <el-icon>
                <component :is="getIconComponent(plan.icon)" />
              </el-icon>
            </div>
            <h3>{{ plan.name }}</h3>
            <p class="subtitle">{{ plan.desc }}</p>
          </div>
          
          <div class="card-body">
            <div class="features-list">
              <div v-for="feature in plan.features" :key="feature" class="feature-item">
                <el-icon :style="{ color: plan.color }"><Check /></el-icon>
                <span>{{ feature }}</span>
              </div>
            </div>
          </div>

          <div class="card-footer">
            <el-button 
              class="select-btn"
              :type="selectedPlan === plan.key ? 'primary' : 'default'"
              :style="selectedPlan === plan.key ? { background: plan.color, borderColor: plan.color } : {}"
              :disabled="isCurrentPlan(plan.key)"
              round
              size="small"
            >
              {{ isCurrentPlan(plan.key) ? $t('dialog.updatePlan.currentPlanTag') : (selectedPlan === plan.key ? $t('dialog.updatePlan.selectedTag') : $t('dialog.updatePlan.selectPlan')) }}
            </el-button>
          </div>
          
          <div v-if="isCurrentPlan(plan.key)" class="current-badge">{{ $t('dialog.updatePlan.currentlyInUse') }}</div>
        </div>
      </div>
      
      <!-- 付款周期选择 -->
      <div class="payment-period-section">
        <div class="section-title">
          <el-icon><Calendar /></el-icon>
          <span>{{ $t('dialog.updatePlan.paymentPeriod') }}</span>
        </div>
        <el-radio-group v-model="paymentPeriod" :disabled="isLooping">
          <el-radio-button :value="1">
            <el-icon><Clock /></el-icon>
            {{ $t('dialog.updatePlan.monthly') }}
          </el-radio-button>
          <el-radio-button :value="2">
            <el-icon><Calendar /></el-icon>
            {{ $t('dialog.updatePlan.yearly') }}
          </el-radio-button>
        </el-radio-group>
        <el-button
          type="info"
          plain
          size="small"
          @click="executePreview"
          :loading="loading"
          :disabled="!selectedPlan || isLooping"
          style="margin-left: 16px;"
        >
          <el-icon><View /></el-icon>
          {{ $t('dialog.updatePlan.previewBilling') }}
        </el-button>
      </div>

      <!-- 计费预览结果 -->
      <div v-if="billingPreview" class="billing-preview">
        <div class="preview-header">
          <el-icon><Ticket /></el-icon>
          <span>{{ $t('dialog.updatePlan.billingPreview') }}</span>
        </div>
        <div class="preview-content">
          <div class="preview-item" v-if="billingPreview.amount_due_immediately !== undefined">
            <span class="label">{{ $t('dialog.updatePlan.immediateDue') }}</span>
            <span class="value">${{ billingPreview.amount_due_immediately?.toFixed(2) }}</span>
          </div>
          <div class="preview-item" v-if="billingPreview.price_per_seat !== undefined">
            <span class="label">{{ $t('dialog.updatePlan.pricePerSeat') }}</span>
            <span class="value">${{ billingPreview.price_per_seat?.toFixed(2) }}</span>
          </div>
          <div class="preview-item" v-if="billingPreview.num_seats !== undefined">
            <span class="label">{{ $t('dialog.updatePlan.seatCount') }}</span>
            <span class="value">{{ billingPreview.num_seats }}</span>
          </div>
          <div class="preview-item" v-if="billingPreview.amount_per_interval !== undefined">
            <span class="label">{{ $t('dialog.updatePlan.amountPerInterval') }}</span>
            <span class="value">${{ billingPreview.amount_per_interval?.toFixed(2) }}{{ billingPreview.sub_interval_name === 'yearly' ? $t('dialog.updatePlan.perYear') : $t('dialog.updatePlan.perMonth') }}</span>
          </div>
          <div class="preview-item" v-if="billingPreview.billing_start">
            <span class="label">{{ $t('dialog.updatePlan.billingStart') }}</span>
            <span class="value">{{ billingPreview.billing_start }}</span>
          </div>
          <div class="preview-item" v-if="billingPreview.billing_end">
            <span class="label">{{ $t('dialog.updatePlan.billingEnd') }}</span>
            <span class="value">{{ billingPreview.billing_end }}</span>
          </div>
        </div>
      </div>

      <div v-if="error" class="error-container">
        <el-alert
          :title="error"
          type="error"
          show-icon
          :closable="false"
        />
      </div>

      <!-- 循环更换设置 -->
      <div class="loop-settings">
        <div class="loop-header">
          <div class="loop-title">
            <el-icon><Refresh /></el-icon>
            <span>{{ $t('dialog.updatePlan.loopMode') }}</span>
          </div>
          <el-switch v-model="loopMode" :disabled="isLooping" />
        </div>
        <p class="loop-desc">{{ $t('dialog.updatePlan.loopModeDesc') }}</p>
        
        <!-- 循环执行状态 -->
        <div v-if="isLooping || loopStats.totalAttempts > 0" class="loop-status">
          <div class="status-row">
            <div class="stat-item success">
              <el-icon><SuccessFilled /></el-icon>
              <span>{{ $t('dialog.updatePlan.success') }}: {{ loopStats.successCount }}</span>
            </div>
            <div class="stat-item failed">
              <el-icon><CircleCloseFilled /></el-icon>
              <span>{{ $t('dialog.updatePlan.failed') }}: {{ loopStats.failedCount }}</span>
            </div>
            <div class="stat-item total">
              <el-icon><DataLine /></el-icon>
              <span>{{ $t('dialog.updatePlan.total') }}: {{ loopStats.totalAttempts }}</span>
            </div>
          </div>
          <div v-if="loopStats.consecutiveFailures > 0" class="consecutive-warn">
            <el-icon><Warning /></el-icon>
            {{ $t('dialog.updatePlan.consecutiveFailures') }}
          </div>
          <div v-if="loopStats.lastError" class="last-error">
            <el-icon><InfoFilled /></el-icon>
            {{ loopStats.lastError }}
          </div>
        </div>
      </div>

      <!-- 订阅管理区域 -->
      <div class="subscription-management">
        <div class="management-header">
          <span class="title">{{ $t('dialog.updatePlan.subscriptionManagement') }}</span>
          <span class="subtitle">{{ $t('dialog.updatePlan.manageSubscriptionStatus') }}</span>
        </div>

        <div class="subscription-actions">
          <el-button
            type="danger"
            plain
            @click="handleCancelSubscription"
            :loading="cancelLoading"
            class="action-btn"
          >
            <el-icon><CircleClose /></el-icon>
            {{ $t('dialog.updatePlan.cancelSubscription') }}
          </el-button>

          <el-button
            type="success"
            plain
            @click="handleResumeSubscription"
            :loading="resumeLoading"
            class="action-btn"
          >
            <el-icon><CircleCheck /></el-icon>
            {{ $t('dialog.updatePlan.resumeSubscription') }}
          </el-button>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose" :disabled="isLooping">{{ $t('dialog.updatePlan.cancel') }}</el-button>
        <el-button
          v-if="isLooping"
          type="danger"
          @click="stopLoop"
        >
          <el-icon><VideoPause /></el-icon>
          {{ $t('dialog.updatePlan.stopLoop') }}
        </el-button>
        <el-button
          v-else
          type="primary"
          @click="handleConfirm"
          :loading="loading"
          :disabled="!selectedPlan"
        >
          {{ loopMode ? $t('dialog.updatePlan.startLoopReplacement') : $t('dialog.updatePlan.confirmChange') }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, watch, computed } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useI18n } from 'vue-i18n';
import { 
  UserFilled, OfficeBuilding, Check, Star, Trophy, CircleClose, CircleCheck, 
  Refresh, SuccessFilled, CircleCloseFilled, DataLine, Warning, InfoFilled, 
  VideoPause, Connection, Monitor, Clock, Medal, Promotion, Briefcase, Calendar, Grid,
  View, Ticket
} from '@element-plus/icons-vue';
import type { Component } from 'vue';
import { apiService } from '@/api';
import type { Account } from '@/types';

const props = defineProps<{
  modelValue: boolean;
  accountId: string;
  account?: Account;
}>();

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  'success': [];
}>();

const { t } = useI18n();

const visible = ref(props.modelValue);
// 所有可用的订阅计划类型
type PlanType = 'teams' | 'pro' | 'enterprise_saas' | 'hybrid' | 'enterprise_self_hosted' | 'waitlist_pro' | 'teams_ultimate' | 'pro_ultimate' | 'trial' | 'enterprise_self_serve' | 'enterprise_saas_pooled' | '';
const selectedPlan = ref<PlanType>('');
// 付款周期: 1=月付, 2=年付
const paymentPeriod = ref<number>(1);

// 订阅计划配置 - 使用 traductions dynamiques
const planConfigs = computed(() => [
  { key: 'teams', name: 'Teams', tier: 1, icon: 'UserFilled', color: '#10b981', desc: t('dialog.updatePlan.planFeatures.teams.name'), features: t('dialog.updatePlan.planFeatures.teams.features') },
  { key: 'pro', name: 'Pro', tier: 2, icon: 'Star', color: '#3b82f6', desc: t('dialog.updatePlan.planFeatures.pro.name'), features: t('dialog.updatePlan.planFeatures.pro.features') },
  { key: 'enterprise_saas', name: 'Enterprise SaaS', tier: 3, icon: 'OfficeBuilding', color: '#8b5cf6', desc: t('dialog.updatePlan.planFeatures.enterprise_saas.name'), features: t('dialog.updatePlan.planFeatures.enterprise_saas.features') },
  { key: 'hybrid', name: 'Hybrid', tier: 4, icon: 'Connection', color: '#f59e0b', desc: t('dialog.updatePlan.planFeatures.hybrid.name'), features: t('dialog.updatePlan.planFeatures.hybrid.features') },
  { key: 'enterprise_self_hosted', name: 'Enterprise Self-Hosted', tier: 5, icon: 'Monitor', color: '#ec4899', desc: t('dialog.updatePlan.planFeatures.enterprise_self_hosted.name'), features: t('dialog.updatePlan.planFeatures.enterprise_self_hosted.features') },
  { key: 'waitlist_pro', name: 'Waitlist Pro', tier: 6, icon: 'Clock', color: '#6366f1', desc: t('dialog.updatePlan.planFeatures.waitlist_pro.name'), features: t('dialog.updatePlan.planFeatures.waitlist_pro.features') },
  { key: 'teams_ultimate', name: 'Teams Ultimate', tier: 7, icon: 'Trophy', color: '#14b8a6', desc: t('dialog.updatePlan.planFeatures.teams_ultimate.name'), features: t('dialog.updatePlan.planFeatures.teams_ultimate.features') },
  { key: 'pro_ultimate', name: 'Pro Ultimate', tier: 8, icon: 'Medal', color: '#f97316', desc: t('dialog.updatePlan.planFeatures.pro_ultimate.name'), features: t('dialog.updatePlan.planFeatures.pro_ultimate.features') },
  { key: 'trial', name: 'Trial', tier: 9, icon: 'Promotion', color: '#84cc16', desc: t('dialog.updatePlan.planFeatures.trial.name'), features: t('dialog.updatePlan.planFeatures.trial.features') },
  { key: 'enterprise_self_serve', name: 'Enterprise Self-Serve', tier: 10, icon: 'Briefcase', color: '#a855f7', desc: t('dialog.updatePlan.planFeatures.enterprise_self_serve.name'), features: t('dialog.updatePlan.planFeatures.enterprise_self_serve.features') },
  { key: 'enterprise_saas_pooled', name: 'Enterprise SaaS Pooled', tier: 11, icon: 'Grid', color: '#0891b2', desc: t('dialog.updatePlan.planFeatures.enterprise_saas_pooled.name'), features: t('dialog.updatePlan.planFeatures.enterprise_saas_pooled.features') },
]);
const loading = ref(false);
const cancelLoading = ref(false);
const resumeLoading = ref(false);
const error = ref('');

// 图标组件映射
const iconMap: Record<string, Component> = {
  UserFilled, OfficeBuilding, Star, Trophy, Connection, Monitor, Clock, Medal, Promotion, Briefcase, Check, Grid
};

// 获取图标组件
function getIconComponent(iconName: string): Component {
  return iconMap[iconName] || Star;
}

// 判断是否为当前套餐
function isCurrentPlan(planKey: string): boolean {
  const currentPlan = props.account?.plan_name?.toLowerCase();
  if (!currentPlan) return false;
  // 处理一些特殊映射
  if (planKey === 'enterprise' && currentPlan.includes('enterprise')) return true;
  return currentPlan === planKey || currentPlan.replace(/[_-]/g, '') === planKey.replace(/[_-]/g, '');
}

// 循环更换相关状态
const loopMode = ref(false);
const isLooping = ref(false);
const shouldStopLoop = ref(false);
const loopStats = reactive({
  successCount: 0,
  failedCount: 0,
  totalAttempts: 0,
  consecutiveFailures: 0,
  lastError: ''
});

// 预览模式
const previewMode = ref(false);
const billingPreview = ref<{
  amount_due_immediately?: number;
  price_per_seat?: number;
  num_seats?: number;
  sub_interval_name?: string;
  amount_per_interval?: number;
  billing_start?: string;
  billing_end?: string;
} | null>(null);

// 取消原因选项
const cancelReasons = [
  { value: 'too_expensive', label: t('dialog.updatePlan.cancelReasons.too_expensive') },
  { value: 'not_using', label: t('dialog.updatePlan.cancelReasons.not_using') },
  { value: 'missing_features', label: t('dialog.updatePlan.cancelReasons.missing_features') },
  { value: 'switching_service', label: t('dialog.updatePlan.cancelReasons.switching_service') },
  { value: 'other', label: t('dialog.updatePlan.cancelReasons.other') }
];

watch(() => props.modelValue, (val) => {
  visible.value = val;
  if (val) {
    selectedPlan.value = '';
    error.value = '';
    loopMode.value = false;
    isLooping.value = false;
    shouldStopLoop.value = false;
    previewMode.value = false;
    billingPreview.value = null;
    resetLoopStats();
  }
});

watch(visible, (val) => {
  emit('update:modelValue', val);
});

function getQuotaStatus(percentage: number) {
  if (percentage >= 0.9) return 'exception';
  if (percentage >= 0.75) return 'warning';
  return 'success';
}

// 重置循环统计
function resetLoopStats() {
  loopStats.successCount = 0;
  loopStats.failedCount = 0;
  loopStats.totalAttempts = 0;
  loopStats.consecutiveFailures = 0;
  loopStats.lastError = '';
}

// 停止循环
function stopLoop() {
  shouldStopLoop.value = true;
  ElMessage.info('正在停止循环...');
}

// 执行单次更换
// 返回: success=是否成功, hasReason=失败时是否有明确原因
async function executeSingleUpdate(): Promise<{ success: boolean; hasReason: boolean }> {
  try {
    const result = await apiService.updatePlan(props.accountId, selectedPlan.value, paymentPeriod.value, false);
    if (result.success) {
      return { success: true, hasReason: false };
    } else {
      const reason = result.payment_failure_reason;
      loopStats.lastError = reason || '更换计划失败';
      // 有明确原因表示支付问题（如卡号错误），不计入连续失败
      return { success: false, hasReason: !!reason };
    }
  } catch (err: any) {
    loopStats.lastError = err.toString();
    return { success: false, hasReason: true }; // 异常也算有原因
  }
}

// 执行预览
async function executePreview(): Promise<void> {
  if (!selectedPlan.value) {
    ElMessage.warning('请先选择订阅计划');
    return;
  }
  
  loading.value = true;
  error.value = '';
  billingPreview.value = null;
  
  try {
    const result = await apiService.updatePlan(props.accountId, selectedPlan.value, paymentPeriod.value, true);
    if (result.success && result.billing_update) {
      billingPreview.value = result.billing_update;
      ElMessage.success('预览成功，请查看计费详情');
    } else if (result.payment_failure_reason) {
      error.value = `支付失败: ${result.payment_failure_reason}`;
      ElMessage.error(error.value);
    } else {
      ElMessage.info('预览完成，无计费变更');
    }
  } catch (err: any) {
    error.value = err.toString();
    ElMessage.error(`预览失败: ${err}`);
  } finally {
    loading.value = false;
  }
}

// 延迟函数
function delay(ms: number): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function handleConfirm() {
  if (!selectedPlan.value) {
    ElMessage.warning('请选择订阅计划');
    return;
  }

  // 如果开启循环模式
  if (loopMode.value) {
    isLooping.value = true;
    shouldStopLoop.value = false;
    resetLoopStats();
    error.value = '';

    while (!shouldStopLoop.value && loopStats.consecutiveFailures < 3) {
      loopStats.totalAttempts++;
      
      const result = await executeSingleUpdate();
      
      if (result.success) {
        loopStats.successCount++;
        loopStats.consecutiveFailures = 0; // 重置连续失败计数
        loopStats.lastError = '';
      } else {
        loopStats.failedCount++;
        // 只有无明确原因的失败才计入连续失败次数
        // 有明确原因（如卡号错误）的失败不计入
        if (!result.hasReason) {
          loopStats.consecutiveFailures++;
        } else {
          // 有原因的失败重置计数器
          loopStats.consecutiveFailures = 0;
        }
      }

      // 检查是否应该停止
      if (shouldStopLoop.value) {
        ElMessage.warning('循环已被手动停止');
        break;
      }

      if (loopStats.consecutiveFailures >= 3) {
        ElMessage.error('连续3次无原因失败，循环已停止');
        error.value = `连续3次无原因失败: ${loopStats.lastError}`;
        break;
      }

      // 短暂延迟，避免请求过快
      await delay(500);
    }

    isLooping.value = false;
    
    // 显示最终统计
    if (loopStats.successCount > 0) {
      ElMessage.success(`循环结束: 成功 ${loopStats.successCount} 次，失败 ${loopStats.failedCount} 次`);
      emit('success');
    }
  } else {
    // 单次执行模式
    loading.value = true;
    error.value = '';
    const periodName = paymentPeriod.value === 2 ? '年付' : '月付';

    try {
      const result = await apiService.updatePlan(props.accountId, selectedPlan.value, paymentPeriod.value, false);
      if (result.success) {
        ElMessage.success(`成功更换到 ${selectedPlan.value.toUpperCase()} 计划（${periodName}）`);
        emit('success');
        handleClose();
      } else {
        error.value = result.payment_failure_reason || '更换计划失败';
        ElMessage.error(error.value);
      }
    } catch (err: any) {
      error.value = err.toString();
      ElMessage.error(`更换计划失败: ${err}`);
    } finally {
      loading.value = false;
    }
  }
}

// 取消订阅
async function handleCancelSubscription() {
  try {
    // 第一步：选择取消原因
    let selectedReason = 'too_expensive'; // 默认值

    const reasonHtml = `
      <div style="text-align: left; padding: 10px 0;">
        <p style="margin-bottom: 12px; color: #606266;">请选择取消订阅的原因：</p>
        <el-radio-group id="cancel-reason-group" style="display: flex; flex-direction: column; gap: 8px;">
          ${cancelReasons.map(r => `
            <label style="display: flex; align-items: center; padding: 8px; cursor: pointer; border-radius: 4px; transition: background 0.2s;"
                   onmouseover="this.style.background='#f5f7fa'"
                   onmouseout="this.style.background='transparent'">
              <input type="radio" name="cancel-reason" value="${r.value}" ${r.value === 'too_expensive' ? 'checked' : ''}
                     style="margin-right: 8px;"
                     onchange="window.__selectedCancelReason='${r.value}'">
              <span style="color: #303133;">${r.label}</span>
            </label>
          `).join('')}
        </el-radio-group>
      </div>
    `;

    // 初始化全局变量
    (window as any).__selectedCancelReason = 'too_expensive';

    await ElMessageBox.confirm(reasonHtml, '取消订阅确认', {
      confirmButtonText: '确认取消',
      cancelButtonText: '返回',
      type: 'warning',
      dangerouslyUseHTMLString: true,
      beforeClose: async (action, instance, done) => {
        if (action === 'confirm') {
          selectedReason = (window as any).__selectedCancelReason || 'too_expensive';

          instance.confirmButtonLoading = true;
          instance.confirmButtonText = '取消中...';

          try {
            console.log('取消订阅，原因:', selectedReason);
            const result = await apiService.cancelSubscription(props.accountId, selectedReason);

            if (result.success) {
              ElMessage.success('订阅已成功取消');
              emit('success');
              done();
              handleClose();
            } else {
              ElMessage.error(result.raw_response || '取消订阅失败');
              instance.confirmButtonLoading = false;
              instance.confirmButtonText = '确认取消';
            }
          } catch (err: any) {
            ElMessage.error(`取消订阅失败: ${err}`);
            instance.confirmButtonLoading = false;
            instance.confirmButtonText = '确认取消';
          } finally {
            // 清理全局变量
            delete (window as any).__selectedCancelReason;
          }
        } else {
          // 清理全局变量
          delete (window as any).__selectedCancelReason;
          done();
        }
      }
    });
  } catch (err) {
    // 用户取消了操作
    console.log('用户取消了取消订阅操作');
    // 清理全局变量
    delete (window as any).__selectedCancelReason;
  }
}

// 恢复订阅
async function handleResumeSubscription() {
  try {
    await ElMessageBox.confirm(
      '确认要恢复订阅吗？恢复后将继续按原计划收费。',
      '恢复订阅确认',
      {
        confirmButtonText: '确认恢复',
        cancelButtonText: '取消',
        type: 'warning'
      }
    );

    resumeLoading.value = true;

    try {
      const result = await apiService.resumeSubscription(props.accountId);

      if (result.success) {
        ElMessage.success('订阅已成功恢复');
        emit('success');
        handleClose();
      } else {
        ElMessage.error(result.raw_response || '恢复订阅失败');
      }
    } catch (err: any) {
      ElMessage.error(`恢复订阅失败: ${err}`);
    } finally {
      resumeLoading.value = false;
    }
  } catch (err) {
    // 用户取消了操作
    console.log('用户取消了恢复订阅操作');
  }
}

// 格式化配额（除以100并显示两位小数）
function formatQuota(num: number | undefined | null) {
  if (!num) return '0.00';
  return (num / 100).toFixed(2);
}

function handleClose() {
  visible.value = false;
}
</script>

<style scoped lang="scss">
.plan-selection {
  padding: 10px;
}

/* 当前套餐信息 */
.current-plan-info {
  background: white;
  border: 1px solid #e4e7ed;
  border-radius: 12px;
  padding: 20px 24px;
  margin-bottom: 30px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);

  .info-left {
    .info-label {
      font-size: 13px;
      color: #909399;
      margin-bottom: 8px;
    }
    
    .plan-tag {
      padding: 6px 16px;
      height: 36px;
      font-size: 14px;
      font-weight: 600;
      border: none;
      
      :deep(.el-icon) {
        margin-right: 6px;
        font-size: 16px;
      }

      &.plan-pro { background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%); }
      &.plan-teams { background: linear-gradient(135deg, #10b981 0%, #059669 100%); }
      &.plan-enterprise { background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%); }
    }
  }

  .info-right {
    text-align: right;
    min-width: 200px;

    .quota-label {
      font-size: 13px;
      color: #909399;
      margin-bottom: 4px;
    }

    .quota-value {
      font-family: 'Roboto Mono', monospace;
      margin-bottom: 6px;
      
      .used { color: #303133; font-weight: 600; font-size: 18px; }
      .separator { margin: 0 4px; color: #c0c4cc; }
      .total { color: #909399; }
    }

    .quota-progress {
      width: 100%;
    }
  }
}

/* 套餐卡片容器 - 适应更多卡片 */
.plans-container {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 12px;
  margin-bottom: 20px;
  max-height: 420px;
  overflow-y: auto;
  padding: 4px;
}

/* 套餐卡片基础样式 - 紧凑版 */
.plan-card {
  position: relative;
  background: #fff;
  border: 2px solid #e4e7ed;
  border-radius: 12px;
  padding: 12px;
  cursor: pointer;
  transition: all 0.25s cubic-bezier(0.4, 0, 0.2, 1);
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 200px;

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.08);
  }

  &.active {
    border-color: var(--theme-color);
    background: var(--theme-bg);
    box-shadow: 0 4px 16px var(--theme-shadow);
  }

  &.is-current {
    border-color: #ffd700;
    border-width: 2px;
  }

  .tier-badge {
    position: absolute;
    top: 8px;
    right: 8px;
    background: rgba(0, 0, 0, 0.05);
    color: #909399;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 600;
  }

  .current-badge {
    position: absolute;
    top: -10px;
    left: 50%;
    transform: translateX(-50%);
    background: linear-gradient(135deg, #ffd700 0%, #ffa500 100%);
    color: #fff;
    padding: 2px 8px;
    border-radius: 10px;
    font-size: 10px;
    font-weight: 600;
    box-shadow: 0 2px 8px rgba(255, 215, 0, 0.3);
    z-index: 1;
    white-space: nowrap;
  }
}

/* 卡片头部 - 紧凑版 */
.card-header {
  text-align: center;
  margin-bottom: 10px;

  .icon-wrapper {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 0 auto 8px;
    font-size: 18px;
    transition: transform 0.2s ease;
  }

  h3 {
    font-size: 13px;
    font-weight: 700;
    margin: 0 0 4px;
    color: #303133;
    line-height: 1.2;
  }

  .subtitle {
    font-size: 11px;
    color: #909399;
    margin: 0;
  }
}

.plan-card:hover .icon-wrapper {
  transform: scale(1.1);
}

/* 卡片主体 - 紧凑版 */
.card-body {
  flex: 1;
  margin-bottom: 10px;
}

.features-list {
  display: flex;
  flex-direction: column;
  gap: 4px;

  .feature-item {
    display: flex;
    align-items: center;
    font-size: 11px;
    color: #606266;
    
    .el-icon {
      margin-right: 4px;
      font-size: 12px;
    }
  }
}

/* 卡片底部 - 紧凑版 */
.card-footer {
  text-align: center;
  
  .select-btn {
    width: 100%;
    height: 28px;
    font-size: 12px;
    font-weight: 600;
    transition: all 0.2s ease;

    &:not(.is-disabled):hover {
      transform: translateY(-1px);
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }
  }
}

/* 动态主题已通过内联样式应用 */

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

  .section-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 14px;
    font-weight: 600;
    color: #92400e;

    .el-icon {
      font-size: 18px;
      color: #d97706;
    }
  }

  :deep(.el-radio-group) {
    .el-radio-button__inner {
      display: flex;
      align-items: center;
      gap: 4px;
      padding: 8px 16px;
    }
  }
}

/* 循环更换设置 */
.loop-settings {
  background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
  border: 1px solid #bae6fd;
  border-radius: 12px;
  padding: 16px 20px;
  margin-bottom: 20px;

  .loop-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;

    .loop-title {
      display: flex;
      align-items: center;
      gap: 8px;
      font-size: 15px;
      font-weight: 600;
      color: #0369a1;

      .el-icon {
        font-size: 18px;
      }
    }
  }

  .loop-desc {
    font-size: 13px;
    color: #64748b;
    margin: 0 0 12px;
  }

  .loop-status {
    background: rgba(255, 255, 255, 0.8);
    border-radius: 8px;
    padding: 12px 16px;

    .status-row {
      display: flex;
      gap: 20px;
      margin-bottom: 8px;

      .stat-item {
        display: flex;
        align-items: center;
        gap: 6px;
        font-size: 14px;
        font-weight: 500;

        &.success {
          color: #16a34a;
          .el-icon { color: #22c55e; }
        }

        &.failed {
          color: #dc2626;
          .el-icon { color: #ef4444; }
        }

        &.total {
          color: #0369a1;
          .el-icon { color: #0ea5e9; }
        }
      }
    }

    .consecutive-warn {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 13px;
      color: #d97706;
      padding: 6px 10px;
      background: #fef3c7;
      border-radius: 6px;
      margin-bottom: 8px;

      .el-icon {
        color: #f59e0b;
      }
    }

    .last-error {
      display: flex;
      align-items: flex-start;
      gap: 6px;
      font-size: 12px;
      color: #64748b;
      padding: 6px 10px;
      background: #f1f5f9;
      border-radius: 6px;
      word-break: break-all;

      .el-icon {
        flex-shrink: 0;
        margin-top: 2px;
      }
    }
  }
}

/* 计费预览 */
.billing-preview {
  background: linear-gradient(135deg, #e8f5e9 0%, #c8e6c9 100%);
  border: 1px solid #81c784;
  border-radius: 12px;
  padding: 16px 20px;
  margin-bottom: 16px;

  .preview-header {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 15px;
    font-weight: 600;
    color: #2e7d32;
    margin-bottom: 12px;

    .el-icon {
      font-size: 18px;
    }
  }

  .preview-content {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 12px;

    .preview-item {
      background: rgba(255, 255, 255, 0.8);
      border-radius: 8px;
      padding: 10px 12px;
      display: flex;
      flex-direction: column;
      gap: 4px;

      .label {
        font-size: 12px;
        color: #666;
      }

      .value {
        font-size: 14px;
        font-weight: 600;
        color: #2e7d32;
      }
    }
  }
}

@media (max-width: 768px) {
  .billing-preview .preview-content {
    grid-template-columns: repeat(2, 1fr);
  }
}

/* 错误提示 */
.error-container {
  margin-bottom: 30px;
}

/* 订阅管理 */
.subscription-management {
  background: linear-gradient(to right, #fdf6ec, #fff);
  border-left: 4px solid #e6a23c;
  border-radius: 4px;
  padding: 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  
  .management-header {
    display: flex;
    flex-direction: column;
    
    .title {
      font-size: 16px;
      font-weight: 600;
      color: #e6a23c;
      margin-bottom: 4px;
    }
    
    .subtitle {
      font-size: 13px;
      color: #909399;
    }
  }
  
  .subscription-actions {
    display: flex;
    gap: 12px;
    
    .action-btn {
      padding: 8px 20px;
    }
  }
}

/* 响应式适配 */
@media (max-width: 1200px) {
  .plans-container {
    grid-template-columns: repeat(4, 1fr);
  }
}

@media (max-width: 900px) {
  .plans-container {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 768px) {
  .plans-container {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .current-plan-info {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
    
    .info-right {
      width: 100%;
      text-align: left;
    }
  }
  
  .subscription-management {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
    
    .subscription-actions {
      width: 100%;
      justify-content: flex-start;
    }
  }
}

@media (max-width: 500px) {
  .plans-container {
    grid-template-columns: 1fr;
  }
}
</style>
