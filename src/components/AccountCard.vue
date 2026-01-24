<template>
  <div
    class="account-card"
    :class="{
      selected: isSelected,
      active: account.status === 'active',
      current: isCurrent,
      'has-tag-color': hasColoredTag,
      'is-disabled': account.is_disabled,
      'subscription-inactive':
        isPaidPlan && account.subscription_active === false,
    }"
    :style="cardBorderStyle"
    @click="handleCardClick"
  >
    <div class="card-header">
      <!-- 拖拽手柄 - simple 版本已禁用
      <div class="drag-handle" :title="$t('dialog.accountCard.dragHandle')">
        <el-icon><Rank /></el-icon>
      </div>
      -->
      <el-checkbox
        :model-value="isSelected"
        @change="handleSelect"
        class="select-checkbox"
      />
      <div class="account-info">
        <div
          class="email"
          :title="$t('account.actions.copyEmail', { email: account.email })"
          @click.stop="copyEmail"
        >
          {{ displayEmail }}
        </div>
        <el-tag
          v-if="account.nickname"
          type="warning"
          size="small"
          effect="light"
          class="nickname-tag"
        >
          {{ account.nickname }}
        </el-tag>
      </div>
      <div class="status-indicator" :class="statusClass">
        <span class="status-dot"></span>
        <span class="status-text">{{ statusText }}</span>
      </div>
    </div>

    <div class="card-body">
      <!-- 配额和套餐信息 -->
      <div class="quota-section" v-if="account.total_quota">
        <div class="quota-header">
          <div class="quota-header-left">
            <el-tag
              v-if="account.plan_name"
              :class="['plan-tag', `plan-${account.plan_name?.toLowerCase()}`]"
              size="small"
            >
              <el-icon>
                <User v-if="account.plan_name?.toLowerCase() === 'free'" />
                <Trophy v-else />
              </el-icon>
              {{ getLocalizedPlanName(account.plan_name) }}
            </el-tag>
          </div>
          <div class="quota-header-right">
            <span class="quota-used">{{
              formatQuota(account.used_quota)
            }}</span>
            <span class="quota-separator">/</span>
            <span class="quota-total">{{
              formatQuota(account.total_quota)
            }}</span>
          </div>
        </div>
        <div class="quota-progress">
          <el-progress
            :percentage="quotaPercentage"
            :stroke-width="8"
            :color="quotaColor"
            :show-text="false"
          />
          <span class="quota-percentage"
            >{{ quotaPercentage }}%
            {{ $t("dialog.accountCard.quotaPercentage") }}</span
          >
        </div>

        <!-- 订阅到期时间（整合在配额区块内） -->
        <div class="quota-expiry" v-if="account.subscription_expires_at">
          <el-icon class="expiry-icon"><Clock /></el-icon>
          <span class="expiry-label">{{ $t("account.quota.expiryDate") }}</span>
          <span class="expiry-date">{{ formattedExpiryDate }}</span>
          <span
            v-if="daysUntilExpiry !== null"
            :class="['expiry-badge', expiryClass]"
          >
            {{ expiryText }}
          </span>
        </div>
      </div>

      <div class="tags">
        <el-tag
          v-for="tag in account.tags"
          :key="tag"
          size="small"
          :style="getTagStyle(tag)"
          class="custom-tag"
        >
          {{ tag }}
        </el-tag>
      </div>

      <!-- 信息标签组 -->
      <div class="info-tags">
        <el-tooltip
          v-if="account.group"
          :content="$t('account.tags.group')"
          placement="top"
        >
          <el-tag size="small" class="info-tag group-tag">
            <el-icon><Folder /></el-icon>
            <span>{{
              account.group === "默认分组"
                ? $t("common.defaultGroup")
                : account.group
            }}</span>
          </el-tag>
        </el-tooltip>

        <el-tooltip
          v-if="account.created_at"
          :content="$t('account.tags.created')"
          placement="top"
        >
          <el-tag size="small" class="info-tag create-tag">
            <el-icon><Calendar /></el-icon>
            <span>{{ formatDate(account.created_at) }}</span>
          </el-tag>
        </el-tooltip>

        <el-tooltip
          v-if="account.token_expires_at"
          :content="tokenExpiryTooltip"
          placement="top"
        >
          <el-tag
            size="small"
            class="info-tag token-tag"
            :type="tokenExpiryType"
          >
            <el-icon><Key /></el-icon>
            <span>{{ formatDate(account.token_expires_at) }}</span>
          </el-tag>
        </el-tooltip>
      </div>
    </div>

    <div class="card-actions">
      <!-- 第一排按钮（6个） -->
      <div class="action-buttons">
        <el-tooltip
          :content="$t('account.actions.batchResetCredits')"
          placement="top"
        >
          <el-button
            size="small"
            :icon="Refresh"
            circle
            type="warning"
            plain
            @click="handleBatchResetTeamCredits"
            :loading="isResettingCredits"
          />
        </el-tooltip>

        <el-tooltip
          :content="$t('account.actions.checkBilling')"
          placement="top"
        >
          <el-button
            size="small"
            :icon="Document"
            circle
            @click="handleGetBilling"
            :loading="isGettingBilling"
          />
        </el-tooltip>

        <el-tooltip :content="$t('account.actions.autoRefill')" placement="top">
          <el-button
            size="small"
            :icon="Money"
            circle
            type="warning"
            plain
            @click="handleAutoRefill"
          />
        </el-tooltip>

        <el-tooltip
          :content="$t('account.actions.creditHistory')"
          placement="top"
        >
          <el-button
            size="small"
            :icon="TrendCharts"
            circle
            @click="handleShowCreditHistory"
            :loading="isLoadingCreditHistory"
          />
        </el-tooltip>

        <el-tooltip :content="refreshButtonTooltip" placement="top">
          <el-button
            size="small"
            :icon="RefreshRight"
            circle
            @click="handleRefreshToken"
            :loading="isRefreshing"
          />
        </el-tooltip>

        <el-tooltip
          :content="$t('account.actions.accountInfo')"
          placement="top"
        >
          <el-button
            size="small"
            :icon="User"
            circle
            @click="handleAccountInfo"
          />
        </el-tooltip>

        <el-tooltip
          :content="$t('account.actions.deleteWindsurfUser')"
          placement="top"
        >
          <el-button
            size="small"
            :icon="UserFilled"
            circle
            type="warning"
            plain
            :loading="deletingUser"
            @click="handleDeleteWindsurfUser"
          />
        </el-tooltip>

        <el-tooltip :content="$t('account.actions.delete')" placement="top">
          <el-button
            size="small"
            :icon="Delete"
            circle
            type="danger"
            plain
            @click="handleDelete"
          />
        </el-tooltip>
      </div>

      <!-- 第二排按钮（5个） -->
      <div class="action-buttons">
        <el-tooltip :content="$t('account.actions.edit')" placement="top">
          <el-button size="small" :icon="Edit" circle @click="handleEdit" />
        </el-tooltip>

        <el-tooltip :content="$t('account.actions.relogin')" placement="top">
          <el-button size="small" :icon="Key" circle @click="handleLogin" />
        </el-tooltip>

        <el-tooltip :content="$t('account.actions.analytics')" placement="top">
          <el-button
            size="small"
            :icon="DataAnalysis"
            circle
            @click="handleShowAnalytics"
          />
        </el-tooltip>

        <el-tooltip
          :content="$t('account.actions.teamSettings')"
          placement="top"
        >
          <el-button
            size="small"
            :icon="Setting"
            circle
            @click="handleTeamSettings"
          />
        </el-tooltip>

        <el-tooltip
          :content="$t('account.actions.teamManagement')"
          placement="top"
        >
          <el-button
            size="small"
            :icon="UserFilled"
            circle
            type="primary"
            plain
            @click="handleTeamManagement"
          />
        </el-tooltip>

        <el-tooltip :content="$t('account.actions.updatePlan')" placement="top">
          <el-button
            size="small"
            :icon="Sell"
            circle
            @click="handleUpdatePlan"
            :loading="isUpdatingPlan"
          />
        </el-tooltip>

        <el-tooltip :content="$t('account.actions.trialLink')" placement="top">
          <el-button
            size="small"
            :icon="Link"
            circle
            @click="handleGetTrialLink"
            :loading="isGettingTrialLink"
          />
        </el-tooltip>

        <el-tooltip
          :content="$t('account.actions.switchAccount')"
          placement="top"
        >
          <el-button
            size="small"
            :icon="Switch"
            circle
            type="success"
            plain
            @click="handleSwitchAccount"
            :loading="isSwitching"
          />
        </el-tooltip>
      </div>
    </div>
  </div>

  <!-- 积分记录对话框 -->
  <CreditHistoryDialog
    v-model="showCreditHistoryDialog"
    :account-id="account.id"
  />

  <!-- 座位更新结果对话框 -->
  <UpdateSeatsResultDialog
    v-model="showSeatsResultDialog"
    :result-data="seatsResultData"
  />

  <!-- 使用分析对话框 -->
  <AnalyticsDialog
    v-model="showAnalyticsDialog"
    :account-id="account.id"
    :account-email="account.email"
  />

  <!-- 团队设置对话框 -->
  <TeamSettingsDialog
    v-model="showTeamSettingsDialog"
    :account-id="account.id"
  />

  <!-- 团队管理对话框 -->
  <TeamManagementDialog
    v-model="showTeamManagementDialog"
    :account-id="account.id"
  />

  <!-- 自动充值设置对话框 -->
  <AutoRefillDialog v-model="showAutoRefillDialog" :account-id="account.id" />

  <!-- 更换订阅对话框 -->
  <UpdatePlanDialog
    v-model="showUpdatePlanDialog"
    :account-id="account.id"
    :account="account"
    @success="handleUpdatePlanSuccess"
  />

  <!-- Turnstile 验证对话框 -->
  <TurnstileDialog
    :visible="showTurnstileDialog"
    @update:visible="showTurnstileDialog = $event"
    @success="handleTurnstileSuccess"
    @cancel="showTurnstileDialog = false"
  />
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage, ElMessageBox } from "element-plus";
import { invoke } from "@tauri-apps/api/core";
import {
  Document,
  RefreshRight,
  Edit,
  Delete,
  Key,
  Clock,
  Calendar,
  Folder,
  User,
  Trophy,
  TrendCharts,
  Link,
  Switch,
  DataAnalysis,
  Setting,
  UserFilled,
  Money,
  Sell,
  Refresh,
} from "@element-plus/icons-vue";
import type { Account } from "@/types";
import { apiService, accountApi } from "@/api";
import { useAccountsStore, useUIStore, useSettingsStore } from "@/store";
import UpdateSeatsResultDialog from "@/components/UpdateSeatsResultDialog.vue";
import CreditHistoryDialog from "@/components/CreditHistoryDialog.vue";
import AnalyticsDialog from "@/components/AnalyticsDialog.vue";
import TeamSettingsDialog from "@/components/TeamSettingsDialog.vue";
import TeamManagementDialog from "@/components/TeamManagementDialog.vue";
import AutoRefillDialog from "@/components/AutoRefillDialog.vue";
import UpdatePlanDialog from "@/components/UpdatePlanDialog.vue";
import TurnstileDialog from "@/components/TurnstileDialog.vue";
import dayjs from "dayjs";
import { maskEmail } from "@/utils/privacy";

const props = defineProps<{
  account: Account;
  isSelected: boolean;
  currentEmail?: string;
}>();

const emit = defineEmits<{
  select: [value: boolean];
  update: [account: Account];
}>();

const accountsStore = useAccountsStore();
const uiStore = useUIStore();
const settingsStore = useSettingsStore();
const { t } = useI18n();

// 是否为当前激活账号
const isCurrent = computed(() => {
  return props.currentEmail && props.account.email === props.currentEmail;
});

// 显示的邮箱（根据隐私模式）
const displayEmail = computed(() => {
  if (settingsStore.settings?.privacyMode) {
    return maskEmail(props.account.email);
  }
  return props.account.email;
});

// 获取标签颜色（优先使用全局标签颜色）
function getTagColor(tagName: string): string | null {
  // 优先使用全局标签的颜色
  const globalTag = settingsStore.tags.find((t) => t.name === tagName);
  if (globalTag?.color) {
    return globalTag.color;
  }
  // 回退到账号保存的颜色
  if (!props.account.tagColors) return null;
  const tagWithColor = props.account.tagColors.find((t) => t.name === tagName);
  return tagWithColor?.color || null;
}

// 解析颜色为RGB值
function parseColor(
  color: string,
): { r: number; g: number; b: number; a: number } | null {
  // 解析RGBA格式
  const rgbaMatch = color.match(
    /rgba?\((\d+),\s*(\d+),\s*(\d+)(?:,\s*([\d.]+))?\)/,
  );
  if (rgbaMatch) {
    return {
      r: parseInt(rgbaMatch[1]),
      g: parseInt(rgbaMatch[2]),
      b: parseInt(rgbaMatch[3]),
      a: parseFloat(rgbaMatch[4] || "1"),
    };
  }

  // 解析HEX格式
  if (color.startsWith("#")) {
    const hex = color.slice(1);
    if (hex.length === 6) {
      return {
        r: parseInt(hex.slice(0, 2), 16),
        g: parseInt(hex.slice(2, 4), 16),
        b: parseInt(hex.slice(4, 6), 16),
        a: 1,
      };
    } else if (hex.length === 8) {
      return {
        r: parseInt(hex.slice(0, 2), 16),
        g: parseInt(hex.slice(2, 4), 16),
        b: parseInt(hex.slice(4, 6), 16),
        a: parseInt(hex.slice(6, 8), 16) / 255,
      };
    }
  }

  return null;
}

// 获取标签样式
function getTagStyle(tagName: string): Record<string, string> {
  const color = getTagColor(tagName);

  // 如果没有找到颜色，返回默认样式（而不是空对象）
  if (!color) {
    return {
      backgroundColor: "rgba(64, 158, 255, 0.1)",
      borderColor: "rgba(64, 158, 255, 0.3)",
      color: "rgba(64, 158, 255, 1)",
      border: "1px solid rgba(64, 158, 255, 0.3)",
    };
  }

  const parsed = parseColor(color);
  if (parsed) {
    const { r, g, b, a } = parsed;
    // 背景色使用低透明度
    const bgAlpha = Math.min(a * 0.2, 0.3);
    // 边框使用稍高透明度
    const borderAlpha = Math.min(a * 0.5, 0.6);
    return {
      backgroundColor: `rgba(${r}, ${g}, ${b}, ${bgAlpha})`,
      borderColor: `rgba(${r}, ${g}, ${b}, ${borderAlpha})`,
      color: `rgba(${r}, ${g}, ${b}, ${Math.max(a, 0.8)})`,
      border: `1px solid rgba(${r}, ${g}, ${b}, ${borderAlpha})`,
    };
  }

  // 如果颜色解析失败，尝试直接使用颜色值
  return {
    backgroundColor: color,
    borderColor: color,
    color: "#fff",
  };
}

// 是否有带颜色的标签
const hasColoredTag = computed(() => {
  // 检查账号的任意标签是否有颜色（全局或账号级别）
  return props.account.tags.some((tagName) => getTagColor(tagName) !== null);
});

// 获取第一个带颜色标签的颜色作为卡片边框色
const primaryTagColor = computed(() => {
  // 遍历账号的标签，找到第一个有颜色的
  for (const tagName of props.account.tags) {
    const color = getTagColor(tagName);
    if (color) return color;
  }
  return null;
});

// 卡片边框样式
const cardBorderStyle = computed(() => {
  const color = primaryTagColor.value;
  if (!color) return {};

  const parsed = parseColor(color);
  if (parsed) {
    const { r, g, b, a } = parsed;
    // 边框透明度
    const borderAlpha = Math.min(a * 0.6, 0.8);
    // 发光效果透明度
    const glowAlpha = Math.min(a * 0.2, 0.3);
    return {
      "--tag-border-color": `rgba(${r}, ${g}, ${b}, ${borderAlpha})`,
      "--tag-glow-color": `rgba(${r}, ${g}, ${b}, ${glowAlpha})`,
      borderColor: `rgba(${r}, ${g}, ${b}, ${borderAlpha})`,
      boxShadow: `0 0 12px rgba(${r}, ${g}, ${b}, ${glowAlpha}), 0 2px 8px rgba(0, 0, 0, 0.06)`,
    };
  }
  return {};
});

const isGettingBilling = ref(false);
const isLoadingCreditHistory = ref(false);
const isRefreshing = ref(false);
const isGettingTrialLink = ref(false);
const deletingUser = ref(false);
const isSwitching = ref(false);
const isResettingCredits = ref(false);
const isUpdatingPlan = ref(false);
const billingData = ref<any>(null);
const showCreditHistoryDialog = ref(false);
const showSeatsResultDialog = ref(false);
const showAnalyticsDialog = ref(false);
const showTeamSettingsDialog = ref(false);
const showTeamManagementDialog = ref(false);
const showAutoRefillDialog = ref(false);
const showUpdatePlanDialog = ref(false);
const showTurnstileDialog = ref(false);
const pendingTurnstileToken = ref("");
const seatsResultData = ref<any>(null);

// 获取本地化后的计划名称
function getLocalizedPlanName(planName?: string): string {
  if (!planName) return "";

  // 规范化键名
  const key = planName.toLowerCase().replace(/\s+/g, "");

  // 映射表：处理后端可能返回的各种格式
  const planMap: Record<string, string> = {
    teams: "teams",
    pro: "pro",
    entsaas: "enterpriseSaaS",
    enterprisesaas: "enterpriseSaaS",
    hybrid: "hybrid",
    entself: "enterpriseSelf",
    enterpriseself: "enterpriseSelf",
    enterpriseselfhosted: "enterpriseSelf",
    waitpro: "waitPro",
    waitlistpro: "waitPro",
    teamsult: "teamsUltimate",
    teamsultimate: "teamsUltimate",
    proult: "proUltimate",
    proultimate: "proUltimate",
    trial: "trial",
    entselfserve: "enterpriseSelfServe",
    enterpriseselfserve: "enterpriseSelfServe",
    entpooled: "enterprisePooled",
    pooledsaas: "enterprisePooled",
    free: "free",
    starter: "starter",
    enterprise: "enterprise",
  };

  const i18nKey = planMap[key] || key;

  // 尝试翻译，如果找不到则返回原字符串
  const translationKey = `dialog.plans.${i18nKey}`;
  const translated = t(translationKey);

  // t() 如果找不到键通常会返回键本身，我们需要检测这种情况
  return translated !== translationKey ? translated : planName;
}

// 判断是否为付费计划（非 Free）
const isPaidPlan = computed(() => {
  const planName = props.account.plan_name?.toLowerCase();
  return planName && planName !== "free";
});

const statusClass = computed(() => {
  // 只有付费计划且 subscription_active 为 false 时才显示未激活
  if (isPaidPlan.value && props.account.subscription_active === false)
    return "status-subscription-inactive";
  // 检查账户禁用状态
  if (props.account.is_disabled) return "status-disabled";
  if (props.account.status === "active") return "status-active";
  if (props.account.status === "inactive") return "status-inactive";
  return "status-error";
});

const statusText = computed(() => {
  // 只有付费计划且 subscription_active 为 false 时才显示未激活
  if (isPaidPlan.value && props.account.subscription_active === false)
    return t("account.status.inactive");
  // 检查账户禁用状态
  if (props.account.is_disabled) return t("account.status.disabled");
  if (props.account.status === "active") return t("account.status.normal");
  if (props.account.status === "inactive") return t("account.status.offline");
  return t("account.status.error");
});

// 配额百分比
const quotaPercentage = computed(() => {
  if (!props.account.total_quota || !props.account.used_quota) return 0;
  return Math.min(
    Math.round((props.account.used_quota / props.account.total_quota) * 100),
    100,
  );
});

// 配额颜色
const quotaColor = computed(() => {
  const percentage = quotaPercentage.value;
  if (percentage < 50) return "#10b981"; // 绿色
  if (percentage < 80) return "#f59e0b"; // 橙色
  return "#ef4444"; // 红色
});

// 刷新按钮提示文本
const refreshButtonTooltip = computed(() => {
  if (!props.account.token_expires_at) {
    return t("account.actions.refreshToken");
  }
  const isExpired = dayjs(props.account.token_expires_at).isBefore(dayjs());
  return isExpired
    ? t("account.actions.refreshTokenExpired")
    : t("account.actions.refreshTokenValid");
});

// 订阅到期日期格式化
const formattedExpiryDate = computed(() => {
  if (!props.account.subscription_expires_at) return "";
  return dayjs(props.account.subscription_expires_at).format(
    "YYYY-MM-DD HH:mm",
  );
});

// 计算距离到期的天数
const daysUntilExpiry = computed(() => {
  if (!props.account.subscription_expires_at) return null;
  const now = dayjs();
  const expiry = dayjs(props.account.subscription_expires_at);
  return expiry.diff(now, "day");
});

// 到期文本
const expiryText = computed(() => {
  const days = daysUntilExpiry.value;
  if (days === null) return "";
  if (days < 0) return t("account.quota.expired");
  if (days === 0) return t("account.quota.expiresToday");
  if (days === 1) return t("account.quota.expiresTomorrow");
  if (days <= 7) return t("account.quota.expiresInDays", { days });
  return t("account.quota.remainingDays", { days });
});

// 到期样式类
const expiryClass = computed(() => {
  const days = daysUntilExpiry.value;
  if (days === null) return "";
  if (days < 0) return "expired";
  if (days <= 7) return "expiring-soon";
  return "normal";
});

// Token过期类型
const tokenExpiryType = computed(() => {
  if (!props.account.token_expires_at) return "info";
  const expiry = dayjs(props.account.token_expires_at);
  const now = dayjs();
  const minutesUntilExpiry = expiry.diff(now, "minutes");
  const hoursUntilExpiry = expiry.diff(now, "hours");

  if (minutesUntilExpiry < 0) return "danger";
  if (minutesUntilExpiry < 60) return "warning"; // 小于1小时显示警告
  if (hoursUntilExpiry <= 24) return "warning"; // 小于24小时显示警告
  return "success";
});

// Token过期提示
const tokenExpiryTooltip = computed(() => {
  if (!props.account.token_expires_at) return t("account.token.none");
  const expiry = dayjs(props.account.token_expires_at);
  const now = dayjs();
  const minutesUntilExpiry = expiry.diff(now, "minutes");
  const hoursUntilExpiry = expiry.diff(now, "hours");
  const daysUntilExpiry = expiry.diff(now, "days");

  if (minutesUntilExpiry < 0) return t("account.token.expired");
  if (minutesUntilExpiry === 0) return t("account.token.expiringSoonMinutes");
  if (minutesUntilExpiry < 5)
    return t("account.token.expiringInMinutes", {
      minutes: minutesUntilExpiry,
    });
  if (minutesUntilExpiry < 60)
    return t("account.token.expiringInMinutes", {
      minutes: minutesUntilExpiry,
    });
  if (hoursUntilExpiry < 24)
    return t("account.token.expiringInHours", { hours: hoursUntilExpiry });
  if (daysUntilExpiry <= 7)
    return t("account.token.expiringInDays", { days: daysUntilExpiry });
  return t("account.token.valid", { days: daysUntilExpiry });
});

function formatDate(date: string) {
  return dayjs(date).format("MM-DD HH:mm");
}

// 格式化配额（除以100并显示两位小数）
function formatQuota(num: number | undefined | null) {
  if (!num) return "0.00";
  return (num / 100).toFixed(2);
}

function handleSelect(value: boolean) {
  emit("select", value);
}

// 点击卡片空白区域触发选择
function handleCardClick(event: MouseEvent) {
  const target = event.target as HTMLElement;

  // 检查是否点击了交互元素
  const isInteractive = target.closest(
    "button, a, input, .el-checkbox, .el-button, .el-tag, .el-tooltip, .el-icon",
  );

  if (!isInteractive) {
    // 点击空白区域，切换选择状态
    emit("select", !props.isSelected);
  }
}

async function copyEmail() {
  try {
    await navigator.clipboard.writeText(props.account.email);
    ElMessage.success("邮箱已复制");
  } catch (error) {
    ElMessage.error("复制失败");
  }
}

async function handleGetBilling() {
  isGettingBilling.value = true;
  try {
    const result = await apiService.getBilling(props.account.id);
    if (result.success) {
      // 显示账单对话框而不是仅仅提示
      uiStore.openBillingDialog(props.account.id);
      // 把账单数据传递给对话框（可以通过store或事件）
      billingData.value = result;
    } else {
      ElMessage.error("账单查询失败");
    }
  } catch (error) {
    ElMessage.error(`操作失败: ${error}`);
  } finally {
    isGettingBilling.value = false;
  }
}

async function handleShowCreditHistory() {
  showCreditHistoryDialog.value = true;
}

async function handleRefreshToken() {
  isRefreshing.value = true;
  try {
    // 检查Token是否过期
    const isTokenExpired =
      !props.account.token_expires_at ||
      dayjs(props.account.token_expires_at).isBefore(dayjs());

    if (isTokenExpired) {
      // Token已过期，执行刷新Token操作
      const result = await apiService.refreshToken(props.account.id);
      if (result.success) {
        // 显示更详细的成功消息
        const message = result.message || "Token刷新成功";
        if (result.old_expires_at && result.old_expires_at !== "未知") {
          ElMessage.success({
            message: `${message}\n旧过期时间: ${new Date(result.old_expires_at).toLocaleString()}\n新过期时间: ${result.expires_at ? new Date(result.expires_at).toLocaleString() : "未知"}`,
            duration: 3000,
            showClose: true,
          });
        } else {
          ElMessage.success(message);
        }
        // Token刷新成功后，重新获取账号信息（包括新的token）
        try {
          const updatedAccount = await accountApi.getAccount(props.account.id);

          // 合并额外的信息
          if (result.token) {
            updatedAccount.token = result.token;
          }
          if (result.expires_at) {
            updatedAccount.token_expires_at = result.expires_at;
          }
          if (result.plan_name) {
            updatedAccount.plan_name = result.plan_name;
          }
          if (result.used_quota !== undefined) {
            updatedAccount.used_quota = result.used_quota;
          }
          if (result.total_quota !== undefined) {
            updatedAccount.total_quota = result.total_quota;
          }
          if (result.subscription_expires_at) {
            updatedAccount.subscription_expires_at =
              result.subscription_expires_at;
          }
          if (result.is_disabled !== undefined) {
            updatedAccount.is_disabled = result.is_disabled;
          }

          // 直接更新store中的账号数据，确保立即同步
          await accountsStore.updateAccount(updatedAccount);
          emit("update", updatedAccount);
        } catch (error) {
          console.error("获取账号信息失败:", error);
          // 如果无法获取最新账号信息，使用现有数据更新
          const updatedAccount = {
            ...props.account,
            status: "active" as const,
          };
          if (result.token) {
            updatedAccount.token = result.token;
          }
          if (result.expires_at) {
            updatedAccount.token_expires_at = result.expires_at;
          }
          if (result.is_disabled !== undefined) {
            updatedAccount.is_disabled = result.is_disabled;
          }
          await accountsStore.updateAccount(updatedAccount);
          emit("update", updatedAccount);
        }
      } else {
        ElMessage.error("Token刷新失败");
      }
    } else {
      // Token仍然有效，只刷新账号信息
      const result = await apiService.getCurrentUser(props.account.id);
      if (result.success && result.user_info) {
        ElMessage.success({
          message: `账号信息已更新\nToken有效期至: ${new Date(props.account.token_expires_at!).toLocaleString()}`,
          duration: 2500,
          showClose: true,
        });

        // 更新账号信息
        const updatedAccount = { ...props.account, status: "active" as const };

        // 更新用户基本信息（包含api_key和禁用状态）
        if (result.user_info.user?.api_key) {
          updatedAccount.windsurf_api_key = result.user_info.user.api_key;
        }
        // 更新账户禁用状态
        if (result.user_info.user?.disable_codeium !== undefined) {
          updatedAccount.is_disabled = result.user_info.user.disable_codeium;
        }

        // 更新套餐信息
        if (result.user_info.plan?.plan_name) {
          updatedAccount.plan_name = result.user_info.plan.plan_name;
        }

        // 更新配额信息
        if (result.user_info.subscription) {
          if (result.user_info.subscription.used_quota !== undefined) {
            updatedAccount.used_quota =
              result.user_info.subscription.used_quota;
          }
          if (result.user_info.subscription.quota !== undefined) {
            updatedAccount.total_quota = result.user_info.subscription.quota;
          }
          if (result.user_info.subscription.expires_at) {
            const expiresTimestamp = result.user_info.subscription.expires_at;
            const expiresDate = dayjs.unix(expiresTimestamp);
            updatedAccount.subscription_expires_at = expiresDate.toISOString();
          }
          // 更新订阅激活状态
          if (result.user_info.subscription.subscription_active !== undefined) {
            updatedAccount.subscription_active =
              result.user_info.subscription.subscription_active;
          }
        }

        updatedAccount.last_quota_update = dayjs().toISOString();
        // 保存到后端数据库
        await accountsStore.updateAccount(updatedAccount);
        emit("update", updatedAccount);
      } else {
        // 显示详细错误信息
        const statusCode = result.status_code;
        const errorMsg = result.error || "未知错误";
        if (statusCode === 401) {
          // Token 实际已失效，尝试自动刷新
          console.log("[AccountCard] Token 已失效 (401)，尝试自动刷新...");
          const refreshResult = await apiService.refreshToken(props.account.id);
          if (refreshResult.success) {
            ElMessage.success({
              message: "Token已自动刷新，请重新操作",
              duration: 3000,
              showClose: true,
            });
            // 更新账号信息
            const updatedAccount = {
              ...props.account,
              status: "active" as const,
            };
            if (refreshResult.token) {
              updatedAccount.token = refreshResult.token;
            }
            if (refreshResult.expires_at) {
              updatedAccount.token_expires_at = refreshResult.expires_at;
            }
            await accountsStore.updateAccount(updatedAccount);
            emit("update", updatedAccount);
          } else {
            ElMessage.error({
              message: `Token已失效且刷新失败\n可能需要重新登录`,
              duration: 5000,
              showClose: true,
            });
            // 更新账户状态为错误
            const errorAccount = { ...props.account, status: "error" as const };
            await accountsStore.updateAccount(errorAccount);
            emit("update", errorAccount);
          }
        } else {
          ElMessage.error({
            message: `获取账号信息失败 (${statusCode || "未知"})\n${errorMsg}`,
            duration: 5000,
            showClose: true,
          });
          // 更新账户状态为错误
          const errorAccount = { ...props.account, status: "error" as const };
          await accountsStore.updateAccount(errorAccount);
          emit("update", errorAccount);
        }
      }
    }
  } catch (error) {
    ElMessage.error(`操作失败: ${error}`);
  } finally {
    isRefreshing.value = false;
  }
}

async function handleLogin() {
  try {
    const result = await apiService.loginAccount(props.account.id);
    if (result.success) {
      ElMessage.success("登录成功");

      // 重新从后端获取完整的账号数据（包括新的 token 和 refresh_token）
      try {
        const updatedAccount = await accountApi.getAccount(props.account.id);

        // 合并返回的额外信息
        if (result.expires_at) {
          updatedAccount.token_expires_at = result.expires_at;
        }
        if (result.plan_name) {
          updatedAccount.plan_name = result.plan_name;
        }
        if (result.used_quota !== undefined) {
          updatedAccount.used_quota = result.used_quota;
        }
        if (result.total_quota !== undefined) {
          updatedAccount.total_quota = result.total_quota;
        }
        if (result.subscription_expires_at) {
          updatedAccount.subscription_expires_at =
            result.subscription_expires_at;
        }
        if (result.is_disabled !== undefined) {
          updatedAccount.is_disabled = result.is_disabled;
        }
        updatedAccount.last_quota_update = dayjs().toISOString();

        // 更新 store 中的账号数据
        await accountsStore.updateAccount(updatedAccount);
        emit("update", updatedAccount);
      } catch (error) {
        console.error("获取账号信息失败:", error);
        // 如果获取失败，使用基本更新
        const updatedAccount = {
          ...props.account,
          status: "active" as const,
          token_expires_at: result.expires_at,
          last_login_at: dayjs().toISOString(),
        };
        if (result.plan_name) updatedAccount.plan_name = result.plan_name;
        if (result.used_quota !== undefined)
          updatedAccount.used_quota = result.used_quota;
        if (result.total_quota !== undefined)
          updatedAccount.total_quota = result.total_quota;
        if (result.subscription_expires_at)
          updatedAccount.subscription_expires_at =
            result.subscription_expires_at;
        emit("update", updatedAccount);
      }
    } else {
      ElMessage.error("登录失败");
    }
  } catch (error) {
    ElMessage.error(`登录失败: ${error}`);
  }
}

function handleEdit() {
  uiStore.openEditAccountDialog(props.account.id);
}

function handleAccountInfo() {
  // 直接打开账户信息对话框
  uiStore.openAccountInfoDialog(props.account.id);
}

function handleShowAnalytics() {
  // 显示使用分析对话框
  showAnalyticsDialog.value = true;
}

function handleTeamSettings() {
  // 显示团队设置对话框
  showTeamSettingsDialog.value = true;
}

function handleTeamManagement() {
  showTeamManagementDialog.value = true;
}

function handleAutoRefill() {
  // 显示自动充值设置对话框
  showAutoRefillDialog.value = true;
}

// 批量重置团队成员积分
async function handleBatchResetTeamCredits() {
  isResettingCredits.value = true;
  try {
    // Step 1: 获取团队成员列表
    const membersResult = await invoke<any>("get_team_members", {
      id: props.account.id,
      groupId: null,
    });

    if (!membersResult.success) {
      ElMessage.error(membersResult.error || "获取团队成员失败");
      return;
    }

    const data = membersResult.data || {};
    let users = data.subMesssage_1 || [];
    if (users && !Array.isArray(users)) {
      users = [users];
    }
    let userRoles = data.subMesssage_2 || [];
    if (userRoles && !Array.isArray(userRoles)) {
      userRoles = [userRoles];
    }

    // 构建成员列表并排除自己
    interface TeamMember {
      api_key: string;
      name: string;
      email: string;
    }
    const otherMembers: TeamMember[] = [];
    const currentEmail = props.account.email?.toLowerCase();

    for (const user of users) {
      const apiKey = user.string_1 || "";
      const name = user.string_2 || "";
      const email = user.string_3 || "";
      const teamStatus = user.int_8 || 0;

      // 只添加已批准的成员，并排除自己（当前账号）
      if (teamStatus !== 1 && email.toLowerCase() !== currentEmail) {
        otherMembers.push({ api_key: apiKey, name, email });
      }
    }

    if (otherMembers.length === 0) {
      ElMessage.warning(t("dialog.messages.noResetableMembers"));
      return;
    }

    // 确认操作
    try {
      await ElMessageBox.confirm(
        `确定要重置 ${otherMembers.length} 位团队成员的积分吗？\n此操作将移除并重新邀请这些成员。`,
        "批量重置团队积分",
        {
          confirmButtonText: "确定",
          cancelButtonText: "取消",
          type: "warning",
        },
      );
    } catch {
      return;
    }

    let successCount = 0;
    let failCount = 0;

    // Step 2: 遍历所有其他成员执行重置
    for (const member of otherMembers) {
      try {
        // 移除成员
        const removeResult = await invoke<any>("remove_team_member", {
          id: props.account.id,
          memberApiKey: member.api_key,
        });

        if (!removeResult.success) {
          console.error(`移除成员 ${member.name} 失败:`, removeResult.error);
          failCount++;
          continue;
        }

        // 重新邀请
        const inviteResult = await invoke<any>("invite_team_members", {
          id: props.account.id,
          users: [{ name: member.name, email: member.email }],
        });

        if (!inviteResult.success) {
          console.error(`邀请成员 ${member.name} 失败:`, inviteResult.error);
          failCount++;
          continue;
        }

        // 自动接受邀请（如果邮箱在管理器中）
        try {
          const accounts = await invoke<any[]>("get_all_accounts");
          const matchedAccount = accounts.find(
            (acc: any) =>
              acc.email?.toLowerCase() === member.email.toLowerCase(),
          );

          if (matchedAccount) {
            await invoke<any>("accept_invitation", {
              id: matchedAccount.id,
              approvalId: "",
            });
          }
        } catch (e) {
          console.log(`自动接受邀请失败 (${member.email}):`, e);
        }

        successCount++;
      } catch (error) {
        console.error(`处理成员 ${member.name} 时出错:`, error);
        failCount++;
      }
    }

    // 显示结果
    if (failCount === 0) {
      ElMessage.success(`成功重置 ${successCount} 位团队成员的积分`);
    } else {
      ElMessage.warning(
        `重置完成：成功 ${successCount} 位，失败 ${failCount} 位`,
      );
    }

    // 刷新账户信息
    accountsStore.refreshAccountToken(props.account);
  } catch (error: any) {
    ElMessage.error("批量重置团队积分失败: " + error.toString());
  } finally {
    isResettingCredits.value = false;
  }
}

// 更换订阅
function handleUpdatePlan() {
  showUpdatePlanDialog.value = true;
}

// 更换订阅成功回调
function handleUpdatePlanSuccess() {
  accountsStore.refreshAccountToken(props.account);
}

// 获取试用绑卡链接 - 根据计划类型决定是否需要 Turnstile 验证
function handleGetTrialLink() {
  // 从设置中获取订阅计划类型
  const teamsTier = settingsStore.settings?.subscriptionPlan ?? 2; // 默认 Pro

  // Pro 计划需要 Turnstile 验证，Teams 计划不需要
  if (teamsTier === 2) {
    showTurnstileDialog.value = true;
  } else {
    // Teams 计划直接调用，不需要 Turnstile
    handleTurnstileSuccess("");
  }
}

// Turnstile 验证成功后的处理
async function handleTurnstileSuccess(turnstileToken: string) {
  pendingTurnstileToken.value = turnstileToken;
  showTurnstileDialog.value = false;

  isGettingTrialLink.value = true;
  try {
    // 从设置中读取订阅参数
    const teamsTier = settingsStore.settings?.subscriptionPlan ?? 2; // 默认 Pro
    const paymentPeriod = settingsStore.settings?.paymentPeriod ?? 1; // 默认月付
    // Pro 计划不能设置团队名称，只有 Teams 才需要
    const teamName =
      teamsTier === 1
        ? settingsStore.settings?.teamName || undefined
        : undefined;
    const seatCount = settingsStore.settings?.seatCount ?? 1;

    // 检查是否启用了自动打开支付页面
    const autoOpen =
      settingsStore.settings?.autoOpenPaymentLinkInWebview || false;
    const autoFill = settingsStore.settings?.autoFillPaymentForm || false;
    const autoSubmit = settingsStore.settings?.autoSubmitPaymentForm || false;

    // 如果启用了自动打开，使用增强的API
    if (autoOpen) {
      // 从store中获取最新的账号数据，确保token是最新的
      const latestAccount = accountsStore.accounts.find(
        (a) => a.id === props.account.id,
      );
      const account = latestAccount || props.account;

      if (!account.token) {
        ElMessage.warning("请先刷新Token后再试");
        isGettingTrialLink.value = false;
        return;
      }

      // 使用增强的支付API
      const { getTrialPaymentLink, autoFillPaymentForm } =
        await import("@/utils/cardGenerator");

      const result = await getTrialPaymentLink(
        account.nickname || account.email,
        account.token,
        true, // 自动打开窗口
        teamsTier,
        paymentPeriod,
        teamName,
        teamsTier === 1 ? seatCount : undefined, // Teams 需要席位
        turnstileToken || undefined, // Pro 需要 Turnstile token
      );

      if (result.success && result.window_opened) {
        ElMessage.success("支付窗口已在Chrome无痕模式下打开");

        // 如果启用了自动填写表单
        if (autoFill && result.virtual_card && result.window_label) {
          // 立即注入填写脚本（只等待1秒让窗口加载）
          setTimeout(async () => {
            try {
              console.log("开始自动填写表单，窗口标签:", result.window_label);

              // 通过Tauri命令注入表单填写代码
              await autoFillPaymentForm(
                result.window_label,
                result.virtual_card,
              );

              ElMessage.success("正在自动填写虚拟卡信息...");

              // 如果启用了自动提交，注入自动提交脚本
              if (autoSubmit) {
                console.log("准备自动提交表单...");
                await invoke("inject_auto_submit_script", {
                  windowLabel: result.window_label,
                });
                ElMessage.warning("已启用自动提交，请注意观察支付流程");
              }

              // 根据设置决定是否显示虚拟卡信息
              const showCardInfo =
                settingsStore.settings?.showVirtualCardInfo || false;
              if (showCardInfo) {
                ElMessageBox.alert(
                  `<div style="text-align: left; font-family: monospace;">
                    <p><strong>卡号:</strong> ${result.virtual_card.card_number}</p>
                    <p><strong>有效期:</strong> ${result.virtual_card.expiry_date}</p>
                    <p><strong>CVC:</strong> ${result.virtual_card.cvv}</p>
                    <p><strong>姓名:</strong> ${result.virtual_card.cardholder_name}</p>
                    <p><strong>地址:</strong> ${result.virtual_card.billing_address.street_address}</p>
                    <p><strong>城市:</strong> ${result.virtual_card.billing_address.city}, ${result.virtual_card.billing_address.state} ${result.virtual_card.billing_address.postal_code}</p>
                  </div>`,
                  "虚拟卡信息（仅用于测试）",
                  {
                    dangerouslyUseHTMLString: true,
                    confirmButtonText: "确定",
                    type: "warning",
                  },
                );
              }
            } catch (fillError) {
              console.error("自动填写表单失败:", fillError);
            }
          }, 1000); // 固定1秒延迟，不再使用pageDelay
        }
      } else {
        ElMessage.error(result.error || "打开支付窗口失败");
      }
    } else {
      // 使用原有的API
      const result = await apiService.getTrialPaymentLink(
        props.account.id,
        teamsTier,
        paymentPeriod,
        teamName,
        teamsTier === 1 ? seatCount : undefined,
        turnstileToken || undefined,
      );

      if (result.success && result.stripe_url) {
        // 复制链接到剪贴板
        try {
          await navigator.clipboard.writeText(result.stripe_url);
          ElMessage.success("Stripe支付链接已复制到剪贴板");

          // 获取设置
          const autoOpen = settingsStore.settings?.autoOpenBrowser ?? true;
          const browserMode =
            settingsStore.settings?.browserMode ?? "incognito";
          const isIncognito = browserMode === "incognito";
          const modeText = isIncognito ? "无痕模式" : "普通模式";
          const openCommand = isIncognito
            ? "open_external_link_incognito"
            : "open_external_link";

          if (autoOpen) {
            // 自动打开浏览器
            try {
              await invoke(openCommand, { url: result.stripe_url });
              ElMessage.success(`已在浏览器${modeText}中打开`);
            } catch (err) {
              ElMessage.error("打开浏览器失败，请手动打开链接");
              console.error("打开链接失败:", err);
            }
          } else {
            // 询问是否在浏览器中打开
            ElMessageBox.confirm(
              `链接已复制到剪贴板，是否在浏览器${modeText}中打开？`,
              "打开链接",
              {
                confirmButtonText: "打开",
                cancelButtonText: "取消",
                type: "info",
              },
            )
              .then(async () => {
                try {
                  await invoke(openCommand, { url: result.stripe_url });
                  ElMessage.success(`已在浏览器${modeText}中打开`);
                } catch (err) {
                  ElMessage.error("打开浏览器失败，请手动打开链接");
                  console.error("打开链接失败:", err);
                }
              })
              .catch(() => {
                // 用户取消，不做任何操作
              });
          }
        } catch (clipboardError) {
          // 如果复制失败，直接显示链接
          const browserMode =
            settingsStore.settings?.browserMode ?? "incognito";
          const isIncognito = browserMode === "incognito";
          const modeText = isIncognito ? "无痕模式" : "普通模式";
          const openCommand = isIncognito
            ? "open_external_link_incognito"
            : "open_external_link";

          ElMessageBox.alert(
            `<div style="word-break: break-all;">${result.stripe_url}</div>`,
            "Stripe支付链接",
            {
              dangerouslyUseHTMLString: true,
              confirmButtonText: `在${modeText}中打开`,
            },
          ).then(async () => {
            try {
              await invoke(openCommand, { url: result.stripe_url });
              ElMessage.success(`已在浏览器${modeText}中打开`);
            } catch (err) {
              ElMessage.error("打开浏览器失败，请手动打开链接");
              console.error("打开链接失败:", err);
            }
          });
        }
      } else {
        ElMessage.error(result.error || "获取支付链接失败");
      }
    }
  } catch (error) {
    ElMessage.error(`获取支付链接失败: ${error}`);
  } finally {
    isGettingTrialLink.value = false;
  }
}

async function handleDelete() {
  try {
    await ElMessageBox.confirm(
      `确定要删除账号 ${props.account.nickname} (${props.account.email}) 吗？`,
      "删除确认",
      {
        confirmButtonText: "删除",
        cancelButtonText: "取消",
        type: "warning",
      },
    );

    await accountsStore.deleteAccount(props.account.id);
    ElMessage.success("账号删除成功");
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(`删除失败: ${error}`);
    }
  }
}

async function handleDeleteWindsurfUser() {
  try {
    await ElMessageBox.confirm(
      `确定要删除 Windsurf 用户 ${props.account.email} 吗？\n\n⚠️ 此操作将从 Windsurf 服务器上删除该用户账号！`,
      "删除 Windsurf 用户",
      {
        confirmButtonText: "确定删除",
        cancelButtonText: "取消",
        type: "error",
      },
    );

    deletingUser.value = true;
    const result = (await invoke("delete_windsurf_user", {
      id: props.account.id,
    })) as any;

    if (result.success) {
      ElMessage.success("Windsurf 用户已删除");
    } else {
      ElMessage.error(result.error || "删除失败");
    }
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(`删除失败: ${error}`);
    }
  } finally {
    deletingUser.value = false;
  }
}

async function handleSwitchAccount() {
  isSwitching.value = true;
  try {
    await ElMessageBox.confirm(
      `确定要切换到账号 ${props.account.nickname || props.account.email} 吗？\n\n此操作将：\n• 使用回调URL自动登录Windsurf\n• 无需重启软件\n• 保持当前工作状态`,
      "切换账号确认",
      {
        confirmButtonText: "确定切换",
        cancelButtonText: "取消",
        type: "warning",
      },
    );

    // 调用后端API执行切号
    const result = await apiService.switchAccount(props.account.id);

    if (result.success) {
      // 显示成功消息
      ElMessage.success({
        message: result.message || "已成功切换账号",
        duration: 5000,
        showClose: true,
      });

      // 根据机器ID重置状态显示不同提示
      if (result.machine_id_reset === false) {
        ElMessage.warning({
          message: "提示：机器ID未重置（可能需要管理员权限），但账号切换已成功",
          duration: 6000,
          showClose: true,
        });
      }

      // 如果返回了auth_token，显示额外信息
      if (result.auth_token) {
        ElMessage.info({
          message: "如果Windsurf未自动登录，请确保Windsurf已打开",
          duration: 5000,
          showClose: true,
        });
      }

      // 更新账号状态
      const updatedAccount = {
        ...props.account,
        status: "active" as const,
        last_login_at: dayjs().toISOString(),
      };
      emit("update", updatedAccount);
    } else {
      ElMessage.error(result.error || "切换账号失败");
    }
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(`切换账号失败: ${error}`);
    }
  } finally {
    isSwitching.value = false;
  }
}
</script>

<style scoped>
.account-card {
  background: white;
  border-radius: 12px;
  padding: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 2px solid transparent;
  width: 100%;
  box-sizing: border-box;
  position: relative;
  cursor: pointer;
  user-select: none;
}

.account-card::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  border-radius: 10px;
  box-shadow: inset 0 0 0 2px rgba(0, 0, 0, 0);
  opacity: 0;
  transition:
    opacity 0.3s ease,
    box-shadow 0.3s ease;
  pointer-events: none;
}

/* 悬浮状态 - 优雅上浮 */
.account-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.1);
  z-index: 1;
}

.account-card:hover::before {
  opacity: 1;
  box-shadow: inset 0 0 0 1px rgba(100, 116, 139, 0.15);
}

/* 选中状态 - 科技蓝 (Selected) */
.account-card.selected {
  border-color: transparent;
  background: linear-gradient(135deg, #f0f7ff 0%, #e0effe 100%);
  box-shadow:
    0 4px 12px rgba(59, 130, 246, 0.15),
    0 0 0 1px rgba(59, 130, 246, 0.1);
}

.account-card.selected::before {
  box-shadow:
    inset 0 0 0 2px rgba(59, 130, 246, 0.6),
    inset 0 0 12px rgba(59, 130, 246, 0.1);
  opacity: 1;
}

/* 当前激活账号 - 琥珀金 (Using) */
.account-card.current {
  background: linear-gradient(135deg, #fffbeb 0%, #fef3c7 100%);
  box-shadow:
    0 4px 12px rgba(245, 158, 11, 0.15),
    0 0 0 1px rgba(245, 158, 11, 0.1);
}

.account-card.current::before {
  box-shadow:
    inset 0 0 0 2px rgba(245, 158, 11, 0.6),
    inset 0 0 12px rgba(245, 158, 11, 0.15);
  opacity: 1;
}

.account-card.current:hover,
.account-card.selected:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
}

/* 已禁用账户 - 红色警告 */
.account-card.is-disabled {
  background: linear-gradient(135deg, #fef2f2 0%, #fee2e2 100%);
  box-shadow:
    0 4px 12px rgba(239, 68, 68, 0.15),
    0 0 0 1px rgba(239, 68, 68, 0.1);
}

.account-card.is-disabled::before {
  box-shadow:
    inset 0 0 0 2px rgba(239, 68, 68, 0.5),
    inset 0 0 12px rgba(239, 68, 68, 0.1);
  opacity: 1;
}

.account-card.is-disabled:hover {
  box-shadow: 0 8px 24px rgba(239, 68, 68, 0.2);
}

/* 订阅未激活 - 灰色样式 */
.account-card.subscription-inactive {
  background: linear-gradient(135deg, #f8fafc 0%, #e2e8f0 100%);
  box-shadow:
    0 4px 12px rgba(100, 116, 139, 0.15),
    0 0 0 1px rgba(100, 116, 139, 0.1);
  filter: grayscale(30%);
}

.account-card.subscription-inactive::before {
  box-shadow:
    inset 0 0 0 2px rgba(100, 116, 139, 0.3),
    inset 0 0 12px rgba(100, 116, 139, 0.1);
  opacity: 1;
}

.account-card.subscription-inactive:hover {
  box-shadow: 0 8px 24px rgba(100, 116, 139, 0.2);
  filter: grayscale(20%);
}

/* 拖拽手柄样式 */
.drag-handle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  margin-right: 6px;
  cursor: grab;
  color: #94a3b8;
  border-radius: 4px;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.drag-handle:hover {
  color: #3b82f6;
  background: rgba(59, 130, 246, 0.1);
}

.drag-handle:active {
  cursor: grabbing;
  color: #2563eb;
  background: rgba(59, 130, 246, 0.2);
}

:root.dark .drag-handle {
  color: #64748b;
}

:root.dark .drag-handle:hover {
  color: #60a5fa;
  background: rgba(96, 165, 250, 0.15);
}

.card-header {
  display: flex;
  align-items: center;
  margin-bottom: 4px;
  gap: 6px;
  padding-bottom: 4px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

.account-info {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  display: flex;
  align-items: center;
  gap: 6px;
}

.email {
  font-size: 14px;
  font-weight: 600;
  color: #1e293b;
  cursor: pointer;
  transition: color 0.2s;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  min-width: 0;
}

.email:hover {
  color: #3b82f6;
}

.nickname-tag {
  flex-shrink: 0;
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
  background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%) !important;
  color: #92400e !important;
  border: none !important;
  font-weight: 600;
  font-size: 10px;
  padding: 1px 6px !important;
  border-radius: 10px;
  box-shadow: 0 1px 3px rgba(146, 64, 14, 0.15);
  letter-spacing: 0.3px;
  margin-left: auto;
  margin-right: 4px;
  opacity: 0.9;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 3px;
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 10px;
  background: rgba(0, 0, 0, 0.02);
  transition: all 0.2s ease;
}

.status-indicator:hover {
  background: rgba(0, 0, 0, 0.04);
}

.status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  position: relative;
  transition: all 0.3s ease;
}

.status-dot::after {
  content: "";
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 100%;
  height: 100%;
  border-radius: 50%;
  opacity: 0.3;
}

.status-active .status-dot {
  background: #10b981;
  box-shadow: 0 0 0 2px rgba(16, 185, 129, 0.2);
}

.status-active .status-dot::after {
  animation: status-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  background: #10b981;
}

.status-inactive .status-dot {
  background: #94a3b8;
}

.status-error .status-dot {
  background: #ef4444;
  box-shadow: 0 0 0 2px rgba(239, 68, 68, 0.2);
}

.status-disabled .status-dot {
  background: #f59e0b;
  box-shadow: 0 0 0 2px rgba(245, 158, 11, 0.2);
}

.status-text {
  font-weight: 500;
  color: #64748b;
}

.status-active .status-text {
  color: #10b981;
  font-weight: 600;
}

.status-error .status-text {
  color: #ef4444;
  font-weight: 600;
}

.status-disabled .status-text {
  color: #f59e0b;
  font-weight: 600;
}

.status-subscription-inactive .status-text {
  color: #64748b;
  font-weight: 600;
}

.status-subscription-inactive .status-dot {
  background: #64748b;
  box-shadow: 0 0 0 2px rgba(100, 116, 139, 0.2);
}

@keyframes status-pulse {
  0%,
  100% {
    transform: translate(-50%, -50%) scale(1);
    opacity: 0.3;
  }
  50% {
    transform: translate(-50%, -50%) scale(2);
    opacity: 0;
  }
}

.card-body {
  margin-top: 4px;
}

/* 配额部分样式 */
.quota-section {
  background: linear-gradient(135deg, #f6f8fb 0%, #f0f3f8 100%);
  border-radius: 8px;
  padding: 6px;
  margin-bottom: 4px;
  border: 1px solid rgba(0, 0, 0, 0.04);
  transition: all 0.3s ease;
}

.quota-section:hover {
  background: linear-gradient(135deg, #f0f3f8 0%, #e8ecf3 100%);
}

.quota-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
  gap: 6px;
}

.quota-header-left {
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.quota-header-right {
  display: flex;
  align-items: baseline;
  gap: 2px;
  margin-left: auto;
  font-family: "Segoe UI", system-ui, sans-serif;
}

.plan-tag {
  color: white !important;
  border: none !important;
  font-weight: 700;
  font-size: 10.5px;
  padding: 2px 8px;
  height: auto;
  min-height: 20px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 3px;
  white-space: nowrap;
  border-radius: 6px;
  box-shadow:
    0 2px 4px rgba(0, 0, 0, 0.1),
    0 1px 2px rgba(0, 0, 0, 0.06),
    inset 0 1px 0 rgba(255, 255, 255, 0.2),
    inset 0 -1px 0 rgba(0, 0, 0, 0.1);
  transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  letter-spacing: 0.3px;
  position: relative;
  overflow: hidden;
  text-shadow: 0 1px 1px rgba(0, 0, 0, 0.1);
}

.plan-tag::before {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    180deg,
    rgba(255, 255, 255, 0.15) 0%,
    rgba(255, 255, 255, 0) 60%,
    rgba(0, 0, 0, 0.05) 100%
  );
  pointer-events: none;
}

.plan-tag:hover {
  transform: translateY(-1px) scale(1.02);
  box-shadow:
    0 4px 8px rgba(0, 0, 0, 0.15),
    0 2px 4px rgba(0, 0, 0, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.3);
  filter: brightness(1.05);
}

.plan-tag :deep(.el-tag__content) {
  display: inline-flex !important;
  flex-direction: row !important;
  align-items: center !important;
  gap: 3px !important;
}

.plan-tag .el-icon {
  font-size: 11px;
  margin-right: 1px;
  filter: drop-shadow(0 1px 1px rgba(0, 0, 0, 0.1));
  display: inline-flex !important;
}

/* 套餐颜色层级：Free < Trial < Pro < Teams < Enterprise */

/* Free套餐 - 银灰色，简约 */
.plan-tag.plan-free {
  background: linear-gradient(135deg, #9ca3af 0%, #6b7280 100%) !important;
  border: 1px solid rgba(255, 255, 255, 0.1) !important;
}

/* Trial套餐 - 琥珀金，醒目 */
.plan-tag.plan-trial {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%) !important;
  box-shadow:
    0 2px 6px rgba(245, 158, 11, 0.25),
    inset 0 1px 0 rgba(255, 255, 255, 0.3) !important;
}

/* Pro套餐 - 科技蓝，专业 */
.plan-tag.plan-pro {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%) !important;
  box-shadow:
    0 2px 6px rgba(59, 130, 246, 0.25),
    inset 0 1px 0 rgba(255, 255, 255, 0.3) !important;
}

/* Teams套餐 - 翡翠绿，活力 */
.plan-tag.plan-teams {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%) !important;
  box-shadow:
    0 2px 6px rgba(16, 185, 129, 0.25),
    inset 0 1px 0 rgba(255, 255, 255, 0.3) !important;
}

/* Enterprise套餐 - 幻彩紫，尊贵 */
.plan-tag.plan-enterprise {
  background: linear-gradient(
    135deg,
    #8b5cf6 0%,
    #7c3aed 50%,
    #6d28d9 100%
  ) !important;
  box-shadow:
    0 2px 8px rgba(124, 58, 237, 0.35),
    inset 0 1px 0 rgba(255, 255, 255, 0.3) !important;
}

/* Enterprise流光动画 */
.plan-tag.plan-enterprise::after {
  content: "";
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: linear-gradient(
    to right,
    rgba(255, 255, 255, 0) 0%,
    rgba(255, 255, 255, 0.15) 50%,
    rgba(255, 255, 255, 0) 100%
  );
  transform: rotate(30deg);
  animation: shimmer 3s infinite;
  pointer-events: none;
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%) rotate(30deg);
  }
  100% {
    transform: translateX(100%) rotate(30deg);
  }
}

/* 默认颜色（其他套餐） */
.plan-tag:not(.plan-free):not(.plan-trial):not(.plan-pro):not(.plan-teams):not(
    .plan-enterprise
  ) {
  background: linear-gradient(135deg, #6366f1 0%, #4f46e5 100%) !important;
}

.quota-used {
  font-size: 14px;
  font-weight: 700;
  color: #475569;
  letter-spacing: 0.3px;
}

.quota-separator {
  font-size: 12px;
  font-weight: 400;
  color: #94a3b8;
  margin: 0 1px;
}

.quota-total {
  font-size: 12px;
  font-weight: 600;
  color: #64748b;
  letter-spacing: 0.3px;
}

.quota-progress {
  position: relative;
  display: flex;
  align-items: center;
  gap: 8px;
}

.quota-progress .el-progress {
  flex: 1;
}

.quota-progress :deep(.el-progress-bar__outer) {
  background-color: #e2e8f0;
  height: 6px !important;
}

.quota-progress :deep(.el-progress-bar__inner) {
  border-radius: 10px;
  transition: all 0.3s ease;
  background-image: linear-gradient(
    90deg,
    var(--color-start) 0%,
    var(--color-end) 100%
  );
}

/* 为不同百分比设置不同的渐变色 */
.quota-progress :deep(.el-progress-bar__inner[style*="#10b981"]) {
  --color-start: #10b981;
  --color-end: #34d399;
}

.quota-progress :deep(.el-progress-bar__inner[style*="#f59e0b"]) {
  --color-start: #f59e0b;
  --color-end: #fbbf24;
}

.quota-progress :deep(.el-progress-bar__inner[style*="#ef4444"]) {
  --color-start: #ef4444;
  --color-end: #f87171;
}

.quota-percentage {
  min-width: 36px;
  text-align: right;
  font-size: 11px;
  font-weight: 600;
  color: #475569;
}

/* 配额区块内的订阅到期时间样式 */
.quota-expiry {
  display: flex;
  align-items: center;
  gap: 3px;
  margin-top: 4px;
  padding-top: 4px;
  border-top: 1px solid rgba(0, 0, 0, 0.06);
  font-size: 10px;
}

.quota-expiry .expiry-icon {
  color: #64748b;
  font-size: 12px;
}

.quota-expiry .expiry-label {
  color: #64748b;
  font-weight: 500;
}

.quota-expiry .expiry-date {
  color: #475569;
  font-weight: 500;
  font-family: "Segoe UI", system-ui, sans-serif;
}

.quota-expiry .expiry-badge {
  font-size: 10px;
  font-weight: 600;
  padding: 1px 6px;
  border-radius: 10px;
  margin-left: auto;
  white-space: nowrap;
}

.quota-expiry .expiry-badge.normal {
  color: #10b981;
  background-color: rgba(16, 185, 129, 0.1);
}

.quota-expiry .expiry-badge.expiring-soon {
  color: #f59e0b;
  background-color: rgba(245, 158, 11, 0.15);
  animation: gentle-pulse 2s ease-in-out infinite;
}

.quota-expiry .expiry-badge.expired {
  color: #ef4444;
  background-color: rgba(239, 68, 68, 0.15);
  font-weight: 700;
}

@keyframes gentle-pulse {
  0%,
  100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.85;
    transform: scale(1.02);
  }
}

.tags {
  display: flex;
  flex-wrap: wrap;
  gap: 2px;
  margin-bottom: 2px;
}

/* 自定义颜色标签 */
.custom-tag {
  border-radius: 4px !important;
  font-weight: 500 !important;
  font-size: 10px !important;
  padding: 0 6px !important;
  height: 18px !important;
  line-height: 18px !important;
  transition: all 0.2s ease !important;
}

.custom-tag:hover {
  transform: scale(1.05);
  filter: brightness(1.1);
}

/* 带颜色标签的卡片边框效果 */
.account-card.has-tag-color {
  border-width: 2px;
  border-style: solid;
}

.account-card.has-tag-color:hover {
  filter: brightness(1.02);
}

.account-card.has-tag-color::before {
  opacity: 0.5;
}

/* 信息标签组 */
.info-tags {
  display: flex;
  justify-content: space-evenly;
  align-items: center;
  gap: 6px;
  margin-top: 4px;
  margin-bottom: 2px;
}

.info-tag {
  font-size: 10px !important;
  padding: 0 10px !important;
  height: 20px !important;
  border-radius: 4px !important;
  display: inline-flex !important;
  flex-direction: row !important;
  align-items: center !important;
  gap: 4px !important;
  flex: 1;
  justify-content: center !important;
  min-width: 0;
  font-weight: 500;
}

.info-tag :deep(.el-tag__content) {
  display: inline-flex !important;
  flex-direction: row !important;
  align-items: center !important;
  gap: 4px !important;
}

.info-tag .el-icon {
  font-size: 10px !important;
  margin-right: 1px;
  display: inline-flex !important;
}

.info-tag span {
  font-size: 10px;
  white-space: nowrap;
  text-overflow: ellipsis;
  overflow: hidden;
}

/* 分组标签 - 浅蓝色 */
.group-tag {
  background: rgba(59, 130, 246, 0.1) !important;
  color: #64748b !important;
  border: 1px solid rgba(59, 130, 246, 0.15) !important;
}

/* 创建时间标签 - 浅紫色 */
.create-tag {
  background: rgba(168, 85, 247, 0.1) !important;
  color: #64748b !important;
  border: 1px solid rgba(168, 85, 247, 0.15) !important;
}

/* Token标签根据过期时间变色 */
.token-tag {
  border: none !important;
}

.token-tag.el-tag--success {
  background: rgba(16, 185, 129, 0.1) !important;
  color: #10b981 !important;
  border: 1px solid rgba(16, 185, 129, 0.15) !important;
}

.token-tag.el-tag--warning {
  background: rgba(245, 158, 11, 0.1) !important;
  color: #f59e0b !important;
  border: 1px solid rgba(245, 158, 11, 0.15) !important;
}

.token-tag.el-tag--danger {
  background: rgba(239, 68, 68, 0.1) !important;
  color: #ef4444 !important;
  border: 1px solid rgba(239, 68, 68, 0.15) !important;
}

.token-tag.el-tag--info {
  background: rgba(107, 114, 128, 0.1) !important;
  color: #6b7280 !important;
  border: 1px solid rgba(107, 114, 128, 0.15) !important;
}

.card-actions {
  padding: 5px 8px;
  border-top: 1px solid #ebeef5;
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.action-buttons {
  display: flex;
  gap: 4px;
  justify-content: space-evenly;
  flex-wrap: nowrap;
  align-items: center;
  padding: 2px;
  height: 28px;
}

.action-buttons .el-button {
  flex: 0 0 auto;
  margin: 0;
}

.action-buttons .el-button.el-button--small.is-circle {
  width: 26px;
  height: 26px;
  padding: 0;
  min-width: 26px;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 调整图标大小 */
.action-buttons .el-button .el-icon {
  font-size: 13px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 暗色主题支持 */
:root.dark .account-card {
  background: #1e1e1e;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4);
}

:root.dark .account-card::before {
  box-shadow: inset 0 0 0 2px rgba(255, 255, 255, 0.05);
}

:root.dark .account-card:hover {
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.6);
  background: #252525;
}

:root.dark .account-card:hover::before {
  opacity: 1;
  box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.1);
}

/* 暗色主题 - 选中状态 (Selected - Tech Blue) */
:root.dark .account-card.selected {
  border-color: transparent;
  background: linear-gradient(135deg, #1e293b 0%, #172554 100%);
  box-shadow:
    0 4px 12px rgba(30, 64, 175, 0.3),
    0 0 0 1px rgba(59, 130, 246, 0.3);
}

:root.dark .account-card.selected::before {
  box-shadow:
    inset 0 0 0 1px rgba(96, 165, 250, 0.5),
    inset 0 0 20px rgba(37, 99, 235, 0.2);
  opacity: 1;
}

/* 暗色主题 - 当前账号 (Current - Amber Gold) */
:root.dark .account-card.current {
  background: linear-gradient(135deg, #2a2515 0%, #453a10 100%);
  box-shadow:
    0 4px 12px rgba(180, 83, 9, 0.25),
    0 0 0 1px rgba(245, 158, 11, 0.3);
}

:root.dark .account-card.current::before {
  box-shadow:
    inset 0 0 0 1px rgba(251, 191, 36, 0.5),
    inset 0 0 20px rgba(217, 119, 6, 0.2);
  opacity: 1;
}

:root.dark .account-card.current:hover,
:root.dark .account-card.selected:hover {
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
  filter: brightness(1.1);
}

/* 暗色主题 - 已禁用账户 (Disabled - Warning Red) */
:root.dark .account-card.is-disabled {
  background: linear-gradient(135deg, #2a1515 0%, #451a1a 100%);
  box-shadow:
    0 4px 12px rgba(185, 28, 28, 0.25),
    0 0 0 1px rgba(239, 68, 68, 0.3);
}

:root.dark .account-card.is-disabled::before {
  box-shadow:
    inset 0 0 0 1px rgba(248, 113, 113, 0.5),
    inset 0 0 20px rgba(220, 38, 38, 0.2);
  opacity: 1;
}

:root.dark .account-card.is-disabled:hover {
  box-shadow: 0 8px 24px rgba(185, 28, 28, 0.4);
  filter: brightness(1.1);
}

/* 暗色主题 - 订阅未激活 (灰色) */
:root.dark .account-card.subscription-inactive {
  background: linear-gradient(135deg, #1e293b 0%, #334155 100%);
  box-shadow:
    0 4px 12px rgba(71, 85, 105, 0.25),
    0 0 0 1px rgba(100, 116, 139, 0.3);
  filter: grayscale(30%);
}

:root.dark .account-card.subscription-inactive::before {
  box-shadow:
    inset 0 0 0 1px rgba(148, 163, 184, 0.3),
    inset 0 0 20px rgba(100, 116, 139, 0.2);
  opacity: 1;
}

:root.dark .account-card.subscription-inactive:hover {
  box-shadow: 0 8px 24px rgba(71, 85, 105, 0.4);
  filter: grayscale(20%) brightness(1.1);
}

:root.dark .card-header {
  border-bottom-color: rgba(255, 255, 255, 0.08);
}

:root.dark .email {
  color: #f1f5f9;
}

:root.dark .nickname-tag {
  background: linear-gradient(
    135deg,
    rgba(251, 191, 36, 0.2) 0%,
    rgba(245, 158, 11, 0.25) 100%
  ) !important;
  color: #fbbf24 !important;
  box-shadow: 0 1px 3px rgba(251, 191, 36, 0.2);
  opacity: 0.9;
}

:root.dark .status-indicator {
  background: rgba(255, 255, 255, 0.03);
}

:root.dark .status-indicator:hover {
  background: rgba(255, 255, 255, 0.05);
}

:root.dark .status-text {
  color: #94a3b8;
}

:root.dark .status-active .status-text {
  color: #34d399;
}

:root.dark .status-error .status-text {
  color: #f87171;
}

:root.dark .status-active .status-dot {
  background: #34d399;
  box-shadow: 0 0 0 2px rgba(52, 211, 153, 0.25);
}

:root.dark .status-active .status-dot::after {
  background: #34d399;
}

:root.dark .status-error .status-dot {
  background: #f87171;
  box-shadow: 0 0 0 2px rgba(248, 113, 113, 0.25);
}

:root.dark .status-disabled .status-text {
  color: #fbbf24;
}

:root.dark .status-disabled .status-dot {
  background: #fbbf24;
  box-shadow: 0 0 0 2px rgba(251, 191, 36, 0.25);
}

:root.dark .status-subscription-inactive .status-text {
  color: #94a3b8;
}

:root.dark .status-subscription-inactive .status-dot {
  background: #94a3b8;
  box-shadow: 0 0 0 2px rgba(148, 163, 184, 0.25);
}

/* 暗色主题下的配额样式 */
:root.dark .quota-section {
  background: linear-gradient(135deg, #2a2a2a 0%, #252525 100%);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

:root.dark .quota-section:hover {
  background: linear-gradient(135deg, #2f2f2f 0%, #2a2a2a 100%);
}

/* 暗色主题下的套餐标签 */
:root.dark .plan-tag {
  box-shadow:
    0 2px 6px rgba(0, 0, 0, 0.3),
    0 1px 3px rgba(0, 0, 0, 0.2),
    inset 0 1px 0 rgba(255, 255, 255, 0.15),
    inset 0 -1px 0 rgba(0, 0, 0, 0.2);
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
}

:root.dark .plan-tag::before {
  background: linear-gradient(
    180deg,
    rgba(255, 255, 255, 0.1) 0%,
    rgba(255, 255, 255, 0) 60%,
    rgba(0, 0, 0, 0.1) 100%
  );
}

:root.dark .plan-tag:hover {
  box-shadow:
    0 4px 10px rgba(0, 0, 0, 0.4),
    0 2px 5px rgba(0, 0, 0, 0.25),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  filter: brightness(1.1);
}

/* 暗色主题套餐颜色 */
:root.dark .plan-tag.plan-free {
  background: linear-gradient(135deg, #4b5563 0%, #6b7280 100%) !important;
  border: 1px solid rgba(255, 255, 255, 0.08) !important;
}

:root.dark .plan-tag.plan-trial {
  background: linear-gradient(135deg, #b45309 0%, #d97706 100%) !important;
  box-shadow:
    0 2px 6px rgba(180, 83, 9, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.15) !important;
}

:root.dark .plan-tag.plan-pro {
  background: linear-gradient(135deg, #1e40af 0%, #2563eb 100%) !important;
  box-shadow:
    0 2px 6px rgba(30, 64, 175, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.15) !important;
}

:root.dark .plan-tag.plan-teams {
  background: linear-gradient(135deg, #047857 0%, #059669 100%) !important;
  box-shadow:
    0 2px 6px rgba(4, 120, 87, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.15) !important;
}

:root.dark .plan-tag.plan-enterprise {
  background: linear-gradient(
    135deg,
    #6d28d9 0%,
    #7c3aed 50%,
    #8b5cf6 100%
  ) !important;
  box-shadow:
    0 2px 10px rgba(109, 40, 217, 0.4),
    inset 0 1px 0 rgba(255, 255, 255, 0.15) !important;
}

:root.dark
  .plan-tag:not(.plan-free):not(.plan-trial):not(.plan-pro):not(
    .plan-teams
  ):not(.plan-enterprise) {
  background: linear-gradient(135deg, #4338ca 0%, #4f46e5 100%) !important;
  box-shadow:
    0 2px 6px rgba(67, 56, 202, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.15) !important;
}

:root.dark .quota-used {
  color: #cbd5e1;
}

:root.dark .quota-separator {
  color: #64748b;
}

:root.dark .quota-total {
  color: #94a3b8;
}

:root.dark .quota-percentage {
  color: #cbd5e1;
}

:root.dark .quota-progress :deep(.el-progress-bar__outer) {
  background-color: #374151;
}

/* 暗色主题下的订阅到期时间样式 */
:root.dark .quota-expiry {
  border-top-color: rgba(255, 255, 255, 0.08);
}

:root.dark .quota-expiry .expiry-icon {
  color: #94a3b8;
}

:root.dark .quota-expiry .expiry-label {
  color: #94a3b8;
}

:root.dark .quota-expiry .expiry-date {
  color: #cbd5e1;
}

/* 深色模式下的功能按钮样式 */
:root.dark .card-actions {
  border-top-color: rgba(255, 255, 255, 0.08);
}

:root.dark .action-buttons .el-button.is-circle {
  background-color: #262729;
  border-color: #4c4d4f;
  color: #cfd3dc;
}

:root.dark .action-buttons .el-button.is-circle:hover {
  background-color: #303133;
  border-color: #5a5b5d;
  color: #409eff;
}

:root.dark .action-buttons .el-button.is-circle:active {
  background-color: #1a1a1c;
  border-color: #409eff;
}

:root.dark .action-buttons .el-button--danger {
  background-color: transparent;
  border-color: rgba(245, 108, 108, 0.3);
  color: #f56c6c;
}

:root.dark .action-buttons .el-button--danger:hover {
  background-color: rgba(245, 108, 108, 0.1);
  border-color: rgba(245, 108, 108, 0.5);
  color: #ff6b6b;
}

:root.dark .quota-expiry .expiry-badge.normal {
  color: #34d399;
  background-color: rgba(52, 211, 153, 0.15);
}

:root.dark .quota-expiry .expiry-badge.expiring-soon {
  color: #fbbf24;
  background-color: rgba(251, 191, 36, 0.2);
}

:root.dark .quota-expiry .expiry-badge.expired {
  color: #f87171;
  background-color: rgba(248, 113, 113, 0.2);
}

/* 暗色主题下的信息标签 */
:root.dark .info-tags {
  gap: 4px;
}

:root.dark .group-tag {
  background: rgba(59, 130, 246, 0.1) !important;
  color: #60a5fa !important;
  border: 1px solid rgba(59, 130, 246, 0.15) !important;
}

:root.dark .create-tag {
  background: rgba(168, 85, 247, 0.1) !important;
  color: #c084fc !important;
  border: 1px solid rgba(168, 85, 247, 0.15) !important;
}

:root.dark .token-tag.el-tag--success {
  background: rgba(16, 185, 129, 0.1) !important;
  color: #34d399 !important;
  border: 1px solid rgba(16, 185, 129, 0.15) !important;
}

:root.dark .token-tag.el-tag--warning {
  background: rgba(245, 158, 11, 0.1) !important;
  color: #fbbf24 !important;
  border: 1px solid rgba(245, 158, 11, 0.15) !important;
}

:root.dark .token-tag.el-tag--danger {
  background: rgba(239, 68, 68, 0.1) !important;
  color: #f87171 !important;
  border: 1px solid rgba(239, 68, 68, 0.15) !important;
}

:root.dark .token-tag.el-tag--info {
  background: rgba(107, 114, 128, 0.1) !important;
  color: #9ca3af !important;
  border: 1px solid rgba(107, 114, 128, 0.15) !important;
}
</style>
