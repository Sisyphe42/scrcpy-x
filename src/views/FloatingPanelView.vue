<script setup lang="ts">
import { onMounted } from 'vue'
import { NConfigProvider, NMessageProvider, NDialogProvider, darkTheme } from 'naive-ui'
import { useSettingsStore } from '../stores/settingsStore'
import { useI18n } from 'vue-i18n'
import DeviceControlPanel from '../components/DeviceControlPanel.vue'

const settingsStore = useSettingsStore()
const { t } = useI18n()

// Listen for session data from main window (the store is shared via localStorage)
onMounted(async () => {
  // Sync sessions from backend on mount
  try {
    const { getSessions } = await import('../api/sessions')
    await getSessions()
  } catch {
    // May not have any sessions yet
  }
})
</script>

<template>
  <n-config-provider :theme="settingsStore.settings.theme === 'dark' ? darkTheme : null">
    <n-message-provider>
      <n-dialog-provider>
        <div class="floating-panel-root">
          <div class="floating-panel-header">
            <span class="floating-panel-title">{{ t('controls.title') }}</span>
          </div>
          <DeviceControlPanel />
        </div>
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<style>
/* Reset body for floating window — no sidebar, compact */
html, body {
  margin: 0;
  padding: 0;
  overflow-x: hidden;
  min-height: auto;
}

#app {
  min-height: auto;
}
</style>

<style scoped>
.floating-panel-root {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow-y: auto;
}

.floating-panel-header {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.08);
  flex-shrink: 0;
}

.floating-panel-title {
  font-weight: 700;
  font-size: 14px;
}
</style>
