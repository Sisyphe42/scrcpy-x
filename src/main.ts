import { createApp } from 'vue';
import { createRouter, createWebHistory } from 'vue-router';
import { createPinia } from 'pinia';
import App from './App.vue';
import { i18n } from './locales';
import MainLayout from './layouts/MainLayout.vue';
import DevicesView from './views/DevicesView.vue';
import ProfilesView from './views/ProfilesView.vue';
import SettingsView from './views/SettingsView.vue';

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: MainLayout,
      children: [
        {
          path: '',
          redirect: '/devices',
        },
        {
          path: 'devices',
          name: 'devices',
          component: DevicesView,
        },
        {
          path: 'profiles',
          name: 'profiles',
          component: ProfilesView,
        },
        {
          path: 'settings',
          name: 'settings',
          component: SettingsView,
        },
      ],
    },
  ],
});

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.use(i18n);
app.mount('#app');
