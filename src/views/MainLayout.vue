<template>
  <el-container class="main-container">
    <!-- 侧边栏 -->
    <el-aside
      :width="sidebarWidth"
      class="sidebar"
      :style="{ overflow: 'hidden' }"
    >
      <div class="app-title">
        <el-icon size="24"><Connection /></el-icon>
        <div v-if="!uiStore.sidebarCollapsed" class="app-title-text">
          <span>Windsurf Manager</span>
          <span class="version-text">v{{ appVersion }}</span>
        </div>
      </div>

      <el-menu
        :collapse="uiStore.sidebarCollapsed"
        :default-active="activeMenu"
        :default-openeds="[]"
        class="sidebar-menu"
        :collapse-transition="false"
      >
        <el-menu-item index="accounts" @click="setActiveMenu('accounts')">
          <el-icon><User /></el-icon>
          <template #title>{{ $t("nav.accounts") }}</template>
        </el-menu-item>

        <el-sub-menu index="groups">
          <template #title>
            <el-icon><Folder /></el-icon>
            <span>{{ $t("dashboard.group.management") }}</span>
          </template>
          <el-menu-item
            v-for="group in settingsStore.groups"
            :key="group"
            :index="`group-${group}`"
            class="group-item"
          >
            <div class="group-item-content">
              <span @click="filterByGroup(group)" class="group-name">{{
                group
              }}</span>
              <div class="group-actions" v-if="group !== '默认分组'">
                <el-icon
                  @click.stop="showRenameGroupDialog(group)"
                  class="group-action-icon"
                >
                  <Edit />
                </el-icon>
                <el-icon
                  @click.stop="showDeleteGroupConfirm(group)"
                  class="group-action-icon delete"
                >
                  <Delete />
                </el-icon>
              </div>
            </div>
          </el-menu-item>
          <el-menu-item index="add-group" @click="showAddGroupDialog">
            <el-icon><Plus /></el-icon>
            {{ $t("dashboard.group.add") }}
          </el-menu-item>
        </el-sub-menu>

        <el-menu-item index="logs" @click="uiStore.openLogsDialog">
          <el-icon><Document /></el-icon>
          <template #title>{{ $t("nav.logs") }}</template>
        </el-menu-item>

        <el-menu-item index="stats" @click="uiStore.openStatsDialog">
          <el-icon><DataAnalysis /></el-icon>
          <template #title>{{ $t("nav.stats") }}</template>
        </el-menu-item>

        <el-menu-item index="auto-reset" @click="showAutoResetDialog = true">
          <el-icon><Timer /></el-icon>
          <template #title>自动重置</template>
        </el-menu-item>

        <el-menu-item
          index="card-generator"
          @click="showCardGeneratorDialog = true"
        >
          <el-icon><CreditCard /></el-icon>
          <template #title>虚拟卡生成</template>
        </el-menu-item>

        <el-menu-item index="about" @click="showAboutDialog">
          <el-icon><InfoFilled /></el-icon>
          <template #title>{{ $t("nav.about") }}</template>
        </el-menu-item>

        <el-menu-item index="settings" @click="uiStore.openSettingsDialog">
          <el-icon><Setting /></el-icon>
          <template #title>{{ $t("nav.settings") }}</template>
        </el-menu-item>
      </el-menu>

      <div class="sidebar-footer">
        <el-button
          :icon="uiStore.sidebarCollapsed ? ArrowRight : ArrowLeft"
          circle
          @click="uiStore.toggleSidebar"
        />
      </div>
    </el-aside>

    <!-- 主内容区 -->
    <el-container>
      <!-- 顶部操作栏 -->
      <el-header class="header">
        <div class="header-left">
          <el-input
            v-model="searchQuery"
            :placeholder="$t('dashboard.searchPlaceholder')"
            :prefix-icon="Search"
            clearable
            class="search-input"
            @input="handleSearch"
          />
          <el-tooltip
            :content="$t('dashboard.advancedFilter')"
            placement="bottom"
          >
            <el-button
              :icon="Filter"
              circle
              :type="hasActiveFilter ? 'primary' : 'default'"
              @click="showFilterPanel = !showFilterPanel"
              class="filter-toggle-btn"
            />
          </el-tooltip>

          <!-- 排序选择器 -->
          <el-select
            v-model="currentSortField"
            :placeholder="$t('dashboard.sort.placeholder')"
            size="default"
            class="sort-select"
            @change="handleSortChange"
          >
            <el-option :label="$t('dashboard.sort.email')" value="email" />
            <el-option
              :label="$t('dashboard.sort.createdAt')"
              value="created_at"
            />
            <el-option
              :label="$t('dashboard.sort.usedQuota')"
              value="used_quota"
            />
            <el-option
              :label="$t('dashboard.sort.remainingQuota')"
              value="remaining_quota"
            />
            <el-option
              :label="$t('dashboard.sort.tokenExpiresAt')"
              value="token_expires_at"
            />
            <el-option
              :label="$t('dashboard.sort.subscriptionExpiresAt')"
              value="subscription_expires_at"
            />
            <el-option
              :label="$t('dashboard.sort.planName')"
              value="plan_name"
            />
          </el-select>
          <el-tooltip
            :content="
              sortDirection === 'asc'
                ? $t('dashboard.sort.asc')
                : $t('dashboard.sort.desc')
            "
            placement="bottom"
          >
            <el-button
              :icon="sortDirection === 'asc' ? SortUp : SortDown"
              circle
              @click="toggleSortDirection"
            />
          </el-tooltip>
        </div>

        <div class="header-right">
          <!-- 批量删除 -->
          <el-tooltip
            :content="$t('dashboard.actions.batchDelete')"
            placement="bottom"
            v-if="accountsStore.selectedAccounts.size > 0"
          >
            <el-badge
              :value="accountsStore.selectedAccounts.size"
              :offset="[12, -8]"
            >
              <el-button
                type="danger"
                :icon="Delete"
                circle
                @click="handleBatchDelete"
              />
            </el-badge>
          </el-tooltip>

          <el-tooltip
            :content="$t('dashboard.actions.batchTransfer')"
            placement="bottom"
            v-if="accountsStore.selectedAccounts.size > 0"
          >
            <el-button
              type="success"
              :icon="Switch"
              circle
              @click="showBatchTransferDialog = true"
            />
          </el-tooltip>

          <!-- 批量刷新状态 -->
          <el-tooltip
            :content="$t('dashboard.actions.batchRefresh')"
            placement="bottom"
            v-if="accountsStore.selectedAccounts.size > 0"
          >
            <el-button
              type="warning"
              :icon="RefreshRight"
              circle
              @click="handleBatchRefresh"
            />
          </el-tooltip>

          <el-tooltip
            :content="$t('dashboard.actions.batchUpdatePlan')"
            placement="bottom"
            v-if="accountsStore.selectedAccounts.size > 0"
          >
            <el-button
              type="primary"
              :icon="Trophy"
              circle
              @click="showBatchUpdatePlanDialog = true"
            />
          </el-tooltip>

          <!-- 导出选中账号 -->
          <el-tooltip
            :content="$t('dashboard.actions.exportSelected')"
            placement="bottom"
            v-if="accountsStore.selectedAccounts.size > 0"
          >
            <el-button
              type="info"
              :icon="Download"
              circle
              @click="handleExportAccounts(true)"
            />
          </el-tooltip>

          <!-- 批量更改分组 -->
          <el-tooltip
            :content="$t('dashboard.group.batchMove')"
            placement="bottom"
            v-if="accountsStore.selectedAccounts.size > 0"
          >
            <el-button
              type="primary"
              :icon="FolderOpened"
              circle
              @click="showBatchGroupDialog = true"
            />
          </el-tooltip>

          <!-- 取消已选 -->
          <el-tooltip
            :content="$t('dashboard.actions.clearSelection')"
            placement="bottom"
            v-if="accountsStore.selectedAccounts.size > 0"
          >
            <el-button
              :icon="Close"
              circle
              style="
                background-color: #909399;
                border-color: #909399;
                color: white;
              "
              @click="accountsStore.clearSelection()"
            />
          </el-tooltip>

          <!-- 选择本页账号 -->
          <el-tooltip
            :content="$t('dashboard.actions.selectCurrentPage')"
            placement="bottom"
          >
            <el-button
              :icon="DocumentChecked"
              circle
              type="default"
              @click="selectCurrentPageAccounts"
            />
          </el-tooltip>

          <!-- 全选按钮（带分隔线） -->
          <el-tooltip
            :content="$t('dashboard.actions.selectAll')"
            placement="bottom"
            class="select-all-button"
          >
            <el-button
              :icon="Select"
              circle
              :type="
                accountsStore.selectedAccounts.size ===
                  accountsStore.filteredAccounts.length &&
                accountsStore.filteredAccounts.length > 0
                  ? 'primary'
                  : 'default'
              "
              @click="toggleSelectAll"
            />
          </el-tooltip>

          <!-- 添加账号 -->
          <el-tooltip
            :content="$t('dashboard.actions.addAccount')"
            placement="bottom"
          >
            <el-button
              type="default"
              :icon="Plus"
              circle
              @click="uiStore.openAddAccountDialog"
            />
          </el-tooltip>

          <!-- 批量添加 -->
          <el-tooltip
            :content="$t('dashboard.actions.batchImport')"
            placement="bottom"
          >
            <el-button
              type="default"
              :icon="Upload"
              circle
              @click="handleBatchImport"
            />
          </el-tooltip>

          <!-- 导出账号 -->
          <el-tooltip
            :content="$t('dashboard.actions.exportAll')"
            placement="bottom"
          >
            <el-button
              :icon="Download"
              circle
              type="default"
              @click="handleExportAccounts"
            />
          </el-tooltip>

          <!-- 标签管理 -->
          <el-tooltip
            :content="$t('dashboard.actions.tagManage')"
            placement="bottom"
          >
            <el-button
              :icon="PriceTag"
              circle
              type="default"
              @click="showTagManageDialog = true"
            />
          </el-tooltip>

          <!-- 全局刷新 -->
          <el-tooltip
            :content="$t('dashboard.actions.refreshAll')"
            placement="bottom"
          >
            <el-button
              :icon="RefreshRight"
              circle
              type="default"
              @click="refreshAccounts"
            />
          </el-tooltip>

          <!-- 语言切换 -->
          <el-dropdown
            @command="handleLanguageCommand"
            trigger="click"
            style="margin-left: 12px"
          >
            <el-button circle class="language-btn">
              <span class="language-flag">{{ currentLanguageFlag }}</span>
            </el-button>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item command="zh">
                  <span class="language-item"
                    ><span class="flag">🇨🇳</span> 中文</span
                  >
                </el-dropdown-item>
                <el-dropdown-item command="en">
                  <span class="language-item"
                    ><span class="flag">🇺🇸</span> English</span
                  >
                </el-dropdown-item>
                <el-dropdown-item command="fr">
                  <span class="language-item"
                    ><span class="flag">🇫🇷</span> Français</span
                  >
                </el-dropdown-item>
                <el-dropdown-item command="es">
                  <span class="language-item"
                    ><span class="flag">🇪🇸</span> Español</span
                  >
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
        </div>
      </el-header>

      <!-- 账号卡片区域 -->
      <el-main class="main-content">
        <!-- 筛选面板 -->
        <transition name="filter-slide">
          <div v-if="showFilterPanel" class="filter-panel">
            <div class="filter-panel-header">
              <span class="filter-title">高级筛选</span>
              <div class="filter-header-actions">
                <el-button size="small" @click="clearAllFilters"
                  >清除筛选</el-button
                >
                <el-button size="small" type="primary" @click="applyFilters"
                  >应用筛选</el-button
                >
                <el-button
                  :icon="Close"
                  circle
                  size="small"
                  @click="showFilterPanel = false"
                />
              </div>
            </div>
            <div class="filter-panel-body">
              <!-- 第一行：数值范围筛选 -->
              <div class="filter-row">
                <div class="filter-item filter-item-range">
                  <span class="filter-label">{{
                    $t("account.quota.remaining")
                  }}</span>
                  <div class="filter-range">
                    <el-input-number
                      v-model="filterForm.remainingQuotaMin"
                      :min="0"
                      :controls="false"
                      placeholder="最小"
                      size="small"
                    />
                    <span class="range-separator">-</span>
                    <el-input-number
                      v-model="filterForm.remainingQuotaMax"
                      :min="0"
                      :controls="false"
                      placeholder="最大"
                      size="small"
                    />
                  </div>
                </div>
                <div class="filter-item filter-item-range">
                  <span class="filter-label">{{
                    $t("account.quota.total")
                  }}</span>
                  <div class="filter-range">
                    <el-input-number
                      v-model="filterForm.totalQuotaMin"
                      :min="0"
                      :controls="false"
                      placeholder="最小"
                      size="small"
                    />
                    <span class="range-separator">-</span>
                    <el-input-number
                      v-model="filterForm.totalQuotaMax"
                      :min="0"
                      :controls="false"
                      placeholder="最大"
                      size="small"
                    />
                  </div>
                </div>
                <div class="filter-item filter-item-range">
                  <span class="filter-label">{{
                    $t("account.quota.expiryDays")
                  }}</span>
                  <div class="filter-range">
                    <el-input-number
                      v-model="filterForm.expiryDaysMin"
                      :controls="false"
                      placeholder="最小"
                      size="small"
                    />
                    <span class="range-separator">-</span>
                    <el-input-number
                      v-model="filterForm.expiryDaysMax"
                      :controls="false"
                      placeholder="最大"
                      size="small"
                    />
                  </div>
                </div>
              </div>
              <!-- 第二行：选择器筛选 -->
              <div class="filter-row filter-row-select">
                <div class="filter-item filter-item-select">
                  <span class="filter-label">{{
                    $t("dashboard.sort.planName")
                  }}</span>
                  <el-select
                    v-model="filterForm.selectedPlans"
                    multiple
                    collapse-tags
                    collapse-tags-tooltip
                    :placeholder="$t('dashboard.filter.all')"
                    size="small"
                  >
                    <el-option
                      v-for="plan in accountsStore.allPlanNames"
                      :key="plan"
                      :label="plan"
                      :value="plan"
                    />
                  </el-select>
                </div>
                <div class="filter-item filter-item-select">
                  <span class="filter-label">{{
                    $t("dashboard.sort.tag")
                  }}</span>
                  <el-select
                    v-model="filterForm.selectedTags"
                    multiple
                    collapse-tags
                    collapse-tags-tooltip
                    :placeholder="$t('dashboard.filter.all')"
                    size="small"
                  >
                    <el-option
                      v-for="tag in accountsStore.allTags"
                      :key="tag"
                      :label="tag"
                      :value="tag"
                    />
                  </el-select>
                </div>
                <div class="filter-item filter-item-select">
                  <span class="filter-label">{{
                    $t("dashboard.sort.domain")
                  }}</span>
                  <el-select
                    v-model="filterForm.selectedDomains"
                    multiple
                    collapse-tags
                    collapse-tags-tooltip
                    :placeholder="$t('dashboard.filter.all')"
                    size="small"
                  >
                    <el-option
                      v-for="domain in accountsStore.allDomains"
                      :key="domain"
                      :label="domain"
                      :value="domain"
                    />
                  </el-select>
                </div>
                <div class="filter-item filter-item-select">
                  <span class="filter-label">{{
                    $t("dashboard.sort.status")
                  }}</span>
                  <el-select
                    v-model="filterForm.selectedStatuses"
                    multiple
                    collapse-tags
                    collapse-tags-tooltip
                    :placeholder="$t('dashboard.filter.all')"
                    size="small"
                  >
                    <el-option
                      v-for="status in statusOptions"
                      :key="status.value"
                      :label="status.label"
                      :value="status.value"
                    />
                  </el-select>
                </div>
              </div>
            </div>
          </div>
        </transition>

        <div v-if="accountsStore.loading" class="loading-container">
          <el-icon class="is-loading" size="32"><Loading /></el-icon>
        </div>

        <div
          v-else-if="accountsStore.filteredAccounts.length === 0"
          class="empty-container"
        >
          <el-empty :description="$t('account.empty.description')">
            <el-button type="primary" @click="uiStore.openAddAccountDialog">
              {{ $t("account.empty.addFirst") }}
            </el-button>
          </el-empty>
        </div>

        <div v-else class="accounts-container">
          <div class="accounts-grid">
            <AccountCard
              v-for="account in accountsStore.paginatedAccounts"
              :key="account.id"
              :account="account"
              :is-selected="accountsStore.selectedAccounts.has(account.id)"
              :current-email="currentWindsurfEmail"
              @select="handleAccountSelect(account.id, $event)"
              @update="handleAccountUpdate"
            />
          </div>

          <!-- 分页组件 -->
          <div
            class="pagination-container"
            v-if="accountsStore.totalCount > accountsStore.pagination.pageSize"
          >
            <el-pagination
              v-model:current-page="accountsStore.pagination.currentPage"
              v-model:page-size="accountsStore.pagination.pageSize"
              :page-sizes="accountsStore.pagination.pageSizes"
              :total="accountsStore.totalCount"
              layout="total, sizes, prev, pager, next, jumper"
              background
              @size-change="handlePageSizeChange"
              @current-change="handleCurrentPageChange"
            />
          </div>
        </div>
      </el-main>
    </el-container>

    <!-- 对话框组件 -->
    <AddAccountDialog />
    <EditAccountDialog />
    <SettingsDialog />
    <BatchImportDialog
      v-model="showBatchImportDialog"
      @import="handleBatchImportConfirm"
      ref="batchImportDialogRef"
    />
    <LogsDialog />
    <StatsDialog />
    <AccountInfoDialog />

    <!-- 关于对话框 -->
    <AboutDialog
      v-model="showAbout"
      :current-email="currentWindsurfEmail"
      :windsurf-version="windsurfVersion"
    />

    <AutoResetDialog v-model="showAutoResetDialog" />

    <!-- 虚拟卡生成对话框 -->
    <CardGeneratorDialog v-model="showCardGeneratorDialog" />

    <!-- 账单对话框（传入当前查看的账号ID和数据） -->
    <BillingDialog
      v-if="uiStore.currentViewingAccountId"
      v-model="uiStore.showBillingDialog"
      :account-id="uiStore.currentViewingAccountId"
      :billing-data="currentBillingData"
      :loading="billingLoading"
      @refresh="refreshBillingData"
    />

    <!-- 批量更换订阅对话框 -->
    <BatchUpdatePlanDialog
      v-model="showBatchUpdatePlanDialog"
      :selected-account-ids="Array.from(accountsStore.selectedAccounts)"
      :accounts="accountsStore.accounts"
      @success="accountsStore.loadAccounts()"
    />

    <!-- 标签管理对话框 -->
    <TagManageDialog
      v-model="showTagManageDialog"
      :selected-account-ids="Array.from(accountsStore.selectedAccounts)"
      @refresh="accountsStore.loadAccounts()"
    />

    <!-- 批量更改分组对话框 -->
    <el-dialog
      v-model="showBatchGroupDialog"
      :title="$t('dashboard.group.batchMove')"
      width="400px"
      :close-on-click-modal="false"
      @close="closeBatchGroupDialog"
    >
      <div class="batch-group-content">
        <p class="batch-group-hint">
          {{
            $t("dashboard.group.moveHint", {
              count: accountsStore.selectedAccounts.size,
            })
          }}
        </p>
        <el-select
          v-model="batchGroupTarget"
          :placeholder="$t('dashboard.group.targetPlaceholder')"
          style="width: 100%"
          size="large"
        >
          <el-option
            v-for="group in settingsStore.groups"
            :key="group"
            :label="group"
            :value="group"
          />
        </el-select>
      </div>
      <template #footer>
        <el-button @click="closeBatchGroupDialog">{{
          $t("common.cancel")
        }}</el-button>
        <el-button
          type="primary"
          :disabled="!batchGroupTarget"
          :loading="isBatchUpdatingGroup"
          @click="handleBatchUpdateGroup"
        >
          {{ $t("common.confirmChange") }}
        </el-button>
      </template>
    </el-dialog>

    <!-- 批量转让订阅对话框 -->
    <el-dialog
      v-model="showBatchTransferDialog"
      :title="$t('dashboard.transfer.title')"
      width="600px"
      :close-on-click-modal="false"
    >
      <el-alert
        :title="$t('dashboard.transfer.warningTitle')"
        type="warning"
        :closable="false"
        show-icon
        style="margin-bottom: 20px"
      >
        <template #default>
          <p>
            {{
              $t("dashboard.transfer.warningContent1", {
                count: accountsStore.selectedAccounts.size,
              })
            }}
          </p>
          <p>
            {{
              $t("dashboard.transfer.warningContent2", {
                count: accountsStore.selectedAccounts.size,
              })
            }}
          </p>
          <p style="color: #e6a23c; margin-top: 8px">
            {{ $t("dashboard.transfer.warningContent3") }}
          </p>
        </template>
      </el-alert>

      <el-form label-width="100px">
        <el-form-item :label="$t('dashboard.transfer.targetEmail')">
          <el-input
            v-model="batchTransferEmails"
            type="textarea"
            :rows="8"
            :placeholder="
              $t('dashboard.transfer.emailPlaceholder', {
                count: accountsStore.selectedAccounts.size,
              })
            "
            name="batch-transfer-emails-no-autofill"
            autocomplete="off"
          />
        </el-form-item>
        <el-form-item>
          <div class="email-count-hint">
            {{
              $t("dashboard.transfer.emailCount", {
                current: parsedTransferEmails.length,
                total: accountsStore.selectedAccounts.size,
              })
            }}
            <span
              v-if="
                parsedTransferEmails.length !==
                accountsStore.selectedAccounts.size
              "
              style="color: #e6a23c"
            >
              {{ $t("dashboard.transfer.countMismatch") }}
            </span>
            <span v-else style="color: #67c23a">
              {{ $t("dashboard.transfer.countMatch") }}
            </span>
          </div>
        </el-form-item>
      </el-form>

      <!-- 转让进度显示 -->
      <div v-if="batchTransferring" class="batch-transfer-progress">
        <el-progress
          :percentage="
            Math.round(
              (batchTransferProgress.current / batchTransferProgress.total) *
                100,
            )
          "
          :stroke-width="12"
        />
        <div class="progress-status">
          {{ batchTransferProgress.status }}
          ({{ batchTransferProgress.current }}/{{
            batchTransferProgress.total
          }})
        </div>
      </div>

      <template #footer>
        <el-button
          @click="showBatchTransferDialog = false"
          :disabled="batchTransferring"
          >{{ $t("common.cancel") }}</el-button
        >
        <el-button
          type="danger"
          :loading="batchTransferring"
          :disabled="
            parsedTransferEmails.length !== accountsStore.selectedAccounts.size
          "
          @click="handleBatchTransfer"
        >
          {{ $t("dashboard.transfer.confirmTransfer") }}
        </el-button>
      </template>
    </el-dialog>
  </el-container>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage, ElMessageBox } from "element-plus";
import { invoke } from "@tauri-apps/api/core";
import {
  User,
  Folder,
  FolderOpened,
  Document,
  Setting,
  ArrowRight,
  ArrowLeft,
  Search,
  Plus,
  Delete,
  Edit,
  RefreshRight,
  Connection,
  Loading,
  DataAnalysis,
  InfoFilled,
  Select,
  Download,
  Upload,
  Trophy,
  Filter,
  Close,
  PriceTag,
  DocumentChecked,
  Timer,
  Switch,
  SortUp,
  SortDown,
  ArrowDown,
} from "@element-plus/icons-vue";
import { useAccountsStore, useSettingsStore, useUIStore } from "@/store";
import { apiService, settingsApi, accountApi } from "@/api";
import type { Account } from "@/types";
import dayjs from "dayjs";
import AccountCard from "@/components/AccountCard.vue";
import AddAccountDialog from "@/components/AddAccountDialog.vue";
import EditAccountDialog from "@/components/EditAccountDialog.vue";
import SettingsDialog from "@/components/SettingsDialog.vue";
import BatchImportDialog from "@/components/BatchImportDialog.vue";
import LogsDialog from "@/components/LogsDialog.vue";
import StatsDialog from "@/components/StatsDialog.vue";
import BillingDialog from "@/components/BillingDialog.vue";
import AccountInfoDialog from "@/components/AccountInfoDialog.vue";
import AboutDialog from "@/components/AboutDialog.vue";
import BatchUpdatePlanDialog from "@/components/BatchUpdatePlanDialog.vue";
import TagManageDialog from "@/components/TagManageDialog.vue";
import AutoResetDialog from "@/components/AutoResetDialog.vue";
import CardGeneratorDialog from "@/components/CardGeneratorDialog.vue";

const accountsStore = useAccountsStore();
const settingsStore = useSettingsStore();
const uiStore = useUIStore();
const { t } = useI18n();

const currentLanguageFlag = computed(() => {
  const map: Record<string, string> = {
    zh: "🇨🇳",
    en: "🇺🇸",
    fr: "🇫🇷",
    es: "🇪🇸",
  };
  return map[settingsStore.settings.language] || "🇨🇳";
});

function handleLanguageCommand(lang: string) {
  settingsStore.setLanguage(lang);
  ElMessage.success(t("settings.basic.languageChanged"));
}

const activeMenu = ref("accounts");
const searchQuery = ref("");
const currentBillingData = ref<any>(null);
const billingLoading = ref(false);
const currentWindsurfEmail = ref<string>("");
const windsurfVersion = ref<string>("");
const showBatchUpdatePlanDialog = ref(false);
const showAbout = ref(false);
const showTagManageDialog = ref(false);
const showBatchImportDialog = ref(false);
const batchImportDialogRef = ref<InstanceType<typeof BatchImportDialog> | null>(
  null,
);
const appVersion = ref<string>(""); // 版本号从后端动态获取
const showBatchGroupDialog = ref(false);
const batchGroupTarget = ref("");
const isBatchUpdatingGroup = ref(false);
const showAutoResetDialog = ref(false);
const showCardGeneratorDialog = ref(false);

// 排序相关
const currentSortField = ref<string>("custom");
const sortDirection = ref<"asc" | "desc">("asc");

// 处理排序变更
async function handleSortChange(field: string) {
  currentSortField.value = field;
  await accountsStore.setSortConfig(field as any, sortDirection.value);
}

// 切换排序方向
async function toggleSortDirection() {
  sortDirection.value = sortDirection.value === "asc" ? "desc" : "asc";
  await accountsStore.setSortConfig(
    currentSortField.value as any,
    sortDirection.value,
  );
}

// 初始化排序配置
async function initSortConfig() {
  await accountsStore.loadSortConfig();
  // 处理旧的 custom 值，自动改为 created_at
  const field = accountsStore.sortConfig.field as string;
  currentSortField.value = (field === "custom" ? "created_at" : field) as any;
  sortDirection.value = accountsStore.sortConfig.direction;
}

// 批量转让订阅
const showBatchTransferDialog = ref(false);
const batchTransferEmails = ref("");
const batchTransferring = ref(false);
const batchTransferProgress = ref({ current: 0, total: 0, status: "" });

// 解析输入的邮箱列表（支持"邮箱"或"邮箱 密码"格式）
const parsedTransferEmails = computed(() => {
  return batchTransferEmails.value
    .split("\n")
    .map((line) => {
      const trimmed = line.trim();
      // 支持空格或制表符分隔的格式，只取第一部分（邮箱）
      const parts = trimmed.split(/[\s\t]+/);
      return parts[0] || "";
    })
    .filter((e) => e && e.includes("@"));
});

// 自动重置定时器
interface AutoResetConfig {
  id: string;
  targetType: string;
  targetId: string;
  enabled: boolean;
  checkInterval: number;
  usageThreshold: number;
  remainingThreshold: number;
}
const autoResetTimerMap = ref<Map<string, ReturnType<typeof setInterval>>>(
  new Map(),
);

// 初始化自动重置定时器
async function initAutoResetTimers() {
  try {
    const configs = await invoke<AutoResetConfig[]>("get_auto_reset_configs");

    // 清除现有定时器
    autoResetTimerMap.value.forEach((timer) => clearInterval(timer));
    autoResetTimerMap.value.clear();

    // 为每个启用的配置设置定时器
    configs
      .filter((c) => c.enabled)
      .forEach((config) => {
        // 立即执行一次检查
        executeAutoResetCheck(config.id);

        // 设置定时器
        const timer = setInterval(
          () => {
            executeAutoResetCheck(config.id);
          },
          config.checkInterval * 60 * 1000,
        );

        autoResetTimerMap.value.set(config.id, timer);
      });

    if (configs.filter((c) => c.enabled).length > 0) {
      console.log(
        `[AutoReset] 已启动 ${configs.filter((c) => c.enabled).length} 个自动重置定时器`,
      );
    }
  } catch (error) {
    console.error("[AutoReset] 初始化定时器失败:", error);
  }
}

// 执行自动重置检查
async function executeAutoResetCheck(configId: string) {
  try {
    const result = await invoke<any>("check_and_auto_reset", { configId });

    if (result.reset_count > 0) {
      ElMessage.success(`自动重置: 重置了 ${result.reset_count} 个账号的积分`);
      await accountsStore.loadAccounts();
    }
  } catch (error) {
    console.error("[AutoReset] 检查失败:", error);
  }
}

// 筛选面板状态
const showFilterPanel = ref(false);
// 状态选项
const statusOptions = computed(() => [
  {
    value: "normal",
    label: `🟢 ${t("account.status.normal")}`,
    desc: "账户正常",
  },
  {
    value: "inactive",
    label: `🔘 ${t("account.status.inactive")}`,
    desc: "订阅未激活",
  },
  {
    value: "disabled",
    label: `🟠 ${t("account.status.disabled")}`,
    desc: "被 Windsurf 禁用",
  },
  {
    value: "offline",
    label: `⚪ ${t("account.status.offline")}`,
    desc: "Token 失效",
  },
  {
    value: "error",
    label: `🔴 ${t("account.status.error")}`,
    desc: "操作异常",
  },
]);

const filterForm = ref({
  remainingQuotaMin: undefined as number | undefined,
  remainingQuotaMax: undefined as number | undefined,
  totalQuotaMin: undefined as number | undefined,
  totalQuotaMax: undefined as number | undefined,
  expiryDaysMin: undefined as number | undefined,
  expiryDaysMax: undefined as number | undefined,
  selectedTags: [] as string[],
  selectedPlans: [] as string[],
  selectedDomains: [] as string[],
  selectedStatuses: [] as string[],
});

// 是否有激活的筛选条件
const hasActiveFilter = computed(() => {
  const f = accountsStore.currentFilter;
  return !!(
    f.remainingQuotaMin !== undefined ||
    f.remainingQuotaMax !== undefined ||
    f.totalQuotaMin !== undefined ||
    f.totalQuotaMax !== undefined ||
    f.expiryDaysMin !== undefined ||
    f.expiryDaysMax !== undefined ||
    (f.tags && f.tags.length > 0) ||
    (f.planNames && f.planNames.length > 0) ||
    (f.domains && f.domains.length > 0) ||
    (f.statuses && f.statuses.length > 0)
  );
});

const sidebarWidth = computed(() =>
  uiStore.sidebarCollapsed ? "64px" : "240px",
);

function setActiveMenu(menu: string) {
  activeMenu.value = menu;
  accountsStore.clearFilter();
}

function filterByGroup(group: string) {
  accountsStore.setFilter({ group });
}

function handleSearch() {
  accountsStore.setFilter({
    ...accountsStore.currentFilter,
    search: searchQuery.value,
  });
}

// 应用筛选
function applyFilters() {
  accountsStore.setFilter({
    ...accountsStore.currentFilter,
    remainingQuotaMin: filterForm.value.remainingQuotaMin,
    remainingQuotaMax: filterForm.value.remainingQuotaMax,
    totalQuotaMin: filterForm.value.totalQuotaMin,
    totalQuotaMax: filterForm.value.totalQuotaMax,
    expiryDaysMin: filterForm.value.expiryDaysMin,
    expiryDaysMax: filterForm.value.expiryDaysMax,
    tags:
      filterForm.value.selectedTags.length > 0
        ? filterForm.value.selectedTags
        : undefined,
    planNames:
      filterForm.value.selectedPlans.length > 0
        ? filterForm.value.selectedPlans
        : undefined,
    domains:
      filterForm.value.selectedDomains.length > 0
        ? filterForm.value.selectedDomains
        : undefined,
    statuses:
      filterForm.value.selectedStatuses.length > 0
        ? (filterForm.value.selectedStatuses as any)
        : undefined,
  });
}

// 清除所有筛选
function clearAllFilters() {
  filterForm.value = {
    remainingQuotaMin: undefined,
    remainingQuotaMax: undefined,
    totalQuotaMin: undefined,
    totalQuotaMax: undefined,
    expiryDaysMin: undefined,
    expiryDaysMax: undefined,
    selectedTags: [],
    selectedPlans: [],
    selectedDomains: [],
    selectedStatuses: [],
  };
  accountsStore.clearFilter();
  searchQuery.value = "";
}

// 分页处理
function handlePageSizeChange(size: number) {
  accountsStore.setPageSize(size);
}

function handleCurrentPageChange(page: number) {
  accountsStore.setCurrentPage(page);
}

function handleAccountSelect(accountId: string, selected: boolean) {
  if (selected) {
    accountsStore.selectedAccounts.add(accountId);
  } else {
    accountsStore.selectedAccounts.delete(accountId);
  }
}

function handleAccountUpdate(account: Account) {
  accountsStore.updateAccount(account);
}

async function refreshAccounts() {
  const loading = ElMessage({
    message: "正在刷新账号列表...",
    duration: 0,
    icon: Loading,
  });

  try {
    // 批量刷新所有账号（使用优化的批量 API）
    if (accountsStore.accounts.length > 0) {
      loading.close();

      const totalCount = accountsStore.accounts.length;
      const allIds = accountsStore.accounts.map((a) => a.id);

      const progressLoading = ElMessage({
        message: `正在批量刷新 ${totalCount} 个账号...`,
        duration: 0,
        icon: Loading,
      });

      // 使用优化的批量刷新 API（后端只保存一次）
      const result = await apiService.batchRefreshTokens(allIds);

      progressLoading.close();

      const successCount = result.success_count || 0;
      const failedCount = totalCount - successCount;

      // 使用后端返回的完整数据更新本地 store（无需重新加载页面）
      if (result.results) {
        for (const item of result.results) {
          const idx = accountsStore.accounts.findIndex((a) => a.id === item.id);
          if (idx === -1) continue;

          if (item.success && item.data) {
            const account = accountsStore.accounts[idx];
            if (item.data.plan_name) account.plan_name = item.data.plan_name;
            if (item.data.used_quota !== undefined)
              account.used_quota = item.data.used_quota;
            if (item.data.total_quota !== undefined)
              account.total_quota = item.data.total_quota;
            if (item.data.expires_at)
              account.token_expires_at = item.data.expires_at;
            if (item.data.windsurf_api_key)
              account.windsurf_api_key = item.data.windsurf_api_key;
            if (item.data.is_disabled !== undefined)
              account.is_disabled = item.data.is_disabled;
            if (item.data.subscription_active !== undefined)
              account.subscription_active = item.data.subscription_active;
            if (
              item.data.subscription_expires_at &&
              typeof item.data.subscription_expires_at === "number" &&
              item.data.subscription_expires_at > 0
            ) {
              account.subscription_expires_at = dayjs
                .unix(item.data.subscription_expires_at)
                .toISOString();
            }
            if (item.data.last_quota_update)
              account.last_quota_update = item.data.last_quota_update;
            account.status = "active";
          } else {
            accountsStore.accounts[idx].status = "error";
          }
        }
      }

      // 显示详细的刷新结果
      if (failedCount === 0) {
        ElMessage.success({
          message: `✅ 全部刷新完成！\n成功: ${successCount}/${totalCount}`,
          duration: 3000,
          showClose: true,
        });
      } else {
        const failedItems =
          result.results?.filter((r: any) => !r.success) || [];
        const failedDetails = failedItems
          .slice(0, 3)
          .map((item: any) => {
            const account = accountsStore.accounts.find(
              (a) => a.id === item.id,
            );
            return `  • ${account?.email || item.id}: ${item.error || "未知错误"}`;
          })
          .join("\n");
        const moreFailures =
          failedItems.length > 3
            ? `\n  ... 还有 ${failedItems.length - 3} 个失败`
            : "";

        ElMessage.warning({
          message: `⚠️ 刷新完成（部分失败）\n成功: ${successCount}/${totalCount}\n失败: ${failedCount}/${totalCount}\n\n失败账号:\n${failedDetails}${moreFailures}`,
          duration: 5000,
          showClose: true,
          dangerouslyUseHTMLString: false,
        });
      }
    } else {
      loading.close();
      ElMessage.success("账号列表已刷新");
    }
  } catch (error) {
    loading.close();
    ElMessage.error(`刷新失败: ${error}`);
  }
}

async function handleBatchDelete() {
  try {
    await ElMessageBox.confirm(
      `确定要删除选中的 ${accountsStore.selectedAccounts.size} 个账号吗？`,
      "批量删除确认",
      {
        confirmButtonText: "删除",
        cancelButtonText: "取消",
        type: "warning",
      },
    );

    const result = await accountsStore.deleteSelectedAccounts();
    ElMessage.success(`成功删除 ${result?.success_count || 0} 个账号`);
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(`批量删除失败: ${error}`);
    }
  }
}

// 批量转让订阅（并发执行）
async function handleBatchTransfer() {
  const selectedIds = Array.from(accountsStore.selectedAccounts);
  const targetEmails = parsedTransferEmails.value;

  if (selectedIds.length !== targetEmails.length) {
    ElMessage.warning("源账户数量与目标邮箱数量不匹配");
    return;
  }

  try {
    await ElMessageBox.confirm(
      `确定要将 ${selectedIds.length} 个账户的订阅转让给对应的目标邮箱吗？\n\n转让后源账户将被移出团队，此操作不可撤销！`,
      "确认批量转让",
      {
        confirmButtonText: "确认转让",
        cancelButtonText: "取消",
        type: "warning",
      },
    );
  } catch {
    return;
  }

  batchTransferring.value = true;
  batchTransferProgress.value = {
    current: 0,
    total: selectedIds.length,
    status: "并发执行中...",
  };

  // 构建转让任务列表
  const transferTasks = selectedIds.map((sourceId, index) => {
    const targetEmail = targetEmails[index];
    const sourceAccount = accountsStore.accounts.find((a) => a.id === sourceId);
    const sourceEmail = sourceAccount?.email || sourceId;

    return (async () => {
      try {
        const result = await invoke<any>("transfer_subscription", {
          id: sourceId,
          targetEmail: targetEmail,
          targetName: targetEmail.split("@")[0],
        });

        // 更新进度
        batchTransferProgress.value = {
          ...batchTransferProgress.value,
          current: batchTransferProgress.value.current + 1,
        };

        if (result.success) {
          return { sourceEmail, targetEmail, success: true };
        } else {
          return {
            sourceEmail,
            targetEmail,
            success: false,
            error: result.error || "转让失败",
          };
        }
      } catch (error: any) {
        batchTransferProgress.value = {
          ...batchTransferProgress.value,
          current: batchTransferProgress.value.current + 1,
        };
        return {
          sourceEmail,
          targetEmail,
          success: false,
          error: error.toString(),
        };
      }
    })();
  });

  // 并发执行所有转让任务
  const results = await Promise.all(transferTasks);

  batchTransferProgress.value = {
    current: selectedIds.length,
    total: selectedIds.length,
    status: "完成",
  };

  batchTransferring.value = false;

  const successCount = results.filter((r) => r.success).length;
  const failedCount = results.filter((r) => !r.success).length;

  if (failedCount === 0) {
    ElMessage.success(
      `批量转让完成！成功: ${successCount}/${selectedIds.length}`,
    );
  } else {
    const failedDetails = results
      .filter((r) => !r.success)
      .slice(0, 3)
      .map((r) => `${r.sourceEmail}: ${r.error}`)
      .join("\n");
    ElMessage.warning({
      message: `批量转让完成\n成功: ${successCount}, 失败: ${failedCount}\n\n失败详情:\n${failedDetails}`,
      duration: 5000,
      showClose: true,
    });
  }

  // 关闭对话框并清理
  showBatchTransferDialog.value = false;
  batchTransferEmails.value = "";
  accountsStore.clearSelection();

  // 刷新账号列表
  await accountsStore.loadAccounts();
}

async function refreshBillingData() {
  if (uiStore.currentViewingAccountId) {
    billingLoading.value = true;
    currentBillingData.value = null;

    try {
      const result = await apiService.getBilling(
        uiStore.currentViewingAccountId,
      );
      currentBillingData.value = result;
    } catch (error) {
      ElMessage.error(`获取账单信息失败: ${error}`);
    } finally {
      billingLoading.value = false;
    }
  }
}

// 监听账单对话框打开
watch(
  () => uiStore.showBillingDialog,
  (show) => {
    if (show && uiStore.currentViewingAccountId) {
      refreshBillingData();
    } else if (!show) {
      currentBillingData.value = null;
    }
  },
);

// 全选/取消全选
function toggleSelectAll() {
  if (
    accountsStore.selectedAccounts.size ===
      accountsStore.filteredAccounts.length &&
    accountsStore.filteredAccounts.length > 0
  ) {
    // 当前是全选状态，取消全选
    accountsStore.clearSelection();
  } else {
    // 选择所有账号
    accountsStore.filteredAccounts.forEach((account) => {
      accountsStore.selectedAccounts.add(account.id);
    });
  }
}

// 选择本页账号
function selectCurrentPageAccounts() {
  const pageAccounts = accountsStore.paginatedAccounts;
  if (pageAccounts.length === 0) {
    ElMessage.info("当前页没有账号");
    return;
  }

  // 检查本页是否已全部选中
  const allSelected = pageAccounts.every((acc) =>
    accountsStore.selectedAccounts.has(acc.id),
  );

  if (allSelected) {
    // 如果本页已全选，则取消本页选择
    pageAccounts.forEach((account) => {
      accountsStore.selectedAccounts.delete(account.id);
    });
    ElMessage.info(`已取消选择本页 ${pageAccounts.length} 个账号`);
  } else {
    // 选择本页所有账号
    pageAccounts.forEach((account) => {
      accountsStore.selectedAccounts.add(account.id);
    });
    ElMessage.success(`已选择本页 ${pageAccounts.length} 个账号`);
  }
}

// 打开批量导入对话框
function handleBatchImport() {
  showBatchImportDialog.value = true;
}

// 批量导入确认（从对话框接收数据）
async function handleBatchImportConfirm(
  accountsToImport: Array<{
    email: string;
    password: string;
    remark: string;
    refreshToken?: string;
  }>,
  autoLogin: boolean,
  group: string = "默认分组",
  tags: string[] = [],
  mode: "password" | "refresh_token" = "password",
) {
  // 获取并发设置
  const unlimitedConcurrent =
    settingsStore.settings?.unlimitedConcurrentRefresh || false;
  const concurrencyLimit = settingsStore.settings?.concurrent_limit || 5;

  const modeLabel =
    mode === "refresh_token"
      ? t("dialog.batchImport.modeToken")
      : t("dialog.batchImport.modePassword");

  // 显示进度提示
  let progressMsg = ElMessage({
    message: unlimitedConcurrent
      ? t("dialog.batchImport.message.progressUnlimited", {
          count: accountsToImport.length,
          mode: modeLabel,
        })
      : t("dialog.batchImport.message.progressLimited", {
          count: accountsToImport.length,
          mode: modeLabel,
          limit: concurrencyLimit,
        }),
    duration: 0,
    icon: Loading,
  });

  const results: Array<{
    email: string;
    success: boolean;
    accountId?: string;
    error?: string;
  }> = [];

  // 单个导入任务
  const importTask = async (item: {
    email: string;
    password: string;
    remark: string;
    refreshToken?: string;
  }) => {
    try {
      if (mode === "refresh_token" && item.refreshToken) {
        // Refresh Token 模式：调用后端命令
        const result = await invoke<any>("add_account_by_refresh_token", {
          refreshToken: item.refreshToken,
          nickname: item.remark || undefined,
          tags: tags.length > 0 ? [...tags] : [],
          group: group,
        });

        if (result.success) {
          return {
            email: result.email,
            success: true,
            accountId: result.account?.id,
          };
        } else {
          return {
            email: item.email,
            success: false,
            error: result.error || t("dialog.batchImport.message.addFailed"),
          };
        }
      } else {
        // 邮箱密码模式
        const newAccount = await accountsStore.addAccount({
          email: item.email,
          password: item.password,
          nickname: item.remark || item.email.split("@")[0],
          tags: tags.length > 0 ? [...tags] : [],
          group: group,
        });
        return { email: item.email, success: true, accountId: newAccount.id };
      }
    } catch (error) {
      console.error(`导入账号 ${item.email} 失败:`, error);
      return { email: item.email, success: false, error: String(error) };
    }
  };

  try {
    if (unlimitedConcurrent) {
      // 全量并发导入
      const allResults = await Promise.all(
        accountsToImport.map((item) => importTask(item)),
      );
      results.push(...allResults);
    } else {
      // 分批并发处理
      for (let i = 0; i < accountsToImport.length; i += concurrencyLimit) {
        const batch = accountsToImport.slice(i, i + concurrencyLimit);
        const batchResults = await Promise.all(
          batch.map((item) => importTask(item)),
        );
        results.push(...batchResults);

        // 更新进度
        progressMsg.close();
        progressMsg = ElMessage({
          message: t("dialog.batchImport.message.importProgress", {
            current: results.length,
            total: accountsToImport.length,
          }),
          duration: 0,
          icon: Loading,
        });
      }
    }

    // 统计添加结果
    const addedAccounts = results.filter((r) => r.success);
    const failedAccounts = results.filter((r) => !r.success);

    // 并发登录成功添加的账号（refresh_token 模式已经获取了账号信息，无需再登录）
    let loginSuccessCount = 0;
    if (autoLogin && addedAccounts.length > 0 && mode === "password") {
      progressMsg.close();
      progressMsg = ElMessage({
        message: unlimitedConcurrent
          ? t("dialog.batchImport.message.loginProgressUnlimited", {
              count: addedAccounts.length,
            })
          : t("dialog.batchImport.message.loginProgressLimited", {
              count: addedAccounts.length,
              limit: concurrencyLimit,
            }),
        duration: 0,
        icon: Loading,
      });

      // 单个登录任务
      const loginTask = async (item: { email: string; accountId?: string }) => {
        try {
          const loginResult = await apiService.loginAccount(item.accountId!);
          if (loginResult.success) {
            // 从后端获取完整的账号信息（包含token）
            const latestAccount = await accountApi.getAccount(item.accountId!);
            await accountsStore.updateAccount(latestAccount);
            return { success: true };
          }
          return { success: false };
        } catch (loginError) {
          console.error(`账号 ${item.email} 登录失败:`, loginError);
          return { success: false };
        }
      };

      const loginResults: Array<{ success: boolean }> = [];

      if (unlimitedConcurrent) {
        // 全量并发登录
        const allLoginResults = await Promise.all(
          addedAccounts.map((item) => loginTask(item)),
        );
        loginResults.push(...allLoginResults);
      } else {
        // 分批并发登录
        for (let i = 0; i < addedAccounts.length; i += concurrencyLimit) {
          const batch = addedAccounts.slice(i, i + concurrencyLimit);
          const batchResults = await Promise.all(
            batch.map((item) => loginTask(item)),
          );
          loginResults.push(...batchResults);

          // 更新进度
          progressMsg.close();
          progressMsg = ElMessage({
            message: t("dialog.batchImport.message.loginProgress", {
              current: loginResults.length,
              total: addedAccounts.length,
            }),
            duration: 0,
            icon: Loading,
          });
        }
      }

      loginSuccessCount = loginResults.filter((r) => r.success).length;
    }

    progressMsg.close();

    // 关闭对话框
    showBatchImportDialog.value = false;
    batchImportDialogRef.value?.resetImporting();

    // 显示最终结果
    if (addedAccounts.length > 0) {
      let message = t("dialog.batchImport.message.resultSuccess", {
        count: addedAccounts.length,
      });
      if (autoLogin && loginSuccessCount > 0) {
        message += t("dialog.batchImport.message.resultLogin", {
          count: loginSuccessCount,
        });
      }
      if (failedAccounts.length > 0) {
        message += t("dialog.batchImport.message.resultFailed", {
          count: failedAccounts.length,
        });
      }
      ElMessage.success({
        message,
        duration: 4000,
        showClose: true,
      });
      await accountsStore.loadAccounts();
    } else {
      let errorMsg = t("dialog.batchImport.message.resultNone");
      if (failedAccounts.length > 0) {
        const details = failedAccounts
          .slice(0, 3)
          .map((f) => f.email)
          .join(", ");
        errorMsg += t("dialog.batchImport.message.resultFailedDetail", {
          details,
        });
        if (failedAccounts.length > 3) {
          errorMsg += t("dialog.batchImport.message.resultMoreFailed", {
            count: failedAccounts.length - 3,
          });
        }
      }
      ElMessage.error({
        message: errorMsg,
        duration: 5000,
        showClose: true,
      });
    }
  } catch (error) {
    progressMsg.close();
    showBatchImportDialog.value = false;
    batchImportDialogRef.value?.resetImporting();
    ElMessage.error(t("dialog.batchImport.message.importFailed", { error }));
  }
}

// 批量刷新状态（使用优化的批量 API，只保存一次）
async function handleBatchRefresh() {
  const selectedIds = Array.from(accountsStore.selectedAccounts);
  if (selectedIds.length === 0) {
    ElMessage.warning("请先选择账号");
    return;
  }

  const totalCount = selectedIds.length;

  const progressLoading = ElMessage({
    message: t("dialog.batchImport.message.refreshProgress", {
      count: totalCount,
    }),
    duration: 0,
    icon: Loading,
  });

  try {
    // 使用优化的批量刷新 API（后端只保存一次）
    const result = await apiService.batchRefreshTokens(selectedIds);

    progressLoading.close();

    const successCount = result.success_count || 0;
    const failedCount = totalCount - successCount;

    // 刷新成功的账号，从后端重新获取数据更新 store
    if (result.results) {
      for (const item of result.results) {
        if (item.success && item.data) {
          // 从后端获取最新账号数据
          try {
            const latestAccount = await accountApi.getAccount(item.id);
            if (item.data.plan_name)
              latestAccount.plan_name = item.data.plan_name;
            if (item.data.used_quota !== undefined)
              latestAccount.used_quota = item.data.used_quota;
            if (item.data.total_quota !== undefined)
              latestAccount.total_quota = item.data.total_quota;
            if (item.data.expires_at)
              latestAccount.token_expires_at = item.data.expires_at;
            latestAccount.status = "active";
            latestAccount.last_quota_update = dayjs().toISOString();
            accountsStore.updateAccount(latestAccount);
          } catch {
            // 忽略单个账号的获取失败
          }
        } else {
          // 刷新失败的账号，标记为 error
          const account = accountsStore.accounts.find((a) => a.id === item.id);
          if (account) {
            accountsStore.updateAccount({
              ...account,
              status: "error" as const,
            });
          }
        }
      }
    }

    // 显示结果
    if (failedCount === 0) {
      ElMessage.success(
        t("dialog.batchImport.message.refreshSuccess", { count: successCount }),
      );
    } else {
      // 收集失败信息
      const failedItems = result.results?.filter((r: any) => !r.success) || [];
      const failedEmails = failedItems.slice(0, 3).map((item: any) => {
        const account = accountsStore.accounts.find((a) => a.id === item.id);
        return `${account?.email || item.id}: ${item.error || t("dialog.batchImport.message.unknownError")}`;
      });

      const moreCount = failedItems.length - 3;
      let message = t("dialog.batchImport.message.refreshResult", {
        success: successCount,
        total: totalCount,
        failed: failedCount,
      });
      if (failedEmails.length > 0) {
        message +=
          "\n\n" +
          t("dialog.batchImport.message.resultFailedDetail", {
            details: "• " + failedEmails.join("\n• "),
          }).trim();
        if (moreCount > 0) {
          message +=
            "\n" +
            t("dialog.batchImport.message.resultMoreFailed", {
              count: moreCount,
            });
        }
      }

      ElMessageBox.alert(message, "刷新结果", {
        type: "warning",
        confirmButtonText: "确定",
      });
    }

    accountsStore.clearSelection();
  } catch (error) {
    progressLoading.close();
    ElMessage.error(t("dialog.batchImport.message.refreshFailed", { error }));
  }
}

// 导出账号
async function handleExportAccounts(selectedOnly: boolean = false) {
  try {
    let accounts;
    if (selectedOnly) {
      // 导出选中的账号
      accounts = accountsStore.filteredAccounts.filter((a) =>
        accountsStore.selectedAccounts.has(a.id),
      );
      if (accounts.length === 0) {
        ElMessage.warning("没有选中的账号");
        return;
      }
    } else {
      // 导出所有账号
      accounts = accountsStore.filteredAccounts;
      if (accounts.length === 0) {
        ElMessage.warning("没有可导出的账号");
        return;
      }
    }

    // 创建 HTML 字符串形式的单选按钮
    const radioHtml = `
      <div style="padding: 20px 0;">
        <div style="margin-bottom: 16px; padding-bottom: 12px; border-bottom: 1px solid #ebeef5;">
          <div style="font-weight: 500; margin-bottom: 10px; color: #606266;">导出内容</div>
          <label style="display: block; margin: 10px 0; cursor: pointer; font-size: 14px;">
            <input type="radio" name="exportContent" value="password" checked style="margin-right: 10px; cursor: pointer; transform: scale(1.2);" />
            <span style="font-weight: 500;">邮箱 + 密码</span>
            <span style="color: #909399; margin-left: 8px;">传统登录凭证</span>
          </label>
          <label style="display: block; margin: 10px 0; cursor: pointer; font-size: 14px;">
            <input type="radio" name="exportContent" value="refresh_token" style="margin-right: 10px; cursor: pointer; transform: scale(1.2);" />
            <span style="font-weight: 500;">邮箱 + Refresh Token</span>
            <span style="color: #909399; margin-left: 8px;">可直接刷新获取账号信息</span>
          </label>
        </div>
        <div style="margin-bottom: 16px; padding-bottom: 12px; border-bottom: 1px solid #ebeef5;">
          <div style="font-weight: 500; margin-bottom: 10px; color: #606266;">导出格式</div>
          <label style="display: block; margin: 10px 0; cursor: pointer; font-size: 14px;">
            <input type="radio" name="exportFormat" value="3" checked style="margin-right: 10px; cursor: pointer; transform: scale(1.2);" />
            <span style="font-weight: 500;">文本格式</span>
            <span style="color: #909399; margin-left: 8px;">简单列表</span>
          </label>
          <label style="display: block; margin: 10px 0; cursor: pointer; font-size: 14px;">
            <input type="radio" name="exportFormat" value="1" style="margin-right: 10px; cursor: pointer; transform: scale(1.2);" />
            <span style="font-weight: 500;">CSV格式</span>
            <span style="color: #909399; margin-left: 8px;">适合 Excel 打开</span>
          </label>
          <label style="display: block; margin: 10px 0; cursor: pointer; font-size: 14px;">
            <input type="radio" name="exportFormat" value="2" style="margin-right: 10px; cursor: pointer; transform: scale(1.2);" />
            <span style="font-weight: 500;">JSON格式</span>
            <span style="color: #909399; margin-left: 8px;">适合程序处理</span>
          </label>
        </div>
        <div>
          <div style="font-weight: 500; margin-bottom: 10px; color: #606266;">导出方式</div>
          <label style="display: block; margin: 10px 0; cursor: pointer; font-size: 14px;">
            <input type="radio" name="exportTarget" value="clipboard" checked style="margin-right: 10px; cursor: pointer; transform: scale(1.2);" />
            <span style="font-weight: 500;">复制到剪贴板</span>
            <span style="color: #909399; margin-left: 8px;">直接粘贴使用</span>
          </label>
          <label style="display: block; margin: 10px 0; cursor: pointer; font-size: 14px;">
            <input type="radio" name="exportTarget" value="file" style="margin-right: 10px; cursor: pointer; transform: scale(1.2);" />
            <span style="font-weight: 500;">下载文件</span>
            <span style="color: #909399; margin-left: 8px;">保存到本地</span>
          </label>
        </div>
      </div>
    `;

    await ElMessageBox({
      title: "选择导出格式",
      message: radioHtml,
      showCancelButton: true,
      confirmButtonText: "导出",
      cancelButtonText: "取消",
      dangerouslyUseHTMLString: true,
      customClass: "export-dialog",
      beforeClose: (action, instance, done) => {
        if (action === "confirm") {
          const radioElement = document.querySelector(
            'input[name="exportFormat"]:checked',
          ) as HTMLInputElement;
          if (radioElement) {
            (instance as any).selectedValue = radioElement.value;
          }
        }
        done();
      },
    });

    // 获取选中的值
    const selectedContentRadio = document.querySelector(
      'input[name="exportContent"]:checked',
    ) as HTMLInputElement;
    const selectedFormatRadio = document.querySelector(
      'input[name="exportFormat"]:checked',
    ) as HTMLInputElement;
    const selectedTargetRadio = document.querySelector(
      'input[name="exportTarget"]:checked',
    ) as HTMLInputElement;
    const exportContent = selectedContentRadio
      ? selectedContentRadio.value
      : "password";
    const format = selectedFormatRadio ? selectedFormatRadio.value : "1";
    const target = selectedTargetRadio ? selectedTargetRadio.value : "file";

    // 根据导出内容类型获取凭证
    const getCredential = (account: any) => {
      if (exportContent === "refresh_token") {
        return account.refresh_token || "";
      }
      return account.password || "";
    };

    const credentialLabel =
      exportContent === "refresh_token" ? "Refresh Token" : "密码";
    const credentialKey =
      exportContent === "refresh_token" ? "refresh_token" : "password";

    let content = "";
    let filename = "";
    const timestamp = new Date()
      .toISOString()
      .replace(/[:.]/g, "-")
      .substring(0, 19);
    const fileSuffix = exportContent === "refresh_token" ? "_token" : "";

    switch (format) {
      case "1": // CSV
        // 剪贴板不需要 BOM
        content =
          target === "clipboard"
            ? `邮箱,${credentialLabel},备注,分组,状态,套餐\n`
            : `\uFEFF邮箱,${credentialLabel},备注,分组,状态,套餐\n`;
        accounts.forEach((account) => {
          content += `"${account.email}","${getCredential(account)}","${account.nickname || ""}","${account.group || ""}","${account.status || ""}","${account.plan_name || ""}"\n`;
        });
        filename = `accounts${fileSuffix}_${timestamp}.csv`;
        break;

      case "2": // JSON
        content = JSON.stringify(
          accounts.map((account) => ({
            email: account.email,
            [credentialKey]: getCredential(account),
            remark: account.nickname,
            group: account.group,
            status: account.status,
            plan: account.plan_name,
          })),
          null,
          2,
        );
        filename = `accounts${fileSuffix}_${timestamp}.json`;
        break;

      case "3": // 文本
        accounts.forEach((account) => {
          content += `${account.email} ${getCredential(account)}\n`;
        });
        filename = `accounts${fileSuffix}_${timestamp}.txt`;
        break;
    }

    if (target === "clipboard") {
      // 复制到剪贴板
      await navigator.clipboard.writeText(content);
      ElMessage.success(`已复制 ${accounts.length} 个账号到剪贴板`);
    } else {
      // 创建下载链接
      const blob = new Blob([content], { type: "text/plain;charset=utf-8" });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement("a");
      link.href = url;
      link.download = filename;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      window.URL.revokeObjectURL(url);

      ElMessage.success(`已导出 ${accounts.length} 个账号`);
    }
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(`导出失败: ${error}`);
    }
  }
}

async function showAddGroupDialog() {
  try {
    const { value } = await ElMessageBox.prompt("请输入分组名称", "添加分组", {
      confirmButtonText: "确定",
      cancelButtonText: "取消",
      inputPattern: /^.{1,20}$/,
      inputErrorMessage: "分组名称长度应为1-20个字符",
    });

    await settingsStore.addGroup(value);
    ElMessage.success("分组添加成功");
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(`添加分组失败: ${error}`);
    }
  }
}

async function showRenameGroupDialog(oldName: string) {
  try {
    const { value } = await ElMessageBox.prompt(
      "请输入新的分组名称",
      `重命名分组 "${oldName}"`,
      {
        confirmButtonText: "确定",
        cancelButtonText: "取消",
        inputPattern: /^.{1,20}$/,
        inputErrorMessage: "分组名称长度应为1-20个字符",
        inputValue: oldName,
      },
    );

    if (value === oldName) {
      return;
    }

    await settingsStore.renameGroup(oldName, value);
    ElMessage.success("分组重命名成功");

    // 刷新账号列表
    await accountsStore.loadAccounts();
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(`重命名分组失败: ${error}`);
    }
  }
}

async function showDeleteGroupConfirm(name: string) {
  try {
    await ElMessageBox.confirm(
      `确定要删除分组 "${name}" 吗？该分组下的账号将被移至"未分组"`,
      "删除分组",
      {
        confirmButtonText: "确定",
        cancelButtonText: "取消",
        type: "warning",
      },
    );

    await settingsStore.deleteGroup(name);
    ElMessage.success("分组删除成功");

    // 刷新账号列表
    await accountsStore.loadAccounts();
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(`删除分组失败: ${error}`);
    }
  }
}

// 获取当前Windsurf账号信息
async function fetchCurrentWindsurfInfo() {
  try {
    const info = await settingsApi.getCurrentWindsurfInfo();
    if (info.is_active && info.email) {
      currentWindsurfEmail.value = info.email;
    }
    if (info.version) {
      windsurfVersion.value = info.version;
    }
  } catch (error) {
    console.error("获取当前Windsurf信息失败:", error);
  }
}

// 关闭批量分组对话框
function closeBatchGroupDialog() {
  showBatchGroupDialog.value = false;
  batchGroupTarget.value = "";
}

// 批量更改分组
async function handleBatchUpdateGroup() {
  const selectedIds = Array.from(accountsStore.selectedAccounts);
  if (selectedIds.length === 0) {
    ElMessage.warning("请先选择账号");
    return;
  }

  if (!batchGroupTarget.value) {
    ElMessage.warning("请选择目标分组");
    return;
  }

  isBatchUpdatingGroup.value = true;

  try {
    let successCount = 0;
    let failedCount = 0;

    // 逐个更新账号的分组
    for (const id of selectedIds) {
      const account = accountsStore.accounts.find((a) => a.id === id);
      if (account) {
        try {
          const updatedAccount = { ...account, group: batchGroupTarget.value };
          await accountsStore.updateAccount(updatedAccount);
          successCount++;
        } catch (error) {
          console.error(`更新账号 ${account.email} 分组失败:`, error);
          failedCount++;
        }
      }
    }

    // 显示结果
    if (failedCount === 0) {
      ElMessage.success(
        `成功将 ${successCount} 个账号移动到"${batchGroupTarget.value}"分组`,
      );
    } else {
      ElMessage.warning(
        `完成：成功 ${successCount} 个，失败 ${failedCount} 个`,
      );
    }

    // 关闭对话框并刷新
    closeBatchGroupDialog();
    accountsStore.clearSelection();
    await accountsStore.loadAccounts();
  } catch (error) {
    ElMessage.error(`批量更改分组失败: ${error}`);
  } finally {
    isBatchUpdatingGroup.value = false;
  }
}

// 显示关于对话框
function showAboutDialog() {
  showAbout.value = true;
}

// 初始化时获取当前账号信息和应用版本
onMounted(async () => {
  fetchCurrentWindsurfInfo();

  // 获取应用版本号
  try {
    const versionInfo = await invoke<any>("get_app_version");
    appVersion.value = versionInfo.version;
  } catch (error) {
    console.error("Failed to get app version:", error);
  }

  // 初始化排序配置
  initSortConfig();

  // 初始化自动重置定时器
  initAutoResetTimers();
});

// 组件卸载时清除自动重置定时器
onUnmounted(() => {
  autoResetTimerMap.value.forEach((timer) => clearInterval(timer));
  autoResetTimerMap.value.clear();
});
</script>

<style scoped>
.main-container {
  height: 100vh;
  width: 100vw;
}

.sidebar {
  background: #fff;
  border-right: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
  transition: width 0.3s;
  overflow: hidden;
}

/* 全局隐藏侧边栏的所有滚动条 */
.el-aside {
  overflow: hidden !important;
}

.sidebar::-webkit-scrollbar,
.sidebar :deep(::-webkit-scrollbar),
.el-aside::-webkit-scrollbar,
.el-menu::-webkit-scrollbar {
  display: none !important;
  width: 0 !important;
  background: transparent !important;
}

.sidebar,
.sidebar :deep(*),
.el-aside,
.el-menu {
  -ms-overflow-style: none !important; /* IE and Edge */
  scrollbar-width: none !important; /* Firefox */
  overflow-x: hidden !important;
}

.app-title {
  padding: 16px;
  display: flex;
  align-items: center;
  gap: 8px;
  border-bottom: 1px solid var(--el-border-color-lighter);
  font-size: 18px;
  font-weight: 600;
  white-space: nowrap;
  overflow: hidden;

  .el-icon {
    flex-shrink: 0;
  }

  .app-title-text {
    display: flex;
    flex-direction: column;
    gap: 2px;

    .version-text {
      font-size: 12px;
      font-weight: normal;
      color: var(--el-text-color-secondary);
      opacity: 0.8;
    }
  }
}

/* 收缩状态下标题样式 */
.sidebar.el-aside--collapse .app-title {
  padding: 16px 8px;
}

.sidebar-menu {
  flex: 1;
  border-right: none;
  overflow: hidden !important;
}

/* 隐藏Element Plus菜单的滚动条 */
.sidebar-menu::-webkit-scrollbar {
  display: none;
}

.sidebar-menu {
  -ms-overflow-style: none; /* IE and Edge */
  scrollbar-width: none; /* Firefox */
}

.sidebar-footer {
  padding: 12px;
  text-align: center;
  border-top: 1px solid #e4e7ed;
  background: #fff;
  position: relative;
  z-index: 1;
}

.header {
  background: linear-gradient(to bottom, #ffffff 0%, #fafbfc 100%);
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 28px;
  height: 64px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.03);
  position: relative;
  z-index: 10;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
}

.search-input {
  width: 220px;
}

.sort-select {
  width: 105px;
}

.sort-select :deep(.el-input__wrapper) {
  border-radius: 8px;
  height: 32px;
}

.search-input :deep(.el-input__wrapper) {
  border-radius: 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
  padding: 0 16px;
  height: 40px;
  background: #ffffff;
}

.search-input :deep(.el-input__wrapper:hover) {
  box-shadow: 0 3px 12px rgba(0, 0, 0, 0.1);
  border-color: var(--el-color-primary-light-5);
}

.search-input :deep(.el-input__wrapper.is-focus) {
  box-shadow: 0 4px 16px rgba(64, 158, 255, 0.15);
  border-color: var(--el-color-primary);
}

.search-input :deep(.el-input__inner) {
  font-size: 14px;
  color: #1e293b;
}

.search-input :deep(.el-input__prefix) {
  font-size: 16px;
  color: #94a3b8;
}

.header-right {
  display: flex;
  gap: 2px;
  align-items: center;
  flex-wrap: nowrap;
}

/* 圆形按钮徒加徽章样式 */
.header-right :deep(.el-badge) {
  vertical-align: middle;
}

.header-right :deep(.el-badge__content) {
  padding: 0 4px;
  height: 16px;
  line-height: 16px;
  font-size: 10px;
  border-radius: 8px;
  background-color: #f56c6c;
  border: none;
}

/* 全选按钮分隔线 */
.header-right .select-all-button {
  margin-left: 4px;
  position: relative;
}

.header-right .select-all-button::before {
  content: "";
  position: absolute;
  left: -6px;
  top: 50%;
  transform: translateY(-50%);
  height: 24px;
  width: 1px;
  background-color: #dcdfe6;
  pointer-events: none;
  z-index: 1;
}

/* 批量操作按钮 - 已通过父元素统一设置间距 */

/* 主要操作按钮样式 */
.header-right :deep(.el-button--primary:not(.is-circle)) {
  background: linear-gradient(135deg, #409eff 0%, #3b8cef 100%);
  border: none;
  border-radius: 20px;
  padding: 10px 20px;
  font-weight: 600;
  box-shadow: 0 3px 10px rgba(64, 158, 255, 0.25);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

/* 圆形主要按钮 - 蓝色 */
.header-right :deep(.el-button--primary.is-circle) {
  background: linear-gradient(135deg, #409eff 0%, #3b8cef 100%);
  border: none;
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.3);
  transition: all 0.3s ease;
  color: #ffffff;
}

.header-right :deep(.el-button--primary.is-circle:hover) {
  background: linear-gradient(135deg, #3b8cef 0%, #2d7ee5 100%);
  transform: scale(1.1) rotate(10deg);
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.4);
}

.header-right :deep(.el-button--primary:not(.is-circle):hover) {
  background: linear-gradient(135deg, #3b8cef 0%, #2d7ee5 100%);
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(64, 158, 255, 0.35);
}

.header-right :deep(.el-button--primary:active) {
  transform: translateY(0);
  box-shadow: 0 2px 6px rgba(64, 158, 255, 0.2);
}

/* 危险按钮 - 红色（批量删除） */
.header-right :deep(.el-button--danger.is-circle) {
  background: linear-gradient(135deg, #f56c6c 0%, #f04848 100%);
  border: none;
  box-shadow: 0 2px 8px rgba(245, 108, 108, 0.3);
  transition: all 0.3s ease;
  color: #ffffff;
}

.header-right :deep(.el-button--danger.is-circle:hover) {
  background: linear-gradient(135deg, #f04848 0%, #e63535 100%);
  transform: scale(1.1) rotate(10deg);
  box-shadow: 0 4px 12px rgba(245, 108, 108, 0.4);
}

.header-right :deep(.el-button--danger:active) {
  transform: translateY(0);
  box-shadow: 0 2px 6px rgba(245, 108, 108, 0.2);
}

/* 成功按钮 - 绿色（批量转让订阅） */
.header-right :deep(.el-button--success.is-circle) {
  background: linear-gradient(135deg, #67c23a 0%, #5daf34 100%);
  border: none;
  box-shadow: 0 2px 8px rgba(103, 194, 58, 0.3);
  transition: all 0.3s ease;
  color: #ffffff;
}

.header-right :deep(.el-button--success.is-circle:hover) {
  background: linear-gradient(135deg, #5daf34 0%, #529b2e 100%);
  transform: scale(1.1) rotate(10deg);
  box-shadow: 0 4px 12px rgba(103, 194, 58, 0.4);
}

/* 警告按钮 - 橙色（批量刷新状态） */
.header-right :deep(.el-button--warning.is-circle) {
  background: linear-gradient(135deg, #e6a23c 0%, #d48a1f 100%);
  border: none;
  box-shadow: 0 2px 8px rgba(230, 162, 60, 0.3);
  transition: all 0.3s ease;
  color: #ffffff;
}

.header-right :deep(.el-button--warning.is-circle:hover) {
  background: linear-gradient(135deg, #d48a1f 0%, #c27c0e 100%);
  transform: scale(1.1) rotate(10deg);
  box-shadow: 0 4px 12px rgba(230, 162, 60, 0.4);
}

.header-right :deep(.el-button--success:active) {
  transform: translateY(0);
  box-shadow: 0 2px 6px rgba(103, 194, 58, 0.2);
}

/* 普通按钮样式 */
.header-right :deep(.el-button--default) {
  background: linear-gradient(135deg, #f5f7fa 0%, #e9ecef 100%);
  border: 1px solid #dcdfe6;
  border-radius: 20px;
  padding: 10px 20px;
  font-weight: 600;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.08);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  color: #606266;
}

.header-right :deep(.el-button--default:hover) {
  background: linear-gradient(135deg, #ecf5ff 0%, #d9ecff 100%);
  border-color: #409eff;
  transform: translateY(-1px);
  box-shadow: 0 4px 10px rgba(64, 158, 255, 0.15);
  color: #409eff;
}

.header-right :deep(.el-button--default:active) {
  transform: translateY(0);
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}

/* 圆形按钮基础样式 */
.header-right :deep(.el-button.is-circle) {
  transition: all 0.3s ease;
  width: 40px;
  height: 40px;
}

/* 默认圆形按钮 - 统一的灰色风格 */
.header-right :deep(.el-button--default.is-circle) {
  color: #606266 !important;
  background: #ffffff;
  border: 1px solid #dcdfe6;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
}

.header-right :deep(.el-button--default.is-circle:hover) {
  color: #409eff !important;
  background: #ecf5ff;
  border-color: #c6e2ff;
  transform: scale(1.1);
  box-shadow: 0 2px 8px rgba(64, 158, 255, 0.2);
}

/* 特别处理圆形按钮的图标颜色 */
.header-right :deep(.el-button.is-circle .el-icon) {
  color: inherit !important;
  font-weight: 500;
}

.header-right :deep(.el-button.is-circle:hover) {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.header-right :deep(.el-button.is-circle:active) {
  transform: rotate(90deg) scale(0.95);
}

/* 按钮图标样式 */
.header-right :deep(.el-button .el-icon) {
  font-size: 18px;
  transition: all 0.3s ease;
}

/* 默认按钮的图标颜色 */
.header-right :deep(.el-button--default .el-icon) {
  color: #606266 !important;
}

.header-right :deep(.el-button--default:hover .el-icon) {
  color: #409eff !important;
}

/* 全选按钮 - 选中时特殊处理 */
.header-right
  :deep(
    .el-button--primary.is-circle[class*="el-button--primary"]:first-child
  ) {
  background: #409eff;
  border: none;
  color: #ffffff;
  box-shadow: 0 2px 6px rgba(64, 158, 255, 0.3);
}

/* 动态出现的按钮动画 */
.header-right :deep(.el-button--danger.is-circle),
.header-right :deep(.el-button--success.is-circle),
.header-right :deep(.el-button--warning.is-circle) {
  animation: slideInFromLeft 0.3s ease;
}

@keyframes slideInFromLeft {
  from {
    opacity: 0;
    transform: translateX(-20px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

/* 按钮内文字样式 */
.header-right :deep(.el-button span) {
  font-size: 14px;
  letter-spacing: 0.3px;
}

/* 按钮加载状态 */
.header-right :deep(.el-button.is-loading) {
  opacity: 0.8;
}

/* 导出对话框样式 */
.export-dialog {
  .el-message-box__message {
    padding: 0 !important;
  }

  input[type="radio"] {
    accent-color: #409eff;
    width: 16px;
    height: 16px;
    vertical-align: middle;
  }

  label:hover {
    background-color: #f5f7fa;
    border-radius: 6px;
    padding: 8px 12px;
    margin-left: -12px;
    margin-right: -12px;
  }
}

/* 按钮禁用状态 */
.header-right :deep(.el-button.is-disabled) {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
}

/* 筛选面板样式 */
.filter-panel {
  background: #ffffff;
  border-radius: 10px;
  padding: 12px 16px;
  margin-bottom: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  border: 1px solid rgba(0, 0, 0, 0.04);
}

.filter-panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.filter-title {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.filter-header-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.filter-panel-body {
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.filter-row {
  display: flex;
  align-items: center;
  gap: 16px;
}

.filter-row-select {
  gap: 20px;
}

.filter-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.filter-item-range {
  flex: none;
  min-width: auto;
}

.filter-item-select {
  flex-shrink: 0;
}

.filter-item-select .el-select {
  width: 120px;
}

.filter-label {
  font-size: 13px;
  color: #606266;
  font-weight: 500;
  white-space: nowrap;
}

.filter-range {
  display: flex;
  align-items: center;
  gap: 6px;
  flex: 1;
}

.filter-range .el-input-number {
  width: 72px;
}

.range-separator {
  color: #c0c4cc;
  font-size: 12px;
}

/* 筛选面板动画 */
.filter-slide-enter-active,
.filter-slide-leave-active {
  transition: all 0.3s ease;
}

.filter-slide-enter-from,
.filter-slide-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

/* 分页容器 */
.pagination-container {
  display: flex;
  justify-content: center;
  padding: 20px 0;
  margin-top: 16px;
  background: transparent;
}

.pagination-container .el-pagination {
  background: #ffffff;
  padding: 12px 20px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

/* 筛选按钮样式 */
.filter-toggle-btn {
  margin-left: 12px;
}

.main-content {
  background: #f5f7fa;
  padding: 8px 6px;
}

.loading-container,
.empty-container {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.accounts-container {
  width: 100%;
}

.accounts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(290px, 1fr));
  gap: 8px;
  width: 100%;
  padding: 0;
}

/* 响应式布局 */
@media (max-width: 1400px) {
  .accounts-grid {
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  }
}

@media (max-width: 1024px) {
  .main-content {
    padding: 10px 6px;
  }

  .accounts-grid {
    gap: 8px;
  }
}

@media (max-width: 768px) {
  .accounts-grid {
    grid-template-columns: 1fr;
  }

  .header-left {
    max-width: 200px;
  }

  .main-content {
    padding: 8px 4px;
  }
}

/* 暗色主题支持 */
:root.dark .sidebar {
  background: #1e1e1e;
  border-color: rgba(255, 255, 255, 0.08);
}

:root.dark .sidebar-menu {
  background: #1e1e1e;
}

:root.dark .sidebar-menu .el-menu-item {
  background: transparent;
  color: #cfd3dc;
}

:root.dark .sidebar-menu .el-menu-item:hover {
  background-color: rgba(64, 158, 255, 0.05);
}

/* 分组项样式 */
.group-item {
  position: relative;
}

.group-item-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.group-name {
  flex: 1;
  cursor: pointer;
}

.group-actions {
  display: flex;
  gap: 8px;
  align-items: center;
}

.group-action-icon {
  cursor: pointer;
  padding: 4px;
  transition: all 0.3s;
  color: #606266;
}

.group-action-icon:hover {
  color: #409eff;
  background-color: rgba(64, 158, 255, 0.1);
  border-radius: 4px;
}

.group-action-icon.delete:hover {
  color: #f56c6c;
  background-color: rgba(245, 108, 108, 0.1);
}

/* 暗色主题适配 */
:root.dark .sidebar-menu .el-menu-item.is-active {
  background-color: rgba(64, 158, 255, 0.1);
  color: #409eff;
}

:root.dark .sidebar-footer {
  background: #1e1e1e;
  border-top-color: rgba(255, 255, 255, 0.08);
}

:root.dark .sidebar-footer .el-button.is-circle {
  background-color: #262729;
  border-color: #4c4d4f;
  color: #cfd3dc;
}

:root.dark .sidebar-footer .el-button.is-circle:hover {
  background-color: #303133;
  border-color: #5a5b5d;
  color: #409eff;
}

:root.dark .sidebar-footer .el-button.is-circle:active {
  background-color: #1a1a1c;
  border-color: #409eff;
}

:root.dark .header {
  background: linear-gradient(to bottom, #1e1e1e 0%, #1a1a1a 100%);
  border-bottom-color: rgba(255, 255, 255, 0.08);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

:root.dark .search-input :deep(.el-input__wrapper) {
  background: #2a2a2a;
  border-color: rgba(255, 255, 255, 0.08);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

:root.dark .search-input :deep(.el-input__wrapper:hover) {
  border-color: var(--el-color-primary-light-3);
  box-shadow: 0 3px 12px rgba(0, 0, 0, 0.3);
}

:root.dark .search-input :deep(.el-input__wrapper.is-focus) {
  border-color: var(--el-color-primary);
  box-shadow: 0 4px 16px rgba(64, 158, 255, 0.25);
}

:root.dark .search-input :deep(.el-input__inner) {
  color: #e4e4e7;
}

:root.dark .search-input :deep(.el-input__prefix) {
  color: #94a3b8;
}

:root.dark .header-right :deep(.el-button--primary) {
  background: linear-gradient(135deg, #409eff 0%, #3b8cef 100%);
  box-shadow: 0 3px 10px rgba(64, 158, 255, 0.3);
}

:root.dark .header-right :deep(.el-button--primary:hover) {
  box-shadow: 0 6px 16px rgba(64, 158, 255, 0.4);
}

:root.dark .header-right :deep(.el-button--danger) {
  background: linear-gradient(135deg, #f56c6c 0%, #f04848 100%);
  box-shadow: 0 3px 10px rgba(245, 108, 108, 0.3);
}

:root.dark .header-right :deep(.el-button--danger:hover) {
  box-shadow: 0 6px 16px rgba(245, 108, 108, 0.4);
}

:root.dark .header-right :deep(.el-button--success) {
  background: linear-gradient(135deg, #67c23a 0%, #5daf34 100%);
  box-shadow: 0 3px 10px rgba(103, 194, 58, 0.3);
}

:root.dark .header-right :deep(.el-button--success:hover) {
  box-shadow: 0 6px 16px rgba(103, 194, 58, 0.4);
}

:root.dark .header-right :deep(.el-button--default) {
  background: linear-gradient(135deg, #2a2a2a 0%, #252525 100%);
  border-color: rgba(255, 255, 255, 0.1);
  color: #e4e4e7;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
}

:root.dark .header-right :deep(.el-button--default:hover) {
  background: linear-gradient(135deg, #303030 0%, #2a2a2a 100%);
  border-color: rgba(255, 255, 255, 0.15);
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.4);
}

:root.dark .header-right :deep(.el-button.is-circle) {
  background: linear-gradient(135deg, #2a2a2a 0%, #252525 100%);
  border-color: rgba(255, 255, 255, 0.1);
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
}

:root.dark .header-right :deep(.el-button.is-circle:hover) {
  background: linear-gradient(135deg, #1e3a5f 0%, #1a3454 100%);
  border-color: var(--el-color-primary-light-3);
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
}

:root.dark .main-content {
  background: #121212;
}

/* 深色模式筛选面板 */
:root.dark .filter-panel {
  background: #1e1e1e;
  border-color: rgba(255, 255, 255, 0.08);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

:root.dark .filter-title {
  color: #e5eaf3;
}

:root.dark .filter-label {
  color: #a0aec0;
}

:root.dark .range-separator {
  color: #4a5568;
}

/* 深色模式分页 */
:root.dark .pagination-container .el-pagination {
  background: #1e1e1e;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
}

/* 批量分组对话框样式 */
.batch-group-content {
  padding: 10px 0;
}

.batch-group-hint {
  margin-bottom: 16px;
  color: #606266;
  font-size: 14px;
}

.batch-group-hint strong {
  color: #409eff;
  font-weight: 600;
}

:root.dark .batch-group-hint {
  color: #a0aec0;
}

/* 批量转让对话框样式 */
.email-count-hint {
  font-size: 14px;
  color: #606266;
}

.batch-transfer-progress {
  margin-top: 20px;
  padding: 16px;
  background: #f8fafc;
  border-radius: 8px;
}

.batch-transfer-progress .progress-status {
  margin-top: 12px;
  text-align: center;
  color: #64748b;
  font-size: 14px;
}

:root.dark .batch-transfer-progress {
  background: #2d3748;
}

:root.dark .batch-transfer-progress .progress-status {
  color: #a0aec0;
}

.language-btn {
  padding: 8px;
  display: flex;
  align-items: center;
}

.language-flag {
  font-size: 18px;
  line-height: 1;
  margin-right: 2px;
}

.language-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.language-item .flag {
  font-size: 18px;
}
</style>
