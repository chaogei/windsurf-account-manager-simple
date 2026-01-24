<script setup lang="ts">
import { onMounted, onUnmounted, computed, ref, watch } from "vue";
import { ElConfigProvider } from "element-plus";
// @ts-ignore
import zhCn from "element-plus/dist/locale/zh-cn.mjs";
// @ts-ignore
import en from "element-plus/dist/locale/en.mjs";
// @ts-ignore
import fr from "element-plus/dist/locale/fr.mjs";
// @ts-ignore
import es from "element-plus/dist/locale/es.mjs";
import { useAccountsStore, useSettingsStore, useUIStore } from "./store";
import { useI18n } from "vue-i18n";
import MainLayout from "./views/MainLayout.vue";
import WelcomeDialog from "./components/WelcomeDialog.vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

const accountsStore = useAccountsStore();
const settingsStore = useSettingsStore();
const uiStore = useUIStore();
const { locale } = useI18n();

// Element Plus locale map
const epLocaleMap = {
  zh: zhCn,
  en: en,
  fr: fr,
  es: es,
};

const showWelcomeDialog = ref(true);

// 事件监听取消函数
let tokenRefreshedUnlisten: UnlistenFn | null = null;

// 用于Element Plus的命名空间，支持深色模式
const elNamespace = computed(() => "el");

// 禁用右键菜单
const disableContextMenu = (e: MouseEvent) => {
  e.preventDefault();
  return false;
};

// 禁用调试快捷键
const disableDebugKeys = (e: KeyboardEvent) => {
  // 禁用F12
  if (e.key === "F12") {
    e.preventDefault();
    return false;
  }

  // 禁用Ctrl+Shift+I (开发者工具)
  if (e.ctrlKey && e.shiftKey && e.key === "I") {
    e.preventDefault();
    return false;
  }

  // 禁用Ctrl+Shift+J (控制台)
  if (e.ctrlKey && e.shiftKey && e.key === "J") {
    e.preventDefault();
    return false;
  }

  // 禁用Ctrl+Shift+C (审查元素)
  if (e.ctrlKey && e.shiftKey && e.key === "C") {
    e.preventDefault();
    return false;
  }

  // 禁用Ctrl+U (查看源代码)
  if (e.ctrlKey && e.key === "u") {
    e.preventDefault();
    return false;
  }

  // 禁用Ctrl+S (保存页面)
  if (e.ctrlKey && e.key === "s") {
    e.preventDefault();
    return false;
  }

  return true;
};

onMounted(async () => {
  // 禁用右键菜单
  document.addEventListener("contextmenu", disableContextMenu);

  // 禁用调试快捷键
  document.addEventListener("keydown", disableDebugKeys);

  // 获取并设置应用标题（包含版本号）
  try {
    const title = await invoke<string>("get_app_title");
    document.title = title;
  } catch (error) {
    console.error("Failed to get app title:", error);
  }

  // 初始化应用数据
  await Promise.all([accountsStore.loadAccounts(), settingsStore.initialize()]);

  // 如果设置中有主题且与当前不同，则应用设置中的主题
  const settingsTheme = settingsStore.settings.theme;
  if (settingsTheme && settingsTheme !== uiStore.theme) {
    uiStore.setTheme(settingsTheme as "light" | "dark");
  } else {
    // 确保当前主题被应用
    uiStore.setTheme(uiStore.theme);
  }

  // 启动自动刷新Token功能
  accountsStore.startAutoRefreshTimer(settingsStore);

  // 监听后端 token 刷新事件，自动更新前端账户数据
  tokenRefreshedUnlisten = await listen<{
    account_id: string;
    token: string;
    token_expires_at: string;
  }>("token-refreshed", (event) => {
    const { account_id, token, token_expires_at } = event.payload;
    console.log("[Token刷新事件] 后端已刷新账户 token:", account_id);

    // 更新对应账户的 token 和过期时间
    const idx = accountsStore.accounts.findIndex(
      (acc) => acc.id === account_id,
    );
    if (idx !== -1) {
      const updatedAccount = {
        ...accountsStore.accounts[idx],
        token,
        token_expires_at,
        status: "active" as const,
      };
      accountsStore.accounts.splice(idx, 1, updatedAccount);
      console.log("[Token刷新事件] 已更新账户:", updatedAccount.email);
    }
  });

  // Watch for language changes to update Element Plus locale
  watch(
    () => settingsStore.settings.language,
    (newLang) => {
      if (newLang) {
        locale.value = newLang;
      }
    },
    { immediate: true },
  );
});

// 组件卸载时停止定时器和移除事件监听
onUnmounted(() => {
  accountsStore.stopAutoRefreshTimer();
  document.removeEventListener("contextmenu", disableContextMenu);
  document.removeEventListener("keydown", disableDebugKeys);
  // 取消 Tauri 事件监听
  if (tokenRefreshedUnlisten) {
    tokenRefreshedUnlisten();
    tokenRefreshedUnlisten = null;
  }
});
</script>

<template>
  <el-config-provider
    :locale="epLocaleMap[locale as keyof typeof epLocaleMap] || zhCn"
    :namespace="elNamespace"
  >
    <MainLayout />
    <WelcomeDialog v-model="showWelcomeDialog" />
  </el-config-provider>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

#app {
  font-family:
    "Microsoft YaHei",
    "微软雅黑",
    -apple-system,
    BlinkMacSystemFont,
    "Segoe UI",
    Roboto,
    Oxygen,
    Ubuntu,
    Cantarell,
    "Fira Sans",
    "Droid Sans",
    "Helvetica Neue",
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
}

::-webkit-scrollbar-thumb {
  background: #888;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #555;
}

/* Element Plus 样式覆盖 */
.el-button-group {
  display: flex;
}

/* 调整消息提示位置，避免遮挡顶部按钮 */
.el-message {
  top: 60px !important;
}

/* 深色模式样式 */
html.dark {
  background-color: #0c0d0e;
  color-scheme: dark;
}

html.dark body {
  background-color: #0c0d0e;
  color: #cfd3dc;
}

/* Element Plus 深色模式对话框 */
html.dark .el-dialog {
  background-color: #1d1e1f !important;
  border: 1px solid #4c4d4f;
}

html.dark .el-dialog__wrapper {
  background-color: rgba(0, 0, 0, 0.7);
}

html.dark .el-dialog__header {
  border-bottom: 1px solid #4c4d4f;
  background-color: #1d1e1f !important;
}

html.dark .el-dialog__title {
  color: #e5eaf3;
}

html.dark .el-dialog__body {
  color: #cfd3dc;
  background-color: #1d1e1f !important;
}

html.dark .el-dialog__footer {
  background-color: #1d1e1f !important;
  border-top: 1px solid #4c4d4f;
}

/* 对话框内的表单和标签页 */
html.dark .el-dialog .el-form {
  background-color: transparent;
}

html.dark .el-dialog .el-tabs__header {
  background-color: #1d1e1f;
}

html.dark .el-dialog .el-tabs__content {
  background-color: #1d1e1f;
  padding: 20px;
}

html.dark .el-dialog .el-tabs__nav-wrap {
  background-color: transparent;
}

html.dark .el-dialog .el-tabs__item {
  color: #cfd3dc;
}

html.dark .el-dialog .el-tabs__item.is-active {
  color: #409eff;
  background-color: transparent;
}

/* 对话框内的描述文字 */
html.dark .el-dialog .el-form-item > div {
  color: #94a3b8 !important;
}

html.dark .el-table {
  background-color: #1d1e1f !important;
  color: #cfd3dc;
}

html.dark .el-table__header-wrapper {
  background-color: #262729 !important;
}

html.dark .el-table th.el-table__cell {
  background-color: #262729 !important;
  color: #e5eaf3;
  border-bottom: 1px solid #4c4d4f;
}

html.dark .el-table tr {
  background-color: #1d1e1f !important;
}

html.dark .el-table td.el-table__cell {
  border-bottom: 1px solid #4c4d4f;
  color: #cfd3dc;
}

html.dark .el-table__empty-block {
  background-color: #1d1e1f !important;
}

html.dark .el-input__wrapper {
  background-color: #262729 !important;
  border-color: #4c4d4f;
}

html.dark .el-input__inner {
  background-color: transparent !important;
  color: #cfd3dc;
}

html.dark .el-select-dropdown {
  background-color: #1d1e1f !important;
  border: 1px solid #4c4d4f;
}

html.dark .el-select-dropdown__item {
  color: #cfd3dc;
}

html.dark .el-select-dropdown__item:hover {
  background-color: #262729;
}

html.dark .el-select-dropdown__item.selected {
  color: #409eff;
}

html.dark .el-popper {
  background-color: #1d1e1f !important;
  border: 1px solid #4c4d4f;
  color: #cfd3dc;
}

html.dark .el-form-item__label {
  color: #cfd3dc;
}

html.dark .el-checkbox__inner {
  background-color: #262729;
  border-color: #4c4d4f;
}

html.dark .el-radio__inner {
  background-color: #262729;
  border-color: #4c4d4f;
}

html.dark .el-switch__core {
  background-color: #4c4d4f !important;
  border-color: #4c4d4f !important;
}

html.dark .el-switch.is-checked .el-switch__core {
  background-color: #409eff !important;
  border-color: #409eff !important;
}

html.dark .el-switch__core .el-switch__inner {
  color: #fff !important;
}

html.dark .el-switch__core .el-switch__action {
  background-color: #fff !important;
}

/* 确保开关在禁用状态下也可见 */
html.dark .el-switch.is-disabled .el-switch__core {
  background-color: #303133 !important;
  border-color: #303133 !important;
}

html.dark .el-switch.is-disabled.is-checked .el-switch__core {
  background-color: rgba(64, 158, 255, 0.5) !important;
  border-color: rgba(64, 158, 255, 0.5) !important;
}

html.dark .el-message-box {
  background-color: #1d1e1f !important;
  border: 1px solid #4c4d4f;
}

html.dark .el-message-box__header {
  background-color: #1d1e1f !important;
}

html.dark .el-message-box__title {
  color: #e5eaf3;
}

html.dark .el-message-box__content {
  color: #cfd3dc;
}

html.dark .el-tabs__nav-wrap::after {
  background-color: #4c4d4f;
}

html.dark .el-tabs__active-bar {
  background-color: #409eff;
}

html.dark .el-tabs__item {
  color: #cfd3dc;
}

html.dark .el-tabs__item.is-active {
  color: #409eff;
}

html.dark .el-descriptions {
  background-color: #1d1e1f;
}

html.dark .el-descriptions__body {
  background-color: #1d1e1f;
}

html.dark .el-descriptions__cell {
  border-color: #4c4d4f !important;
}

html.dark .el-descriptions__label {
  background-color: #262729;
  color: #cfd3dc;
}

html.dark .el-descriptions__content {
  background-color: #1d1e1f;
  color: #e5eaf3;
}

/* 深色模式滚动条 */
html.dark ::-webkit-scrollbar-track {
  background: #262729;
}

html.dark ::-webkit-scrollbar-thumb {
  background: #4c4d4f;
}

html.dark ::-webkit-scrollbar-thumb:hover {
  background: #5a5b5d;
}

/* 深色模式输入框数字步进器 */
html.dark .el-input-number__decrease,
html.dark .el-input-number__increase {
  background-color: #262729 !important;
  color: #cfd3dc;
  border-color: #4c4d4f;
}

html.dark .el-input-number__decrease:hover,
html.dark .el-input-number__increase:hover {
  background-color: #303133 !important;
  color: #409eff;
}

/* 深色模式单选按钮组 */
html.dark .el-radio-button__inner {
  background-color: #262729 !important;
  color: #cfd3dc;
  border-color: #4c4d4f;
}

html.dark .el-radio-button__original-radio:checked + .el-radio-button__inner {
  background-color: #409eff !important;
  border-color: #409eff !important;
  color: #fff !important;
}

/* 深色模式标签页 */
html.dark .el-tag {
  background-color: #262729;
  border-color: #4c4d4f;
  color: #cfd3dc;
}

html.dark .el-tag--primary {
  background-color: rgba(64, 158, 255, 0.1);
  border-color: rgba(64, 158, 255, 0.2);
  color: #409eff;
}

html.dark .el-tag--success {
  background-color: rgba(103, 194, 58, 0.1);
  border-color: rgba(103, 194, 58, 0.2);
  color: #67c23a;
}

html.dark .el-tag--warning {
  background-color: rgba(230, 162, 60, 0.1);
  border-color: rgba(230, 162, 60, 0.2);
  color: #e6a23c;
}

html.dark .el-tag--danger {
  background-color: rgba(245, 108, 108, 0.1);
  border-color: rgba(245, 108, 108, 0.2);
  color: #f56c6c;
}

html.dark .el-tag--info {
  background-color: rgba(144, 147, 153, 0.1);
  border-color: rgba(144, 147, 153, 0.2);
  color: #909399;
}

/* 深色模式Alert */
html.dark .el-alert {
  background-color: #262729;
  border-color: #4c4d4f;
}

html.dark .el-alert--info {
  background-color: rgba(144, 147, 153, 0.1);
  border-color: rgba(144, 147, 153, 0.3);
}

html.dark .el-alert__title {
  color: #e5eaf3;
}

html.dark .el-alert__description {
  color: #cfd3dc;
}

/* 深色模式按钮 - 全面覆盖所有按钮类型 */
html.dark .el-button--default {
  background-color: #262729;
  border-color: #4c4d4f;
  color: #cfd3dc;
}

html.dark .el-button--default:hover {
  background-color: #303133;
  border-color: #5a5b5d;
  color: #409eff;
}

html.dark .el-button--primary {
  background-color: #409eff;
  border-color: #409eff;
  color: #fff;
}

html.dark .el-button--primary:hover {
  background-color: #66b1ff;
  border-color: #66b1ff;
  color: #fff;
}

html.dark .el-button--success {
  background-color: #67c23a;
  border-color: #67c23a;
  color: #fff;
}

html.dark .el-button--success:hover {
  background-color: #85ce61;
  border-color: #85ce61;
  color: #fff;
}

html.dark .el-button--warning {
  background-color: #e6a23c;
  border-color: #e6a23c;
  color: #fff;
}

html.dark .el-button--warning:hover {
  background-color: #ebb563;
  border-color: #ebb563;
  color: #fff;
}

html.dark .el-button--danger {
  background-color: #f56c6c;
  border-color: #f56c6c;
  color: #fff;
}

html.dark .el-button--danger:hover {
  background-color: #f78989;
  border-color: #f78989;
  color: #fff;
}

html.dark .el-button--info {
  background-color: #909399;
  border-color: #909399;
  color: #fff;
}

html.dark .el-button--info:hover {
  background-color: #a6a9ad;
  border-color: #a6a9ad;
  color: #fff;
}

/* plain按钮样式 */
html.dark .el-button--primary.is-plain {
  background-color: rgba(64, 158, 255, 0.1);
  border-color: rgba(64, 158, 255, 0.5);
  color: #409eff;
}

html.dark .el-button--primary.is-plain:hover {
  background-color: #409eff;
  border-color: #409eff;
  color: #fff;
}

html.dark .el-button--danger.is-plain {
  background-color: rgba(245, 108, 108, 0.1);
  border-color: rgba(245, 108, 108, 0.5);
  color: #f56c6c;
}

html.dark .el-button--danger.is-plain:hover {
  background-color: #f56c6c;
  border-color: #f56c6c;
  color: #fff;
}

/* text按钮样式 */
html.dark .el-button--text {
  background-color: transparent;
  border-color: transparent;
  color: #409eff;
}

html.dark .el-button--text:hover {
  background-color: transparent;
  border-color: transparent;
  color: #66b1ff;
}

html.dark .el-button--text:active {
  background-color: transparent;
  border-color: transparent;
  color: #3a8ee6;
}

/* disabled按钮样式 */
html.dark .el-button.is-disabled,
html.dark .el-button.is-disabled:hover {
  background-color: #303133;
  border-color: #4c4d4f;
  color: #5a5b5d;
  cursor: not-allowed;
}

/* 圆形按钮 */
html.dark .el-button.is-circle {
  background-color: #262729;
  border-color: #4c4d4f;
  color: #cfd3dc;
}

html.dark .el-button.is-circle:hover {
  background-color: #303133;
  border-color: #5a5b5d;
  color: #409eff;
}

/* loading按钮 */
html.dark .el-button.is-loading {
  opacity: 0.7;
}

/* 按钮组 */
html.dark .el-button-group .el-button {
  border-left-color: #4c4d4f;
  border-right-color: #4c4d4f;
}

html.dark .el-button-group .el-button:first-child {
  border-left-color: #4c4d4f;
}

html.dark .el-button-group .el-button:last-child {
  border-right-color: #4c4d4f;
}

/* 深色模式下拉菜单 */
html.dark .el-dropdown-menu {
  background-color: #1d1e1f !important;
  border: 1px solid #4c4d4f;
}

html.dark .el-dropdown-menu__item {
  color: #cfd3dc;
}

html.dark .el-dropdown-menu__item:hover {
  background-color: #262729;
  color: #409eff;
}

/* 深色模式菜单 */
html.dark .el-menu {
  background-color: #1e1e1e !important;
  border-right-color: rgba(255, 255, 255, 0.08) !important;
}

html.dark .el-menu-item {
  background-color: transparent !important;
  color: #cfd3dc !important;
}

html.dark .el-menu-item:hover {
  background-color: rgba(255, 255, 255, 0.05) !important;
  color: #409eff !important;
}

html.dark .el-menu-item.is-active {
  background-color: rgba(64, 158, 255, 0.1) !important;
  color: #409eff !important;
}

html.dark .el-menu-item i {
  color: inherit !important;
}

/* 深色模式下的复选框组 */
html.dark .el-checkbox-group {
  color: #cfd3dc;
}

html.dark .el-checkbox__label {
  color: #cfd3dc !important;
}

html.dark .el-checkbox.is-checked .el-checkbox__label {
  color: #409eff !important;
}

/* 深色模式下的tooltip */
html.dark .el-tooltip__popper {
  background-color: #303133 !important;
  border: 1px solid #4c4d4f;
  color: #cfd3dc !important;
}

html.dark
  .el-tooltip__popper[data-popper-placement^="top"]
  .el-tooltip__popper-arrow::before {
  background-color: #303133 !important;
  border-color: #4c4d4f !important;
}

/* 深色模式下的输入框append按钮 */
html.dark .el-input-group__append {
  background-color: #262729 !important;
  border-color: #4c4d4f !important;
  color: #cfd3dc !important;
}

html.dark .el-input-group__append .el-button {
  background-color: transparent !important;
  border: none !important;
  color: #409eff !important;
}

html.dark .el-input-group__append .el-button:hover {
  color: #66b1ff !important;
}

/* 深色模式下的卡片 - 增强优先级 */
html.dark .el-card {
  background-color: #1d1e1f !important;
  border-color: #4c4d4f !important;
  color: #cfd3dc;
}

html.dark .el-card__header {
  background-color: #262729 !important;
  border-bottom-color: #4c4d4f !important;
  color: #e5eaf3;
}

html.dark .el-card__body {
  background-color: #1d1e1f !important;
  color: #cfd3dc;
}

/* 嵌套卡片的深色模式 */
html.dark .el-card .el-card {
  background-color: #262729 !important;
}

html.dark .el-card .el-card__body {
  background-color: #262729 !important;
}

/* 对话框内的卡片 */
html.dark .el-dialog .el-card {
  background-color: #262729 !important;
}

html.dark .el-dialog .el-card__body {
  background-color: #262729 !important;
}

/* 席位统计卡片 */
html.dark .el-col .el-card {
  background-color: #262729 !important;
}

html.dark .el-col .el-card__body {
  background-color: #262729 !important;
}

/* 深色模式下的进度条 */
html.dark .el-progress {
  background-color: transparent;
}

html.dark .el-progress-bar__outer {
  background-color: #262729 !important;
}

html.dark .el-progress-bar__inner {
  background-color: #409eff !important;
}

html.dark .el-progress__text {
  color: #cfd3dc !important;
}

/* 深色模式下的时间线 */
html.dark .el-timeline-item__node {
  background-color: #262729 !important;
  border-color: #4c4d4f !important;
}

html.dark .el-timeline-item__wrapper {
  color: #cfd3dc;
}

html.dark .el-timeline-item__content {
  color: #cfd3dc;
}

html.dark .el-timeline-item__timestamp {
  color: #94a3b8;
}

/* 深色模式下的分割线 */
html.dark .el-divider {
  background-color: #4c4d4f !important;
}

html.dark .el-divider__text {
  background-color: #1d1e1f !important;
  color: #94a3b8;
}

/* 深色模式下的统计数值 */
html.dark .el-statistic {
  color: #cfd3dc;
}

html.dark .el-statistic__head {
  color: #94a3b8;
}

html.dark .el-statistic__content {
  color: #e5eaf3;
}

html.dark .el-statistic__value {
  color: #e5eaf3;
}

/* 深色模式下的结果页 */
html.dark .el-result {
  background-color: transparent;
}

html.dark .el-result__title {
  color: #e5eaf3;
}

html.dark .el-result__subtitle {
  color: #cfd3dc;
}

/* 深色模式下的空状态 */
html.dark .el-empty {
  background-color: transparent;
}

html.dark .el-empty__description {
  color: #94a3b8;
}

/* 深色模式下的骨架屏 */
html.dark .el-skeleton__item {
  background-color: #262729 !important;
}

/* 深色模式下的加载 */
html.dark .el-loading-mask {
  background-color: rgba(0, 0, 0, 0.8) !important;
}

html.dark .el-loading-spinner .circular circle {
  stroke: #409eff !important;
}

html.dark .el-loading-text {
  color: #cfd3dc !important;
}

/* 深色模式下的步骤条 */
html.dark .el-steps {
  background-color: transparent;
}

html.dark .el-step__head {
  color: #94a3b8;
}

html.dark .el-step__title {
  color: #cfd3dc;
}

html.dark .el-step__description {
  color: #94a3b8;
}

html.dark .el-step__icon {
  background-color: #262729;
  border-color: #4c4d4f;
  color: #94a3b8;
}

html.dark .el-step.is-finish .el-step__icon {
  background-color: #409eff;
  border-color: #409eff;
  color: #fff;
}

html.dark .el-step.is-process .el-step__icon {
  background-color: #409eff;
  border-color: #409eff;
  color: #fff;
}

/* 深色模式下的分页 */
html.dark .el-pagination {
  color: #cfd3dc;
}

html.dark .el-pager li {
  background-color: #262729 !important;
  color: #cfd3dc !important;
}

html.dark .el-pager li:hover {
  color: #409eff !important;
}

html.dark .el-pager li.is-active {
  background-color: #409eff !important;
  color: #fff !important;
}

html.dark .el-pagination__total {
  color: #cfd3dc !important;
}

html.dark .el-pagination__jump {
  color: #cfd3dc !important;
}

/* 深色模式下的弹出确认框 */
html.dark .el-popconfirm__main {
  color: #cfd3dc !important;
}

/* 深色模式下自定义的信息块 */
html.dark .info-card,
html.dark .info-block,
html.dark .data-card {
  background-color: #262729 !important;
  border: 1px solid #4c4d4f !important;
  color: #cfd3dc !important;
}

html.dark .info-card .label,
html.dark .info-block .label,
html.dark .data-card .label {
  color: #94a3b8 !important;
}

html.dark .info-card .value,
html.dark .info-block .value,
html.dark .data-card .value {
  color: #e5eaf3 !important;
}

/* 深色模式下的列表项 */
html.dark .list-item {
  background-color: #262729 !important;
  border-color: #4c4d4f !important;
  color: #cfd3dc !important;
}

html.dark .list-item:hover {
  background-color: #303133 !important;
}

/* 深色模式下的徽章 */
html.dark .el-badge__content {
  background-color: #f56c6c !important;
  color: #fff !important;
}

/* 深色模式下的面包屑 */
html.dark .el-breadcrumb__inner {
  color: #cfd3dc !important;
}

html.dark .el-breadcrumb__inner:hover {
  color: #409eff !important;
}

html.dark .el-breadcrumb__separator {
  color: #94a3b8 !important;
}

/* 深色模式下的评分 */
html.dark .el-rate__icon {
  color: #4c4d4f !important;
}

html.dark .el-rate__icon.is-active {
  color: #f7ba2a !important;
}

/* 强制修复对话框内所有白色背景 - 最高优先级 */
html.dark .el-dialog * {
  background-color: transparent !important;
}

html.dark .el-dialog .el-dialog__body {
  background-color: #1d1e1f !important;
}

/* 统计组件容器 */
html.dark .el-statistic {
  background-color: #262729 !important;
  padding: 15px !important;
  border-radius: 4px !important;
}

html.dark .el-col > .el-statistic {
  background-color: #262729 !important;
}

/* 修复el-row和el-col的背景 */
html.dark .el-dialog .el-row {
  background-color: transparent !important;
}

html.dark .el-dialog .el-col {
  background-color: transparent !important;
}

html.dark .el-dialog .el-col > div:not(.el-statistic) {
  background-color: transparent !important;
}

/* 席位信息区块 */
html.dark .seats-section .el-col {
  background-color: transparent !important;
}

html.dark .seats-section .el-statistic {
  background-color: #262729 !important;
  border: 1px solid #4c4d4f !important;
  padding: 15px !important;
  border-radius: 4px !important;
}

/* 支付信息和计费信息表格 */
html.dark .el-descriptions {
  background-color: #262729 !important;
}

html.dark .el-descriptions__body {
  background-color: #262729 !important;
}

html.dark .el-descriptions__label.el-descriptions__cell {
  background-color: #303133 !important;
  color: #94a3b8 !important;
}

html.dark .el-descriptions__content.el-descriptions__cell {
  background-color: #262729 !important;
  color: #e5eaf3 !important;
}

html.dark .el-descriptions--border .el-descriptions__cell {
  border-color: #4c4d4f !important;
}

/* 确保卡片内的所有内容都是深色 */
html.dark .billing-card,
html.dark .billing-card * {
  background-color: transparent !important;
}

html.dark .billing-card .el-card__body {
  background-color: #1d1e1f !important;
}

/* 修复任何可能遗漏的白色背景 */
html.dark .el-dialog [style*="background"],
html.dark .el-dialog [style*="background-color"] {
  background-color: inherit !important;
}

/* 强制修复el-card的内联样式白色背景 */
html.dark .el-card[style*="background: #f8f9fa"] {
  background-color: #262729 !important;
}

html.dark .el-card[style*="background:#f8f9fa"] {
  background-color: #262729 !important;
}

/* 强制所有el-card使用深色背景 */
html.dark .el-dialog .el-card {
  background-color: #1d1e1f !important;
}

html.dark .el-dialog .el-card__header {
  background: linear-gradient(135deg, #262729 0%, #2a2c2f 100%) !important;
}

html.dark .el-dialog .el-card__body {
  background-color: #1d1e1f !important;
}

/* 嵌套在el-space中的卡片 */
html.dark .el-space .el-card {
  background-color: #262729 !important;
}

html.dark .el-space .el-card__body {
  background-color: #262729 !important;
}

/* 确保shadow="never"的卡片也被覆盖 */
html.dark .el-card[shadow="never"] {
  background-color: #1d1e1f !important;
  box-shadow: none !important;
}

/* 强制覆盖所有可能的白色内联样式 */
html.dark [style*="background: white"],
html.dark [style*="background-color: white"],
html.dark [style*="background: #fff"],
html.dark [style*="background-color: #fff"],
html.dark [style*="background: #ffffff"],
html.dark [style*="background-color: #ffffff"],
html.dark [style*="background: #f8f9fa"],
html.dark [style*="background: #f5f7fa"] {
  background: #262729 !important;
  background-color: #262729 !important;
}
</style>
