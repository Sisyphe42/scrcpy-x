import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Device } from '../types';
import { listen } from '@tauri-apps/api/event';
import { refreshDevices as apiRefreshDevices } from '../api/devices';
import { useSessionStore } from './sessionStore';

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
    try {
      await apiRefreshDevices();
    } finally {
      loading.value = false;
    }
  }

  // Initialize event listeners for device discovery via Tauri events
  // and wire them into the local store. This runs once when the store is created.
  (async () => {
    try {
      // device-connected
      void await listen('device-connected', (event) => {
        const device = (event.payload ?? {}) as Device;
        const idx = devices.value.findIndex(d => d.id === device.id);
        if (idx >= 0) {
          devices.value[idx] = device;
        } else {
          devices.value.push(device);
        }
        persistDevices(devices.value);
        // notify UI layer for toasts
        window.dispatchEvent(new CustomEvent('device-update', { detail: { type: 'connected', device } }));
      });
      // device-disconnected
      void await listen('device-disconnected', (event) => {
        const payload = (event.payload ?? {}) as { deviceId: string };
        const id = payload.deviceId;
        devices.value = devices.value.filter(d => d.id !== id);
        persistDevices(devices.value);
        window.dispatchEvent(new CustomEvent('device-update', { detail: { type: 'disconnected', deviceId: id } }));

        // If there is an active session on this device, mark it as stopped with an explanatory error
        try {
          const sessionStore = useSessionStore();
          const active = sessionStore.activeSession;
          if (active?.deviceId === id) {
            sessionStore.updateSessionStatus(active.id, 'Stopped', 'Device disconnected');
          }
        } catch {
          // ignore if sessionStore is not ready yet
        }
      });

      // Auto-refresh when app gains focus (no polling, just a refresh on focus)
      if (typeof window !== 'undefined') {
        const onFocus = async () => {
          try {
            await refreshDevices();
          } catch {
            // ignore refresh errors on focus
          }
        };
        window.addEventListener('focus', onFocus);
      }
      // Unlisten is not strictly required in this app lifetime; kept for completeness
      // (Return value not used here since store persists for app lifetime)
    } catch {
      // If event system isn't available (e.g., SSR during tests), skip gracefully
    }
  })();

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
