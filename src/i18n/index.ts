import { createI18n } from 'vue-i18n';

import zh from './locales/zh.json';
import en from './locales/en.json';
import fr from './locales/fr.json';
import es from './locales/es.json';

// Get stored language or default to zh
const savedLanguage = localStorage.getItem('language') || 'zh';

const i18n = createI18n({
  legacy: false, // Use Composition API
  locale: savedLanguage,
  fallbackLocale: 'zh', // Default to Chinese as it's the original language
  globalInjection: true, // Inject $t globally
  messages: {
    zh,
    en,
    fr,
    es
  }
});

export default i18n;
