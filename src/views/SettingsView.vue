<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { NCard, NForm, NFormItem, NInput, NSelect, NSwitch, NInputNumber, NButton, NSpace, useMessage } from 'naive-ui';
import { useSettingsStore } from '../stores/settingsStore';
import { open } from '@tauri-apps/api/dialog';

const store = useSettingsStore();
const message = useMessage();

const themeOptions = [
  { label: 'Light', value: 'light' },
  { label: 'Dark', value: 'dark' },
  { label: 'System', value: 'system' }
];

const settings = ref({
  theme: store.settings.theme || 'system',
  maxSessions: store.settings.maxSessions || 1,
  adbPath: store.settings.binaryPaths?.adb || '',
  scrcpyPath: store.settings.binaryPaths?.scrcpy || ''
});

let saveTimeout: ReturnType<typeof setTimeout> | null = null;

function debouncedSave() {
  if (saveTimeout) clearTimeout(saveTimeout);
  saveTimeout = setTimeout(() => {
    store.updateSettings({
      theme: settings.value.theme,
      maxSessions: settings.value.maxSessions,
      binaryPaths: {
        adb: settings.value.adbPath || undefined,
        scrcpy: settings.value.scrcpyPath || undefined
      }
    });
    message.success('Settings saved');
  }, 300);
}

watch(settings, debouncedSave, { deep: true });

async function browseAdb() {
  try {
    const path = await open({
      filters: [{ name: 'Executable', extensions: ['exe', ''] }],
      title: 'Select ADB executable'
    });
    if (path && typeof path === 'string') {
      settings.value.adbPath = path;
    }
  } catch {
    // User cancelled
  }
}

async function browseScrcpy() {
  try {
    const path = await open({
      filters: [{ name: 'Executable', extensions: ['exe', ''] }],
      title: 'Select scrcpy executable'
    });
    if (path && typeof path === 'string') {
      settings.value.scrcpyPath = path;
    }
  } catch {
    // User cancelled
  }
}
</script>

<template>
  <div style="padding: 20px; max-width: 600px;">
    <n-card title="Settings">
      <n-form label-placement="left" label-width="140">
        <n-form-item label="Theme">
          <n-select v-model:value="settings.theme" :options="themeOptions" />
        </n-form-item>
        
        <n-form-item label="Max Sessions">
          <n-input-number v-model:value="settings.maxSessions" :min="1" :max="10" />
        </n-form-item>
        
        <n-form-item label="ADB Path">
          <n-space vertical style="width: 100%;">
            <n-input v-model:value="settings.adbPath" placeholder="Leave empty to use PATH" />
            <n-button size="small" @click="browseAdb">Browse</n-button>
          </n-space>
        </n-form-item>
        
        <n-form-item label="Scrcpy Path">
          <n-space vertical style="width: 100%;">
            <n-input v-model:value="settings.scrcpyPath" placeholder="Leave empty to use PATH" />
            <n-button size="small" @click="browseScrcpy">Browse</n-button>
          </n-space>
        </n-form-item>
      </n-form>
    </n-card>
  </div>
</template>
