<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { format } from 'date-fns';
import {
  NIcon, NCard, NForm, NFormItem, NInput, NSelect, NInputNumber,
  NSwitch, useMessage, NDivider, NRadioGroup, NRadio, NTooltip, NButton, NPopconfirm,
} from 'naive-ui';
import {
  ColorPaletteOutline,
  PhonePortraitOutline,
  CameraOutline,
  InformationCircleOutline,
  SettingsOutline,
  HeartOutline,
  FolderOutline,
  RefreshOutline,
  HelpCircleOutline,
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
  adbSource: store.settings.binaryConfig?.adbSource || 'auto',
  adbPath: store.settings.binaryConfig?.adbPath || '',
  scrcpySource: store.settings.binaryConfig?.scrcpySource || 'auto',
  scrcpyPath: store.settings.binaryConfig?.scrcpyPath || '',
  autoStart: false,
  language: store.settings.language || 'en',
  // Screenshot settings
  screenshotFilename: store.settings.screenshotFilename || 'screenshot_{device}_{date:yyyy-MM-dd}_{time:HH-mm-ss}_scrcpyx',
  screenshotPath: store.settings.screenshotPath || '',
  screenshotClipboard: store.settings.screenshotClipboard ?? false,
  screenshotFormat: store.settings.screenshotFormat || 'png',
});

const imageFormatOptions = computed(() => [
  { label: 'PNG', value: 'png' },
  { label: 'JPG', value: 'jpg' },
  { label: 'WebP', value: 'webp' },
  { label: 'BMP', value: 'bmp' },
]);

let saveTimeout: ReturnType<typeof setTimeout> | null = null;

function debouncedSave() {
  if (saveTimeout) clearTimeout(saveTimeout);
  saveTimeout = setTimeout(() => {
    store.updateSettings({
      theme: settings.value.theme,
      maxSessions: settings.value.maxSessions,
      language: settings.value.language,
      binaryConfig: {
        adbSource: settings.value.adbSource,
        adbPath: settings.value.adbSource === 'custom' ? settings.value.adbPath || undefined : undefined,
        scrcpySource: settings.value.scrcpySource,
        scrcpyPath: settings.value.scrcpySource === 'custom' ? settings.value.scrcpyPath || undefined : undefined,
      },
      screenshotFilename: settings.value.screenshotFilename || undefined,
      screenshotPath: settings.value.screenshotPath || undefined,
      screenshotClipboard: settings.value.screenshotClipboard,
      screenshotFormat: settings.value.screenshotFormat || 'png',
    });
    message.success(t('settings.settingsSaved'));
  }, 300);
}

function handleLanguageChange(locale: string) {
  setLocale(locale);
  settings.value.language = locale;
  debouncedSave();
}

function generateScreenshotExample(): string {
  const now = new Date();
  const device = 'Pixel_6';
  const session = '001';
  
  // Parse format strings from filename template
  let result = settings.value.screenshotFilename;
  
  // Replace {date} and {time} with actual formatted values
  // Extract format from template if present, otherwise use defaults
  const dateMatch = result.match(/\{date:([^}]+)\}/);
  const timeMatch = result.match(/\{time:([^}]+)\}/);
  
  const dateFormat = dateMatch ? dateMatch[1] : 'yyyy-MM-dd';
  const timeFormat = timeMatch ? timeMatch[1] : 'HH-mm-ss';
  
  const dateStr = format(now, dateFormat);
  const timeStr = format(now, timeFormat);
  
  result = result
    .replace(/\{date(?::[^}]+)?\}/, dateStr)
    .replace(/\{time(?::[^}]+)?\}/, timeStr)
    .replace('{device}', device)
    .replace('{session}', session);
  
  return result + '.' + settings.value.screenshotFormat;
}

function resetScreenshotSettings() {
  settings.value.screenshotFilename = 'screenshot_{device}_{date:yyyy-MM-dd}_{time:HH-mm-ss}_scrcpyx';
  settings.value.screenshotPath = '';
  settings.value.screenshotClipboard = false;
  settings.value.screenshotFormat = 'png';
  message.success(t('settings.resetSuccess'));
}

function selectScreenshotPath() {
  // TODO: Implement system folder picker via Tauri dialog
  message.info('Folder picker coming soon');
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
            <n-select v-model:value="settings.theme" :options="themeOptions" style="width: 200px;" />
          </n-form-item>
          <n-form-item :label="t('settings.language')">
            <n-select
              v-model:value="settings.language"
              :options="languageOptions"
              @update:value="handleLanguageChange"
              style="width: 200px;"
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

      <!-- Tool Configuration Card -->
      <n-card class="settings-card">
        <template #header>
          <div class="card-header">
            <n-icon :size="20"><SettingsOutline /></n-icon>
            <span>{{ t('settings.toolConfiguration') }}</span>
          </div>
        </template>
        <n-form label-placement="left" label-width="120">
          <!-- ADB Configuration -->
          <div style="margin-bottom: 24px">
            <div style="font-weight: 500; margin-bottom: 12px">ADB</div>
            <n-form-item>
              <n-radio-group v-model:value="settings.adbSource">
                <n-radio value="auto">
                  {{ t('settings.binarySourceAuto') }}
                </n-radio>
                <n-tooltip>
                  <template #trigger>
                    <n-radio value="bundled" disabled>
                      {{ t('settings.binarySourceBundled') }}
                    </n-radio>
                  </template>
                  {{ t('settings.binarySourceBundledHint') }}
                </n-tooltip>
                <n-radio value="custom">
                  {{ t('settings.binarySourceCustom') }}
                </n-radio>
              </n-radio-group>
            </n-form-item>
            <div class="form-hint">
              <template v-if="settings.adbSource === 'auto'">
                {{ t('settings.binarySourceAutoHint') }}
              </template>
              <template v-else-if="settings.adbSource === 'bundled'">
                {{ t('settings.binarySourceBundledHint') }}
              </template>
              <template v-else>
                {{ t('settings.binarySourceCustomHint') }}
              </template>
            </div>
            <template v-if="settings.adbSource === 'custom'">
              <n-form-item :label="t('settings.adbPath')">
                <n-input v-model:value="settings.adbPath" :placeholder="t('settings.pathPlaceholder')" />
              </n-form-item>
            </template>
          </div>

          <n-divider style="margin: 16px 0" />

          <!-- Scrcpy Configuration -->
          <div>
            <div style="font-weight: 500; margin-bottom: 12px">Scrcpy</div>
            <n-form-item>
              <n-radio-group v-model:value="settings.scrcpySource">
                <n-radio value="auto">
                  {{ t('settings.binarySourceAuto') }}
                </n-radio>
                <n-tooltip>
                  <template #trigger>
                    <n-radio value="bundled" disabled>
                      {{ t('settings.binarySourceBundled') }}
                    </n-radio>
                  </template>
                  {{ t('settings.binarySourceBundledHint') }}
                </n-tooltip>
                <n-radio value="custom">
                  {{ t('settings.binarySourceCustom') }}
                </n-radio>
              </n-radio-group>
            </n-form-item>
            <div class="form-hint">
              <template v-if="settings.scrcpySource === 'auto'">
                {{ t('settings.binarySourceAutoHint') }}
              </template>
              <template v-else-if="settings.scrcpySource === 'bundled'">
                {{ t('settings.binarySourceBundledHint') }}
              </template>
              <template v-else>
                {{ t('settings.binarySourceCustomHint') }}
              </template>
            </div>
            <template v-if="settings.scrcpySource === 'custom'">
              <n-form-item :label="t('settings.scrcpyPath')">
                <n-input v-model:value="settings.scrcpyPath" :placeholder="t('settings.pathPlaceholder')" />
              </n-form-item>
            </template>
          </div>
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
            <div style="width: 100%;">
              <div style="display: flex; gap: 8px; width: 100%; align-items: center; margin-bottom: 6px;">
                <n-input v-model:value="settings.screenshotFilename" placeholder="screenshot_{device}_{date:yyyy-MM-dd}_{time:HH-mm-ss}_scrcpyx" style="flex: 1;" />
                <n-select v-model:value="settings.screenshotFormat" :options="imageFormatOptions" style="width: 80px;" />
              </div>
              <div style="position: relative; margin-bottom: 12px;">
                <div style="position: absolute; left: -24px; top: 8px; width: 16px; display: flex; align-items: center;">
                  <n-tooltip placement="top">
                    <template #trigger>
                      <n-icon :size="16" style="cursor: help; color: #666;"><HelpCircleOutline /></n-icon>
                    </template>
                    <div style="font-size: 12px; max-width: 300px;">
                      <div style="margin-bottom: 8px;">
                        <strong>{{ t('settings.variables') }}:</strong>
                      </div>
                      <div style="margin-left: 12px; margin-bottom: 12px;">
                        <div>{device} - {{ t('settings.variableDevice') }}</div>
                        <div>{session} - {{ t('settings.variableSession') }}</div>
                        <div>{date:format} - {{ t('settings.variableDateWithFormat') }}</div>
                        <div>{time:format} - {{ t('settings.variableTimeWithFormat') }}</div>
                      </div>
                      <div style="margin-bottom: 8px;">
                        <strong>{{ t('settings.examples') }}:</strong>
                      </div>
                      <div style="margin-left: 12px; font-family: monospace; font-size: 11px; color: #999;">
                        <div>{date:yyyy-MM-dd} → 2024-12-25</div>
                        <div>{date:MM/dd/yyyy} → 12/25/2024</div>
                        <div>{time:HH-mm-ss} → 14-30-45</div>
                        <div>{time:HHmmss} → 143045</div>
                      </div>
                      <div style="margin-top: 8px;">
                        <a href="https://date-fns.org/docs/format" target="_blank" style="font-size: 12px; color: #0066cc;">
                          {{ t('settings.dateFormatDocs') }} →
                        </a>
                      </div>
                    </div>
                  </n-tooltip>
                </div>
                <div style="padding: 8px; background: #f5f5f5; border-radius: 4px; font-size: 12px;">
                  <span><strong>{{ t('settings.example') }}:</strong> {{ generateScreenshotExample() }}</span>
                </div>
              </div>
            </div>
          </n-form-item>
          <n-form-item :label="t('settings.screenshotPath')">
            <div style="display: flex; gap: 8px; width: 100%;">
              <n-input v-model:value="settings.screenshotPath" :placeholder="t('settings.screenshotPathDefault')" style="flex: 1;" />
              <n-button @click="selectScreenshotPath" :type="'default'" style="padding: 0 12px;">
                <template #icon>
                  <n-icon><FolderOutline /></n-icon>
                </template>
              </n-button>
            </div>
          </n-form-item>
          <n-form-item :label="t('settings.screenshotClipboard')">
            <n-switch v-model:value="settings.screenshotClipboard" />
          </n-form-item>
          <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 16px;">
            <n-popconfirm @positive-click="resetScreenshotSettings">
              <template #trigger>
                <n-button :type="'default'" size="small">
                  <template #icon>
                    <n-icon><RefreshOutline /></n-icon>
                  </template>
                  {{ t('settings.resetDefaults') }}
                </n-button>
              </template>
              {{ t('settings.resetConfirm') }}
            </n-popconfirm>
          </div>
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
              Sisyphe42/scrcpy-x
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
