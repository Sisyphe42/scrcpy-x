<script setup lang="ts">
import { ref, watch } from 'vue';
import { NCard, NForm, NFormItem, NInput, NSelect, NInputNumber, useMessage } from 'naive-ui';
import { useSettingsStore } from '../stores/settingsStore';

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
          <n-input v-model:value="settings.adbPath" placeholder="Leave empty to use PATH" />
        </n-form-item>

        <n-form-item label="Scrcpy Path">
          <n-input v-model:value="settings.scrcpyPath" placeholder="Leave empty to use PATH" />
        </n-form-item>
      </n-form>
    </n-card>
  </div>
</template>
