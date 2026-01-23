import { defineStore } from 'pinia';
import { ref } from 'vue';
import type { Settings, OperationLog, GlobalTag } from '@/types';
import { settingsApi } from '@/api';
import i18n from '@/i18n';

// 默认设置值
const defaultSettings: Partial<Settings> = {
  auto_refresh_token: true,
  seat_count_options: [18, 19, 20],
  retry_times: 2,
  theme: 'light',
  concurrent_limit: 5,
  show_seats_result_dialog: false,
  useLightweightApi: true,
  proxyEnabled: false,
  proxyUrl: null,
  language: 'zh',
};

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<Settings>({ ...defaultSettings } as Settings);
  
  const groups = ref<string[]>([]);
  const tags = ref<GlobalTag[]>([]);
  const logs = ref<OperationLog[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // Actions
  async function loadSettings() {
    loading.value = true;
    error.value = null;
    try {
      const loaded = await settingsApi.getSettings();
      // 合并默认值，确保新增字段有默认值
      settings.value = { ...defaultSettings, ...loaded } as Settings;
      
      // Fix: Prioritize localStorage > Settings API > Default
      // If the backend doesn't store language yet, we rely on localStorage
      const localLang = localStorage.getItem('language');
      if (localLang) {
        settings.value.language = localLang;
        // @ts-ignore
        i18n.global.locale.value = localLang;
      } else if (settings.value.language) {
        // @ts-ignore
        i18n.global.locale.value = settings.value.language;
      }
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateSettings(newSettings: Settings) {
    loading.value = true;
    error.value = null;
    try {
      await settingsApi.updateSettings(newSettings);
      settings.value = newSettings;
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function loadGroups() {
    loading.value = true;
    error.value = null;
    try {
      groups.value = await settingsApi.getGroups();
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function addGroup(name: string) {
    loading.value = true;
    error.value = null;
    try {
      await settingsApi.addGroup(name);
      if (!groups.value.includes(name)) {
        groups.value.push(name);
      }
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteGroup(name: string) {
    loading.value = true;
    error.value = null;
    try {
      await settingsApi.deleteGroup(name);
      groups.value = groups.value.filter(g => g !== name);
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }
  
  async function renameGroup(oldName: string, newName: string) {
    loading.value = true;
    error.value = null;
    try {
      await settingsApi.renameGroup(oldName, newName);
      const index = groups.value.indexOf(oldName);
      if (index !== -1) {
        groups.value[index] = newName;
      }
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  // 标签管理
  async function loadTags() {
    loading.value = true;
    error.value = null;
    try {
      tags.value = await settingsApi.getTags();
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function addTag(tag: GlobalTag) {
    loading.value = true;
    error.value = null;
    try {
      await settingsApi.addTag(tag);
      // 检查是否已存在
      const existingIndex = tags.value.findIndex(t => t.name === tag.name);
      if (existingIndex === -1) {
        tags.value.push(tag);
      }
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function updateTag(oldName: string, tag: GlobalTag) {
    loading.value = true;
    error.value = null;
    try {
      await settingsApi.updateTag(oldName, tag);
      const index = tags.value.findIndex(t => t.name === oldName);
      if (index !== -1) {
        tags.value[index] = tag;
      }
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function deleteTag(name: string) {
    loading.value = true;
    error.value = null;
    try {
      await settingsApi.deleteTag(name);
      tags.value = tags.value.filter(t => t.name !== name);
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function batchUpdateAccountTags(accountIds: string[], addTags: string[], removeTags: string[]) {
    loading.value = true;
    error.value = null;
    try {
      return await settingsApi.batchUpdateAccountTags(accountIds, addTags, removeTags);
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function loadLogs(limit?: number) {
    loading.value = true;
    error.value = null;
    try {
      logs.value = await settingsApi.getLogs(limit);
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function clearLogs() {
    loading.value = true;
    error.value = null;
    try {
      await settingsApi.clearLogs();
      logs.value = [];
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function exportData() {
    loading.value = true;
    error.value = null;
    try {
      return await settingsApi.exportData();
    } catch (e) {
      error.value = (e as Error).message;
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function initialize() {
    await Promise.all([
      loadSettings(),
      loadGroups(),
      loadTags(),
    ]);
  }

  async function setLanguage(lang: string) {
    settings.value.language = lang;
    // @ts-ignore
    i18n.global.locale.value = lang;
    localStorage.setItem('language', lang);
    await updateSettings(settings.value);
  }

  return {
    // State
    settings,
    groups,
    tags,
    logs,
    loading,
    error,
    
    // Actions
    loadSettings,
    updateSettings,
    loadGroups,
    addGroup,
    deleteGroup,
    renameGroup,
    loadTags,
    addTag,
    updateTag,
    deleteTag,
    batchUpdateAccountTags,
    loadLogs,
    clearLogs,
    exportData,
    initialize,
    setLanguage,
  };
});
