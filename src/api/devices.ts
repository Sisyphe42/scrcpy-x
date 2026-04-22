import { invoke } from '@tauri-apps/api/core';
import type { Device } from '../types';
import { useDeviceStore } from '../stores/deviceStore';

// Fetch devices from backend and sync store
export async function getDevices(): Promise<Device[]> {
  try {
    const devices = await invoke<Device[]>('get_devices');
    const store = useDeviceStore();
    if (Array.isArray(devices)) {
      store.setDevices(devices);
    }
    return devices;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to fetch devices: ${msg}`);
  }
}

// Refresh device list on backend and re-sync store
export async function refreshDevices(): Promise<void> {
  try {
    await invoke<void>('refresh_devices');
    // Best-effort re-fetch to keep UI in sync
    const devices = await invoke<Device[]>('get_devices');
    const store = useDeviceStore();
    if (Array.isArray(devices)) {
      store.setDevices(devices);
    }
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to refresh devices: ${msg}`);
  }
}
