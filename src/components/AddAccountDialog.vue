<template>
  <el-dialog
    v-model="uiStore.showAddAccountDialog"
    :title="$t('dialog.addAccount.title')"
    width="500px"
    :close-on-click-modal="false"
  >
    <el-form
      ref="formRef"
      :model="formData"
      :rules="currentRules"
      label-width="100px"
      autocomplete="off"
    >
      <!-- 添加方式切换 -->
      <el-form-item :label="$t('dialog.addAccount.addMode')">
        <el-radio-group v-model="addMode" @change="handleModeChange">
          <el-radio value="password">{{
            $t("dialog.addAccount.modePassword")
          }}</el-radio>
          <el-radio value="refresh_token">{{ $t('dialog.addAccount.modeToken') }}</el-radio>
        </el-radio-group>
      </el-form-item>

      <!-- 邮箱密码模式 -->
      <template v-if="addMode === 'password'">
        <el-form-item :label="$t('dialog.addAccount.email')" prop="email">
          <el-input
            v-model="formData.email"
            :placeholder="$t('dialog.addAccount.emailPlaceholder')"
            :prefix-icon="Message"
            autocomplete="off"
          />
        </el-form-item>

        <el-form-item :label="$t('dialog.addAccount.password')" prop="password">
          <el-input
            v-model="formData.password"
            type="password"
            :placeholder="$t('dialog.addAccount.passwordPlaceholder')"
            :prefix-icon="Lock"
            show-password
            autocomplete="new-password"
          />
        </el-form-item>
      </template>

      <!-- Refresh Token 模式 -->
      <template v-else>
        <el-form-item :label="$t('dialog.addAccount.refreshToken')" prop="refreshToken">
          <el-input
            v-model="formData.refreshToken"
            type="textarea"
            :rows="3"
            :placeholder="$t('dialog.addAccount.tokenPlaceholder')"
          />
        </el-form-item>
      </template>

      <el-form-item :label="$t('dialog.addAccount.nickname')" prop="nickname">
        <el-input
          v-model="formData.nickname"
          :placeholder="$t('dialog.addAccount.nicknamePlaceholder')"
          :prefix-icon="User"
        />
      </el-form-item>

      <el-form-item :label="$t('dialog.addAccount.group')">
        <el-select
          v-model="formData.group"
          :placeholder="$t('dialog.addAccount.groupPlaceholder')"
          clearable
        >
          <el-option
            v-for="group in settingsStore.groups"
            :key="group"
            :label="group"
            :value="group"
          />
        </el-select>
      </el-form-item>

      <el-form-item :label="$t('dialog.addAccount.tags')">
        <el-select
          v-model="formData.tags"
          multiple
          filterable
          allow-create
          :placeholder="$t('dialog.addAccount.tagsPlaceholder')"
          style="width: 100%"
        >
          <el-option
            v-for="tag in settingsStore.tags"
            :key="tag.name"
            :label="tag.name"
            :value="tag.name"
          >
            <span :style="getTagOptionStyle(tag.color)">{{ tag.name }}</span>
          </el-option>
        </el-select>
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="handleClose">{{
        $t("dialog.addAccount.cancel")
      }}</el-button>
      <el-button type="primary" @click="handleSubmit" :loading="loading">
        {{ $t("dialog.addAccount.confirm") }}
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from "vue";
import { useI18n } from "vue-i18n";
import { ElMessage } from "element-plus";
import type { FormInstance, FormRules } from "element-plus";
import { Message, Lock, User } from "@element-plus/icons-vue";
import { useAccountsStore, useSettingsStore, useUIStore } from "@/store";
import { apiService, accountApi } from "@/api";
import { invoke } from "@tauri-apps/api/core";

const accountsStore = useAccountsStore();
const settingsStore = useSettingsStore();
const uiStore = useUIStore();
const { t } = useI18n();

const formRef = ref<FormInstance>();
const loading = ref(false);
const addMode = ref<"password" | "refresh_token">("password");

const formData = reactive({
  email: "",
  password: "",
  refreshToken: "",
  nickname: "",
  group: "默认分组",
  tags: [] as string[],
});

// 邮箱密码模式的验证规则
const passwordRules = computed<FormRules>(() => ({
  email: [
    {
      required: true,
      message: t("dialog.addAccount.validation.emailRequired"),
      trigger: "blur",
    },
    {
      type: "email",
      message: t("dialog.addAccount.validation.emailInvalid"),
      trigger: "blur",
    },
  ],
  password: [
    {
      required: true,
      message: t("dialog.addAccount.validation.passwordRequired"),
      trigger: "blur",
    },
    {
      min: 6,
      message: t("dialog.addAccount.validation.passwordLength"),
      trigger: "blur",
    },
  ],
  nickname: [
    {
      max: 20,
      message: t("dialog.addAccount.validation.nicknameLength"),
      trigger: "blur",
    },
  ],
}));

// Refresh Token 模式的验证规则
const refreshTokenRules = computed<FormRules>(() => ({
  refreshToken: [
    {
      required: true,
      message: t("dialog.addAccount.validation.tokenRequired"),
      trigger: "blur",
    },
    {
      min: 10,
      message: t("dialog.addAccount.validation.tokenInvalid"),
      trigger: "blur",
    },
  ],
  nickname: [
    {
      max: 20,
      message: t("dialog.addAccount.validation.nicknameLength"),
      trigger: "blur",
    },
  ],
}));

// 根据模式选择验证规则
const currentRules = computed(() => {
  return addMode.value === "password"
    ? passwordRules.value
    : refreshTokenRules.value;
});

// 切换模式时重置表单
function handleModeChange() {
  formRef.value?.resetFields();
}

// 获取标签选项样式
function getTagOptionStyle(color: string): Record<string, string> {
  if (!color) return {};

  let r = 0,
    g = 0,
    b = 0;
  let parsed = false;

  // 解析 rgba 或 rgb 格式
  if (color.startsWith("rgba") || color.startsWith("rgb")) {
    const match = color.match(/rgba?\((\d+),\s*(\d+),\s*(\d+)/);
    if (match) {
      r = parseInt(match[1]);
      g = parseInt(match[2]);
      b = parseInt(match[3]);
      parsed = true;
    }
  }
  // 解析 HEX 格式
  if (!parsed && color.startsWith("#")) {
    const hex = color.slice(1);
    if (hex.length >= 6) {
      r = parseInt(hex.slice(0, 2), 16);
      g = parseInt(hex.slice(2, 4), 16);
      b = parseInt(hex.slice(4, 6), 16);
      parsed = true;
    }
  }

  if (!parsed) return {};

  return {
    color: `rgb(${r}, ${g}, ${b})`,
    fontWeight: "500",
  };
}

async function handleSubmit() {
  if (!formRef.value) return;

  await formRef.value.validate(async (valid) => {
    if (!valid) return;

    loading.value = true;
    try {
      if (addMode.value === "refresh_token") {
        // Refresh Token 模式
        const trimmedToken = formData.refreshToken.trim();
        const trimmedNickname = formData.nickname.trim() || undefined;

        if (!trimmedToken) {
          ElMessage.error($t('dialog.addAccount.messages.tokenEmpty'));
          loading.value = false;
          return;
        }

        // 调用后端接口添加账号
        const result = await invoke<any>("add_account_by_refresh_token", {
          refreshToken: trimmedToken,
          nickname: trimmedNickname,
          tags: formData.tags,
          group: formData.group || "默认分组",
        });

        if (result.success) {
          ElMessage.success(`账号 ${result.email} 添加成功`);
          // 刷新账号列表
          await accountsStore.loadAccounts();
          handleClose();
        } else {
          ElMessage.error($t('dialog.addAccount.messages.addFailed'));
        }
      } else {
        // 邮箱密码模式
        const trimmedEmail = formData.email.trim();
        const trimmedPassword = formData.password.trim();
        const trimmedNickname =
          formData.nickname.trim() || trimmedEmail.split("@")[0];

        if (!trimmedPassword) {
          ElMessage.error($t('dialog.addAccount.messages.passwordEmpty'));
          loading.value = false;
          return;
        }

        // 添加账号
        const newAccount = await accountsStore.addAccount({
          email: trimmedEmail,
          password: trimmedPassword,
          nickname: trimmedNickname,
          tags: formData.tags,
          group: formData.group || "默认分组",
        });

        ElMessage.success($t('dialog.addAccount.messages.addSuccess'));

        // 自动登录并获取账号详细信息
        try {
          const loginResult = await apiService.loginAccount(newAccount.id);

          if (loginResult.success) {
            const latestAccount = await accountApi.getAccount(newAccount.id);
            await accountsStore.updateAccount(latestAccount);
            ElMessage.success($t('dialog.addAccount.messages.infoUpdated'));
          } else {
            ElMessage.warning($t('dialog.addAccount.messages.loginFailed'));
          }
        } catch (infoError) {
          console.error("获取账号信息失败:", infoError);
          ElMessage.warning($t('dialog.addAccount.messages.infoFailed'));
        }

        handleClose();
      }
    } catch (error) {
      ElMessage.error(`添加失败: ${error}`);
    } finally {
      loading.value = false;
    }
  });
}

function handleClose() {
  uiStore.closeAddAccountDialog();
  formRef.value?.resetFields();

  // 重置表单数据
  formData.email = "";
  formData.password = "";
  formData.refreshToken = "";
  formData.nickname = "";
  formData.group = "默认分组";
  formData.tags = [];
  addMode.value = "password";
}
</script>
