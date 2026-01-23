<template>
  <el-dialog
    v-model="uiStore.showSettingsDialog"
    :title="$t('settings.title')"
    width="700px"
  >
    <el-tabs v-model="activeTab" type="border-card">
      <!-- Basic Settings Tab -->
      <el-tab-pane :label="$t('dialog.settings.basic')" name="basic">
        <el-form :model="settings" label-width="140px">
          <el-form-item :label="$t('settings.language')">
            <el-select
              v-model="settings.language"
              @change="handleLanguageChange"
            >
              <el-option label="中文" value="zh" />
              <el-option label="English" value="en" />
              <el-option label="Français" value="fr" />
              <el-option label="Español" value="es" />
            </el-select>
          </el-form-item>

          <el-form-item :label="$t('settings.autoRefresh')">
            <el-switch v-model="settings.auto_refresh_token" />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.autoRefreshTokenDesc") }}
            </div>
          </el-form-item>

          <el-form-item
            :label="$t('dialog.settings.fullConcurrent')"
            v-if="settings.auto_refresh_token"
          >
            <el-switch v-model="settings.unlimitedConcurrentRefresh" />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.fullConcurrentDesc") }}
            </div>
          </el-form-item>

          <!-- 座位数选项 - simple 版本已禁用
          <el-form-item label="座位数选项">
            <el-input
              v-model="seatCountOptionsInput"
              placeholder="例如: 18, 19, 20"
              style="width: 200px;"
              @blur="parseSeatCountOptions"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px;">
              积分重置时轮番使用的座位数，用逗号分隔（如：18, 19, 20）
            </div>
          </el-form-item>
          -->

          <el-form-item :label="$t('settings.retryTimes')">
            <el-input-number
              v-model="settings.retry_times"
              :min="1"
              :max="5"
              :step="1"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.retryTimesDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('settings.concurrentLimit')">
            <el-input-number
              v-model="settings.concurrent_limit"
              :min="1"
              :max="10"
              :step="1"
              :disabled="settings.unlimitedConcurrentRefresh"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{
                settings.unlimitedConcurrentRefresh
                  ? $t("dialog.settings.fullConcurrentDesc")
                  : $t("dialog.settings.concurrencyLimitDesc")
              }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('settings.theme')">
            <el-radio-group v-model="settings.theme">
              <el-radio-button label="light">{{
                $t("dialog.settings.themeLight")
              }}</el-radio-button>
              <el-radio-button label="dark">{{
                $t("dialog.settings.themeDark")
              }}</el-radio-button>
            </el-radio-group>
          </el-form-item>

          <el-form-item :label="$t('settings.showSeatsResult')">
            <el-switch
              v-model="settings.show_seats_result_dialog"
              active-text="开启"
              inactive-text="关闭"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.showSeatsResultDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('dialog.settings.privacyMode')">
            <el-switch
              v-model="settings.privacyMode"
              active-text="开启"
              inactive-text="关闭"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.privacyModeDesc") }}
            </div>
          </el-form-item>

          <el-divider content-position="left">{{
            $t("dialog.settings.networkMaintenance")
          }}</el-divider>

          <el-form-item :label="$t('dialog.settings.lightweightApi')">
            <el-switch
              v-model="settings.useLightweightApi"
              active-text="开启"
              inactive-text="关闭"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.lightweightApiDesc") }}
            </div>
          </el-form-item>

          <el-form-item :label="$t('settings.proxy')">
            <el-switch
              v-model="settings.proxyEnabled"
              active-text="开启"
              inactive-text="关闭"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              {{ $t("dialog.settings.proxySettingsDesc") }}
            </div>
          </el-form-item>

          <el-form-item label="代理地址" v-if="settings.proxyEnabled">
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
              支持 HTTP/HTTPS/SOCKS5 代理，格式：http://host:port 或
              socks5://host:port
            </div>
          </el-form-item>

          <el-form-item label="重置网络连接">
            <el-button
              type="warning"
              @click="handleResetHttpClient"
              :loading="resettingHttp"
            >
              重置HTTP客户端
            </el-button>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              当遇到连续的API请求失败时，可点击此按钮重置网络连接池
            </div>
          </el-form-item>
        </el-form>
      </el-tab-pane>

      <!-- 支付设置标签页 -->
      <el-tab-pane label="支付设置" name="payment">
        <el-form :model="settings" label-width="140px">
          <el-form-item label="自动打开支付页面">
            <el-switch
              v-model="settings.autoOpenPaymentLinkInWebview"
              active-text="开启"
              inactive-text="关闭"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              开启后，获取绑卡链接成功时将自动在内置浏览器窗口中打开支付页面（隐私模式，不保存任何数据）
            </div>
          </el-form-item>

          <el-divider content-position="left">外部浏览器设置</el-divider>

          <el-form-item label="自动打开外部浏览器">
            <el-switch
              v-model="settings.autoOpenBrowser"
              active-text="开启"
              inactive-text="关闭"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              开启后，获取绑卡链接时将自动在外部浏览器中打开（无需点击确认）
            </div>
          </el-form-item>

          <el-form-item label="浏览器模式">
            <el-radio-group v-model="settings.browserMode">
              <el-radio-button label="incognito">无痕模式</el-radio-button>
              <el-radio-button label="normal">普通模式</el-radio-button>
            </el-radio-group>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              选择打开外部浏览器时使用的模式（无痕模式更安全，推荐使用）
            </div>
          </el-form-item>

          <el-divider content-position="left">自动填写设置</el-divider>

          <el-form-item label="自动填写支付表单">
            <el-switch
              v-model="settings.autoFillPaymentForm"
              active-text="开启"
              inactive-text="关闭"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              开启后，将自动使用虚拟卡信息填写Stripe支付表单（仅用于测试）
            </div>
          </el-form-item>

          <el-form-item label="显示虚拟卡信息">
            <el-switch
              v-model="settings.showVirtualCardInfo"
              active-text="开启"
              inactive-text="关闭"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              开启后，自动填写表单时会弹窗显示生成的虚拟卡信息
            </div>
          </el-form-item>

          <el-form-item label="自动提交表单">
            <el-switch
              v-model="settings.autoSubmitPaymentForm"
              active-text="开启"
              inactive-text="关闭"
              :disabled="!settings.autoFillPaymentForm"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              开启后，表单填写完成后将自动提交（谨慎使用）
            </div>
          </el-form-item>

          <el-form-item label="支付页面延迟(秒)">
            <el-input-number
              v-model="settings.paymentPageDelay"
              :min="1"
              :max="10"
              :step="1"
              :disabled="!settings.autoFillPaymentForm"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              等待多少秒后开始自动填写表单
            </div>
          </el-form-item>

          <el-form-item label="自定义卡头">
            <el-input
              v-model="settings.customCardBin"
              placeholder="请输入4-12位数字"
              maxlength="12"
              @input="validateCardBin"
            >
              <template #append>
                <el-button @click="resetCardBin">恢复默认</el-button>
              </template>
            </el-input>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              设置虚拟卡的前缀（BIN），必须是4-12位数字，默认为626202
            </div>
          </el-form-item>

          <el-form-item label="卡段范围（可选）">
            <el-input
              v-model="settings.customCardBinRange"
              placeholder="如：626200-626300"
              @input="validateCardBinRange"
            >
              <template #append>
                <el-button @click="clearCardBinRange">清除</el-button>
              </template>
            </el-input>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              设置卡段范围后，绑卡时将从范围内随机选择一个BIN。格式：起始BIN-结束BIN
            </div>
          </el-form-item>

          <el-form-item label="绑卡失败重试次数">
            <el-input-number
              v-model="settings.cardBindRetryTimes"
              :min="0"
              :max="20"
              :step="1"
              controls-position="right"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              绑卡失败后自动重新生成卡号重试的次数，设为0则不重试
            </div>
          </el-form-item>

          <el-divider content-position="left">卡BIN池功能</el-divider>

          <el-form-item label="测试模式">
            <div style="display: flex; align-items: center; gap: 10px">
              <el-switch v-model="settings.testModeEnabled" />
              <el-button
                size="small"
                type="warning"
                @click="resetTestModeProgress"
                :disabled="!testModeProgress"
              >
                重置进度
              </el-button>
            </div>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              开启后，按顺序遍历卡BIN范围，并收集成功的BIN（池数量：{{
                successBinCount
              }}）
              <span v-if="testModeProgress" style="color: #67c23a">
                <br />当前进度：{{ testModeProgress }}
              </span>
            </div>
          </el-form-item>

          <el-form-item label="使用本地BIN池">
            <el-switch
              v-model="settings.useLocalSuccessBins"
              :disabled="successBinCount === 0"
            />
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              开启后，自动从本地成功BIN池中随机获取卡BIN生成卡号
            </div>
          </el-form-item>

          <el-form-item label="BIN池管理">
            <el-button-group>
              <el-button
                size="small"
                @click="viewSuccessBins"
                :disabled="successBinCount === 0"
              >
                查看BIN池
              </el-button>
              <el-button
                size="small"
                type="danger"
                @click="clearSuccessBins"
                :disabled="successBinCount === 0"
              >
                清空BIN池
              </el-button>
            </el-button-group>
          </el-form-item>

          <el-alert
            title="重要提示"
            type="warning"
            :closable="false"
            show-icon
            style="margin-top: 20px"
          >
            <template #default>
              <div style="font-size: 12px; line-height: 1.6">
                <p>
                  🔒
                  内置浏览器使用隐私模式，不会保存任何浏览数据、Cookies或历史记录。
                </p>
                <p>⚠️ 虚拟卡信息生成功能仅用于测试目的，请勿用于实际支付。</p>
                <p>
                  ⚠️ 使用本功能时，请确保遵守Stripe及相关支付服务的使用条款。
                </p>
                <p>⚠️ 不要将生成的虚拟卡信息用于任何欺诈或非法用途。</p>
              </div>
            </template>
          </el-alert>
        </el-form>
      </el-tab-pane>

      <!-- 无感换号标签页 -->
      <el-tab-pane label="无感换号" name="seamless">
        <el-form :model="settings" label-width="140px">
          <el-form-item label="Windsurf路径">
            <el-input
              v-model="windsurfPath"
              placeholder="请输入或点击自动检测获取路径"
              @blur="handlePathChange"
            >
              <template #append>
                <el-button-group>
                  <el-button
                    @click="detectWindsurfPath"
                    :loading="detectingPath"
                  >
                    自动检测
                  </el-button>
                  <el-button @click="browseWindsurfPath"> 浏览 </el-button>
                </el-button-group>
              </template>
            </el-input>
            <div style="margin-top: 5px; color: #909399; font-size: 12px">
              可手动输入路径或从开始菜单自动检测Windsurf安装路径
            </div>
          </el-form-item>

          <el-form-item label="启用无感换号">
            <el-switch
              v-model="settings.seamlessSwitchEnabled"
              active-text="开启"
              inactive-text="关闭"
              :loading="patchLoading"
              @change="handleSeamlessSwitch"
              :disabled="!windsurfPath"
            />
          </el-form-item>

          <el-form-item label="补丁状态">
            <el-tag v-if="patchStatus.installed" type="success">已安装</el-tag>
            <el-tag v-else-if="patchStatus.error" type="danger">{{
              patchStatus.error
            }}</el-tag>
            <el-tag v-else type="info">未安装</el-tag>
            <el-button
              v-if="patchStatus.installed"
              size="small"
              style="margin-left: 10px"
              @click="checkPatchStatus"
            >
              重新检测
            </el-button>
          </el-form-item>

          <el-alert
            title="功能说明"
            type="info"
            :closable="false"
            show-icon
            style="margin-top: 20px"
          >
            <template #default>
              <div style="font-size: 12px; line-height: 1.6">
                <p>🚀 无感换号功能：实现 Windsurf 账号无感切换</p>
                <p>⚠️ 注意：开启/关闭时会自动重启 Windsurf</p>
              </div>
            </template>
          </el-alert>

          <el-divider content-position="left">Windsurf 伟哥</el-divider>

          <el-form-item label="启用伟哥功能">
            <el-switch
              v-model="settings.cunzhiEnabled"
              active-text="开启"
              inactive-text="关闭"
              :loading="cunzhiLoading"
              @change="handleCunzhiSwitch"
            />
          </el-form-item>

          <el-form-item label="寸止状态">
            <el-tag v-if="cunzhiStatus.installed" type="success">已安装</el-tag>
            <el-tag v-else-if="cunzhiStatus.error" type="danger">{{
              cunzhiStatus.error
            }}</el-tag>
            <el-tag v-else type="info">未安装</el-tag>
            <el-button
              v-if="cunzhiStatus.installed"
              size="small"
              style="margin-left: 10px"
              @click="checkCunzhiStatus"
            >
              重新检测
            </el-button>
          </el-form-item>

          <el-alert
            title="伟哥功能说明"
            type="success"
            :closable="false"
            show-icon
            style="margin-top: 10px"
          >
            <template #default>
              <div style="font-size: 12px; line-height: 1.6">
                <p>💊 伟哥功能：防止 AI 擅自结束对话，让你掌控对话节奏</p>
                <p>⚠️ 注意：开启/关闭后需要重启 Windsurf 生效</p>
              </div>
            </template>
          </el-alert>
        </el-form>
      </el-tab-pane>
    </el-tabs>

    <template #footer>
      <el-button @click="handleClose">取消</el-button>
      <el-button type="primary" @click="handleSave" :loading="loading">
        保存
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, watch, onMounted } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import { Connection } from "@element-plus/icons-vue";
import { useSettingsStore, useUIStore } from "@/store";
import { invoke } from "@tauri-apps/api/core";
import { systemApi } from "@/api";

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
    ElMessage.warning("请输入有效的座位数");
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
      "确定要重置测试模式进度吗？下次将从范围起始位置开始。",
      "确认重置",
      {
        type: "warning",
      },
    );
    await invoke("reset_test_mode_progress");
    testModeProgress.value = null;
    ElMessage.success("进度已重置");
  } catch (e) {
    // 用户取消
  }
}

async function viewSuccessBins() {
  try {
    const bins = await invoke<string[]>("get_success_bins");
    if (bins.length === 0) {
      ElMessage.info("BIN池为空");
      return;
    }
    ElMessageBox.alert(
      `<div style="max-height: 300px; overflow-y: auto;">
        <p><b>共 ${bins.length} 个成功BIN：</b></p>
        <p style="font-family: monospace; word-break: break-all;">${bins.join(", ")}</p>
      </div>`,
      "成功BIN池",
      { dangerouslyUseHTMLString: true },
    );
  } catch (e) {
    ElMessage.error("获取BIN池失败");
  }
}

async function clearSuccessBins() {
  try {
    await ElMessageBox.confirm("确定要清空所有成功的卡BIN吗？", "确认清空", {
      type: "warning",
    });
    await invoke("clear_success_bins");
    successBinCount.value = 0;
    ElMessage.success("BIN池已清空");
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
    ElMessage.success("设置保存成功");
    handleClose();
  } catch (error) {
    ElMessage.error(`保存失败: ${error}`);
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
    ElMessage.warning("卡头必须是4-12位数字");
  }
}

// 恢复默认卡头
function resetCardBin() {
  settings.customCardBin = "626202";
  ElMessage.success("已恢复默认卡头");
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
          ElMessage.warning("起始BIN必须小于或等于结束BIN");
        }
      } else if (start && end && start.length !== end.length) {
        ElMessage.warning("起始和结束BIN的长度必须相同");
      }
    }
  }
}

// 清除卡段范围
function clearCardBinRange() {
  settings.customCardBinRange = "";
  ElMessage.success("已清除卡段范围");
}

// 检测Windsurf路径
async function detectWindsurfPath() {
  detectingPath.value = true;
  try {
    const path = await invoke<string>("get_windsurf_path");
    windsurfPath.value = path;
    settings.windsurfPath = path;
    ElMessage.success("已找到Windsurf安装路径");
    // 检查补丁状态
    await checkPatchStatus();
    // 保存路径设置到本地
    await settingsStore.updateSettings(settings);
  } catch (error) {
    ElMessage.error(`检测失败: ${error}`);
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
      title: "选择Windsurf安装目录",
    });

    if (selected && typeof selected === "string") {
      // 验证选择的路径是否包含extension.js文件
      const isValid = await invoke<boolean>("validate_windsurf_path", {
        path: selected,
      });

      if (isValid) {
        windsurfPath.value = selected;
        settings.windsurfPath = selected;
        ElMessage.success("已选择Windsurf路径");
        await checkPatchStatus();
        // 保存路径设置到本地
        await settingsStore.updateSettings(settings);
      } else {
        ElMessage.error("所选目录不是有效的Windsurf安装目录");
      }
    }
  } catch (error) {
    ElMessage.error(`选择路径失败: ${error}`);
  }
}

// 处理无感换号开关
async function handleSeamlessSwitch(value: boolean) {
  if (!windsurfPath.value) {
    ElMessage.error("请先检测或设置Windsurf路径");
    settings.seamlessSwitchEnabled = !value;
    return;
  }

  // 确认对话框
  const action = value ? "开启" : "关闭";
  const message = value
    ? "开启无感换号将修改Windsurf的extension.js文件并重启Windsurf，是否继续？"
    : "关闭无感换号将还原原始文件并重启Windsurf，是否继续？";

  try {
    await ElMessageBox.confirm(message, `${action}无感换号`, {
      confirmButtonText: "确定",
      cancelButtonText: "取消",
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
      ElMessage.success(result.message || `无感换号已${action}`);
      if (result.already_patched) {
        ElMessage.info("补丁已经应用过了");
      }
      // 更新状态
      await checkPatchStatus();
      // 保存设置到本地
      settings.windsurfPath = windsurfPath.value;
      settings.patchBackupPath = result.backup_file || settings.patchBackupPath;
      // 立即保存到本地文件
      await settingsStore.updateSettings(settings);
    } else {
      ElMessage.error(result.message || `${action}失败`);
      settings.seamlessSwitchEnabled = !value;
    }
  } catch (error) {
    ElMessage.error(`${action}失败: ${error}`);
    settings.seamlessSwitchEnabled = !value;
  } finally {
    patchLoading.value = false;
  }
}

// 重置HTTP客户端
async function handleResetHttpClient() {
  resettingHttp.value = true;
  try {
    const result = await systemApi.resetHttpClient();
    if (result.success) {
      ElMessage.success(result.message || "HTTP客户端已重置");
    } else {
      ElMessage.error("重置失败");
    }
  } catch (error) {
    ElMessage.error(`重置失败: ${error}`);
  } finally {
    resettingHttp.value = false;
  }
}

// 检查伟哥(寸止)状态
async function checkCunzhiStatus() {
  try {
    const status = await invoke<any>("check_cunzhi_status");
    cunzhiStatus.installed = status.installed;
    cunzhiStatus.error = status.error || "";

    // 同步开关状态与实际状态
    if (status.installed !== settings.cunzhiEnabled) {
      settings.cunzhiEnabled = status.installed;
      await settingsStore.updateSettings(settings);
    }
  } catch (error) {
    cunzhiStatus.installed = false;
    cunzhiStatus.error = error as string;
  }
}

// 处理伟哥开关
async function handleCunzhiSwitch(value: boolean) {
  const action = value ? "开启" : "关闭";
  const message = value
    ? "开启伟哥功能将安装 MCP 服务器和全局规则，是否继续？"
    : "关闭伟哥功能将删除 MCP 配置和全局规则，是否继续？";

  try {
    await ElMessageBox.confirm(message, `${action}伟哥功能`, {
      confirmButtonText: "确定",
      cancelButtonText: "取消",
      type: "warning",
    });
  } catch {
    // 用户取消，恢复开关状态
    settings.cunzhiEnabled = !value;
    return;
  }

  cunzhiLoading.value = true;
  try {
    let result;
    if (value) {
      // 安装伟哥
      result = await invoke<any>("install_cunzhi", {
        windsurfPath: settings.windsurfPath || null,
      });
    } else {
      // 卸载伟哥
      result = await invoke<any>("uninstall_cunzhi", {
        windsurfPath: settings.windsurfPath || null,
      });
    }

    if (result.success) {
      ElMessage.success(result.message || `伟哥功能已${action}`);
      // 更新状态
      await checkCunzhiStatus();
      // 保存设置
      await settingsStore.updateSettings(settings);
      // 提示重启
      ElMessage.warning("请重启 Windsurf 以使更改生效");
    } else {
      ElMessage.error(result.message || `${action}失败`);
      settings.cunzhiEnabled = !value;
    }
  } catch (error) {
    ElMessage.error(`${action}失败: ${error}`);
    settings.cunzhiEnabled = !value;
  } finally {
    cunzhiLoading.value = false;
  }
}

// simple 版本已禁用的功能
void parseSeatCountOptions;
</script>

<style scoped>
/* 深色模式样式 */
:deep(.el-dialog) {
  /* 在深色模式下由全局样式控制 */
}

/* 深色模式下的描述文字 */
:root.dark .el-form-item > div[style*="color: #909399"] {
  color: #94a3b8 !important;
}

/* 深色模式下的标签页内容 */
:root.dark .el-tabs__content {
  background-color: transparent;
}

/* 深色模式下的表单项标签 */
:root.dark .el-form-item__label {
  color: #cfd3dc;
}

/* 深色模式下的alert */
:root.dark .el-alert--warning {
  background-color: rgba(230, 162, 60, 0.1);
  border-color: rgba(230, 162, 60, 0.3);
}

:root.dark .el-alert--warning .el-alert__description {
  color: #cfd3dc;
}
</style>
