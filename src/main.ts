import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus';
import 'element-plus/dist/index.css';
import * as ElementPlusIconsVue from '@element-plus/icons-vue';
import { pinia } from './store';
import zhCn from 'element-plus/dist/locale/zh-cn.mjs';

import i18n from './i18n';

const app = createApp(App);

app.use(i18n);

// 注册Element Plus
app.use(ElementPlus, {
  locale: zhCn,
});

// 注册所有图标
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component);
}

// 注册Pinia
app.use(pinia);

app.mount("#app");
