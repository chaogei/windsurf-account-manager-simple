<template>
  <el-dialog
    v-model="visible"
    :title="t('dialog.billing.title')"
    width="800px"
    class="billing-dialog"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <div v-if="loading" class="loading-container">
      <el-icon class="is-loading" size="32"><Loading /></el-icon>
      <p>{{ t("dialog.billing.loading") }}</p>
    </div>

    <div v-else-if="billingData" class="billing-content">
      <!-- 顶部订阅卡片 -->
      <div
        class="subscription-card"
        :class="`plan-${billingData.plan_name?.toLowerCase() || 'free'}`"
      >
        <div class="card-bg-icon">
          <el-icon><Trophy /></el-icon>
        </div>
        <div class="sub-header">
          <div class="plan-info">
            <div class="plan-name">
              <el-icon><Trophy /></el-icon>
              {{ formatPlanName(billingData.plan_name) }}
            </div>
            <div class="plan-status">
              <el-tag
                v-if="billingData.on_trial"
                type="warning"
                effect="dark"
                round
                size="small"
                >{{ t("dialog.billing.trial") }}</el-tag
              >
              <el-tag
                v-if="billingData.subscription_active"
                type="success"
                effect="dark"
                round
                size="small"
                >{{ t("dialog.billing.active") }}</el-tag
              >
              <el-tag
                v-else-if="billingData.subscription_active === false"
                type="danger"
                effect="dark"
                round
                size="small"
                >{{ t("dialog.billing.inactive") }}</el-tag
              >
              <el-tag
                v-if="billingData.cancel_at_period_end"
                type="danger"
                effect="dark"
                round
                size="small"
                >{{ t("dialog.billing.cancelAtEnd") }}</el-tag
              >
            </div>
          </div>
          <div class="sub-price" v-if="billingData.plan_unit_amount">
            <span class="currency">$</span>
            <span class="amount">{{
              billingData.plan_unit_amount.toFixed(2)
            }}</span>
            <span class="unit" v-if="billingData.sub_interval">
              /
              {{
                billingData.sub_interval === "yearly"
                  ? t("dialog.billing.yearly")
                  : t("dialog.billing.monthly")
              }}</span
            >
          </div>
        </div>

        <div
          class="sub-dates"
          v-if="
            billingData.next_billing_date ||
            billingData.subscription_renewal_time
          "
        >
          <div class="date-item" v-if="billingData.subscription_renewal_time">
            <span class="label">{{ t("dialog.billing.renewalTime") }}</span>
            <span class="value">{{
              billingData.subscription_renewal_time
            }}</span>
          </div>
          <div class="date-item" v-if="billingData.next_billing_date">
            <span class="label">{{ t("dialog.billing.nextBilling") }}</span>
            <span class="value">{{ billingData.next_billing_date }}</span>
          </div>
        </div>
      </div>

      <div class="info-grid">
        <!-- 席位使用情况 -->
        <div
          class="info-card seats-card"
          v-if="billingData.num_seats || billingData.num_users"
        >
          <div class="card-title">
            <el-icon><User /></el-icon>
            <span>{{ t("dialog.billing.seatUsage") }}</span>
          </div>
          <div class="card-content">
            <div class="usage-circle-container">
              <el-progress
                type="dashboard"
                :percentage="getSeatUsagePercentage()"
                :color="getSeatUsageColor()"
                :width="120"
                :stroke-width="10"
              >
                <template #default="{ percentage }">
                  <div class="percentage-value">{{ percentage }}%</div>
                  <div class="percentage-label">
                    {{ t("dialog.billing.used") }}
                  </div>
                </template>
              </el-progress>
            </div>
            <div class="usage-details">
              <div class="detail-row">
                <span class="label">{{ t("dialog.billing.totalSeats") }}</span>
                <span class="value"
                  >{{ billingData.num_users || 0 }} /
                  {{ billingData.num_seats || 0 }}</span
                >
              </div>
              <div
                class="detail-row"
                v-if="billingData.num_cascade_users !== undefined"
              >
                <span class="label">Cascade</span>
                <span class="value"
                  >{{ billingData.num_cascade_users || 0 }} /
                  {{ billingData.num_cascade_seats || "-" }}</span
                >
              </div>
              <div
                class="detail-row"
                v-if="billingData.num_core_users !== undefined"
              >
                <span class="label">Core</span>
                <span class="value"
                  >{{ billingData.num_core_users || 0 }} /
                  {{ billingData.num_core_seats || "-" }}</span
                >
              </div>
            </div>
          </div>
        </div>

        <!-- 配额使用情况 -->
        <div class="info-card quota-card" v-if="billingData.total_quota">
          <div class="card-title">
            <el-icon><DataAnalysis /></el-icon>
            <span>{{ t("dialog.billing.quotaUsage") }}</span>
          </div>
          <div class="card-content">
            <div class="quota-main">
              <div class="quota-text">
                <span class="current">{{
                  formatQuota(billingData.used_quota)
                }}</span>
                <span class="divider">/</span>
                <span class="total">{{
                  formatQuota(billingData.total_quota)
                }}</span>
              </div>
              <el-progress
                :percentage="quotaPercentage"
                :stroke-width="12"
                :color="quotaColor"
                :show-text="false"
                class="quota-bar"
              />
            </div>

            <div class="quota-tags">
              <el-tag
                size="small"
                type="info"
                effect="plain"
                v-if="billingData.base_quota"
              >
                {{ t("dialog.billing.base") }}:
                {{ formatQuota(billingData.base_quota) }}
              </el-tag>
              <el-tag
                size="small"
                type="success"
                effect="plain"
                v-if="billingData.extra_credits"
              >
                {{ t("dialog.billing.extra") }}: +{{
                  formatQuota(billingData.extra_credits)
                }}
              </el-tag>
            </div>

            <div class="cache-info" v-if="billingData.cache_limit">
              <div class="cache-header">
                <span
                  >{{ t("dialog.billing.cacheUsage") }} ({{
                    getCacheUsagePercentage()
                  }}%)</span
                >
                <span>{{ formatQuota(billingData.cache_limit) }}</span>
              </div>
              <el-progress
                :percentage="getCacheUsagePercentage()"
                :stroke-width="6"
                :color="
                  getCacheUsageType() === 'danger'
                    ? '#f56c6c'
                    : getCacheUsageType() === 'warning'
                      ? '#e6a23c'
                      : '#67c23a'
                "
                :show-text="false"
              />
            </div>
          </div>
        </div>

        <!-- 支付信息 -->
        <div
          class="info-card payment-card"
          v-if="billingData.payment_method || billingData.plan_unit_amount"
        >
          <div class="card-title">
            <el-icon><CreditCard /></el-icon>
            <span>{{ t("dialog.billing.paymentMethod") }}</span>
          </div>
          <div class="card-content">
            <div class="payment-method" v-if="billingData.payment_method">
              <div class="method-icon">
                <el-icon><CreditCard /></el-icon>
              </div>
              <div class="method-info">
                <div class="method-type">
                  {{ formatPaymentType(billingData.payment_method.type) }}
                </div>
                <div
                  class="method-number"
                  v-if="billingData.payment_method?.last4"
                >
                  **** {{ billingData.payment_method.last4 }}
                </div>
                <div
                  class="method-exp"
                  v-if="billingData.payment_method?.exp_month"
                >
                  有效期: {{ billingData.payment_method.exp_month }}/{{
                    billingData.payment_method.exp_year
                  }}
                </div>
              </div>
            </div>
            <div v-else class="no-payment">
              {{ t("dialog.billing.noPaymentMethod") }}
            </div>

            <div class="invoice-link" v-if="billingData.invoice_url">
              <el-link
                type="primary"
                :href="billingData.invoice_url"
                target="_blank"
              >
                <el-icon><Link /></el-icon>
                {{ t("dialog.billing.viewRecentInvoices") }}
              </el-link>
            </div>
          </div>
        </div>
      </div>

      <!-- 警告信息区域 -->
      <div
        class="alerts-container"
        v-if="
          billingData.failed_payment_message ||
          billingData.top_up_error ||
          isApproachingCacheLimit()
        "
      >
        <el-alert
          v-if="billingData.failed_payment_message"
          :title="`${t('dialog.billing.paymentFailed')}: ${billingData.failed_payment_message}`"
          type="error"
          :closable="false"
          show-icon
          class="mb-10"
        />
        <el-alert
          v-if="billingData.top_up_error"
          :title="`${t('dialog.billing.topUpError')}: ${billingData.top_up_error}`"
          type="warning"
          :closable="false"
          show-icon
          class="mb-10"
        />
        <el-alert
          v-if="isApproachingCacheLimit()"
          :title="`${t('dialog.billing.cacheWarning')}: ${getCacheUsagePercentage()}%`"
          type="warning"
          :closable="false"
          show-icon
        />
      </div>

      <!-- 错误信息 -->
      <el-alert
        v-if="!billingData.success"
        :title="billingData.error || t('dialog.billing.fetchFailed')"
        type="error"
        :closable="false"
        show-icon
      />

      <!-- 原始数据（折叠） -->
      <el-collapse v-if="billingData.raw_data" class="raw-data-collapse">
        <el-collapse-item :title="t('dialog.billing.rawData')">
          <pre class="raw-data">{{
            JSON.stringify(billingData.raw_data, null, 2)
          }}</pre>
        </el-collapse-item>
      </el-collapse>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="handleClose">{{ t("common.close") }}</el-button>
        <el-button type="primary" @click="copyToClipboard" v-if="billingData">
          <el-icon><CopyDocument /></el-icon>
          {{ t("dialog.billing.copyData") }}
        </el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { ElMessage } from "element-plus";
import { useI18n } from "vue-i18n";
import {
  Loading,
  Trophy,
  Link,
  User,
  DataAnalysis,
  CreditCard,
  CopyDocument,
} from "@element-plus/icons-vue";

const { t } = useI18n();

const props = defineProps<{
  modelValue: boolean;
  accountId: string;
  billingData?: any;
  loading?: boolean;
}>();

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
  refresh: [];
}>();

const visible = ref(props.modelValue);
const showFullResponse = ref(false);

watch(
  () => props.modelValue,
  (val) => {
    visible.value = val;
  },
);

watch(visible, (val) => {
  emit("update:modelValue", val);
});

// 配额百分比
const quotaPercentage = computed(() => {
  if (!props.billingData?.total_quota || !props.billingData?.used_quota)
    return 0;
  return Math.min(
    Math.round(
      (props.billingData.used_quota / props.billingData.total_quota) * 100,
    ),
    100,
  );
});

// 配额颜色
const quotaColor = computed(() => {
  const percentage = quotaPercentage.value;
  if (percentage < 50) return "#10b981";
  if (percentage < 80) return "#f59e0b";
  return "#ef4444";
});

// 缓存使用率百分比
function getCacheUsagePercentage() {
  if (
    !props.billingData?.cache_limit ||
    props.billingData?.used_quota === undefined
  )
    return 0;
  const percentage = Math.round(
    (props.billingData.used_quota / props.billingData.cache_limit) * 100,
  );
  return Math.min(percentage, 100);
}

// 缓存使用率标签类型
function getCacheUsageType() {
  const percentage = getCacheUsagePercentage();
  if (percentage < 50) return "success";
  if (percentage < 80) return "warning";
  return "danger";
}

// 是否接近缓存限制
function isApproachingCacheLimit() {
  const percentage = getCacheUsagePercentage();
  return percentage >= 80;
}

// 格式化配额
function formatQuota(num: number | undefined | null) {
  if (!num) return "0.00";
  return (num / 100).toFixed(2);
}

// 计算席位使用百分比
function getSeatUsagePercentage() {
  if (!props.billingData?.num_seats || !props.billingData?.num_users) return 0;
  return Math.min(
    Math.round(
      (props.billingData.num_users / props.billingData.num_seats) * 100,
    ),
    100,
  );
}

// 获取席位使用颜色
function getSeatUsageColor() {
  const percentage = getSeatUsagePercentage();
  if (percentage < 50) return "#10b981";
  if (percentage < 80) return "#f59e0b";
  if (percentage >= 100) return "#ef4444";
  return "#ef4444";
}

// 格式化支付方式
function formatPaymentType(type: string) {
  const types: Record<string, string> = {
    unionpay: t("dialog.billing.paymentTypes.unionpay"),
    card: t("dialog.billing.paymentTypes.card"),
    visa: "Visa",
    mastercard: "MasterCard",
    alipay: t("dialog.billing.paymentTypes.alipay"),
    wechat: t("dialog.billing.paymentTypes.wechat"),
  };
  return types[type?.toLowerCase()] || type || t("dialog.billing.unknown");
}

// 格式化套餐名称
function formatPlanName(name: string) {
  const names: Record<string, string> = {
    pro: t("dialog.billing.plans.pro"),
    teams: t("dialog.billing.plans.teams"),
    enterprise: t("dialog.billing.plans.enterprise"),
    enterprise_self_serve: t("dialog.billing.plans.enterpriseSelfServe"),
    trial: t("dialog.billing.plans.trial"),
    free: t("dialog.billing.plans.free"),
    starter: t("dialog.billing.plans.starter"),
  };
  return names[name?.toLowerCase()] || name || t("dialog.billing.unknown");
}

function handleClose() {
  visible.value = false;
  showFullResponse.value = false;
}

async function copyToClipboard() {
  if (props.billingData) {
    try {
      await navigator.clipboard.writeText(
        JSON.stringify(props.billingData, null, 2),
      );
      ElMessage.success(t("dialog.billing.copiedToClipboard"));
    } catch (error) {
      ElMessage.error(t("dialog.billing.copyFailed"));
    }
  }
}
</script>

<style scoped lang="scss">
.billing-dialog {
  :deep(.el-dialog__body) {
    padding: 20px 24px;
  }
}

.loading-container {
  text-align: center;
  padding: 60px 0;
  color: #909399;

  p {
    margin-top: 16px;
  }
}

.billing-content {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

/* 顶部订阅卡片 */
.subscription-card {
  position: relative;
  padding: 24px;
  border-radius: 16px;
  color: white;
  overflow: hidden;
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.1);
  transition: transform 0.3s ease;

  &:hover {
    transform: translateY(-2px);
  }

  .card-bg-icon {
    position: absolute;
    right: -20px;
    top: -20px;
    font-size: 180px;
    opacity: 0.1;
    transform: rotate(15deg);
  }

  /* 不同套餐的主题色 */
  &.plan-pro {
    background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  }
  &.plan-teams {
    background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  }
  &.plan-enterprise {
    background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
  }
  &.plan-free {
    background: linear-gradient(135deg, #9ca3af 0%, #6b7280 100%);
  }
  &.plan-trial {
    background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  }
  &.plan-enterprise_self_serve {
    background: linear-gradient(135deg, #a855f7 0%, #9333ea 100%);
  }

  .sub-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 24px;
    position: relative;
    z-index: 1;

    .plan-info {
      .plan-name {
        font-size: 24px;
        font-weight: 700;
        display: flex;
        align-items: center;
        gap: 8px;
        margin-bottom: 8px;
      }

      .plan-status {
        display: flex;
        gap: 6px;
      }
    }

    .sub-price {
      text-align: right;
      .currency {
        font-size: 20px;
        opacity: 0.9;
      }
      .amount {
        font-size: 32px;
        font-weight: 700;
      }
      .unit {
        font-size: 14px;
        opacity: 0.8;
      }
    }
  }

  .sub-dates {
    display: flex;
    gap: 32px;
    position: relative;
    z-index: 1;
    padding-top: 16px;
    border-top: 1px solid rgba(255, 255, 255, 0.2);

    .date-item {
      display: flex;
      flex-direction: column;
      gap: 4px;

      .label {
        font-size: 12px;
        opacity: 0.8;
        text-transform: uppercase;
        letter-spacing: 0.5px;
      }

      .value {
        font-size: 14px;
        font-weight: 500;
        font-family: "Roboto Mono", monospace;
      }
    }
  }
}

/* 信息网格 */
.info-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
}

.info-card {
  background: #fff;
  border: 1px solid #e4e7ed;
  border-radius: 12px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  transition: all 0.3s ease;

  &:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
    border-color: #dcdfe6;
  }

  .card-title {
    font-size: 15px;
    font-weight: 600;
    color: #303133;
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 16px;

    .el-icon {
      color: #909399;
    }
  }

  .card-content {
    flex: 1;
  }
}

/* 席位卡片 */
.seats-card {
  .usage-circle-container {
    display: flex;
    justify-content: center;
    margin-bottom: 16px;
    position: relative;

    .percentage-value {
      font-size: 24px;
      font-weight: 700;
      color: #303133;
    }

    .percentage-label {
      font-size: 12px;
      color: #909399;
    }
  }

  .usage-details {
    display: flex;
    flex-direction: column;
    gap: 8px;

    .detail-row {
      display: flex;
      justify-content: space-between;
      font-size: 13px;

      .label {
        color: #606266;
      }
      .value {
        font-family: "Roboto Mono", monospace;
        font-weight: 500;
      }
    }
  }
}

/* 配额卡片 */
.quota-card {
  .quota-main {
    margin-bottom: 16px;

    .quota-text {
      display: flex;
      align-items: baseline;
      gap: 4px;
      margin-bottom: 8px;

      .current {
        font-size: 20px;
        font-weight: 700;
        color: #303133;
      }
      .divider {
        font-size: 14px;
        color: #909399;
      }
      .total {
        font-size: 14px;
        color: #606266;
      }
    }
  }

  .quota-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 16px;
  }

  .cache-info {
    background: #f5f7fa;
    border-radius: 8px;
    padding: 12px;

    .cache-header {
      display: flex;
      justify-content: space-between;
      font-size: 12px;
      color: #606266;
      margin-bottom: 6px;
    }
  }
}

/* 支付卡片 */
.payment-card {
  .payment-method {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 16px;

    .method-icon {
      width: 40px;
      height: 40px;
      background: #f0f2f5;
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 20px;
      color: #606266;
    }

    .method-info {
      flex: 1;

      .method-type {
        font-weight: 600;
        color: #303133;
        font-size: 14px;
      }
      .method-number {
        font-family: "Roboto Mono", monospace;
        color: #606266;
        font-size: 13px;
      }
      .method-exp {
        font-size: 12px;
        color: #909399;
        margin-top: 2px;
      }
    }
  }

  .no-payment {
    text-align: center;
    padding: 20px 0;
    color: #909399;
    font-size: 13px;
    background: #f5f7fa;
    border-radius: 8px;
    margin-bottom: 16px;
  }

  .invoice-link {
    text-align: center;
    border-top: 1px solid #f0f2f5;
    padding-top: 12px;
  }
}

/* 警告和原始数据 */
.alerts-container {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.mb-10 {
  margin-bottom: 10px;
}

.raw-data-collapse {
  border: 1px solid #e4e7ed;
  border-radius: 8px;
  overflow: hidden;

  :deep(.el-collapse-item__header) {
    padding: 0 16px;
    background: #f8f9fa;
  }

  :deep(.el-collapse-item__content) {
    padding: 0;
  }
}

.raw-data {
  margin: 0;
  padding: 16px;
  background: #282c34;
  color: #abb2bf;
  font-size: 12px;
  font-family: "Roboto Mono", monospace;
  overflow-x: auto;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

/* 响应式适配 */
@media (max-width: 768px) {
  .info-grid {
    grid-template-columns: 1fr;
  }

  .subscription-card .sub-header {
    flex-direction: column;
    gap: 16px;

    .sub-price {
      text-align: left;
    }
  }
}

/* 深色模式适配 */
:root.dark {
  .subscription-card.plan-pro {
    background: linear-gradient(135deg, #1e40af 0%, #1d4ed8 100%);
  }
  .subscription-card.plan-teams {
    background: linear-gradient(135deg, #065f46 0%, #059669 100%);
  }
  .subscription-card.plan-enterprise {
    background: linear-gradient(135deg, #5b21b6 0%, #7c3aed 100%);
  }
  .subscription-card.plan-trial {
    background: linear-gradient(135deg, #b45309 0%, #d97706 100%);
  }
  .subscription-card.plan-enterprise_self_serve {
    background: linear-gradient(135deg, #7e22ce 0%, #9333ea 100%);
  }

  .info-card {
    background: #1d1e1f;
    border-color: #4c4d4f;

    .card-title {
      color: #e5eaf3;
    }

    .usage-circle-container .percentage-value {
      color: #e5eaf3;
    }
    .detail-row .label {
      color: #a3a6ad;
    }
    .detail-row .value {
      color: #cfd3dc;
    }

    .quota-text .current {
      color: #e5eaf3;
    }
    .quota-text .total {
      color: #a3a6ad;
    }

    .cache-info {
      background: #262729;
    }

    .payment-method .method-icon {
      background: #262729;
      color: #a3a6ad;
    }
    .method-type {
      color: #e5eaf3;
    }
    .method-number {
      color: #cfd3dc;
    }

    .no-payment {
      background: #262729;
    }
    .invoice-link {
      border-top-color: #4c4d4f;
    }
  }

  .raw-data-collapse {
    border-color: #4c4d4f;
    :deep(.el-collapse-item__header) {
      background: #262729;
      color: #e5eaf3;
    }
  }
}
</style>
