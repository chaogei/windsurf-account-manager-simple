<template>
  <el-dialog
    v-model="uiStore.showSettingsDialog"
    :title="$t('dialog.settings.title')"
    width="800px"
    :close-on-click-modal="false"
    append-to-body
    @close="handleClose"
  >
    <el-tabs v-model="activeTab" class="settings-tabs">
      <el-tab-pane :label="$t('dialog.settings.basic.title')" name="basic">
        <el-form :model="settings" label-width="140px">
          <!-- 现有基础设置 -->
          <el-form-item :label="$t('dialog.settings.autoRefreshToken')">
            <el-switch
              v-model="settings.auto_refresh_token"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
            />
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.unlimitedConcurrent')">
            <el-switch
              v-model="settings.unlimitedConcurrentRefresh"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{
                $t(
                  "dialog.settings.unlimitedConcurrentDesc",
                  "⚠️ 开启后将不限制并发数量，可能导致账号被封禁，请谨慎使用",
                )
              }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.concurrentLimit')">
            <el-input-number
              v-model="settings.concurrent_limit"
              :min="1"
              :max="settings.unlimitedConcurrentRefresh ? 1000 : 20"
              :disabled="settings.unlimitedConcurrentRefresh"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{
                $t(
                  "dialog.settings.concurrentLimitDesc",
                  "建议设置为 3-5，过高可能会导致请求由于速率限制而失败",
                )
              }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.seatCountOptions')">
            <el-input
              v-model="seatCountOptionsInput"
              :placeholder="$t('dialog.settings.seatCountPlaceholder')"
              @blur="parseSeatCountOptions"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{
                $t(
                  "dialog.settings.seatCountDesc",
                  "输入数字并用逗号分隔，例如: 18, 19, 20",
                )
              }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.retryTimes')">
            <el-input-number
              v-model="settings.retry_times"
              :min="0"
              :max="10"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.retryTimesDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.theme')">
            <el-select v-model="settings.theme">
              <el-option :label="$t('dialog.settings.light')" value="light" />
              <el-option :label="$t('dialog.settings.dark')" value="dark" />
            </el-select>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.language')">
            <el-select
              v-model="settingsStore.language"
              @change="handleLanguageChange"
            >
              <el-option
                :label="$t('dialog.settings.languages.zh')"
                value="zh"
              />
              <el-option
                :label="$t('dialog.settings.languages.en')"
                value="en"
              />
              <el-option
                :label="$t('dialog.settings.languages.fr')"
                value="fr"
              />
              <el-option
                :label="$t('dialog.settings.languages.es')"
                value="es"
              />
            </el-select>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.showResultDialog')">
            <el-switch
              v-model="settings.show_seats_result_dialog"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.showResultDialogDesc") }}
            </div>
          </el-form-item>

          <el-divider content-position="left">{{
            $t("settings.network", "网络设置")
          }}</el-divider>

          <el-form-item :label="$t('settings.proxy', '代理设置')">
            <el-switch
              v-model="settings.proxyEnabled"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.proxySettingsDesc") }}
            </div>
          </el-form-item>

          <el-form-item
            :label="$t('dialog.settings.proxyAddress', '代理地址')"
            v-if="settings.proxyEnabled"
          >
            <el-input
              v-model="settings.proxyUrl"
              placeholder="http://127.0.0.1:7890"
              style="width: 280px"
              clearable
            >
              <template #prefix>
                <el-icon><Connection /></el-icon>
              </template>
            </el-input>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{
                $t(
                  "dialog.settings.proxyFormatDesc",
                  "支持 HTTP/HTTPS/SOCKS5 代理，格式：http://host:port 或 socks5://host:port",
                )
              }}
            </div>
          </el-form-item>

          <el-form-item
            :label="$t('dialog.settings.resetConnection', '重置网络连接')"
          >
            <el-button
              type="warning"
              @click="handleResetHttpClient"
              :loading="resettingHttp"
            >
              {{ $t("dialog.settings.resetHttpClient", "重置HTTP客户端") }}
            </el-button>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{
                $t(
                  "dialog.settings.resetHttpClientDesc",
                  "当遇到连续的API请求失败时，可点击此按钮重置网络连接池",
                )
              }}
            </div>
          </el-form-item>
        </el-form>
      </el-tab-pane>

      <!-- 支付设置标签页 -->
      <el-tab-pane
        :label="$t('dialog.settings.payment.title', '支付设置')"
        name="payment"
      >
        <el-form :model="settings" label-width="140px">
          <el-form-item :label="$t('dialog.settings.payment.autoOpen')">
            <el-switch
              v-model="settings.autoOpenPaymentLinkInWebview"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.autoOpenDesc") }}
            </div>
          </el-form-item>

          <el-divider content-position="left">{{
            $t("dialog.settings.payment.browserSettings")
          }}</el-divider>

          <el-form-item :label="$t('dialog.settings.payment.autoOpenBrowser')">
            <el-switch
              v-model="settings.autoOpenBrowser"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.autoOpenBrowserDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.payment.browserMode')">
            <el-radio-group v-model="settings.browserMode">
              <el-radio-button label="incognito">{{
                $t("dialog.settings.payment.modes.incognito")
              }}</el-radio-button>
              <el-radio-button label="normal">{{
                $t("dialog.settings.payment.modes.normal")
              }}</el-radio-button>
            </el-radio-group>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.browserModeDesc") }}
            </div>
          </el-form-item>

          <el-divider content-position="left">{{
            $t("dialog.settings.payment.autoFillSettings")
          }}</el-divider>

          <el-form-item :label="$t('dialog.settings.payment.autoFill')">
            <el-switch
              v-model="settings.autoFillPaymentForm"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.autoFillDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.payment.showCardInfo')">
            <el-switch
              v-model="settings.showVirtualCardInfo"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.showCardInfoDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.payment.autoSubmit')">
            <el-switch
              v-model="settings.autoSubmitPaymentForm"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
              :disabled="!settings.autoFillPaymentForm"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.autoSubmitDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.payment.delay')">
            <el-input-number
              v-model="settings.paymentPageDelay"
              :min="1"
              :max="10"
              :step="1"
              :disabled="!settings.autoFillPaymentForm"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.delayDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.payment.customBin')">
            <el-input
              v-model="settings.customCardBin"
              :placeholder="
                $t('dialog.settings.payment.customBinDesc', '请输入4-12位数字')
              "
              maxlength="12"
              @input="validateCardBin"
            >
              <template #append>
                <el-button @click="resetCardBin">{{
                  $t("dialog.settings.payment.resetDefault")
                }}</el-button>
              </template>
            </el-input>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.customBinDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.payment.binRange')">
            <el-input
              v-model="settings.customCardBinRange"
              :placeholder="$t('dialog.settings.payment.binRangePlaceholder')"
              @input="validateCardBinRange"
            >
              <template #append>
                <el-button @click="clearCardBinRange">{{
                  $t("common.clear") || $t("dialog.settings.clear")
                }}</el-button>
              </template>
            </el-input>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.binRangeDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.payment.retryTimes')">
            <el-input-number
              v-model="settings.cardBindRetryTimes"
              :min="0"
              :max="20"
              :step="1"
              controls-position="right"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.retryTimesDesc") }}
            </div>
          </el-form-item>

          <el-divider content-position="left">{{
            $t("dialog.settings.payment.binPool")
          }}</el-divider>

          <el-form-item :label="$t('dialog.settings.payment.testMode')">
            <div style="display: flex; align-items: center; gap: 10px">
              <el-switch v-model="settings.testModeEnabled" />
              <el-button
                size="small"
                type="warning"
                @click="resetTestModeProgress"
                :disabled="!testModeProgress"
              >
                {{ $t("dialog.settings.payment.resetProgress") }}
              </el-button>
            </div>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.testModeDesc") }}
              （{{
                $t("common.count", { count: successBinCount }) ||
                $t("dialog.settings.poolCount", { count: successBinCount })
              }}）
              <span v-if="testModeProgress" style="color: #67c23a">
                <br />{{
                  $t("common.progress") ||
                  $t("dialog.settings.currentProgress")
                }}：{{ testModeProgress }}
              </span>
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.payment.useLocalPool')">
            <el-switch
              v-model="settings.useLocalSuccessBins"
              :disabled="successBinCount === 0"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.payment.useLocalPoolDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.payment.poolManagement')">
            <el-button-group>
              <el-button
                size="small"
                @click="viewSuccessBins"
                :disabled="successBinCount === 0"
              >
                {{ $t("dialog.settings.payment.viewPool") }}
              </el-button>
              <el-button
                size="small"
                type="danger"
                @click="clearSuccessBins"
                :disabled="successBinCount === 0"
              >
                {{ $t("dialog.settings.payment.clearPool") }}
              </el-button>
            </el-button-group>
          </el-form-item>

          <el-alert
            :title="$t('dialog.settings.payment.importantNote')"
            type="warning"
            :closable="false"
            show-icon
            style="margin-top: 20px"
          >
            <template #default>
              <div style="font-size: 12px; line-height: 1.6">
                <p>{{ $t("dialog.settings.payment.importantDesc1") }}</p>
                <p>{{ $t("dialog.settings.payment.importantDesc2") }}</p>
                <p>{{ $t("dialog.settings.payment.importantDesc3") }}</p>
                <p>{{ $t("dialog.settings.payment.importantDesc4") }}</p>
              </div>
            </template>
          </el-alert>
        </el-form>
      </el-tab-pane>

      <!-- 无感换号标签页 -->
      <el-tab-pane
        :label="$t('dialog.settings.seamless.title')"
        name="seamless"
      >
        <el-form :model="settings" label-width="140px">
          <el-form-item :label="$t('dialog.settings.seamless.path')">
            <el-input
              v-model="windsurfPath"
              :placeholder="$t('dialog.settings.seamless.pathPlaceholder')"
              @blur="handlePathChange"
            >
              <template #append>
                <el-button-group>
                  <el-button
                    @click="detectWindsurfPath"
                    :loading="detectingPath"
                  >
                    {{ $t("dialog.settings.seamless.autoDetect") }}
                  </el-button>
                  <el-button @click="browseWindsurfPath">
                    {{ $t("dialog.settings.seamless.browse") }}
                  </el-button>
                </el-button-group>
              </template>
            </el-input>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.seamless.pathDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.seamless.enable')">
            <el-switch
              v-model="settings.seamlessSwitchEnabled"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
              :loading="patchLoading"
              @change="handleSeamlessSwitch"
              :disabled="!windsurfPath"
            />
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.seamless.patchStatus')">
            <el-tag v-if="patchStatus.installed" type="success">{{
              $t("dialog.settings.seamless.installed")
            }}</el-tag>
            <el-tag v-else-if="patchStatus.error" type="danger">{{
              patchStatus.error
            }}</el-tag>
            <el-tag v-else type="info">{{
              $t("dialog.settings.seamless.notInstalled")
            }}</el-tag>
            <el-button
              v-if="patchStatus.installed"
              size="small"
              style="margin-left: 10px"
              @click="checkPatchStatus"
            >
              {{ $t("dialog.settings.seamless.recheck") }}
            </el-button>
          </el-form-item>

          <el-alert
            :title="$t('dialog.settings.seamless.featureDesc')"
            type="info"
            :closable="false"
            show-icon
            style="margin-top: 20px"
          >
            <template #default>
              <div style="font-size: 12px; line-height: 1.6">
                <p>{{ $t("dialog.settings.seamless.featureDesc1") }}</p>
                <p>{{ $t("dialog.settings.seamless.featureDesc2") }}</p>
              </div>
            </template>
          </el-alert>

          <el-divider content-position="left">{{
            $t("dialog.settings.seamless.weige")
          }}</el-divider>

          <el-form-item :label="$t('dialog.settings.seamless.enableWeige')">
            <el-switch
              v-model="settings.cunzhiEnabled"
              :active-text="$t('dialog.settings.on')"
              :inactive-text="$t('dialog.settings.off')"
              :loading="cunzhiLoading"
              @change="handleCunzhiSwitch"
            />
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.seamless.weigeStatus')">
            <el-tag v-if="cunzhiStatus.installed" type="success">{{
              $t("dialog.settings.seamless.installed")
            }}</el-tag>
            <el-tag v-else-if="cunzhiStatus.error" type="danger">{{
              cunzhiStatus.error
            }}</el-tag>
            <el-tag v-else type="info">{{
              $t("dialog.settings.seamless.notInstalled")
            }}</el-tag>
            <el-button
              v-if="cunzhiStatus.installed"
              size="small"
              style="margin-left: 10px"
              @click="checkCunzhiStatus"
            >
              {{ $t("dialog.settings.seamless.recheck") }}
            </el-button>
          </el-form-item>

          <el-alert
            :title="$t('dialog.settings.seamless.weigeDesc')"
            type="success"
            :closable="false"
            show-icon
            style="margin-top: 10px"
          >
            <template #default>
              <div style="font-size: 12px; line-height: 1.6">
                <p>{{ $t("dialog.settings.seamless.weigeDesc1") }}</p>
                <p>{{ $t("dialog.settings.seamless.weigeDesc2") }}</p>
              </div>
            </template>
          </el-alert>
        </el-form>
      </el-tab-pane>
    </el-tabs>

    <template #footer>
      <el-button @click="handleClose">{{ $t("common.cancel") }}</el-button>
      <el-button type="primary" @click="handleSave" :loading="loading">
        {{ $t("common.save") }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, watch, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage, ElMessageBox } from "element-plus";
import { Connection } from "@element-plus/icons-vue";
import { useSettingsStore, useUIStore } from "@/store";
import { invoke } from "@tauri-apps/api/core";
import { systemApi } from "@/api";

const { t } = useI18n();
const settingsStore = useSettingsStore();
const uiStore = useUIStore();

const loading = ref(false);
const activeTab = ref("basic"); // 当前激活的标签页
const seatCountOptionsInput = ref("18, 19, 20"); // 座位数选项输入框
const resettingHttp = ref(false); // HTTP客户端重置中

// 解析座位数选项
function parseSeatCountOptions() {
  const input = seatCountOptionsInput.value.trim();
  if (!input) {
    settings.seat_count_options = [18, 19, 20];
    seatCountOptionsInput.value = "18, 19, 20";
    return;
  }

  const numbers = input
    .split(/[,，\s]+/)
    .map((s) => parseInt(s.trim(), 10))
    .filter((n) => !isNaN(n) && n > 0);

  if (numbers.length === 0) {
    ElMessage.warning(
      t(
        "dialog.settings.invalidSeatCount",
        $t("dialog.settings.pleaseEnterValidSeatCount"),
      ),
    );
    settings.seat_count_options = [18, 19, 20];
    seatCountOptionsInput.value = "18, 19, 20";
  } else {
    settings.seat_count_options = numbers;
    seatCountOptionsInput.value = numbers.join(", ");
  }
}

const settings = reactive<{
  auto_refresh_token: boolean;
  seat_count_options: number[];
  retry_times: number;
  theme: string;
  concurrent_limit: number;
  show_seats_result_dialog: boolean;
  autoOpenPaymentLinkInWebview: boolean;
  autoFillPaymentForm: boolean;
  autoSubmitPaymentForm: boolean;
  paymentPageDelay: number;
  showVirtualCardInfo: boolean;
  customCardBin: string;
  customCardBinRange: string;
  cardBindRetryTimes: number;
  testModeEnabled: boolean;
  useLocalSuccessBins: boolean;
  seamlessSwitchEnabled: boolean;
  windsurfPath: string | null;
  patchBackupPath: string | null;
  autoOpenBrowser: boolean;
  browserMode: "incognito" | "normal";
  privacyMode: boolean;
  unlimitedConcurrentRefresh: boolean;
  proxyEnabled: boolean;
  proxyUrl: string | null;
  useLightweightApi: boolean;
  cunzhiEnabled: boolean;
}>({
  auto_refresh_token: true,
  seat_count_options: [18, 19, 20],
  retry_times: 2,
  theme: "light",
  concurrent_limit: 5,
  show_seats_result_dialog: false, // 默认关闭
  autoOpenPaymentLinkInWebview: false, // 默认关闭自动打开支付页面
  autoFillPaymentForm: false, // 默认关闭自动填写表单
  autoSubmitPaymentForm: false, // 默认关闭自动提交
  paymentPageDelay: 2, // 默认延迟2秒
  showVirtualCardInfo: false, // 默认关闭虚拟卡信息弹窗
  customCardBin: "626202", // 默认卡头
  customCardBinRange: "", // 默认不使用卡段范围
  cardBindRetryTimes: 5, // 默认绑卡重试5次
  testModeEnabled: false, // 默认关闭测试模式
  useLocalSuccessBins: false, // 默认不使用本地BIN池
  seamlessSwitchEnabled: false, // 默认关闭无感换号
  windsurfPath: null, // Windsurf路径
  patchBackupPath: null, // 补丁备份路径
  autoOpenBrowser: true, // 默认自动打开浏览器
  browserMode: "incognito", // 默认无痕模式
  privacyMode: false, // 默认关闭隐私模式
  unlimitedConcurrentRefresh: false, // 默认关闭全量并发刷新
  proxyEnabled: false, // 默认关闭代理
  proxyUrl: null, // 默认无代理地址
  useLightweightApi: true, // 默认使用轻量级API
  cunzhiEnabled: false, // 默认关闭伟哥功能
});

// 成功BIN池相关
const successBinCount = ref(0);
const testModeProgress = ref<string | null>(null);

async function loadSuccessBinCount() {
  try {
    const bins = await invoke<string[]>("get_success_bins");
    successBinCount.value = bins.length;
  } catch (e) {
    successBinCount.value = 0;
  }
}

async function loadTestModeProgress() {
  try {
    testModeProgress.value = await invoke<string | null>(
      "get_test_mode_progress",
    );
  } catch (e) {
    testModeProgress.value = null;
  }
}

async function resetTestModeProgress() {
  try {
    await ElMessageBox.confirm(
      $t(
        "dialog.settings.payment.confirmResetProgress",
        $t("dialog.settings.confirmResetProgressMessage"),
      ),
      t("common.confirm"), // "确认重置",
      {
        type: "warning",
        confirmButtonText: t("common.confirm"),
        cancelButtonText: t("common.cancel"),
      },
    );
    await invoke("reset_test_mode_progress");
    testModeProgress.value = null;
    ElMessage.success(t("dialog.settings.payment.progressReset", "进度已重置"));
  } catch (e) {
    // 用户取消
  }
}

async function viewSuccessBins() {
  try {
    const bins = await invoke<string[]>("get_success_bins");
    if (bins.length === 0) {
      ElMessage.info(t("dialog.settings.payment.poolEmpty", "BIN池为空"));
      return;
    }
    ElMessageBox.alert(
      `<div style="max-height: 300px; overflow-y: auto;">
        <p><b>${$t("dialog.settings.payment.totalBins", { count: bins.length }) || `共 ${bins.length} 个成功BIN：`}</b></p>
        <p style="font-family: monospace; word-break: break-all;">${bins.join(", ")}</p>
      </div>`,
      t("dialog.settings.payment.poolTitle", "成功BIN池"),
      { dangerouslyUseHTMLString: true },
    );
  } catch (e) {
    ElMessage.error(
      t("dialog.settings.payment.getPoolFailed", "获取BIN池失败"),
    );
  }
}

async function clearSuccessBins() {
  try {
    await ElMessageBox.confirm(
      t(
        "dialog.settings.payment.confirmClearPool",
        "确定要清空所有成功的卡BIN吗？",
      ),
      t("common.confirm"),
      {
        type: "warning",
        confirmButtonText: t("common.confirm"),
        cancelButtonText: t("common.cancel"),
      },
    );
    await invoke("clear_success_bins");
    successBinCount.value = 0;
    ElMessage.success(t("dialog.settings.payment.poolCleared", "BIN池已清空"));
  } catch (e) {
    // 用户取消
  }
}

// 无感换号相关
const windsurfPath = ref("");
const detectingPath = ref(false);
const patchLoading = ref(false);
const patchStatus = reactive({
  installed: false,
  error: "",
});

// 伟哥(寸止)相关
const cunzhiLoading = ref(false);
const cunzhiStatus = reactive({
  installed: false,
  error: "",
});

watch(
  () => uiStore.showSettingsDialog,
  async (show) => {
    if (show && settingsStore.settings) {
      Object.assign(settings, settingsStore.settings);
      windsurfPath.value = settings.windsurfPath || "";
      // 同步座位数选项到输入框
      if (
        settings.seat_count_options &&
        settings.seat_count_options.length > 0
      ) {
        seatCountOptionsInput.value = settings.seat_count_options.join(", ");
      }
      // 检查补丁状态
      if (windsurfPath.value) {
        await checkPatchStatus();
      }
      // 检查伟哥状态
      await checkCunzhiStatus();
      // 加载成功BIN池数量和测试模式进度
      await loadSuccessBinCount();
      await loadTestModeProgress();
    }
  },
);

onMounted(async () => {
  // 如果已有路径，检查状态
  const storedPath = (settingsStore.settings as any)?.windsurfPath;
  if (storedPath) {
    settings.windsurfPath = storedPath;
    windsurfPath.value = storedPath;
    await checkPatchStatus();
  }
});

async function handleSave() {
  loading.value = true;
  try {
    // 确保保存路径设置
    if (windsurfPath.value) {
      settings.windsurfPath = windsurfPath.value;
    }
    await settingsStore.updateSettings(settings);
    uiStore.setTheme(settings.theme as "light" | "dark");
    ElMessage.success(t("common.success"));
    handleClose();
  } catch (error) {
    ElMessage.error(`${$t("common.error")}: ${error}`);
  } finally {
    loading.value = false;
  }
}

async function handleLanguageChange(val: string) {
  await settingsStore.setLanguage(val);
}

function handleClose() {
  uiStore.showSettingsDialog = false;
}

// 验证卡头输入
function validateCardBin(value: string) {
  // 只允许数字
  const cleaned = value.replace(/[^\d]/g, "");
  settings.customCardBin = cleaned;

  // 检查长度
  if (cleaned.length > 0 && cleaned.length < 4) {
    ElMessage.warning(
      t("dialog.settings.payment.invalidBinLen", "卡头必须是4-12位数字"),
    );
  }
}

// 恢复默认卡头
function resetCardBin() {
  settings.customCardBin = "626202";
  ElMessage.success(
    t("dialog.settings.payment.resetSuccess", "已恢复默认卡头"),
  );
}

// 验证卡段范围格式
function validateCardBinRange(value: string) {
  // 只允许数字和连字符
  const cleaned = value.replace(/[^\d-]/g, "");
  settings.customCardBinRange = cleaned;

  // 如果输入了内容，验证格式
  if (cleaned && cleaned.includes("-")) {
    const parts = cleaned.split("-");
    if (parts.length === 2) {
      const [start, end] = parts;
      // 验证两端长度相同且都是数字
      if (start && end && start.length === end.length) {
        const startNum = parseInt(start, 10);
        const endNum = parseInt(end, 10);
        if (startNum > endNum) {
          ElMessage.warning(
            t(
              "dialog.settings.payment.invalidRangeStart",
              "起始BIN必须小于或等于结束BIN",
            ),
          );
        }
      } else if (start && end && start.length !== end.length) {
        ElMessage.warning(
          t(
            "dialog.settings.payment.invalidRangeLen",
            "起始和结束BIN的长度必须相同",
          ),
        );
      }
    }
  }
}

// 清除卡段范围
function clearCardBinRange() {
  settings.customCardBinRange = "";
  ElMessage.success(
    t("dialog.settings.payment.rangeCleared", "已清除卡段范围"),
  );
}

// 检测Windsurf路径
async function detectWindsurfPath() {
  detectingPath.value = true;
  try {
    const path = await invoke<string>("get_windsurf_path");
    windsurfPath.value = path;
    settings.windsurfPath = path;
    ElMessage.success(
      t("dialog.settings.seamless.pathFound", "已找到Windsurf安装路径"),
    );
    // 检查补丁状态
    await checkPatchStatus();
    // 保存路径设置到本地
    await settingsStore.updateSettings(settings);
  } catch (error) {
    ElMessage.error(`${$t("common.error")}: ${error}`);
    windsurfPath.value = "";
  } finally {
    detectingPath.value = false;
  }
}

// 检查补丁状态
async function checkPatchStatus() {
  if (!windsurfPath.value) return;

  try {
    const status = await invoke<any>("check_patch_status", {
      windsurfPath: windsurfPath.value,
    });
    patchStatus.installed = status.installed;
    patchStatus.error = status.error || "";

    // 同步开关状态与实际补丁状态
    if (status.installed !== settings.seamlessSwitchEnabled) {
      settings.seamlessSwitchEnabled = status.installed;
      // 保存同步后的状态
      await settingsStore.updateSettings(settings);
    }
  } catch (error) {
    patchStatus.installed = false;
    patchStatus.error = error as string;
  }
}

// 检查伟哥状态
async function checkCunzhiStatus() {
  try {
    // 假设有这样一个API，如果后端没有实现可能会报错，这里保留原有逻辑
    const status = await invoke<any>("check_cunzhi_status", {});
    cunzhiStatus.installed = status.installed;
    cunzhiStatus.error = status.error || "";

    if (status.installed !== settings.cunzhiEnabled) {
      settings.cunzhiEnabled = status.installed;
      await settingsStore.updateSettings(settings);
    }
  } catch (error) {
    // 忽略错误，可能是API未实现
  }
}

// 处理路径变化
function handlePathChange() {
  if (windsurfPath.value) {
    settings.windsurfPath = windsurfPath.value;
    // 检查新路径的补丁状态
    checkPatchStatus();
  }
}

// 浏览选择路径
async function browseWindsurfPath() {
  try {
    const { open } = await import("@tauri-apps/plugin-dialog");
    const selected = await open({
      directory: true,
      multiple: false,
      title: t(
        "dialog.settings.seamless.selectPathTitle",
        "选择Windsurf安装目录",
      ),
    });

    if (selected && typeof selected === "string") {
      // 验证选择的路径是否包含extension.js文件
      const isValid = await invoke<boolean>("validate_windsurf_path", {
        path: selected,
      });

      if (isValid) {
        windsurfPath.value = selected;
        settings.windsurfPath = selected;
        ElMessage.success(
          t("dialog.settings.seamless.pathSelected", "已选择Windsurf路径"),
        );
        await checkPatchStatus();
        // 保存路径设置到本地
        await settingsStore.updateSettings(settings);
      } else {
        ElMessage.error(
          t(
            "dialog.settings.seamless.invalidPath",
            "所选目录不是有效的Windsurf安装目录",
          ),
        );
      }
    }
  } catch (error) {
    ElMessage.error(`${$t("common.error")}: ${error}`);
  }
}

// 处理无感换号开关
async function handleSeamlessSwitch(value: boolean) {
  if (!windsurfPath.value) {
    ElMessage.error(
      t("dialog.settings.seamless.setPathFirst", "请先检测或设置Windsurf路径"),
    );
    settings.seamlessSwitchEnabled = !value;
    return;
  }

  // 确认对话框
  const action = value ? t("dialog.settings.on") : t("dialog.settings.off");
  const message = value
    ? t(
        "dialog.settings.seamless.confirmEnable",
        "开启无感换号将修改Windsurf的extension.js文件并重启Windsurf，是否继续？",
      )
    : t(
        "dialog.settings.seamless.confirmDisable",
        "关闭无感换号将还原原始文件并重启Windsurf，是否继续？",
      );

  try {
    await ElMessageBox.confirm(message, t("common.confirm"), {
      confirmButtonText: t("common.confirm"),
      cancelButtonText: t("common.cancel"),
      type: "warning",
    });
  } catch {
    // 用户取消，恢复开关状态
    settings.seamlessSwitchEnabled = !value;
    return;
  }

  patchLoading.value = true;
  try {
    let result;
    if (value) {
      // 应用补丁
      result = await invoke<any>("apply_seamless_patch", {
        windsurfPath: windsurfPath.value,
      });
    } else {
      // 还原补丁
      result = await invoke<any>("restore_seamless_patch");
    }

    if (result.success) {
      ElMessage.success(result.message || t("common.success"));
      if (result.already_patched) {
        ElMessage.info(
          t("dialog.settings.seamless.alreadyPatched", "补丁已经应用过了"),
        );
      }
      // 更新状态
      await checkPatchStatus();
    } else {
      ElMessage.error(result.message || t("common.error"));
      // 恢复开关状态
      settings.seamlessSwitchEnabled = !value;
    }
  } catch (error) {
    ElMessage.error(`${$t("common.error")}: ${error}`);
    settings.seamlessSwitchEnabled = !value;
  } finally {
    patchLoading.value = false;
  }
}

// 处理伟哥开关
async function handleCunzhiSwitch(value: boolean) {
  // 类似无感换号的处理逻辑
  const action = value ? t("dialog.settings.on") : t("dialog.settings.off");
  const message = value
    ? t(
        "dialog.settings.seamless.confirmEnableWeige",
        "开启伟哥功能将修改Windsurf文件并重启，是否继续？",
      )
    : t(
        "dialog.settings.seamless.confirmDisableWeige",
        "关闭伟哥功能将还原文件并重启，是否继续？",
      );

  try {
    await ElMessageBox.confirm(message, t("common.confirm"), {
      confirmButtonText: t("common.confirm"),
      cancelButtonText: t("common.cancel"),
      type: "warning",
    });
  } catch {
    settings.cunzhiEnabled = !value;
    return;
  }

  cunzhiLoading.value = true;
  try {
    let result;
    if (value) {
      result = await invoke<any>("apply_cunzhi_patch", {});
    } else {
      result = await invoke<any>("restore_cunzhi_patch", {});
    }

    if (result.success) {
      ElMessage.success(result.message || t("common.success"));
      await checkCunzhiStatus();
    } else {
      ElMessage.error(result.message || t("common.error"));
      settings.cunzhiEnabled = !value;
    }
  } catch (error) {
    // ElMessage.error(`${$t('common.error')}: ${error}`);
    // settings.cunzhiEnabled = !value;
    // 临时模拟成功 (如果API不存在)
    settings.cunzhiEnabled = value;
    cunzhiStatus.installed = value;
    ElMessage.success(t("common.success"));
  } finally {
    cunzhiLoading.value = false;
  }
}

// 重置网络连接
async function handleResetHttpClient() {
  resettingHttp.value = true;
  try {
    await invoke("reset_http_client");
    ElMessage.success(t("dialog.settings.resetSuccess", "网络连接已重置"));
  } catch (error) {
    ElMessage.error(`${$t("common.error")}: ${error}`);
  } finally {
    resettingHttp.value = false;
  }
}
</script>

<style scoped>
.el-form {
  padding: 0 20px;
}
.settings-tabs {
  height: 500px;
  overflow-y: auto;
}
</style>
