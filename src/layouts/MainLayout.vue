<script setup lang="ts">
import { h, ref, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { NLayout, NLayoutSider, NLayoutContent, NMenu, NIcon } from 'naive-ui';
import type { MenuOption } from 'naive-ui';
import { 
  RocketOutline, 
  DesktopOutline,
  SettingsOutline,
  ChevronBackOutline,
  ChevronForwardOutline
} from '@vicons/ionicons5';

const router = useRouter();
const route = useRoute();
const { t } = useI18n();
const collapsed = ref(false);

// Icon render helper
function renderIcon(icon: any) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

// Make menu options reactive to language changes
const menuOptions = computed<MenuOption[]>(() => [
  { label: () => t('nav.launch'), key: 'launch', icon: renderIcon(RocketOutline) },
  { label: () => t('nav.sessions'), key: 'sessions', icon: renderIcon(DesktopOutline) },
  { label: () => t('nav.settings'), key: 'settings', icon: renderIcon(SettingsOutline) },
]);

const activeKey = computed(() => route.name as string);

function handleMenuSelect(key: string) {
  router.push({ name: key });
}
</script>

<template>
  <n-layout has-sider style="height: 100vh;">
    <n-layout-sider
      bordered
      collapse-mode="width"
      :collapsed-width="64"
      :width="200"
      :collapsed="collapsed"
      @collapse="collapsed = true"
      @expand="collapsed = false"
    >
      <!-- Logo -->
      <div class="sidebar-logo" :class="{ collapsed }">
        <img src="/tauri.svg" alt="ScrcpyX" class="logo-icon" />
        <span v-if="!collapsed" class="logo-text">ScrcpyX</span>
      </div>
      <n-menu
        :options="menuOptions"
        :value="activeKey"
        @update:value="handleMenuSelect"
        :collapsed-width="64"
        :collapsed-icon-size="22"
      />
      
      <!-- Custom collapse trigger -->
      <div class="collapse-trigger" @click="collapsed = !collapsed">
        <n-icon size="18">
          <ChevronBackOutline v-if="!collapsed" />
          <ChevronForwardOutline v-else />
        </n-icon>
      </div>
    </n-layout-sider>
    <n-layout-content>
      <router-view />
    </n-layout-content>
  </n-layout>
</template>

<style scoped>
.sidebar-logo {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 16px 12px;
  transition: padding 0.2s ease;
  overflow: hidden;
}

.sidebar-logo.collapsed {
  padding: 16px 18px;
}

.logo-icon {
  width: 28px;
  height: 28px;
  flex-shrink: 0;
  transition: width 0.2s ease, height 0.2s ease;
}

.logo-text {
  font-size: 16px;
  font-weight: 700;
  white-space: nowrap;
}

.collapse-trigger {
  position: absolute;
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%);
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-radius: 6px;
  transition: background 0.2s ease, opacity 0.2s ease;
  opacity: 0.6;
}

.collapse-trigger:hover {
  opacity: 1;
}

:deep(.n-layout-sider) {
  overflow: hidden !important;
}

:deep(.n-layout-sider-content) {
  overflow-x: hidden !important;
}

:deep(.n-menu--collapsed .n-menu-item-content) {
  justify-content: center;
}

:deep(.n-menu--collapsed .n-menu-item-content__icon) {
  margin-right: 0 !important;
}

:deep(.n-menu-item-content) {
  justify-content: center;
}

:deep(.n-menu-item-content__icon) {
  margin-right: 8px !important;
}
</style>
