import { invoke } from '@tauri-apps/api/core';
import type { AppSettings } from '../types';
import { useSettingsStore } from '../stores/settingsStore';

// Get current application settings from backend and sync store
export async function getSettings(): Promise<AppSettings> {
  try {
    const settings = await invoke<AppSettings>('get_settings');
    const store = useSettingsStore();
    store.updateSettings(settings);
    return settings;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to fetch settings: ${msg}`);
  }
}

// Save application settings to backend
export async function saveSettings(settings: AppSettings): Promise<void> {
  try {
    await invoke<void>('save_settings', { settings });
    const store = useSettingsStore();
    store.updateSettings(settings);
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to save settings: ${msg}`);
  }
}

// Convenience accessors for common paths
export async function getAdbPath(): Promise<string> {
  try {
    return await invoke<string>('get_adb_path');
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to fetch adb path: ${msg}`);
  }
}

export async function getScrcpyPath(): Promise<string> {
  try {
    return await invoke<string>('get_scrcpy_path');
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to fetch scrcpy path: ${msg}`);
  }
}
