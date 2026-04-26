<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { NCard, NForm, NFormItem, NInput, NSelect, NInputNumber, NSwitch, useMessage } from 'naive-ui';
import { useSettingsStore } from '../stores/settingsStore';
import { setLocale, availableLocales } from '../locales';

const { t } = useI18n();
const store = useSettingsStore();
const message = useMessage();

const themeOptions = computed(() => [
  { label: t('settings.themeLight'), value: 'light' },
  { label: t('settings.themeDark'), value: 'dark' },
  { label: t('settings.themeSystem'), value: 'system' }
]);

const languageOptions = availableLocales;

const settings = ref({
  theme: store.settings.theme || 'system',
  maxSessions: store.settings.maxSessions || 1,
  adbPath: store.settings.binaryPaths?.adb || '',
  scrcpyPath: store.settings.binaryPaths?.scrcpy || '',
  autoStart: false, // Platform-specific, not yet implemented
  language: store.settings.language || 'en'
});

let saveTimeout: ReturnType<typeof setTimeout> | null = null;

function debouncedSave() {
  if (saveTimeout) clearTimeout(saveTimeout);
  saveTimeout = setTimeout(() => {
    store.updateSettings({
      theme: settings.value.theme,
      maxSessions: settings.value.maxSessions,
      language: settings.value.language,
      binaryPaths: {
        adb: settings.value.adbPath || undefined,
        scrcpy: settings.value.scrcpyPath || undefined
      }
    });
    message.success(t('settings.settingsSaved'));
  }, 300);
}

// Handle language change immediately (not debounced)
function handleLanguageChange(locale: string) {
  setLocale(locale);
  settings.value.language = locale;
  debouncedSave();
}

watch(settings, debouncedSave, { deep: true });
</script>

<template>
  <div class="settings-view">
    <n-card :title="t('settings.title')">
      <n-form label-placement="left" label-width="140" responsive="screen">
        <n-form-item :label="t('settings.language')">
          <n-select 
            v-model:value="settings.language" 
            :options="languageOptions" 
            @update:value="handleLanguageChange"
          />
        </n-form-item>

        <n-form-item :label="t('settings.theme')">
          <n-select v-model:value="settings.theme" :options="themeOptions" />
        </n-form-item>

        <n-form-item :label="t('settings.maxSessions')">
          <n-input-number v-model:value="settings.maxSessions" :min="1" :max="10" />
        </n-form-item>

        <n-form-item :label="t('settings.autoStart')">
          <n-switch v-model:value="settings.autoStart" disabled />
          <span class="hint-text">{{ t('settings.autoStartHint') }}</span>
        </n-form-item>

        <n-form-item :label="t('settings.adbPath')">
          <n-input v-model:value="settings.adbPath" :placeholder="t('settings.pathPlaceholder')" />
        </n-form-item>

        <n-form-item :label="t('settings.scrcpyPath')">
          <n-input v-model:value="settings.scrcpyPath" :placeholder="t('settings.pathPlaceholder')" />
        </n-form-item>
      </n-form>
    </n-card>
  </div>
</template>

<style scoped>
.settings-view {
  padding: 20px;
  max-width: 700px;
  width: 100%;
  box-sizing: border-box;
}

.hint-text {
  margin-left: 8px;
  font-size: 12px;
  color: #888;
}

@media (max-width: 600px) {
  .settings-view {
    padding: 12px;
  }
  
  .settings-view :deep(.n-form-item) {
    flex-direction: column;
    align-items: flex-start;
  }
  
  .settings-view :deep(.n-form-item-label) {
    width: 100% !important;
    text-align: left;
    margin-bottom: 4px;
  }
}
</style>
