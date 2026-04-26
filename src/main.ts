import { createApp } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';
import { createPinia } from 'pinia';
import naive from 'naive-ui';
import App from './App.vue';
import { i18n } from './locales';

// Clear potentially corrupted storage on first load error
try {
  const testKey = '__scrcpyx_test__';
  localStorage.setItem(testKey, '1');
  localStorage.removeItem(testKey);
} catch (err) {
  console.error('localStorage not available or corrupted, clearing all data');
  try {
    localStorage.clear();
  } catch {
    // ignore
  }
}

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('./layouts/MainLayout.vue'),
      children: [
        {
          path: '',
          name: 'launch',
          component: () => import('./views/LaunchView.vue'),
        },
        {
          path: 'sessions',
          name: 'sessions',
          component: () => import('./views/SessionView.vue'),
        },
        {
          path: 'settings',
          name: 'settings',
          component: () => import('./views/SettingsView.vue'),
        },
      ],
    },
    // Floating panel — standalone, no sidebar layout
    {
      path: '/floating-panel',
      name: 'floating-panel',
      component: () => import('./views/FloatingPanelView.vue'),
    },
  ],
});

const pinia = createPinia();
const app = createApp(App);

app.use(router);
app.use(pinia);
app.use(i18n);
app.use(naive);
app.mount('#app');
