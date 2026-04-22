import { createApp } from "vue";
import { createPinia } from "pinia";
import { createRouter, createWebHistory } from "vue-router";
import naive from "naive-ui";
import { i18n } from "./locales";
import App from "./App.vue";

// Create Pinia store
const pinia = createPinia();

// Create Vue Router (basic setup for now)
const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("./views/HomeView.vue"),
    },
  ],
});

// Create and configure app
const app = createApp(App);

// Use plugins
app.use(pinia);
app.use(router);
app.use(naive);
app.use(i18n);

// Mount app
app.mount("#app");
