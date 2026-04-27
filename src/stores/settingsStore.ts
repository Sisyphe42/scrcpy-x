import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { AppSettings, BinarySourceConfig } from '../types';

const SETTINGS_KEY = 'scrcpyx-settings';

function loadSettingsFromStorage(): AppSettings {
  try {
    const raw = typeof localStorage !== 'undefined' ? localStorage.getItem(SETTINGS_KEY) : null;
    if (raw) {
      const parsed = JSON.parse(raw);
      // Validate that we got a valid object
      if (parsed && typeof parsed === 'object') {
        return {
          lastProfile: parsed.lastProfile ?? undefined,
          windowBounds: parsed.windowBounds ?? { x: 0, y: 0, width: 800, height: 600 },
          binaryConfig: parsed.binaryConfig ?? {
            adbSource: 'auto',
            scrcpySource: 'auto',
            adbPath: undefined,
            scrcpyPath: undefined,
          },
          theme: parsed.theme ?? 'system',
          maxSessions: parsed.maxSessions ?? 1,
          language: parsed.language ?? 'en',
          screenshotFilename: parsed.screenshotFilename ?? 'screenshot_{device}_{date:yyyy-MM-dd}_{time:HH-mm-ss}_scrcpyx',
          screenshotPath: parsed.screenshotPath ?? '',
          screenshotClipboard: parsed.screenshotClipboard ?? false,
          screenshotFormat: parsed.screenshotFormat ?? 'png',
        };
      }
    }
  } catch (err) {
    console.warn('Failed to load settings from storage:', err);
    // Clear corrupted data
    try {
      localStorage.removeItem(SETTINGS_KEY);
    } catch {
      // ignore
    }
  }
  // Default settings
  return {
    lastProfile: undefined,
    windowBounds: { x: 0, y: 0, width: 800, height: 600 },
    binaryConfig: {
      adbSource: 'auto',
      scrcpySource: 'auto',
      adbPath: undefined,
      scrcpyPath: undefined,
    },
    theme: 'system',
    maxSessions: 1,
    language: 'en',
    screenshotFilename: 'screenshot_{device}_{date:yyyy-MM-dd}_{time:HH-mm-ss}_scrcpyx',
    screenshotPath: '',
    screenshotClipboard: false,
    screenshotFormat: 'png',
  };
}

function persistSettings(settings: AppSettings) {
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings));
    }
  } catch {
    // ignore
  }
}

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<AppSettings>(loadSettingsFromStorage());

  const theme = computed(() => settings.value.theme);
  const binaryConfig = computed(() => settings.value.binaryConfig);
  const maxSessions = computed<number>(() => settings.value.maxSessions ?? 1);

  function updateSettings(partial: Partial<AppSettings>) {
    settings.value = { ...settings.value, ...partial } as AppSettings;
    persistSettings(settings.value);
  }

  function setTheme(t: string) {
    updateSettings({ theme: t });
  }

  function setBinaryConfig(config: Partial<BinarySourceConfig>) {
    updateSettings({
      binaryConfig: { ...settings.value.binaryConfig, ...config }
    });
  }

  return {
    settings,
    theme,
    binaryConfig,
    maxSessions,
    updateSettings,
    setTheme,
    setBinaryConfig,
  };
});
