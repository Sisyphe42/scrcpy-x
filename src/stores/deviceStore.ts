import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Device } from '../types';

// Simple persistent storage keys (localStorage used as fallback for Tauri storage)
const DEVICES_KEY = 'scrcpyx-devices';
const SELECTED_DEVICE_KEY = 'scrcpyx-selectedDevice';

function loadDevicesFromStorage(): Device[] {
  try {
    const raw = typeof localStorage !== 'undefined' ? localStorage.getItem(DEVICES_KEY) : null;
    if (raw) return JSON.parse(raw) as Device[];
  } catch {
    // ignore
  }
  return [];
}

function persistDevices(devices: Device[]) {
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(DEVICES_KEY, JSON.stringify(devices));
    }
  } catch {
    // ignore
  }
}

function loadSelectedDeviceId(): string | null {
  try {
    const raw = typeof localStorage !== 'undefined' ? localStorage.getItem(SELECTED_DEVICE_KEY) : null;
    return raw ?? null;
  } catch {
    return null;
  }
}

function persistSelectedDeviceId(id: string | null) {
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(SELECTED_DEVICE_KEY, id ?? '');
    }
  } catch {
    // ignore
  }
}

export const useDeviceStore = defineStore('device', () => {
  // state
  const devices = ref<Device[]>(loadDevicesFromStorage());
  const selectedDeviceId = ref<string | null>(loadSelectedDeviceId());
  const loading = ref(false);

  // getters
  const selectedDevice = computed(() => devices.value.find(d => d.id === selectedDeviceId.value) ?? null);
  const onlineDevices = computed(() => devices.value.filter(d => d.status === 'Online'));

  // actions
  function setDevices(dvs: Device[]) {
    devices.value = dvs;
    persistDevices(dvs);
  }

  function selectDevice(id: string) {
    selectedDeviceId.value = id;
    persistSelectedDeviceId(id);
  }

  async function refreshDevices() {
    loading.value = true;
    // Placeholder for real device refresh logic (to be wired with backend later)
    await new Promise(resolve => setTimeout(resolve, 200));
    loading.value = false;
  }

  return {
    devices,
    selectedDeviceId,
    loading,
    selectedDevice,
    onlineDevices,
    setDevices,
    selectDevice,
    refreshDevices,
  };
});
