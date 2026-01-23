<template>
  <el-dialog
    v-model="uiStore.showLogsDialog"
    :title="$t('dialog.logs.title')"
    width="800px"
  >
    <div class="logs-container">
      <div class="logs-header">
        <el-button size="small" @click="loadLogs" :icon="Refresh">
          {{ $t("common.reset") }}
        </el-button>
        <el-button size="small" @click="clearLogs" :icon="Delete">
          {{ $t("dialog.logs.clear") }}
        </el-button>
      </div>

      <el-table :data="logs" style="width: 100%" max-height="400">
        <el-table-column
          prop="timestamp"
          :label="$t('dialog.logs.time')"
          width="180"
        >
          <template #default="{ row }">
            {{ formatDate(row.timestamp) }}
          </template>
        </el-table-column>

        <el-table-column
          prop="operation_type"
          :label="$t('dialog.logs.type')"
          width="120"
        >
          <template #default="{ row }">
            <el-tag :type="getOperationTypeTag(row.operation_type)">
              {{ formatOperationType(row.operation_type) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column
          prop="account_email"
          :label="$t('dialog.logs.account')"
          width="180"
        />

        <el-table-column prop="message" :label="$t('dialog.logs.message')" />

        <el-table-column
          prop="status"
          :label="$t('dialog.logs.status')"
          width="80"
        >
          <template #default="{ row }">
            <el-tag :type="row.status === 'success' ? 'success' : 'danger'">
              {{
                row.status === "success"
                  ? $t("dialog.logs.success")
                  : $t("dialog.logs.error")
              }}
            </el-tag>
          </template>
        </el-table-column>
      </el-table>
    </div>
  </el-dialog>
</template>

<script setup lang="ts">
import { onMounted, computed } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage, ElMessageBox } from "element-plus";
import { Refresh, Delete } from "@element-plus/icons-vue";
import { useSettingsStore, useUIStore } from "@/store";
import dayjs from "dayjs";

const { t } = useI18n();
const settingsStore = useSettingsStore();
const uiStore = useUIStore();

const logs = computed(() => {
  return [...settingsStore.logs].sort((a, b) => {
    return new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime();
  });
});

onMounted(() => {
  loadLogs();
});

async function loadLogs() {
  try {
    await settingsStore.loadLogs(100);
  } catch (error) {
    ElMessage.error(`${$t("dialog.logs.error")}: ${error}`);
  }
}

async function clearLogs() {
  try {
    await ElMessageBox.confirm(
      t("dialog.logs.clear") + "?",
      t("common.confirm"),
      {
        confirmButtonText: t("common.confirm"),
        cancelButtonText: t("common.cancel"),
        type: "warning",
      },
    );

    await settingsStore.clearLogs();
    ElMessage.success(t("dialog.logs.success"));
  } catch (error) {
    if (error !== "cancel") {
      ElMessage.error(`${$t("dialog.logs.error")}: ${error}`);
    }
  }
}

function formatDate(date: string) {
  return dayjs(date).format("YYYY-MM-DD HH:mm:ss");
}

function formatOperationType(type: string) {
  const typeMap: Record<string, string> = {
    login: t("dialog.logs.operationTypes.login"),
    refresh_token: t("dialog.logs.operationTypes.refreshToken"),
    reset_credits: t("dialog.logs.operationTypes.resetCredits"),
    update_seats: t("dialog.logs.operationTypes.updateSeats"),
    get_billing: t("dialog.logs.operationTypes.getBilling"),
    update_plan: t("dialog.logs.operationTypes.updatePlan"),
    add_account: t("dialog.logs.operationTypes.addAccount"),
    delete_account: t("dialog.logs.operationTypes.deleteAccount"),
    edit_account: t("dialog.logs.operationTypes.editAccount"),
    batch_operation: t("dialog.logs.operationTypes.batchOperation"),
  };
  return typeMap[type] || type;
}

function getOperationTypeTag(type: string) {
  const tagMap: Record<string, string> = {
    login: "primary",
    refresh_token: "info",
    reset_credits: "success",
    update_seats: "warning",
    get_billing: "info",
    update_plan: "warning",
    add_account: "success",
    delete_account: "danger",
    edit_account: "warning",
    batch_operation: "primary",
  };
  return tagMap[type] || "info";
}
</script>

<style scoped>
.logs-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.logs-header {
  display: flex;
  gap: 8px;
  padding-bottom: 12px;
  border-bottom: 1px solid #ebeef5;
}
</style>
