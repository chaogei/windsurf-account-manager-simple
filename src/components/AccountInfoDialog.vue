<template>
  <el-dialog
    v-model="visible"
    :title="$t('dialog.accountInfo.title')"
    width="850px"
    class="account-info-dialog"
    :close-on-click-modal="false"
    @close="handleClose"
  >
    <div v-if="loading" class="loading-container">
      <el-icon class="is-loading" size="32"><Loading /></el-icon>
      <p>{{$t("dialog.accountInfo.loading") }}</p>
    </div>

    <div v-else-if="accountInfo" class="dialog-content">
      <el-tabs class="custom-tabs">
        <!-- 用户详情页面 -->
        <el-tab-pane :label="$t('dialog.accountInfo.tabs.userDetails')">
          <template #label>
            <span class="tab-label"
              ><el-icon><User /></el-icon>
              {{ t("dialog.accountInfo.tabs.userDetails") }}</span
            >
          </template>

          <div v-if="loadingUserDetails" class="loading-container">
            <el-icon class="is-loading" size="24"><Loading /></el-icon>
            <p>{{ t("dialog.accountInfo.loading") }}...</p>
          </div>

          <div v-else-if="!userDetails" class="empty-container">
            <el-empty
              :description="$t('dialog.accountInfo.noData')"
              :image-size="100"
            >
              <el-button @click="loadUserDetails" type="primary" size="small">{{
                t("dialog.accountInfo.reload")
              }}</el-button>
            </el-empty>
          </div>

          <div v-else class="details-container">
            <!-- 用户头部卡片 -->
            <div
              class="profile-header"
              :class="`plan-${(userDetails.plan?.plan_name || 'free').toLowerCase()}`"
            >
              <div class="header-bg-icon">
                <el-icon><Avatar /></el-icon>
              </div>
              <div class="profile-main">
                <div class="avatar-wrapper">
                  <div class="avatar-placeholder">
                    {{ userDetails.user?.name?.charAt(0).toUpperCase() || "U" }}
                  </div>
                  <div
                    class="status-dot"
                    :class="
                      userDetails.subscription?.subscription_active
                        ? 'active'
                        : 'inactive'
                    "
                  ></div>
                </div>
                <div class="profile-info">
                  <div class="name-row">
                    <h3 class="user-name">
                      {{ userDetails.user?.name || $t('dialog.accountInfo.unknownUser') }}
                    </h3>
                    <el-tag
                      size="small"
                      :type="userDetails.is_root_admin ? 'danger' : 'info'"
                      effect="dark"
                      round
                    >
                      {{
                        userDetails.role?.role_name ||
                        (userDetails.is_root_admin ? $t('dialog.accountInfo.rootAdmin') : $t('dialog.accountInfo.member'))
                      }}
                    </el-tag>
                  </div>
                  <div class="email-row">
                    <span class="email">{{
                      displayEmail(userDetails.user?.email)
                    }}</span>
                    <el-tooltip :content="$t('dialog.accountInfo.copyEmail')"
                      ><el-icon
                        class="copy-icon"
                        @click="copyText(userDetails.user?.email)"
                        ><CopyDocument /></el-icon
                    ></el-tooltip>
                  </div>
                  <div class="meta-row">
                    <span class="meta-item" v-if="userDetails.user?.username"
                      ><el-icon><User /></el-icon> @{{
                        userDetails.user.username
                      }}</span
                    >
                    <span class="meta-item" v-if="userDetails.user?.timezone"
                      ><el-icon><Location /></el-icon>
                      {{ userDetails.user.timezone }}</span
                    >
                    <span class="meta-item" v-if="userDetails.roles"
                      ><el-icon><Key /></el-icon> {{ userDetails.roles }}</span
                    >
                  </div>
                </div>
              </div>
              <div
                class="profile-plan-badge"
                v-if="userDetails.plan?.plan_name"
              >
                <el-icon><Trophy /></el-icon>
                {{ formatPlanName(userDetails.plan.plan_name) }}
              </div>
              <!-- 订阅到期时间标签 -->
              <div
                class="profile-expire-badge"
                v-if="userDetails.team?.current_billing_period_end"
              >
                <el-icon><Calendar /></el-icon>
                <span class="expire-text">{{
                  formatTimestamp(userDetails.team.current_billing_period_end)
                }}</span>
                <span
                  class="expire-countdown-tag"
                  :class="
                    getExpireClass(userDetails.team.current_billing_period_end)
                  "
                >
                  {{
                    getExpireCountdown(
                      userDetails.team.current_billing_period_end,
                    )
                  }}
                </span>
              </div>
            </div>

            <!-- 主布局：基础信息 + 订阅套餐 -->
            <div class="main-info-layout">
              <!-- 左侧：基础信息表格 -->
              <div class="info-card basic-info-card">
                <div class="card-title">
                  <el-icon><Postcard /></el-icon>
                  {{ t("dialog.accountInfo.basicInfo.title") }}
                </div>
                <table class="basic-info-table">
                  <tbody>
                    <tr v-if="userDetails.user?.api_key">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.userId") }}
                      </td>
                      <td class="value-cell">{{ userDetails.user.api_key }}</td>
                    </tr>
                    <tr>
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.name") }}
                      </td>
                      <td class="value-cell">
                        {{ userDetails.user?.name || "-" }}
                      </td>
                    </tr>
                    <tr>
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.email") }}
                      </td>
                      <td class="value-cell">
                        {{ displayEmail(userDetails.user?.email) }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.user?.id">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.firebaseUid") }}
                      </td>
                      <td class="value-cell text-muted">
                        {{ userDetails.user.id }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.user?.username">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.team") }}
                      </td>
                      <td class="value-cell">
                        <el-tag size="small" type="info" effect="plain">{{
                          userDetails.user.username
                        }}</el-tag>
                      </td>
                    </tr>
                    <tr v-if="userDetails.user?.timezone">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.timezone") }}
                      </td>
                      <td class="value-cell">
                        {{ userDetails.user.timezone }}
                      </td>
                    </tr>
                    <tr>
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.signupTime") }}
                      </td>
                      <td class="value-cell text-primary">
                        {{ formatDateTime(userDetails.user?.signup_time) }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.user?.last_update_time">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.lastUpdate") }}
                      </td>
                      <td class="value-cell text-primary">
                        {{ formatDateTime(userDetails.user.last_update_time) }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.user?.windsurf_pro_trial_end_time">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.trialEnd") }}
                      </td>
                      <td class="value-cell text-warning">
                        {{
                          formatDateTime(
                            userDetails.user.windsurf_pro_trial_end_time,
                          )
                        }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.user?.referral_code">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.referralCode") }}
                      </td>
                      <td class="value-cell">
                        {{ userDetails.user.referral_code }}
                        <el-tooltip content="复制推荐链接">
                          <el-icon
                            class="copy-btn"
                            @click="
                              copyReferralLink(userDetails.user.referral_code)
                            "
                            ><Link
                          /></el-icon>
                        </el-tooltip>
                      </td>
                    </tr>
                    <tr>
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.subStatus") }}
                      </td>
                      <td class="value-cell">
                        <el-tag
                          size="small"
                          :type="getTeamsTierType(subscriptionTier)"
                          effect="plain"
                          >{{ formatTeamsTier(subscriptionTier) }}</el-tag
                        >
                        <el-tag
                          size="small"
                          :type="isSubscriptionActive ? 'success' : 'info'"
                          effect="plain"
                          style="margin-left: 4px"
                        >
                          {{
                            isSubscriptionActive
                              ? t("dialog.accountInfo.basicInfo.active")
                              : t("dialog.accountInfo.basicInfo.inactive")
                          }}
                        </el-tag>
                      </td>
                    </tr>
                    <tr v-if="userDetails.team?.stripe_subscription_id">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.stripeSubId") }}
                      </td>
                      <td class="value-cell text-muted">
                        {{ userDetails.team.stripe_subscription_id }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.team?.stripe_customer_id">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.stripeCustId") }}
                      </td>
                      <td class="value-cell text-muted">
                        {{ userDetails.team.stripe_customer_id }}
                      </td>
                    </tr>
                    <tr>
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.seatCount") }}
                      </td>
                      <td class="value-cell">
                        <span class="seat-count">{{ seatCount }}</span>
                        {{ t("dialog.accountInfo.team.seats") }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.team?.current_billing_period_start">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.billingStart") }}
                      </td>
                      <td class="value-cell text-success">
                        {{
                          formatTimestamp(
                            userDetails.team.current_billing_period_start,
                          )
                        }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.team?.current_billing_period_end">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.billingEnd") }}
                      </td>
                      <td class="value-cell text-warning">
                        {{
                          formatTimestamp(
                            userDetails.team.current_billing_period_end,
                          )
                        }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.team?.cascade_usage_month_start">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.cascadeStart") }}
                      </td>
                      <td class="value-cell text-primary">
                        {{
                          formatTimestamp(
                            userDetails.team.cascade_usage_month_start,
                          )
                        }}
                      </td>
                    </tr>
                    <tr v-if="userDetails.team?.cascade_usage_month_end">
                      <td class="label-cell">
                        {{ t("dialog.accountInfo.basicInfo.cascadeEnd") }}
                      </td>
                      <td class="value-cell text-primary">
                        {{
                          formatTimestamp(
                            userDetails.team.cascade_usage_month_end,
                          )
                        }}
                      </td>
                    </tr>
                  </tbody>
                </table>

                <!-- 使用量/配额美化展示 -->
                <div class="quota-display-card">
                  <div class="quota-header">
                    <span class="quota-title">{{
                      t("dialog.accountInfo.quota.title")
                    }}</span>
                    <span
                      class="quota-percentage"
                      :class="getQuotaClass(quotaPercentage)"
                      >{{ quotaPercentage }}%</span
                    >
                  </div>
                  <div class="quota-progress-wrap">
                    <div class="quota-progress-bar">
                      <div
                        class="quota-progress-fill"
                        :style="{ width: quotaPercentage + '%' }"
                        :class="getQuotaClass(quotaPercentage)"
                      ></div>
                    </div>
                  </div>
                  <div class="quota-details">
                    <div class="quota-used">
                      <span class="quota-label">{{
                        t("dialog.accountInfo.quota.used")
                      }}</span>
                      <span class="quota-value">{{
                        formatCredits(totalUsedCredits)
                      }}</span>
                    </div>
                    <div class="quota-divider">/</div>
                    <div class="quota-total">
                      <span class="quota-label">{{
                        t("dialog.accountInfo.quota.total")
                      }}</span>
                      <span class="quota-value">{{
                        formatCredits(totalQuotaCredits)
                      }}</span>
                    </div>
                  </div>
                </div>
                <!-- 用户标志 -->
                <div class="flag-tags-bottom" v-if="hasUserFlags">
                  <el-tag
                    size="small"
                    type="success"
                    effect="plain"
                    v-if="userDetails.user?.pro"
                    ><el-icon><Star /></el-icon>
                    {{ t("dialog.accountInfo.flags.pro") }}</el-tag
                  >
                  <el-tag
                    size="small"
                    type="success"
                    effect="plain"
                    v-if="userDetails.user?.public_profile_enabled"
                    ><el-icon><View /></el-icon>
                    {{ t("dialog.accountInfo.flags.public") }}</el-tag
                  >
                  <el-tag
                    size="small"
                    type="info"
                    effect="plain"
                    v-if="userDetails.user?.newsletter"
                    ><el-icon><Message /></el-icon>
                    {{ t("dialog.accountInfo.flags.newsletter") }}</el-tag
                  >
                  <el-tag
                    size="small"
                    type="warning"
                    effect="plain"
                    v-if="userDetails.user?.used_trial"
                    ><el-icon><Clock /></el-icon>
                    {{ t("dialog.accountInfo.flags.usedTrial") }}</el-tag
                  >
                  <el-tag
                    size="small"
                    type="danger"
                    effect="plain"
                    v-if="userDetails.user?.disable_codeium"
                    ><el-icon><Close /></el-icon>
                    {{ t("dialog.accountInfo.flags.disabled") }}</el-tag
                  >
                  <el-tag
                    size="small"
                    type="info"
                    effect="plain"
                    v-if="userDetails.user?.disabled_telemetry"
                    ><el-icon><Hide /></el-icon>
                    {{ t("dialog.accountInfo.flags.noTelemetry") }}</el-tag
                  >
                </div>
              </div>

              <!-- 右侧：订阅与套餐 + 积分与配额 -->
              <div class="right-column">
                <!-- 订阅与套餐 -->
                <div class="info-card plan-card-bg">
                  <div class="card-title">
                    <el-icon><Trophy /></el-icon>
                    {{ t("dialog.accountInfo.plan.title") }}
                  </div>
                  <div class="card-content">
                    <div class="plan-badge">
                      <span class="plan-name">{{
                        formatTeamsTier(subscriptionTier)
                      }}</span>
                      <div class="status-tags">
                        <el-tag
                          v-if="isSubscriptionActive"
                          type="success"
                          size="small"
                          effect="dark"
                          >{{
                            t("dialog.accountInfo.basicInfo.active")
                          }}</el-tag
                        >
                        <el-tag v-else type="info" size="small" effect="dark">{{
                          t("dialog.accountInfo.basicInfo.inactive")
                        }}</el-tag>
                        <el-tag
                          v-if="userDetails.plan?.is_teams"
                          type="primary"
                          size="small"
                          effect="dark"
                          >{{ t("dialog.accountInfo.plan.type.team") }}</el-tag
                        >
                        <el-tag
                          v-if="userDetails.plan?.is_enterprise"
                          type="danger"
                          size="small"
                          effect="dark"
                          >{{
                            t("dialog.accountInfo.plan.type.enterprise")
                          }}</el-tag
                        >
                      </div>
                    </div>
                    <!-- 积分配额 -->
                    <div class="limits-grid compact">
                      <div class="limit-item">
                        <span class="limit-val">{{
                          formatCredits(
                            userDetails.plan?.monthly_prompt_credits,
                          )
                        }}</span>
                        <span class="limit-label">{{
                          t("dialog.accountInfo.plan.credits.prompt")
                        }}</span>
                      </div>
                      <div class="limit-item">
                        <span class="limit-val">{{
                          formatCredits(userDetails.plan?.monthly_flow_credits)
                        }}</span>
                        <span class="limit-label">{{
                          t("dialog.accountInfo.plan.credits.flow")
                        }}</span>
                      </div>
                      <div class="limit-item">
                        <span class="limit-val">{{
                          formatCredits(
                            userDetails.plan
                              ?.monthly_flex_credit_purchase_amount,
                          )
                        }}</span>
                        <span class="limit-label">{{
                          t("dialog.accountInfo.plan.credits.flex")
                        }}</span>
                      </div>
                      <div class="limit-item">
                        <span class="limit-val">{{
                          userDetails.plan?.max_num_premium_chat_messages || 0
                        }}</span>
                        <span class="limit-label">{{
                          t("dialog.accountInfo.plan.credits.premiumChat")
                        }}</span>
                      </div>
                      <div
                        class="limit-item"
                        v-if="userDetails.plan?.max_num_chat_input_tokens"
                      >
                        <span class="limit-val">{{
                          formatLargeNumber(
                            userDetails.plan.max_num_chat_input_tokens,
                          )
                        }}</span>
                        <span class="limit-label">{{
                          t("dialog.accountInfo.plan.credits.chatTokens")
                        }}</span>
                      </div>
                    </div>
                    <!-- 限制信息表格 -->
                    <table class="plan-limits-table">
                      <tbody>
                        <tr>
                          <td class="label-cell">
                            {{ t("dialog.accountInfo.plan.team.seats") }}
                          </td>
                          <td class="value-cell">
                            <el-tag size="small" type="primary" effect="plain"
                              >{{ seatCount }}
                              {{ t("dialog.accountInfo.team.seats") }}</el-tag
                            >
                          </td>
                          <td class="label-cell">
                            {{ t("dialog.accountInfo.plan.credits.bandwidth") }}
                          </td>
                          <td class="value-cell">
                            {{
                              formatCredits(
                                userDetails.plan?.monthly_prompt_credits,
                              )
                            }}
                          </td>
                        </tr>
                        <tr>
                          <td class="label-cell">
                            {{ t("dialog.accountInfo.plan.credits.cache") }}
                          </td>
                          <td class="value-cell">
                            {{
                              formatCredits(
                                userDetails.plan?.monthly_flow_credits,
                              )
                            }}
                          </td>
                          <td class="label-cell">
                            {{ t("dialog.accountInfo.plan.credits.storage") }}
                          </td>
                          <td class="value-cell">
                            {{
                              formatStorageSize(
                                userDetails.plan?.max_num_chat_input_tokens,
                              )
                            }}
                          </td>
                        </tr>
                        <tr>
                          <td class="label-cell">
                            {{ t("dialog.accountInfo.plan.credits.api") }}
                          </td>
                          <td class="value-cell">
                            {{
                              formatApiLimit(
                                userDetails.plan?.max_num_premium_chat_messages,
                              )
                            }}
                          </td>
                          <td class="label-cell">
                            {{ t("dialog.accountInfo.plan.credits.timeout") }}
                          </td>
                          <td class="value-cell">
                            {{
                              userDetails.plan
                                ?.max_custom_chat_instruction_characters || 0
                            }}
                            s
                          </td>
                        </tr>
                      </tbody>
                    </table>
                    <!-- 功能开关 -->
                    <div
                      class="feature-switches compact"
                      v-if="userDetails.plan"
                    >
                      <div class="feature-label">
                        {{ t("dialog.accountInfo.plan.switches.title") }}
                      </div>
                      <div class="feature-tags">
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan.has_autocomplete_fast_mode
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="
                                userDetails.plan.has_autocomplete_fast_mode
                              " /><Close v-else
                          /></el-icon>
                          {{ t("dialog.accountInfo.plan.switches.fastMode") }}
                        </el-tag>
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan.allow_sticky_premium_models
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="
                                userDetails.plan.allow_sticky_premium_models
                              " /><Close v-else
                          /></el-icon>
                          {{
                            t("dialog.accountInfo.plan.switches.premiumModels")
                          }}
                        </el-tag>
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan.cascade_web_search_enabled
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="
                                userDetails.plan.cascade_web_search_enabled
                              " /><Close v-else
                          /></el-icon>
                          {{ t("dialog.accountInfo.plan.switches.webSearch") }}
                        </el-tag>
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan.cascade_can_auto_run_commands
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="
                                userDetails.plan.cascade_can_auto_run_commands
                              " /><Close v-else
                          /></el-icon>
                          {{ t("dialog.accountInfo.plan.switches.autoCmd") }}
                        </el-tag>
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan.has_tab_to_jump
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="userDetails.plan.has_tab_to_jump" /><Close
                              v-else
                          /></el-icon>
                          {{ t("dialog.accountInfo.plan.switches.tabJump") }}
                        </el-tag>
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan.knowledge_base_enabled
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="
                                userDetails.plan?.knowledge_base_enabled
                              " /><Close v-else
                          /></el-icon>
                          {{
                            t("dialog.accountInfo.plan.switches.knowledgeBase")
                          }}
                        </el-tag>
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan?.browser_enabled
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="userDetails.plan?.browser_enabled" /><Close
                              v-else
                          /></el-icon>
                          浏览器
                        </el-tag>
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan?.can_share_conversations
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="
                                userDetails.plan?.can_share_conversations
                              " /><Close v-else
                          /></el-icon>
                          分享对话
                        </el-tag>
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan?.can_buy_more_credits
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="
                                userDetails.plan?.can_buy_more_credits
                              " /><Close v-else
                          /></el-icon>
                          购买积分
                        </el-tag>
                        <el-tag
                          size="small"
                          :type="
                            userDetails.plan?.can_customize_app_icon
                              ? 'success'
                              : 'info'
                          "
                          effect="plain"
                        >
                          <el-icon
                            ><Check
                              v-if="
                                userDetails.plan?.can_customize_app_icon
                              " /><Close v-else
                          /></el-icon>
                          自定义图标
                        </el-tag>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- 积分与配额 -->
                <div
                  class="info-card"
                  v-if="userDetails.plan || userDetails.team"
                >
                  <div class="card-title">
                    <el-icon><Coin /></el-icon>
                    {{ t("dialog.accountInfo.plan.credits.title") }}
                  </div>
                  <div class="card-content">
                    <div class="credits-grid compact">
                      <div class="credit-item">
                        <span class="credit-val">{{
                          formatCredits(remainingPromptCredits)
                        }}</span>
                        <span class="credit-label">{{
                          t("dialog.accountInfo.plan.credits.remainingPrompt")
                        }}</span>
                      </div>
                      <div class="credit-item">
                        <span class="credit-val">{{
                          formatCredits(remainingFlowCredits)
                        }}</span>
                        <span class="credit-label">{{
                          t("dialog.accountInfo.plan.credits.remainingFlow")
                        }}</span>
                      </div>
                      <div
                        class="credit-item"
                        v-if="userDetails.team?.flex_credit_quota"
                      >
                        <span class="credit-val">{{
                          formatCredits(remainingFlexCredits)
                        }}</span>
                        <span class="credit-label">{{
                          t("dialog.accountInfo.plan.credits.remainingFlex")
                        }}</span>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- 团队信息卡片（移到右侧） -->
                <div class="info-card team-info-card" v-if="userDetails.team">
                  <div class="card-title">
                    <el-icon><Connection /></el-icon>
                    {{ t("dialog.accountInfo.team.title") }}
                  </div>
                  <div class="card-content">
                    <!-- 团队基本信息 -->
                    <div class="team-basic-info">
                      <div class="team-info-row">
                        <span class="info-label">{{
                          t("dialog.accountInfo.team.name")
                        }}</span>
                        <span class="info-value team-name">{{
                          userDetails.team?.name
                        }}</span>
                      </div>
                      <div
                        class="team-info-row"
                        v-if="userDetails.team?.teams_tier"
                      >
                        <span class="info-label">{{
                          t("dialog.accountInfo.team.tier")
                        }}</span>
                        <el-tag
                          size="small"
                          :type="getTeamsTierType(userDetails.team.teams_tier)"
                          effect="dark"
                          >{{
                            formatTeamsTier(userDetails.team.teams_tier)
                          }}</el-tag
                        >
                      </div>
                    </div>

                    <!-- ID信息区块 -->
                    <div class="id-info-section">
                      <div class="id-row" v-if="userDetails.team?.id">
                        <span class="id-label">{{
                          t("dialog.accountInfo.team.id")
                        }}</span>
                        <div class="id-value-wrap">
                          <code class="id-code" :title="userDetails.team.id">{{
                            userDetails.team.id
                          }}</code>
                          <el-button
                            size="small"
                            :icon="CopyDocument"
                            circle
                            @click="copyText(userDetails.team.id)"
                          />
                        </div>
                      </div>
                      <div class="id-row" v-if="userDetails.team?.invite_id">
                        <span class="id-label">{{
                          t("dialog.accountInfo.team.inviteId")
                        }}</span>
                        <div class="id-value-wrap">
                          <code
                            class="id-code"
                            :title="userDetails.team.invite_id"
                            >{{ userDetails.team.invite_id }}</code
                          >
                          <el-button
                            size="small"
                            :icon="CopyDocument"
                            circle
                            @click="copyText(userDetails.team.invite_id)"
                          />
                        </div>
                      </div>
                      <div
                        class="id-row"
                        v-if="userDetails.team?.stripe_customer_id"
                      >
                        <span class="id-label">{{
                          t("dialog.accountInfo.team.stripeCustomer")
                        }}</span>
                        <div class="id-value-wrap">
                          <code
                            class="id-code stripe"
                            :title="userDetails.team.stripe_customer_id"
                            >{{ userDetails.team.stripe_customer_id }}</code
                          >
                          <el-button
                            size="small"
                            :icon="CopyDocument"
                            circle
                            @click="
                              copyText(userDetails.team.stripe_customer_id)
                            "
                          />
                        </div>
                      </div>
                      <div
                        class="id-row"
                        v-if="userDetails.team?.stripe_subscription_id"
                      >
                        <span class="id-label">{{
                          t("dialog.accountInfo.team.stripeSub")
                        }}</span>
                        <div class="id-value-wrap">
                          <code
                            class="id-code stripe"
                            :title="userDetails.team.stripe_subscription_id"
                            >{{ userDetails.team.stripe_subscription_id }}</code
                          >
                          <el-button
                            size="small"
                            :icon="CopyDocument"
                            circle
                            @click="
                              copyText(userDetails.team.stripe_subscription_id)
                            "
                          />
                        </div>
                      </div>
                    </div>

                    <!-- 统计数据 -->
                    <div class="team-stats">
                      <div class="stat-box">
                        <span class="stat-number">{{
                          userDetails.team?.num_users || 1
                        }}</span>
                        <span class="stat-text">{{
                          t("dialog.accountInfo.team.members")
                        }}</span>
                      </div>
                      <div class="stat-box">
                        <span class="stat-number">{{
                          userDetails.team?.num_seats_current_billing_period ||
                          1
                        }}</span>
                        <span class="stat-text">{{
                          t("dialog.accountInfo.team.seats")
                        }}</span>
                      </div>
                      <div
                        class="stat-box"
                        v-if="userDetails.team?.num_cascade_seats"
                      >
                        <span class="stat-number">{{
                          userDetails.team.num_cascade_seats
                        }}</span>
                        <span class="stat-text">{{
                          t("dialog.accountInfo.team.cascade")
                        }}</span>
                      </div>
                    </div>

                    <!-- Cascade周期 -->
                    <div
                      class="cascade-period"
                      v-if="
                        userDetails.team?.cascade_usage_month_start ||
                        userDetails.team?.cascade_usage_month_end
                      "
                    >
                      <div class="period-header">
                        <el-icon><Clock /></el-icon>
                        <span>{{
                          t("dialog.accountInfo.team.period.title")
                        }}</span>
                      </div>
                      <div class="period-dates">
                        <div class="period-date start">
                          <span class="date-label">{{
                            t("dialog.accountInfo.team.period.start")
                          }}</span>
                          <span class="date-value">{{
                            formatTimestamp(
                              userDetails.team?.cascade_usage_month_start,
                            )
                          }}</span>
                        </div>
                        <el-icon class="period-arrow"><Right /></el-icon>
                        <div class="period-date end">
                          <span class="date-label">{{
                            t("dialog.accountInfo.team.period.end")
                          }}</span>
                          <span class="date-value">{{
                            formatTimestamp(
                              userDetails.team?.cascade_usage_month_end,
                            )
                          }}</span>
                        </div>
                      </div>
                    </div>

                    <!-- 积分使用情况 -->
                    <div
                      class="credits-usage"
                      v-if="
                        userDetails.team?.used_prompt_credits ||
                        userDetails.team?.used_flow_credits
                      "
                    >
                      <div
                        class="usage-item"
                        v-if="userDetails.team?.used_prompt_credits"
                      >
                        <span class="usage-label">{{
                          t("dialog.accountInfo.team.usage.prompt")
                        }}</span>
                        <span class="usage-value">{{
                          formatCredits(userDetails.team.used_prompt_credits)
                        }}</span>
                      </div>
                      <div
                        class="usage-item"
                        v-if="userDetails.team?.used_flow_credits"
                      >
                        <span class="usage-label">{{
                          t("dialog.accountInfo.team.usage.flow")
                        }}</span>
                        <span class="usage-value">{{
                          formatCredits(userDetails.team.used_flow_credits)
                        }}</span>
                      </div>
                      <div
                        class="usage-item"
                        v-if="userDetails.team?.used_flex_credits"
                      >
                        <span class="usage-label">{{
                          t("dialog.accountInfo.team.usage.flex")
                        }}</span>
                        <span class="usage-value">{{
                          formatCredits(userDetails.team.used_flex_credits)
                        }}</span>
                      </div>
                    </div>

                    <!-- 团队特性标志 -->
                    <div class="team-flags" v-if="hasTeamFlags">
                      <el-tag
                        size="small"
                        type="success"
                        effect="dark"
                        round
                        v-if="userDetails.team?.subscription_active"
                        ><el-icon><Check /></el-icon>
                        {{
                          t("dialog.accountInfo.team.flags.subActive")
                        }}</el-tag
                      >
                      <el-tag
                        size="small"
                        type="warning"
                        effect="plain"
                        round
                        v-if="userDetails.team?.used_trial"
                        ><el-icon><Clock /></el-icon>
                        {{
                          t("dialog.accountInfo.team.flags.usedTrial")
                        }}</el-tag
                      >
                      <el-tag
                        size="small"
                        type="primary"
                        effect="plain"
                        round
                        v-if="userDetails.team?.attribution_enabled"
                        ><el-icon><DataAnalysis /></el-icon>
                        {{
                          t("dialog.accountInfo.team.flags.attribution")
                        }}</el-tag
                      >
                      <el-tag
                        size="small"
                        type="danger"
                        effect="plain"
                        round
                        v-if="userDetails.team?.sso_provider_id"
                        ><el-icon><Lock /></el-icon>
                        {{ t("dialog.accountInfo.team.flags.sso") }}</el-tag
                      >
                      <el-tag
                        size="small"
                        type="info"
                        effect="plain"
                        round
                        v-if="userDetails.team?.top_up_enabled"
                        ><el-icon><Coin /></el-icon>
                        {{ t("dialog.accountInfo.team.flags.topUp") }}</el-tag
                      >
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 角色与权限卡片（移到左侧基础信息下方，独立一行） -->
            <div
              class="role-permission-section"
              v-if="userDetails.role || userDetails.permissions"
            >
              <div class="info-card">
                <div class="card-title">
                  <el-icon><Key /></el-icon>
                  {{ t("dialog.accountInfo.role.title") }}
                </div>
                <div class="card-content role-content">
                  <div class="role-info-row">
                    <div class="info-item" v-if="userDetails.role?.role_id">
                      <span class="label">{{
                        t("dialog.accountInfo.role.id")
                      }}</span>
                      <span class="value">{{ userDetails.role.role_id }}</span>
                    </div>
                    <div class="info-item" v-if="userDetails.role?.role_name">
                      <span class="label">{{
                        t("dialog.accountInfo.role.name")
                      }}</span>
                      <el-tag size="small" type="danger" effect="dark">{{
                        userDetails.role.role_name
                      }}</el-tag>
                    </div>
                    <div
                      class="info-item"
                      v-if="userDetails.user?.team_status !== undefined"
                    >
                      <span class="label">{{
                        t("dialog.accountInfo.role.teamStatus")
                      }}</span>
                      <el-tag
                        size="small"
                        :type="getTeamStatusType(userDetails.user.team_status)"
                        >{{
                          formatTeamStatus(userDetails.user.team_status)
                        }}</el-tag
                      >
                    </div>
                  </div>
                  <!-- 权限位图可视化 -->
                  <div class="permission-visual" v-if="userDetails.permissions">
                    <div class="perm-label">
                      {{ t("dialog.accountInfo.role.permissionMap") }} ({{
                        getPermissionCount(userDetails.permissions)
                      }}/31)
                    </div>
                    <div class="perm-dots">
                      <el-tooltip
                        v-for="i in 31"
                        :key="i"
                        :content="`${$t('dialog.accountInfo.role.permissionMap')} ${i}: ${hasPermission(userDetails.permissions, i) ? $t('dialog.accountInfo.role.granted') : $t('dialog.accountInfo.role.notGranted')}`"
                      >
                        <span
                          class="perm-dot"
                          :class="{
                            active: hasPermission(userDetails.permissions, i),
                          }"
                        ></span>
                      </el-tooltip>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 原始数据折叠 -->
            <el-collapse v-if="parsedData" class="raw-data-collapse">
              <el-collapse-item :title="$t('dialog.accountInfo.firebase.raw')">
                <pre class="raw-data">{{
                  JSON.stringify(parsedData, null, 2)
                }}</pre>
              </el-collapse-item>
            </el-collapse>
          </div>
        </el-tab-pane>

        <!-- 本地信息 -->
        <el-tab-pane :label="$t('dialog.accountInfo.tabs.localInfo')">
          <template #label>
            <span class="tab-label"
              ><el-icon><Monitor /></el-icon>
              {{ t("dialog.accountInfo.tabs.localInfo") }}</span
            >
          </template>

          <div class="local-info-container">
            <table class="local-info-table">
              <tbody>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.id") }}
                  </td>
                  <td class="value-cell">{{ accountInfo.local_info?.id }}</td>
                </tr>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.email") }}
                  </td>
                  <td class="value-cell">
                    {{ displayEmail(accountInfo.local_info?.email) }}
                  </td>
                </tr>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.nickname") }}
                  </td>
                  <td class="value-cell">
                    {{ accountInfo.local_info?.nickname || "-" }}
                  </td>
                </tr>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.group") }}
                  </td>
                  <td class="value-cell">
                    <el-tag size="small" type="primary" effect="plain">{{
                      accountInfo.local_info?.group || t("common.defaultGroup")
                    }}</el-tag>
                  </td>
                </tr>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.tags") }}
                  </td>
                  <td class="value-cell">
                    <template v-if="accountInfo.local_info?.tags?.length">
                      <el-tag
                        v-for="tag in accountInfo.local_info.tags"
                        :key="tag"
                        size="small"
                        type="info"
                        effect="plain"
                        style="margin-right: 4px"
                        >{{ tag }}</el-tag
                      >
                    </template>
                    <span v-else class="empty-text">{{
                      t("dialog.accountInfo.local.none")
                    }}</span>
                  </td>
                </tr>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.createdAt") }}
                  </td>
                  <td class="value-cell">
                    {{ formatDate(accountInfo.local_info?.created_at) || "-" }}
                  </td>
                </tr>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.lastLogin") }}
                  </td>
                  <td class="value-cell">
                    {{
                      formatDate(accountInfo.local_info?.last_login_at) || "-"
                    }}
                  </td>
                </tr>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.tokenExpires") }}
                  </td>
                  <td class="value-cell">
                    {{
                      formatDate(accountInfo.local_info?.token_expires_at) ||
                      "-"
                    }}
                  </td>
                </tr>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.seatCount") }}
                  </td>
                  <td class="value-cell">
                    {{ accountInfo.local_info?.last_seat_count ?? "-" }}
                  </td>
                </tr>
                <tr>
                  <td class="label-cell">
                    {{ t("dialog.accountInfo.local.status") }}
                  </td>
                  <td class="value-cell">
                    <el-tag
                      :type="
                        accountInfo.local_info?.status === 'active'
                          ? 'success'
                          : accountInfo.local_info?.status === 'error'
                            ? 'danger'
                            : 'info'
                      "
                      size="small"
                      effect="plain"
                    >
                      {{ accountInfo.local_info?.status }}
                    </el-tag>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </el-tab-pane>

        <!-- Firebase信息 -->
        <el-tab-pane
          :label="$t('dialog.accountInfo.tabs.firebase')"
          v-if="accountInfo.firebase_info"
        >
          <template #label>
            <span class="tab-label"
              ><el-icon><Key /></el-icon>
              {{ t("dialog.accountInfo.tabs.firebase") }}</span
            >
          </template>

          <div class="firebase-container">
            <!-- 用户身份信息卡片 -->
            <div class="info-card wide">
              <div class="card-title">
                <el-icon color="#409eff"><User /></el-icon>
                <span>{{ t("dialog.accountInfo.firebase.identity") }}</span>
                <div class="header-tags">
                  <el-tag
                    v-if="
                      accountInfo.firebase_info?.emailVerified ??
                      accountInfo.firebase_info?.email_verified
                    "
                    type="success"
                    size="small"
                    effect="plain"
                  >
                    <el-icon><Check /></el-icon>
                    {{ t("dialog.accountInfo.firebase.verified") }}
                  </el-tag>
                  <el-tag v-else type="warning" size="small" effect="plain">{{
                    t("dialog.accountInfo.firebase.unverified")
                  }}</el-tag>
                </div>
              </div>
              <div class="card-content">
                <div class="firebase-info-grid">
                  <div class="info-row">
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.basicInfo.firebaseUid")
                      }}</span>
                      <span class="value text-ellipsis" :title="firebaseUid">{{
                        firebaseUid
                      }}</span>
                    </div>
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.basicInfo.email")
                      }}</span>
                      <span class="value">{{
                        displayEmail(accountInfo.firebase_info?.email)
                      }}</span>
                    </div>
                  </div>
                  <div class="info-row">
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.firebase.displayName")
                      }}</span>
                      <span class="value">{{
                        accountInfo.firebase_info?.displayName ||
                        accountInfo.firebase_info?.display_name ||
                        "-"
                      }}</span>
                    </div>
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.firebase.emailStatus")
                      }}</span>
                      <span class="value">
                        <el-tag
                          size="small"
                          :type="
                            (accountInfo.firebase_info?.emailVerified ??
                            accountInfo.firebase_info?.email_verified)
                              ? 'success'
                              : 'warning'
                          "
                          effect="plain"
                        >
                          {{
                            (accountInfo.firebase_info?.emailVerified ??
                            accountInfo.firebase_info?.email_verified)
                              ? t("dialog.accountInfo.firebase.verified")
                              : t("dialog.accountInfo.firebase.unverified")
                          }}
                        </el-tag>
                      </span>
                    </div>
                  </div>
                  <div class="info-row">
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.firebase.accountStatus")
                      }}</span>
                      <span class="value">
                        <el-tag
                          size="small"
                          :type="
                            accountInfo.firebase_info?.disabled
                              ? 'danger'
                              : 'success'
                          "
                          effect="plain"
                        >
                          {{
                            accountInfo.firebase_info?.disabled
                              ? t("dialog.accountInfo.basicInfo.inactive")
                              : t("dialog.accountInfo.basicInfo.active")
                          }}
                        </el-tag>
                      </span>
                    </div>
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.firebase.validSince")
                      }}</span>
                      <span class="value">{{
                        formatFirebaseTimestamp(
                          accountInfo.firebase_info?.validSince ||
                            accountInfo.firebase_info?.valid_since,
                        )
                      }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 时间信息卡片 -->
            <div class="info-card wide timeline-card">
              <div class="card-title">
                <el-icon color="#409eff"><Clock /></el-icon>
                {{ t("dialog.accountInfo.firebase.timeline.title") }}
              </div>
              <div class="horizontal-timeline four-items">
                <div class="timeline-item">
                  <div class="timeline-dot dot-blue">
                    <el-icon><UserFilled /></el-icon>
                  </div>
                  <div class="timeline-content">
                    <div class="timeline-title">
                      {{ t("dialog.accountInfo.firebase.timeline.created") }}
                    </div>
                    <div class="timeline-time">
                      {{
                        formatFirebaseTimestamp(
                          accountInfo.firebase_info?.createdAt ||
                            accountInfo.firebase_info?.created_at,
                        )
                      }}
                    </div>
                  </div>
                </div>
                <div class="timeline-item">
                  <div class="timeline-dot dot-orange">
                    <el-icon><Key /></el-icon>
                  </div>
                  <div class="timeline-content">
                    <div class="timeline-title">
                      {{ t("dialog.accountInfo.firebase.timeline.password") }}
                    </div>
                    <div class="timeline-time">
                      {{
                        formatFirebaseTimestamp(
                          accountInfo.firebase_info?.passwordUpdatedAt ||
                            accountInfo.firebase_info?.password_updated_at,
                        )
                      }}
                    </div>
                  </div>
                </div>
                <div class="timeline-item">
                  <div class="timeline-dot dot-green">
                    <el-icon><Check /></el-icon>
                  </div>
                  <div class="timeline-content">
                    <div class="timeline-title">
                      {{ t("dialog.accountInfo.firebase.timeline.login") }}
                    </div>
                    <div class="timeline-time">
                      {{
                        formatFirebaseTimestamp(
                          accountInfo.firebase_info?.lastLoginAt ||
                            accountInfo.firebase_info?.last_login_at,
                        )
                      }}
                    </div>
                  </div>
                </div>
                <div class="timeline-item">
                  <div class="timeline-dot dot-gray">
                    <el-icon><Refresh /></el-icon>
                  </div>
                  <div class="timeline-content">
                    <div class="timeline-title">
                      {{ t("dialog.accountInfo.firebase.timeline.refresh") }}
                    </div>
                    <div class="timeline-time">
                      {{
                        formatFirebaseTimestamp(
                          accountInfo.firebase_info?.lastRefreshAt ||
                            accountInfo.firebase_info?.last_refresh_at,
                        )
                      }}
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 认证提供商卡片 -->
            <div
              class="info-card wide"
              v-if="
                (
                  accountInfo.firebase_info?.providerUserInfo ||
                  accountInfo.firebase_info?.provider_user_info
                )?.length
              "
            >
              <div class="card-title">
                <el-icon color="#409eff"><Link /></el-icon>
                {{ t("dialog.accountInfo.firebase.provider.title") }}
              </div>
              <div
                class="card-content"
                v-for="(provider, idx) in accountInfo.firebase_info
                  .providerUserInfo ||
                accountInfo.firebase_info.provider_user_info"
                :key="idx"
              >
                <div class="firebase-info-grid">
                  <div class="info-row">
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.firebase.provider.id")
                      }}</span>
                      <span class="value">
                        <el-tag size="small" type="info" effect="plain">{{
                          formatProviderName(
                            provider.providerId || provider.provider_id,
                          )
                        }}</el-tag>
                      </span>
                    </div>
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.firebase.provider.userId")
                      }}</span>
                      <span class="value">{{
                        provider.rawId ||
                        provider.raw_id ||
                        displayEmail(provider.email)
                      }}</span>
                    </div>
                  </div>
                  <div class="info-row">
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.firebase.provider.fedId")
                      }}</span>
                      <span class="value text-ellipsis">{{
                        provider.federatedId ||
                        provider.federated_id ||
                        displayEmail(provider.email)
                      }}</span>
                    </div>
                    <div class="info-cell">
                      <span class="label">{{
                        t("dialog.accountInfo.basicInfo.email")
                      }}</span>
                      <span class="value">{{
                        displayEmail(provider.email)
                      }}</span>
                    </div>
                  </div>
                  <div
                    class="info-row"
                    v-if="provider.displayName || provider.display_name"
                  >
                    <div class="info-cell full-width">
                      <span class="label">显示名称</span>
                      <span class="value">{{
                        provider.displayName || provider.display_name
                      }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- 查看Firebase原始数据 -->
            <el-collapse class="raw-data-collapse">
              <el-collapse-item>
                <template #title>
                  <span class="collapse-title">查看Firebase原始数据</span>
                  <el-icon class="collapse-arrow"><Right /></el-icon>
                </template>
                <pre class="raw-json">{{
                  JSON.stringify(accountInfo.firebase_info, null, 2)
                }}</pre>
              </el-collapse-item>
            </el-collapse>
          </div>
        </el-tab-pane>
      </el-tabs>
    </div>

    <div v-else-if="error" class="error-container">
      <el-alert
        title="获取账号信息失败"
        :description="error"
        type="error"
        show-icon
        :closable="false"
      />
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button @click="refresh" :icon="Refresh" circle />
        <el-button @click="handleClose">{{
          t("dialog.accountInfo.close")
        }}</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage } from "element-plus";
import {
  Loading,
  Refresh,
  Trophy,
  UserFilled,
  User,
  Clock,
  Key,
  Check,
  Close,
  Link,
  Postcard,
  Connection,
  Coin,
  Monitor,
  CopyDocument,
  Avatar,
  Location,
  View,
  Message,
  Right,
  Star,
  Hide,
  DataAnalysis,
  Lock,
  Calendar,
} from "@element-plus/icons-vue";
import { useUIStore, useSettingsStore } from "@/store";
import { apiService } from "@/api";
import { maskEmail } from "@/utils/privacy";
import dayjs from "dayjs";

const uiStore = useUIStore();
const settingsStore = useSettingsStore();
const { t } = useI18n();

// 邮箱脱敏处理
function displayEmail(email: string | undefined | null): string {
  if (!email) return "-";
  if (settingsStore.settings?.privacyMode) {
    return maskEmail(email);
  }
  return email;
}

const visible = ref(false);
const loading = ref(false);
const accountInfo = ref<any>(null);
const error = ref("");
const userDetails = ref<any>(null);
const parsedData = ref<any>(null);
const loadingUserDetails = ref(false);

// 监听对话框显示状态
watch(
  () => uiStore.showAccountInfoDialog,
  (show) => {
    visible.value = show;
    if (show && uiStore.currentViewingAccountId) {
      loadAccountInfo();
      // 延迟加载用户详情，确保账户信息先加载
      setTimeout(() => {
        loadUserDetails();
      }, 500);
    }
  },
);

watch(visible, (val) => {
  if (!val) {
    uiStore.closeAccountInfoDialog();
    accountInfo.value = null;
    userDetails.value = null;
    parsedData.value = null;
    error.value = "";
  }
});

async function loadAccountInfo() {
  if (!uiStore.currentViewingAccountId) return;

  loading.value = true;
  error.value = "";
  accountInfo.value = null;

  try {
    const result = await apiService.getAccountInfo(
      uiStore.currentViewingAccountId,
    );
    if (result.success) {
      accountInfo.value = result;
    } else {
      error.value = result.error || "获取失败";
    }
  } catch (err: any) {
    error.value = err.toString();
    ElMessage.error(`获取账号信息失败: ${err}`);
  } finally {
    loading.value = false;
  }
}

// 获取用户详细信息
async function loadUserDetails() {
  if (!uiStore.currentViewingAccountId) return;

  console.log("开始获取用户详情, ID:", uiStore.currentViewingAccountId);

  loadingUserDetails.value = true;
  userDetails.value = null;
  parsedData.value = null;

  try {
    const result = await apiService.getCurrentUserParsed(
      uiStore.currentViewingAccountId,
    );
    console.log("API返回结果:", result);

    if (result && result.success && result.data) {
      userDetails.value = result.data;
      parsedData.value = result.parsed_data;
      console.log("用户详情已设置:", userDetails.value);
    } else {
      console.warn("API返回失败或没有数据:", result);
      // 显示错误信息
      if (result && result.error) {
        ElMessage.warning(`获取用户详情失败: ${result.error}`);
      } else {
        console.log("未获取到用户详情数据");
      }
    }
  } catch (err: any) {
    console.error("获取用户详情失败:", err);
    ElMessage.error(`获取用户详情失败: ${err.message || err}`);
  } finally {
    loadingUserDetails.value = false;
  }
}

function refresh() {
  loadAccountInfo();
  loadUserDetails();
}

function handleClose() {
  visible.value = false;
}

function formatDate(date: string | null | undefined) {
  if (!date) return "";
  return dayjs(date).format("YYYY-MM-DD HH:mm:ss");
}

// 格式化日期时间（从时间戳）
function formatDateTime(timestamp: number | undefined | null) {
  if (!timestamp) return "N/A";
  return dayjs(timestamp * 1000).format("YYYY-MM-DD HH:mm:ss");
}

// 格式化时间戳为日期（计费周期等使用）
function formatTimestamp(timestamp: number | undefined | null) {
  if (!timestamp) return "N/A";
  return dayjs(timestamp * 1000).format("YYYY-MM-DD HH:mm");
}

// 获取到期倒计时文字
function getExpireCountdown(timestamp: number | undefined | null): string {
  if (!timestamp) return "";
  const expireDate = dayjs(timestamp * 1000);
  const now = dayjs();
  const diffDays = expireDate.diff(now, "day");

  if (diffDays < 0) {
    return `已过期 ${Math.abs(diffDays)} 天`;
  } else if (diffDays === 0) {
    const diffHours = expireDate.diff(now, "hour");
    if (diffHours <= 0) {
      return "即将到期";
    }
    return `剩余 ${diffHours} 小时`;
  } else if (diffDays <= 7) {
    return `剩余 ${diffDays} 天`;
  } else if (diffDays <= 30) {
    return `剩余 ${diffDays} 天`;
  } else {
    const diffMonths = expireDate.diff(now, "month");
    if (diffMonths >= 1) {
      return `剩余 ${diffMonths} 个月`;
    }
    return `剩余 ${diffDays} 天`;
  }
}

// 获取到期状态样式类
function getExpireClass(timestamp: number | undefined | null): string {
  if (!timestamp) return "";
  const expireDate = dayjs(timestamp * 1000);
  const now = dayjs();
  const diffDays = expireDate.diff(now, "day");

  if (diffDays < 0) {
    return "expired";
  } else if (diffDays <= 3) {
    return "critical";
  } else if (diffDays <= 7) {
    return "warning";
  } else {
    return "normal";
  }
}

// 获取配额使用状态样式类
function getQuotaClass(percentage: number): string {
  if (percentage >= 90) {
    return "critical";
  } else if (percentage >= 70) {
    return "warning";
  } else {
    return "normal";
  }
}

// 格式化大数字（转换为K/M/B格式）
function formatLargeNumber(num: number | undefined | null) {
  if (!num) return "0";
  if (num >= 1000000000) return `${(num / 1000000000).toFixed(1)}B`;
  if (num >= 1000000) return `${(num / 1000000).toFixed(1)}M`;
  if (num >= 1000) return `${(num / 1000).toFixed(1)}K`;
  return num.toString();
}

// 格式化存储大小（转换为GB）
function formatStorageSize(bytes: number | undefined | null) {
  if (!bytes) return "0 GB";
  // 如果是很大的数（无限制），显示为"无限制"
  if (bytes > 1000000000000) return "无限制";
  // 假设输入是KB，转换为GB
  const gb = bytes / 1024;
  return `${gb.toFixed(2)} GB`;
}

// 格式化API限制（无限制显示为-1）
function formatApiLimit(limit: number | undefined | null) {
  if (!limit) return "0";
  // 如果是很大的数（无限制），显示为-1
  if (limit > 1000000000000) return "-1";
  return limit.toString();
}

// 格式化套餐名称
function formatPlanName(name: string | undefined | null) {
  if (!name) return t("dialog.accountInfo.helper.plan.unknown");
  const names: Record<string, string> = {
    pro: t("dialog.accountInfo.helper.plan.pro"),
    teams: t("dialog.accountInfo.helper.plan.teams"),
    enterprise: t("dialog.accountInfo.helper.plan.enterprise"),
    free: t("dialog.accountInfo.helper.plan.free"),
    starter: t("dialog.accountInfo.helper.plan.starter"),
  };
  return names[name.toLowerCase()] || name;
}

// 格式化Firebase时间戳
function formatFirebaseTimestamp(
  timestamp: string | number | null | undefined,
) {
  if (!timestamp) return "N/A";

  // 如果是ISO字符串格式（2025-11-20T12:32:28.415381Z）
  if (typeof timestamp === "string" && timestamp.includes("T")) {
    return dayjs(timestamp).format("YYYY-MM-DD HH:mm:ss");
  }

  // 如果是毫秒级时间戳
  if (
    typeof timestamp === "number" ||
    (typeof timestamp === "string" && /^\d+$/.test(timestamp))
  ) {
    const ts = parseInt(timestamp.toString());
    // 判断是秒还是毫秒
    if (ts < 10000000000) {
      // 秒级时间戳
      return dayjs(ts * 1000).format("YYYY-MM-DD HH:mm:ss");
    } else {
      // 毫秒级时间戳
      return dayjs(ts).format("YYYY-MM-DD HH:mm:ss");
    }
  }

  return formatDate(timestamp);
}

// 格式化认证提供商名称
function formatProviderName(providerId: string | null | undefined) {
  if (!providerId) return t("dialog.accountInfo.helper.provider.unknown");

  const providerNames: Record<string, string> = {
    password: t("dialog.accountInfo.helper.provider.password"),
    "google.com": "Google",
    "facebook.com": "Facebook",
    "twitter.com": "Twitter",
    "github.com": "GitHub",
    "apple.com": "Apple",
    "microsoft.com": "Microsoft",
    phone: t("dialog.accountInfo.helper.provider.phone"),
    anonymous: t("dialog.accountInfo.helper.provider.anonymous"),
  };

  return providerNames[providerId.toLowerCase()] || providerId;
}

// 复制推荐链接
async function copyReferralLink(referralCode: string | undefined) {
  if (!referralCode) {
    ElMessage.warning("推荐码不存在");
    return;
  }

  const referralLink = `https://windsurf.com/refer?referral_code=${referralCode}`;
  await copyText(referralLink, "推荐链接已复制到剪贴板");
}

// 通用复制函数
async function copyText(
  text: string | undefined,
  message: string = "内容已复制",
) {
  if (!text) {
    ElMessage.warning("无可复制内容");
    return;
  }

  try {
    await navigator.clipboard.writeText(text);
    ElMessage.success(message);
  } catch (err) {
    // 如果 Clipboard API 失败，使用备用方法
    const textarea = document.createElement("textarea");
    textarea.value = text;
    textarea.style.position = "fixed";
    textarea.style.opacity = "0";
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand("copy");
    document.body.removeChild(textarea);
    ElMessage.success(message);
  }
}

// 格式化积分显示（原始值÷100）
function formatCredits(value: number | undefined | null) {
  if (value === undefined || value === null) return 0;
  const result = value / 100;
  // 如果是整数则不显示小数，否则保留最多2位小数
  return Number.isInteger(result) ? result : Math.round(result * 100) / 100;
}

// 检查是否有用户标志
const hasUserFlags = computed(() => {
  const u = userDetails.value?.user;
  return (
    u?.pro ||
    u?.public_profile_enabled ||
    u?.newsletter ||
    u?.used_trial ||
    u?.disable_codeium ||
    u?.disabled_telemetry
  );
});

// 检查是否有团队标志
const hasTeamFlags = computed(() => {
  const t = userDetails.value?.team;
  return (
    t?.subscription_active ||
    t?.used_trial ||
    t?.attribution_enabled ||
    t?.sso_provider_id ||
    t?.offers_enabled ||
    t?.top_up_enabled
  );
});

// 计算剩余积分 = 月度配额 - 已用积分（优先使用团队数据，否则用用户数据）
const remainingPromptCredits = computed(() => {
  const monthly = userDetails.value?.plan?.monthly_prompt_credits || 0;
  const used =
    userDetails.value?.team?.used_prompt_credits ??
    userDetails.value?.user?.used_prompt_credits ??
    0;
  return Math.max(0, monthly - used);
});

const remainingFlowCredits = computed(() => {
  const monthly = userDetails.value?.plan?.monthly_flow_credits || 0;
  const used =
    userDetails.value?.team?.used_flow_credits ??
    userDetails.value?.user?.used_flow_credits ??
    0;
  return Math.max(0, monthly - used);
});

const remainingFlexCredits = computed(() => {
  const quota = userDetails.value?.team?.flex_credit_quota || 0;
  const used = userDetails.value?.team?.used_flex_credits || 0;
  return Math.max(0, quota - used);
});

// Firebase UID
const firebaseUid = computed(() => {
  const info = accountInfo.value?.firebase_info;
  return info?.localId || info?.local_id || info?.uid || "-";
});

// 总已用积分 (使用subscription中的used_quota或team中的used_prompt_credits)
const totalUsedCredits = computed(() => {
  // 优先使用subscription中的used_quota
  if (userDetails.value?.subscription?.used_quota) {
    return userDetails.value.subscription.used_quota;
  }
  // 否则使用team中的used_prompt_credits
  return (
    userDetails.value?.team?.used_prompt_credits ??
    userDetails.value?.user?.used_prompt_credits ??
    0
  );
});

// 总配额积分 (基础配额 + flex配额)
const totalQuotaCredits = computed(() => {
  // 优先使用subscription中的quota（已经是计算后的总配额）
  if (userDetails.value?.subscription?.quota) {
    return userDetails.value.subscription.quota;
  }
  // 否则手动计算：plan中的月度积分 + team中的flex配额
  const monthlyPrompt = userDetails.value?.plan?.monthly_prompt_credits || 0;
  const flexQuota = userDetails.value?.team?.flex_credit_quota || 0;
  return monthlyPrompt + flexQuota;
});

// 获取座位数
const seatCount = computed(() => {
  // 优先从subscription获取
  if (userDetails.value?.subscription?.seats) {
    return userDetails.value.subscription.seats;
  }
  // 否则从team获取
  return userDetails.value?.team?.num_seats_current_billing_period || 0;
});

// 获取订阅层级（优先plan > team）
const subscriptionTier = computed(() => {
  // 优先从plan获取
  if (userDetails.value?.plan?.teams_tier) {
    return userDetails.value.plan.teams_tier;
  }
  // 否则从team获取
  return userDetails.value?.team?.teams_tier || 0;
});

// 获取订阅是否激活
const isSubscriptionActive = computed(() => {
  // 优先从subscription获取
  if (userDetails.value?.subscription?.subscription_active !== undefined) {
    return userDetails.value.subscription.subscription_active;
  }
  // 否则从team获取
  return userDetails.value?.team?.subscription_active || false;
});

// 配额使用百分比
const quotaPercentage = computed(() => {
  if (totalQuotaCredits.value === 0) return 0;
  return Math.min(
    100,
    Math.round((totalUsedCredits.value / totalQuotaCredits.value) * 100),
  );
});

// 格式化团队层级 (对应 codeium_common_pb.TeamsTier 枚举)
function formatTeamsTier(tier: number | undefined | null) {
  if (!tier) return t("dialog.accountInfo.helper.tier.unspecified");
  const tiers: Record<number, string> = {
    0: t("dialog.accountInfo.helper.tier.unspecified"),
    1: t("dialog.accountInfo.helper.tier.teams"),
    2: t("dialog.accountInfo.helper.tier.pro"),
    3: t("dialog.accountInfo.helper.tier.enterpriseSaaS"),
    4: t("dialog.accountInfo.helper.tier.hybrid"),
    5: t("dialog.accountInfo.helper.tier.enterpriseSelf"),
    6: t("dialog.accountInfo.helper.tier.proWaitlist"),
    7: t("dialog.accountInfo.helper.tier.teamsUltimate"),
    8: t("dialog.accountInfo.helper.tier.proUltimate"),
    9: t("dialog.accountInfo.helper.tier.trial"),
    10: t("dialog.accountInfo.helper.tier.enterpriseSelfServe"),
  };
  return (
    tiers[tier] || t("dialog.accountInfo.helper.tier.level", { level: tier })
  );
}

// 获取团队层级标签类型
function getTeamsTierType(
  tier: number | undefined | null,
): "primary" | "success" | "warning" | "danger" | "info" {
  if (!tier) return "info";
  // Enterprise 相关
  if ([3, 4, 5, 10].includes(tier)) return "danger";
  // Pro 相关
  if ([2, 6, 8].includes(tier)) return "success";
  // Teams 相关
  if ([1, 7].includes(tier)) return "primary";
  // Trial
  if (tier === 9) return "warning";
  return "info";
}

// 格式化用户团队状态 (对应 codeium_common_pb.UserTeamStatus 枚举)
function formatTeamStatus(status: number | undefined | null) {
  if (status === undefined || status === null)
    return t("dialog.accountInfo.helper.status.unknown");
  const statuses: Record<number, string> = {
    0: t("dialog.accountInfo.helper.status.unspecified"),
    1: t("dialog.accountInfo.helper.status.pending"),
    2: t("dialog.accountInfo.helper.status.approved"),
    3: t("dialog.accountInfo.helper.status.rejected"),
  };
  return (
    statuses[status] || t("dialog.accountInfo.helper.status.status", { status })
  );
}

// 获取用户团队状态标签类型
function getTeamStatusType(
  status: number | undefined | null,
): "primary" | "success" | "warning" | "danger" | "info" {
  if (status === 3) return "danger"; // 已拒绝
  if (status === 2) return "success"; // 已批准
  if (status === 1) return "warning"; // 待审批
  return "info";
}

// 检查是否有某项权限
function hasPermission(permissions: any, index: number): boolean {
  if (!permissions) return false;
  // 如果是对象格式 {0: 1, 1: 2, ...}
  if (typeof permissions === "object" && !Array.isArray(permissions)) {
    return permissions[index - 1] !== undefined;
  }
  // 如果是数组格式
  if (Array.isArray(permissions)) {
    return permissions.includes(index);
  }
  return false;
}

// 获取权限数量
function getPermissionCount(permissions: any): number {
  if (!permissions) return 0;
  if (typeof permissions === "object" && !Array.isArray(permissions)) {
    return Object.keys(permissions).length;
  }
  if (Array.isArray(permissions)) {
    return permissions.length;
  }
  return 0;
}
</script>

<style scoped lang="scss">
.account-info-dialog {
  :deep(.el-dialog__body) {
    padding: 0;
  }
}

.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px;
  color: #909399;

  p {
    margin-top: 12px;
  }
}

.dialog-content {
  display: flex;
  flex-direction: column;
  min-height: 500px;
}

.custom-tabs {
  :deep(.el-tabs__header) {
    margin: 0;
    padding: 0 20px;
    background: #f5f7fa;
    border-bottom: 1px solid #e4e7ed;
  }

  :deep(.el-tabs__content) {
    padding: 24px;
    overflow-y: auto;
    max-height: 600px;
  }

  .tab-label {
    display: flex;
    align-items: center;
    gap: 6px;
  }
}

/* 用户详情样式 */
.details-container {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.profile-header {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 20px;
  background: linear-gradient(135deg, #f6f8fb 0%, #e9ecef 100%);
  border-radius: 16px;
  overflow: hidden;

  // 套餐主题色（层级：Free < Trial < Pro < Teams < Enterprise）
  &.plan-free {
    background: linear-gradient(135deg, #f5f5f5 0%, #e8e8e8 100%);
  }
  &.plan-trial {
    background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
  }
  &.plan-pro {
    background: linear-gradient(135deg, #dbeafe 0%, #bfdbfe 100%);
  }
  &.plan-teams {
    background: linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%);
  }
  &.plan-enterprise {
    background: linear-gradient(135deg, #ede9fe 0%, #ddd6fe 100%);
  }

  .header-bg-icon {
    position: absolute;
    right: -20px;
    top: -20px;
    font-size: 120px;
    opacity: 0.06;
    color: #000;
  }

  .profile-main {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .avatar-wrapper {
    position: relative;

    .avatar-placeholder {
      width: 64px;
      height: 64px;
      background: linear-gradient(135deg, #409eff 0%, #3a8ee6 100%);
      color: white;
      font-size: 28px;
      font-weight: 700;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 50%;
      box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
    }

    .status-dot {
      position: absolute;
      bottom: 2px;
      right: 2px;
      width: 14px;
      height: 14px;
      border-radius: 50%;
      border: 2px solid #fff;

      &.active {
        background: #67c23a;
      }
      &.inactive {
        background: #909399;
      }
    }
  }

  .profile-info {
    flex: 1;
    min-width: 0;

    .name-row {
      display: flex;
      align-items: center;
      gap: 10px;
      margin-bottom: 4px;
      flex-wrap: wrap;

      .user-name {
        margin: 0;
        font-size: 18px;
        font-weight: 700;
        color: #303133;
      }
    }

    .email-row {
      display: flex;
      align-items: center;
      gap: 6px;
      color: #606266;
      font-size: 14px;
      margin-bottom: 6px;
    }

    .meta-row {
      display: flex;
      align-items: center;
      gap: 12px;
      flex-wrap: wrap;

      .meta-item {
        display: flex;
        align-items: center;
        gap: 4px;
        font-size: 12px;
        color: #909399;

        .el-icon {
          font-size: 14px;
        }
      }
    }

    .copy-icon {
      cursor: pointer;
      font-size: 14px;
      color: #a0a5a8;
      transition: color 0.2s;

      &:hover {
        color: #409eff;
      }
    }
  }

  .profile-plan-badge {
    position: absolute;
    top: 16px;
    right: 16px;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 12px;
    background: linear-gradient(135deg, #e6a23c, #d4940d);
    color: #fff;
    font-size: 12px;
    font-weight: 600;
    border-radius: 20px;
    box-shadow: 0 2px 8px rgba(230, 162, 60, 0.4);
  }

  .profile-expire-badge {
    position: absolute;
    bottom: 16px;
    right: 16px;
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    background: rgba(255, 255, 255, 0.95);
    border-radius: 16px;
    font-size: 11px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);

    .el-icon {
      color: #409eff;
      font-size: 13px;
    }

    .expire-text {
      color: #606266;
      font-weight: 500;
    }

    .expire-countdown-tag {
      padding: 2px 8px;
      border-radius: 10px;
      font-size: 10px;
      font-weight: 600;

      &.normal {
        background: linear-gradient(135deg, #e1f3d8 0%, #c2e7b0 100%);
        color: #67c23a;
      }

      &.warning {
        background: linear-gradient(135deg, #fdf6ec 0%, #faecd8 100%);
        color: #e6a23c;
      }

      &.critical {
        background: linear-gradient(135deg, #fef0f0 0%, #fde2e2 100%);
        color: #f56c6c;
        animation: pulse 1.5s ease-in-out infinite;
      }

      &.expired {
        background: linear-gradient(135deg, #909399 0%, #606266 100%);
        color: white;
      }
    }
  }
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.6;
  }
}

.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;

  .grid-column {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
}

.info-card {
  background: #fff;
  border: 1px solid #e4e7ed;
  border-radius: 12px;
  padding: 16px;
  transition: all 0.3s ease;

  &:hover {
    border-color: #c0c4cc;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.05);
  }

  &.wide {
    width: 100%;
  }

  &.plan-card-bg {
    background: linear-gradient(to bottom right, #fff, #fcfcfc);
  }

  .card-title {
    font-size: 14px;
    font-weight: 600;
    color: #303133;
    margin-bottom: 12px;
    display: flex;
    align-items: center;
    gap: 8px;

    .el-icon {
      color: #909399;
    }

    &.compact {
      margin-bottom: 8px;
      font-size: 13px;
    }

    .header-tags {
      margin-left: auto;
    }
  }

  .card-content {
    font-size: 13px;
  }
}

/* 使用量/配额美化展示 */
.quota-display-card {
  margin-top: 12px;
  padding: 12px 14px;
  background: linear-gradient(135deg, #f8fafc 0%, #f1f5f9 100%);
  border: 1px solid #e2e8f0;
  border-radius: 10px;

  .quota-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;

    .quota-title {
      font-size: 12px;
      color: #64748b;
      font-weight: 500;
    }

    .quota-percentage {
      font-size: 14px;
      font-weight: 700;
      padding: 2px 8px;
      border-radius: 12px;

      &.normal {
        color: #10b981;
        background: linear-gradient(135deg, #d1fae5 0%, #a7f3d0 100%);
      }

      &.warning {
        color: #f59e0b;
        background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
      }

      &.critical {
        color: #ef4444;
        background: linear-gradient(135deg, #fee2e2 0%, #fecaca 100%);
        animation: pulse 1.5s ease-in-out infinite;
      }
    }
  }

  .quota-progress-wrap {
    margin-bottom: 10px;

    .quota-progress-bar {
      height: 8px;
      background: #e2e8f0;
      border-radius: 4px;
      overflow: hidden;

      .quota-progress-fill {
        height: 100%;
        border-radius: 4px;
        transition: width 0.5s ease;

        &.normal {
          background: linear-gradient(90deg, #34d399 0%, #10b981 100%);
        }

        &.warning {
          background: linear-gradient(90deg, #fbbf24 0%, #f59e0b 100%);
        }

        &.critical {
          background: linear-gradient(90deg, #f87171 0%, #ef4444 100%);
        }
      }
    }
  }

  .quota-details {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 16px;

    .quota-used,
    .quota-total {
      display: flex;
      flex-direction: column;
      align-items: center;

      .quota-label {
        font-size: 10px;
        color: #94a3b8;
      }

      .quota-value {
        font-size: 16px;
        font-weight: 700;
        color: #334155;
      }
    }

    .quota-divider {
      font-size: 18px;
      color: #cbd5e1;
      font-weight: 300;
    }
  }
}

/* 团队信息卡片美化 */
.team-info-card {
  .team-basic-info {
    margin-bottom: 12px;

    .team-info-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 6px 0;

      &:not(:last-child) {
        border-bottom: 1px dashed #f0f2f5;
      }

      .info-label {
        color: #909399;
        font-size: 12px;
      }

      .info-value {
        color: #303133;
        font-weight: 500;
        font-size: 13px;

        &.team-name {
          color: #409eff;
          font-weight: 600;
        }
      }
    }
  }

  .id-info-section {
    background: #f8f9fa;
    border-radius: 8px;
    padding: 10px;
    margin-bottom: 12px;

    .id-row {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 6px 0;

      &:not(:last-child) {
        border-bottom: 1px solid #ebeef5;
      }

      .id-label {
        color: #909399;
        font-size: 11px;
        min-width: 60px;
        flex-shrink: 0;
      }

      .id-value-wrap {
        display: flex;
        align-items: center;
        gap: 6px;
        flex: 1;
        min-width: 0;
        justify-content: flex-end;

        .id-code {
          font-family: "Consolas", "Monaco", monospace;
          font-size: 10px;
          color: #606266;
          background: #fff;
          padding: 3px 6px;
          border-radius: 4px;
          border: 1px solid #e4e7ed;
          max-width: 180px;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
          cursor: pointer;

          &.stripe {
            color: #635bff;
            border-color: #e8e6ff;
            background: #fafaff;
          }

          &:hover {
            background: #ecf5ff;
            border-color: #409eff;
          }
        }

        .el-button {
          padding: 4px;
          height: 22px;
          width: 22px;
        }
      }
    }
  }

  .team-stats {
    display: flex;
    gap: 8px;
    margin-bottom: 12px;

    .stat-box {
      flex: 1;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 10px 8px;
      background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
      border-radius: 8px;
      border: 1px solid #bae6fd;

      .stat-number {
        font-size: 20px;
        font-weight: 700;
        color: #0284c7;
      }

      .stat-text {
        font-size: 10px;
        color: #64748b;
        margin-top: 2px;
      }
    }
  }

  .cascade-period {
    background: linear-gradient(135deg, #fefce8 0%, #fef9c3 100%);
    border: 1px solid #fde047;
    border-radius: 8px;
    padding: 10px;
    margin-bottom: 12px;

    .period-header {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 11px;
      color: #a16207;
      font-weight: 600;
      margin-bottom: 8px;

      .el-icon {
        font-size: 14px;
      }
    }

    .period-dates {
      display: flex;
      align-items: center;
      justify-content: space-between;

      .period-date {
        display: flex;
        flex-direction: column;
        align-items: center;

        .date-label {
          font-size: 10px;
          color: #92400e;
        }

        .date-value {
          font-size: 11px;
          font-weight: 600;
          color: #78350f;
          font-family: "Consolas", monospace;
        }
      }

      .period-arrow {
        color: #d97706;
        font-size: 16px;
      }
    }
  }

  .credits-usage {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 12px;

    .usage-item {
      display: flex;
      align-items: center;
      gap: 6px;
      padding: 4px 10px;
      background: #f5f7fa;
      border-radius: 20px;
      font-size: 11px;

      .usage-label {
        color: #909399;
      }

      .usage-value {
        color: #303133;
        font-weight: 600;
      }
    }
  }

  .team-flags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;

    .el-tag {
      font-size: 11px;

      .el-icon {
        margin-right: 2px;
      }
    }
  }
}

.info-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;

  &:last-child {
    margin-bottom: 0;
  }

  .label {
    color: #909399;
  }
  .value {
    color: #606266;
    font-weight: 500;
    text-align: right;
    max-width: 65%;
  }

  .text-ellipsis {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .value-group {
    display: flex;
    align-items: center;
    gap: 4px;

    .action-icon {
      cursor: pointer;
      color: #409eff;
    }
  }

  &.small {
    font-size: 12px;
    margin-bottom: 4px;
  }
}

.stats-row {
  display: flex;
  justify-content: space-around;
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid #f0f2f5;

  .stat-item {
    display: flex;
    flex-direction: column;
    align-items: center;

    .stat-num {
      font-size: 18px;
      font-weight: 700;
      color: #303133;
    }
    .stat-label {
      font-size: 12px;
      color: #909399;
    }
  }

  &.compact {
    margin-top: 8px;
    padding-top: 8px;
    gap: 16px;
    justify-content: flex-start;

    .stat-item {
      .stat-num {
        font-size: 16px;
        color: #409eff;
      }
      .stat-label {
        font-size: 11px;
      }
    }
  }
}

.credits-stats {
  margin-top: 10px;
  padding: 8px 12px;
  background: linear-gradient(135deg, #f0f9ff 0%, #e0f2fe 100%);
  border-radius: 8px;
}

/* 订阅时间和配额信息 */
.subscription-time,
.quota-info {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px dashed #ebeef5;

  .period-range {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: #606266;

    .el-icon {
      color: #909399;
      font-size: 12px;
    }
  }
}

.quota-info .money {
  color: #67c23a;
  font-weight: 600;
}

/* 功能开关样式 */
.feature-switches {
  margin-top: 16px;
  padding-top: 12px;
  border-top: 1px dashed #ebeef5;

  .feature-label {
    font-size: 12px;
    color: #909399;
    margin-bottom: 8px;
  }

  .feature-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;

    .el-tag {
      display: flex;
      align-items: center;
      gap: 4px;

      .el-icon {
        font-size: 12px;
      }
    }
  }
}

/* 套餐卡片特殊样式 */
.plan-badge {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;

  .plan-name {
    font-size: 16px;
    font-weight: 700;
    color: #e6a23c;
  }
}

.usage-progress {
  margin-bottom: 16px;

  .progress-label {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    color: #606266;
    margin-bottom: 4px;
  }
}

.limits-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;

  .limit-item {
    background: #f5f7fa;
    padding: 8px;
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    align-items: center;

    .limit-val {
      font-weight: 600;
      color: #303133;
    }
    .limit-label {
      font-size: 11px;
      color: #909399;
    }
  }
}

.credits-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
  margin-bottom: 12px;

  .credit-item {
    border: 1px dashed #dcdfe6;
    border-radius: 8px;
    padding: 10px 4px;
    text-align: center;

    .credit-val {
      display: block;
      font-size: 16px;
      font-weight: 700;
      color: #67c23a;
    }
    .credit-label {
      font-size: 12px;
      color: #909399;
    }
  }
}

/* 用户标志标签 */
.flag-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px dashed #ebeef5;

  .el-tag {
    display: flex;
    align-items: center;
    gap: 4px;
    .el-icon {
      font-size: 12px;
    }
  }
}

/* Stripe代码样式 */
.stripe-code {
  font-size: 11px;
  font-family: "Roboto Mono", monospace;
  background: #f5f7fa;
  padding: 2px 6px;
  border-radius: 4px;
  color: #606266;
}

/* 权限位图可视化 */
.permission-visual {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px dashed #ebeef5;

  .perm-label {
    font-size: 12px;
    color: #909399;
    margin-bottom: 8px;
  }

  .perm-dots {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;

    .perm-dot {
      width: 10px;
      height: 10px;
      border-radius: 2px;
      background: #ebeef5;
      cursor: help;
      transition: all 0.2s;

      &.active {
        background: linear-gradient(135deg, #67c23a, #85ce61);
      }
    }
  }
}

/* 计费周期 */
.billing-period {
  margin: 12px 0;
  padding: 10px;
  background: #f5f7fa;
  border-radius: 8px;

  .period-label {
    font-size: 11px;
    color: #909399;
    margin-bottom: 4px;
  }

  .period-range {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 12px;
    color: #606266;

    .el-icon {
      color: #c0c4cc;
    }
  }
}

/* 充值信息 */
.topup-info {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px dashed #ebeef5;
}

/* 配额详情 */
.quota-details {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px dashed #ebeef5;

  .detail-row {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    margin-bottom: 4px;

    .label {
      color: #909399;
    }
    .value {
      color: #606266;
      font-weight: 500;
    }
  }
}

/* 功能标签 */
.feature-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

/* 状态标签组 */
.status-tags {
  display: flex;
  gap: 6px;
}

/* 套餐卡片grid改为4列 */
.limits-grid {
  grid-template-columns: repeat(4, 1fr) !important;
}

/* 权限标签 */
.permissions-section {
  margin-top: 10px;

  .section-title {
    font-size: 13px;
    font-weight: 600;
    color: #303133;
    margin-bottom: 8px;
  }

  .permissions-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
}

/* Firebase信息网格布局 */
.firebase-info-grid {
  display: flex;
  flex-direction: column;
  gap: 12px;

  .info-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 24px;
    padding: 8px 0;
    border-bottom: 1px solid #f0f2f5;

    &:last-child {
      border-bottom: none;
    }
  }

  .info-cell {
    display: flex;
    flex-direction: column;
    gap: 4px;

    &.full-width {
      grid-column: 1 / -1;
    }

    .label {
      font-size: 12px;
      color: #909399;
      font-weight: 500;
    }

    .value {
      font-size: 13px;
      color: #303133;
      word-break: break-all;
    }
  }
}

/* 时间轴样式 */
.horizontal-timeline {
  display: flex;
  justify-content: space-between;
  position: relative;
  padding: 20px 10px 0;

  &::before {
    content: "";
    position: absolute;
    top: 34px;
    left: 20px;
    right: 20px;
    height: 2px;
    background: #ebeef5;
    z-index: 0;
  }

  &.four-items {
    padding: 20px 0 0;

    .timeline-item {
      min-width: 0;
    }
  }

  .timeline-item {
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    flex: 1;

    .timeline-dot {
      width: 30px;
      height: 30px;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;
      margin-bottom: 8px;
      box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);

      &.dot-blue {
        background: #409eff;
      }
      &.dot-green {
        background: #67c23a;
      }
      &.dot-orange {
        background: #e6a23c;
      }
      &.dot-gray {
        background: #909399;
      }
    }

    .timeline-content {
      text-align: center;

      .timeline-title {
        font-size: 12px;
        font-weight: 600;
        color: #303133;
      }
      .timeline-time {
        font-size: 11px;
        color: #909399;
        margin-top: 2px;
      }
    }
  }
}

/* Firebase提供商列表 */
.provider-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}

/* 其他通用样式 */
.raw-data-collapse {
  margin-top: 24px;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid #e4e7ed;

  :deep(.el-collapse-item__header) {
    background: #f5f7fa;
    padding-left: 16px;
    font-size: 12px;
    color: #909399;
  }
}

.raw-data,
.raw-json {
  margin: 0;
  padding: 12px;
  font-size: 11px;
  font-family: "Roboto Mono", monospace;
  background: #282c34;
  color: #abb2bf;
  overflow-x: auto;
  border-radius: 4px;
  max-height: 300px;
}

.collapse-title {
  font-size: 13px;
  color: #606266;
}

.collapse-arrow {
  margin-left: 8px;
  transition: transform 0.3s;
}

.info-grid-compact {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px 24px;

  .grid-item {
    display: flex;
    flex-direction: column;
    gap: 4px;

    .label {
      font-size: 12px;
      color: #909399;
    }
    .value {
      font-size: 13px;
      color: #606266;
      font-weight: 500;
    }
  }
}

/* 主布局：基础信息 + 订阅套餐 */
.main-info-layout {
  display: grid;
  grid-template-columns: 1fr 340px;
  gap: 16px;
  margin-bottom: 16px;

  .basic-info-card {
    margin-bottom: 0;
  }

  .right-column {
    display: flex;
    flex-direction: column;
    gap: 12px;

    .info-card {
      margin-bottom: 0;
    }
  }
}

/* 角色与权限区域 */
.role-permission-section {
  margin-bottom: 16px;

  .info-card {
    margin-bottom: 0;
  }

  .role-content {
    .role-info-row {
      display: flex;
      flex-wrap: wrap;
      gap: 24px;
      margin-bottom: 12px;

      .info-item {
        display: flex;
        align-items: center;
        gap: 8px;

        .label {
          color: #909399;
          font-size: 13px;
        }

        .value {
          color: #303133;
          font-weight: 500;
        }
      }
    }
  }
}

.limits-grid.compact {
  grid-template-columns: repeat(3, 1fr);
  gap: 8px;

  .limit-item {
    padding: 8px;

    .limit-val {
      font-size: 16px;
    }
    .limit-label {
      font-size: 10px;
    }
  }
}

.credits-grid.compact {
  .credit-item {
    padding: 12px 16px;

    .credit-val {
      font-size: 22px;
    }
  }
}

.feature-switches.compact {
  margin-top: 8px;
  padding-top: 8px;

  .feature-label {
    font-size: 11px;
    margin-bottom: 6px;
  }
  .feature-tags {
    gap: 4px;
  }
  .feature-tags .el-tag {
    font-size: 11px;
    padding: 2px 6px;
  }
}

/* 套餐限制信息表格 */
.plan-limits-table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 12px;
  border-top: 1px solid #f0f2f5;

  tr {
    border-bottom: 1px solid #f0f2f5;
    &:last-child {
      border-bottom: none;
    }
  }

  td {
    padding: 10px 8px;
    font-size: 12px;
    vertical-align: middle;
  }

  .label-cell {
    color: #909399;
    white-space: nowrap;
    width: 70px;
  }

  .value-cell {
    color: #303133;
    font-weight: 500;
  }
}

.local-info-container {
  padding: 8px 0;
}

.local-info-table {
  width: 100%;
  border-collapse: collapse;

  tr {
    border-bottom: 1px solid #f0f2f5;

    &:last-child {
      border-bottom: none;
    }
  }

  td {
    padding: 14px 16px;
    font-size: 13px;
    vertical-align: middle;
  }

  .label-cell {
    width: 120px;
    color: #303133;
    font-weight: 500;
    white-space: nowrap;
  }

  .value-cell {
    color: #606266;
    word-break: break-all;
  }

  .empty-text {
    color: #909399;
  }
}

/* 基础信息表格 */
.basic-info-card {
  margin-bottom: 16px;
}

.basic-info-table {
  width: 100%;
  border-collapse: collapse;

  tr {
    border-bottom: 1px solid #f0f2f5;

    &:last-child {
      border-bottom: none;
    }
  }

  td {
    padding: 12px 16px;
    font-size: 13px;
    vertical-align: middle;
  }

  .label-cell {
    width: 120px;
    color: #303133;
    font-weight: 500;
    white-space: nowrap;
  }

  .value-cell {
    color: #606266;
    word-break: break-all;

    &.text-muted {
      color: #909399;
    }
    &.text-primary {
      color: #409eff;
    }
    &.text-warning {
      color: #e6a23c;
    }
  }

  .copy-btn {
    margin-left: 8px;
    color: #909399;
    cursor: pointer;
    transition: color 0.2s;

    &:hover {
      color: #409eff;
    }
  }

  .seat-count {
    font-size: 20px;
    font-weight: 600;
    color: #303133;
  }

  .quota-cell {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .quota-text {
    color: #606266;
    font-size: 13px;
  }
}

.flag-tags-bottom {
  padding: 12px 16px;
  border-top: 1px solid #f0f2f5;
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: 12px;

  .info-row {
    display: flex;
    justify-content: space-between;
    border-bottom: 1px dashed #f0f2f5;
    padding-bottom: 8px;

    &:last-child {
      border-bottom: none;
      padding-bottom: 0;
    }

    .label {
      color: #909399;
    }
    .value {
      color: #303133;
      font-weight: 500;
    }
  }
}

.dialog-footer {
  padding: 16px 24px;
  border-top: 1px solid #e4e7ed;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

/* 响应式 */
@media (max-width: 700px) {
  .info-grid {
    grid-template-columns: 1fr;
  }
  .profile-header {
    flex-direction: column;
    text-align: center;
  }
  .profile-info {
    .name-row,
    .email-row,
    .id-row {
      justify-content: center;
    }
  }
  .info-grid-compact {
    grid-template-columns: 1fr;
  }
}

/* 暗色模式适配 */
:root.dark {
  .custom-tabs :deep(.el-tabs__header) {
    background: #1d1e1f;
    border-bottom-color: #4c4d4f;
  }

  .profile-header {
    background: linear-gradient(135deg, #262729 0%, #1d1e1f 100%);

    // 暗色主题套餐主题色
    &.plan-free {
      background: linear-gradient(135deg, #2a2a2a 0%, #1f1f1f 100%);
    }
    &.plan-trial {
      background: linear-gradient(135deg, #3d2e1a 0%, #2d2215 100%);
    }
    &.plan-pro {
      background: linear-gradient(135deg, #1a2a3a 0%, #152535 100%);
    }
    &.plan-teams {
      background: linear-gradient(135deg, #1a2a25 0%, #152520 100%);
    }
    &.plan-enterprise {
      background: linear-gradient(135deg, #2a1a3a 0%, #251535 100%);
    }

    .user-name {
      color: #e5eaf3;
    }
    .email-row {
      color: #a3a6ad;
    }
    .meta-row .meta-item {
      color: #a3a6ad;
    }
    .profile-plan-badge {
      background: linear-gradient(135deg, #c88a30, #a67520);
    }
  }

  .info-card {
    background: #1d1e1f;
    border-color: #4c4d4f;

    &:hover {
      border-color: #606266;
    }

    &.plan-card-bg {
      background: #262729;
    }

    .card-title {
      color: #e5eaf3;
      .el-icon {
        color: #a3a6ad;
      }
    }
  }

  .info-item {
    .label {
      color: #a3a6ad;
    }
    .value {
      color: #cfd3dc;
    }
  }

  .stats-row {
    border-top-color: #4c4d4f;
    .stat-num {
      color: #e5eaf3;
    }
    .stat-label {
      color: #a3a6ad;
    }
  }

  .limit-item {
    background: #262729;
    .limit-val {
      color: #e5eaf3;
    }
    .limit-label {
      color: #a3a6ad;
    }
  }

  .credit-item {
    border-color: #4c4d4f;
    .credit-label {
      color: #a3a6ad;
    }
  }

  .info-list .info-row {
    border-bottom-color: #4c4d4f;
    .label {
      color: #a3a6ad;
    }
    .value {
      color: #e5eaf3;
    }
  }

  .dialog-footer {
    border-top-color: #4c4d4f;
  }

  .raw-data-collapse :deep(.el-collapse-item__header) {
    background: #262729;
    color: #a3a6ad;
  }

  .horizontal-timeline::before {
    background: #4c4d4f;
  }

  .timeline-content {
    .timeline-title {
      color: #e5eaf3;
    }
    .timeline-time {
      color: #a3a6ad;
    }
  }

  // 新增样式的暗色模式
  .flag-tags {
    border-top-color: #4c4d4f;
  }

  .stripe-code {
    background: #262729;
    color: #a3a6ad;
  }

  .permission-visual {
    border-top-color: #4c4d4f;
    .perm-label {
      color: #a3a6ad;
    }
    .perm-dots .perm-dot {
      background: #4c4d4f;
    }
  }

  .billing-period {
    background: #262729;
    .period-label {
      color: #a3a6ad;
    }
    .period-range {
      color: #cfd3dc;
    }
  }

  .topup-info,
  .quota-details {
    border-top-color: #4c4d4f;
  }

  .quota-details .detail-row {
    .label {
      color: #a3a6ad;
    }
    .value {
      color: #cfd3dc;
    }
  }

  .credits-stats {
    background: linear-gradient(135deg, #1a2332 0%, #1e3a5f 100%);
    .stat-num {
      color: #67c23a;
    }
  }

  .firebase-info-grid {
    .info-row {
      border-bottom-color: #4c4d4f;
    }
    .info-cell {
      .label {
        color: #a3a6ad;
      }
      .value {
        color: #e5eaf3;
      }
    }
  }

  .collapse-title {
    color: #a3a6ad;
  }

  .local-info-table {
    tr {
      border-bottom-color: #4c4d4f;
    }
    .label-cell {
      color: #e5eaf3;
    }
    .value-cell {
      color: #cfd3dc;
    }
    .empty-text {
      color: #a3a6ad;
    }
  }

  .basic-info-table {
    tr {
      border-bottom-color: #4c4d4f;
    }
    .label-cell {
      color: #e5eaf3;
    }
    .value-cell {
      color: #cfd3dc;
      &.text-muted {
        color: #a3a6ad;
      }
      &.text-primary {
        color: #79bbff;
      }
      &.text-warning {
        color: #f0a020;
      }
    }
    .seat-count {
      color: #e5eaf3;
    }
    .quota-text {
      color: #a3a6ad;
    }
    .copy-btn {
      color: #a3a6ad;
    }
  }

  .flag-tags-bottom {
    border-top-color: #4c4d4f;
  }

  .plan-limits-table {
    border-top-color: #4c4d4f;
    tr {
      border-bottom-color: #4c4d4f;
    }
    .label-cell {
      color: #a3a6ad;
    }
    .value-cell {
      color: #cfd3dc;
    }
  }
}
</style>
