import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { AppSettings, BinaryPaths } from '../types';

const SETTINGS_KEY = 'scrcpyx-settings';

function loadSettingsFromStorage(): AppSettings {
  try {
    const raw = typeof localStorage !== 'undefined' ? localStorage.getItem(SETTINGS_KEY) : null;
    if (raw) return JSON.parse(raw) as AppSettings;
  } catch {
    // ignore
  }
  // Default settings
  return {
    lastProfile: undefined,
    windowBounds: { x: 0, y: 0, width: 800, height: 600 },
    binaryPaths: {},
    theme: 'system',
    maxSessions: 1,
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
  const binaryPaths = computed<BinaryPaths>(() => settings.value.binaryPaths ?? {});
  const maxSessions = computed<number>(() => settings.value.maxSessions ?? 1);

  function updateSettings(partial: Partial<AppSettings>) {
    settings.value = { ...settings.value, ...partial } as AppSettings;
    persistSettings(settings.value);
  }

  function setTheme(t: string) {
    updateSettings({ theme: t });
  }

  function setBinaryPaths(paths: BinaryPaths) {
    updateSettings({ binaryPaths: paths });
  }

  return {
    settings,
    theme,
    binaryPaths,
    maxSessions,
    updateSettings,
    setTheme,
    setBinaryPaths,
  };
});
