   
<script setup lang="ts">
import { onErrorCaptured, onMounted, onUnmounted, computed, ref } from 'vue';
import { NConfigProvider, NMessageProvider, NDialogProvider, darkTheme, NAlert, NButton } from 'naive-ui';
import { handleKeyboard } from './utils/shortcuts';
import { useSettingsStore } from './stores/settingsStore';
import NaiveProvider from './components/NaiveProvider.vue';

const settingsStore = useSettingsStore();
const globalError = ref<string | null>(null);

// Compute the Naive UI theme based on settings
const naiveTheme = computed(() => {
  const theme = settingsStore.settings.theme;
  if (theme === 'dark') {
    return darkTheme;
  } else if (theme === 'system') {
    // Use system preference
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    return prefersDark ? darkTheme : null;
  }
  // 'light' or default
  return null;
});

// Global error boundary: capture errors from child components and present user-friendly messages
onMounted(() => {
  // Register global keyboard shortcuts
  window.addEventListener('keydown', handleKeyboard);
  
  // Listen for system theme changes when using 'system' theme
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  mediaQuery.addEventListener('change', () => {
    if (settingsStore.settings.theme === 'system') {
      // Force re-computation by touching the store
      settingsStore.updateSettings({ theme: 'system' });
    }
  });
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyboard);
});

onErrorCaptured((err: any, _vm: any, _info: string) => {
  console.error('Global error captured:', err);
  const msg = err instanceof Error ? err.message : String(err);
  globalError.value = msg;
  // Allow error to propagate for debugging
  return true;
});

function clearError() {
  globalError.value = null;
}

function clearCacheAndReload() {
  localStorage.clear();
  location.reload();
}
</script>

<template>
  <n-config-provider :theme="naiveTheme">
    <n-message-provider>
      <n-dialog-provider>
        <NaiveProvider>
          <div v-if="globalError" style="margin: 16px;">
            <n-alert
              type="error"
              title="Application Error"
              closable
              @close="clearError"
            >
              {{ globalError }}
            </n-alert>
            <n-button size="small" style="margin-top: 8px;" @click="clearCacheAndReload">
              Clear Cache & Reload
            </n-button>
          </div>
          <router-view v-else />
        </NaiveProvider>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  padding: 0;
  min-height: 100vh;
}

#app {
  min-height: 100vh;
}
</style>
