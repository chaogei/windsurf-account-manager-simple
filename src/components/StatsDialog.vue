<template>
  <el-dialog
    v-model="uiStore.showStatsDialog"
    :title="t('dialog.stats.title')"
    width="700px"
  >
    <div v-if="loading" class="loading-container">
      <el-icon class="is-loading" size="32"><Loading /></el-icon>
    </div>

    <div v-else-if="stats">
      <el-row :gutter="20">
        <el-col :span="8">
          <el-statistic
            :title="t('dialog.stats.totalAccounts')"
            :value="stats.total_accounts"
          />
        </el-col>
        <el-col :span="8">
          <el-statistic
            :title="t('dialog.stats.activeAccounts')"
            :value="stats.active_accounts"
          />
        </el-col>
        <el-col :span="8">
          <el-statistic
            :title="t('dialog.stats.groupCount')"
            :value="stats.groups"
          />
        </el-col>
      </el-row>

      <el-divider />

      <el-row :gutter="20">
        <el-col :span="12">
          <el-statistic
            :title="t('dialog.stats.successRate')"
            :value="stats.success_rate"
            suffix="%"
            :precision="1"
          />
          <div class="stat-detail">
            {{ t("dialog.logs.success") }}: {{ stats.successful_operations }} /
            {{ t("dialog.logs.error") }}: {{ stats.failed_operations }}
          </div>
        </el-col>
        <el-col :span="12">
          <el-statistic
            :title="t('dialog.stats.resetSuccessRate')"
            :value="stats.reset_success_rate"
            suffix="%"
            :precision="1"
          />
          <div class="stat-detail">
            {{ t("dialog.logs.success") }}: {{ stats.successful_resets }} /
            {{ t("dialog.logs.error") }}: {{ stats.failed_resets }}
          </div>
        </el-col>
      </el-row>

      <el-divider />

      <el-descriptions :column="1" border>
        <el-descriptions-item :label="t('dialog.stats.totalOperations')">
          {{ stats.total_operations }}
        </el-descriptions-item>

        <el-descriptions-item :label="t('dialog.stats.totalResets')">
          {{ stats.total_resets }}
        </el-descriptions-item>

        <el-descriptions-item :label="t('dialog.stats.lastOperation')">
          {{
            stats.last_operation
              ? formatDate(stats.last_operation)
              : t("dialog.stats.noData")
          }}
        </el-descriptions-item>

        <el-descriptions-item :label="t('settings.autoRefresh')">
          <el-tag
            :type="stats.settings?.auto_refresh_token ? 'success' : 'info'"
          >
            {{
              stats.settings?.auto_refresh_token
                ? t("dialog.settings.on")
                : t("dialog.settings.off")
            }}
          </el-tag>
        </el-descriptions-item>

        <el-descriptions-item :label="t('settings.retryTimes')">
          {{ stats.settings?.retry_times || 2 }}
        </el-descriptions-item>

        <el-descriptions-item :label="t('settings.concurrentLimit')">
          {{ stats.settings?.concurrent_limit || 5 }}
        </el-descriptions-item>
      </el-descriptions>
    </div>

    <template #footer>
      <el-button @click="refresh" :icon="Refresh">{{
        t("common.reset")
      }}</el-button>
      <el-button @click="uiStore.closeStatsDialog">{{
        t("common.close")
      }}</el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from "vue";
import { ElMessage } from "element-plus";
import { Loading, Refresh } from "@element-plus/icons-vue";
import { useUIStore } from "@/store";
import { settingsApi } from "@/api";
import dayjs from "dayjs";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const uiStore = useUIStore();

const loading = ref(false);
const stats = ref<any>(null);

watch(
  () => uiStore.showStatsDialog,
  (show) => {
    if (show) {
      loadStats();
    }
  },
);

onMounted(() => {
  if (uiStore.showStatsDialog) {
    loadStats();
  }
});

async function loadStats() {
  loading.value = true;
  try {
    stats.value = await settingsApi.getStats();
  } catch (error) {
    ElMessage.error(`${t("dialog.stats.loadError")}: ${error}`);
  } finally {
    loading.value = false;
  }
}

function refresh() {
  loadStats();
}

function formatDate(date: string) {
  return dayjs(date).format("YYYY-MM-DD HH:mm:ss");
}
</script>

<style scoped>
.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 40px;
}

.stat-detail {
  margin-top: 8px;
  font-size: 12px;
  color: #909399;
  text-align: center;
}

.el-statistic {
  text-align: center;
}

.el-row {
  margin-bottom: 20px;
}
</style>
