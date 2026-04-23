import { createApp } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';
import { createPinia } from 'pinia';
import App from './App.vue';
import { i18n } from './locales';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('./layouts/MainLayout.vue'),
      redirect: '/devices',
      children: [
        {
          path: 'devices',
          name: 'devices',
          component: () => import('./views/DevicesView.vue'),
        },
        {
          path: 'profiles',
          name: 'profiles',
          component: () => import('./views/ProfilesView.vue'),
        },
        {
          path: 'settings',
          name: 'settings',
          component: () => import('./views/SettingsView.vue'),
        },
      ],
    },
  ],
});

const pinia = createPinia();
const app = createApp(App);

app.use(router);
app.use(pinia);
app.use(i18n);
app.mount('#app');
