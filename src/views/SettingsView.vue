<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  NIcon, NCard, NForm, NFormItem, NInput, NSelect, NInputNumber,
  NSwitch, useMessage, NDivider,
} from 'naive-ui';
import {
  ColorPaletteOutline,
  PhonePortraitOutline,
  CameraOutline,
  InformationCircleOutline,
  GitBranchOutline,
  HeartOutline,
} from '@vicons/ionicons5';
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
  autoStart: false,
  language: store.settings.language || 'en',
  // Screenshot settings
  screenshotFilename: store.settings.screenshotFilename || '{device}_{date}_{time}',
  screenshotPath: store.settings.screenshotPath || '',
  screenshotClipboard: store.settings.screenshotClipboard ?? false,
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
      },
      screenshotFilename: settings.value.screenshotFilename || undefined,
      screenshotPath: settings.value.screenshotPath || undefined,
      screenshotClipboard: settings.value.screenshotClipboard,
    });
    message.success(t('settings.settingsSaved'));
  }, 300);
}

function handleLanguageChange(locale: string) {
  setLocale(locale);
  settings.value.language = locale;
  debouncedSave();
}

watch(settings, debouncedSave, { deep: true });
</script>

<template>
  <div class="settings-view">
    <div class="page-header">
      <h2 class="page-title">{{ t('settings.title') }}</h2>
    </div>

    <div class="settings-cards">
      <!-- Appearance Card -->
      <n-card class="settings-card">
        <template #header>
          <div class="card-header">
            <n-icon :size="20"><ColorPaletteOutline /></n-icon>
            <span>{{ t('settings.theme') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120">
          <n-form-item :label="t('settings.theme')">
            <n-select v-model:value="settings.theme" :options="themeOptions" />
          </n-form-item>
          <n-form-item :label="t('settings.language')">
            <n-select
              v-model:value="settings.language"
              :options="languageOptions"
              @update:value="handleLanguageChange"
            />
          </n-form-item>
        </n-form>
      </n-card>

      <!-- Device Card -->
      <n-card class="settings-card">
        <template #header>
          <div class="card-header">
            <n-icon :size="20"><PhonePortraitOutline /></n-icon>
            <span>{{ t('settings.maxSessions') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120">
          <n-form-item :label="t('settings.maxSessions')">
            <n-input-number v-model:value="settings.maxSessions" :min="1" :max="10" />
          </n-form-item>
          <n-form-item :label="t('settings.autoStart')">
            <n-switch v-model:value="settings.autoStart" disabled />
            <span class="hint-text">{{ t('settings.autoStartHint') }}</span>
          </n-form-item>
        </n-form>
      </n-card>

      <!-- Binary Paths Card -->
      <n-card class="settings-card">
        <template #header>
          <div class="card-header">
            <n-icon :size="20"><GitBranchOutline /></n-icon>
            <span>Binary Paths</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120">
          <n-form-item :label="t('settings.adbPath')">
            <n-input v-model:value="settings.adbPath" :placeholder="t('settings.pathPlaceholder')" />
          </n-form-item>
          <n-form-item :label="t('settings.scrcpyPath')">
            <n-input v-model:value="settings.scrcpyPath" :placeholder="t('settings.pathPlaceholder')" />
          </n-form-item>
        </n-form>
      </n-card>

      <!-- Screenshot Settings Card -->
      <n-card class="settings-card">
        <template #header>
          <div class="card-header">
            <n-icon :size="20"><CameraOutline /></n-icon>
            <span>{{ t('settings.screenshot') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="140">
          <n-form-item :label="t('settings.screenshotFilename')">
            <n-input v-model:value="settings.screenshotFilename" placeholder="{device}_{date}_{time}" />
          </n-form-item>
          <div class="form-hint">{{ t('settings.screenshotFilenameHint') }}</div>
          <n-form-item :label="t('settings.screenshotPath')">
            <n-input v-model:value="settings.screenshotPath" :placeholder="t('settings.screenshotPathPlaceholder')" />
          </n-form-item>
          <n-form-item :label="t('settings.screenshotClipboard')">
            <n-switch v-model:value="settings.screenshotClipboard" />
          </n-form-item>
        </n-form>
      </n-card>

      <!-- About Card -->
      <n-card class="settings-card">
        <template #header>
          <div class="card-header">
            <n-icon :size="20"><InformationCircleOutline /></n-icon>
            <span>{{ t('about.title') }}</span>
          </div>
        </template>
        <div class="about-content">
          <div class="about-row">
            <span class="about-label">ScrcpyX</span>
            <span class="about-value">{{ t('about.description') }}</span>
          </div>
          <div class="about-row">
            <span class="about-label">{{ t('about.version') }}</span>
            <span class="about-value">0.1.0</span>
          </div>
          <div class="about-row">
            <span class="about-label">GitHub</span>
            <a href="https://github.com/Sisyphe42/scrcpy-x" target="_blank" class="about-link">
              github.com/Sisyphe42/scrcpy-x
            </a>
          </div>
          <n-divider style="margin: 12px 0" />
          <div class="credits">
            <div class="credits-title">
              <n-icon :size="16"><HeartOutline /></n-icon>
              {{ t('about.credits') }}
            </div>
            <ul class="credits-list">
              <li>
                <a href="https://github.com/Genymobile/scrcpy" target="_blank" class="about-link">scrcpy</a>
                — {{ t('about.scrcpyCredit') }}
              </li>
              <li>
                <a href="https://tauri.app" target="_blank" class="about-link">Tauri</a>
                — {{ t('about.tauriCredit') }}
              </li>
              <li>
                <a href="https://www.naiveui.com" target="_blank" class="about-link">Naive UI</a>
                — {{ t('about.naiveCredit') }}
              </li>
            </ul>
          </div>
        </div>
      </n-card>
    </div>
  </div>
</template>

<style scoped>
.settings-view {
  padding: 20px;
  max-width: 800px;
  width: 100%;
  box-sizing: border-box;
}

.page-header {
  margin-bottom: 16px;
}

.page-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.settings-cards {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.settings-card {
  border-radius: 12px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  font-weight: 600;
}

.hint-text {
  margin-left: 8px;
  font-size: 12px;
  color: #888;
}

.form-hint {
  font-size: 11px;
  color: #888;
  margin: -8px 0 8px;
  padding-left: 2px;
}

.about-content {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.about-row {
  display: flex;
  align-items: center;
  gap: 12px;
}

.about-label {
  font-weight: 600;
  font-size: 13px;
  min-width: 70px;
}

.about-value {
  font-size: 13px;
  opacity: 0.8;
}

.about-link {
  color: #18a058;
  text-decoration: none;
  font-size: 13px;
}

.about-link:hover {
  text-decoration: underline;
}

.credits-title {
  display: flex;
  align-items: center;
  gap: 6px;
  font-weight: 600;
  font-size: 13px;
  margin-bottom: 6px;
}

.credits-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.credits-list li {
  font-size: 12px;
  opacity: 0.8;
  padding-left: 4px;
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
