<script setup lang="ts">
import { h, ref, onMounted, onUnmounted } from 'vue';
import { NDataTable, NButton, NSpin, NEmpty, NTag, useMessage } from 'naive-ui';
import type { DataTableColumns, DataTableRowKey } from 'naive-ui';
import { useDeviceStore } from '../stores/deviceStore';
import { refreshDevices } from '../api';
import type { Device } from '../types';

const store = useDeviceStore();
const message = useMessage();

const columns: DataTableColumns<Device> = [
  { title: 'Name', key: 'name' },
  { title: 'Model', key: 'model' },
  { title: 'ID', key: 'id' },
  {
    title: 'Status',
    key: 'status',
    render(row) {
      const color = row.status === 'Online' ? 'success' : row.status === 'Offline' ? 'default' : 'warning';
      return h(NTag, { type: color, size: 'small' }, { default: () => row.status });
    },
  },
];

const selectedKeys = ref<DataTableRowKey[]>(store.selectedDeviceId ? [store.selectedDeviceId] : []);

function updateSelectedKeys(keys: DataTableRowKey[]) {
  selectedKeys.value = keys;
  store.selectDevice(String(keys[0] ?? ''));
}

async function handleRefresh() {
  try {
    await refreshDevices();
    message.success('Devices refreshed');
  } catch {
    message.error('Failed to refresh devices');
  }
}

onMounted(() => {
  const handler = (ev: Event) => {
    const detail = (ev as CustomEvent).detail as { type: string; device?: Device; deviceId?: string };
    if (!detail) return;
    if (detail.type === 'connected') {
      const d = detail.device as Device | undefined;
      message.success(`Device connected: ${d?.name ?? d?.id ?? ''}`);
    } else if (detail.type === 'disconnected') {
      message.success(`Device disconnected: ${detail.deviceId ?? ''}`);
    }
  };
  window.addEventListener('device-update', handler as EventListener);
  onUnmounted(() => {
    window.removeEventListener('device-update', handler as EventListener);
  });
});
</script>

<template>
  <n-spin :show="store.loading">
    <div style="padding: 16px;">
      <n-button @click="handleRefresh" style="margin-bottom: 12px;">Refresh</n-button>
      <n-data-table
        v-if="store.devices?.length > 0"
        :columns="columns"
        :data="store.devices"
        :row-key="(row: Device) => row.id"
        :checked-row-keys="selectedKeys"
        @update:checked-row-keys="updateSelectedKeys"
        :single-line="false"
      />
      <n-empty v-else description="No devices found" />
    </div>
  </n-spin>
</template>
