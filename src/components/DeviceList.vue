<script setup lang="ts">
import { h, ref, computed, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { NDataTable, NButton, NSpin, NEmpty, NTag, useMessage } from 'naive-ui';
import type { DataTableColumns, DataTableRowKey } from 'naive-ui';
import { Refresh as RefreshIcon } from '@vicons/ionicons5';
import { useDeviceStore } from '../stores/deviceStore';
import { refreshDevices } from '../api';
import type { Device } from '../types';

const { t } = useI18n();
const store = useDeviceStore();
const message = useMessage();

const columns = computed<DataTableColumns<Device>>(() => [
  { type: 'selection' },
  { title: t('devices.columns.name'), key: 'name' },
  { title: t('devices.columns.model'), key: 'model' },
  { title: t('devices.columns.id'), key: 'id' },
  {
    title: t('devices.columns.status'),
    key: 'status',
    render(row) {
      const color = row.status === 'Online' ? 'success' : row.status === 'Offline' ? 'default' : 'warning';
      const statusText = row.status === 'Online' ? t('devices.status.online') : 
                         row.status === 'Offline' ? t('devices.status.offline') : row.status;
      return h(NTag, { type: color, size: 'small' }, () => statusText);
    },
  },
]);

const selectedKeys = ref<DataTableRowKey[]>(store.selectedDeviceId ? [store.selectedDeviceId] : []);

function updateSelectedKeys(keys: DataTableRowKey[]) {
  selectedKeys.value = keys;
  store.selectDevice(String(keys[0] ?? ''));
}

async function handleRefresh() {
  try {
    await refreshDevices();
    message.success(t('devices.refreshSuccess'));
  } catch {
    message.error(t('devices.refreshFailed'));
  }
}

onMounted(async () => {
  // Initial device load
  try {
    await refreshDevices();
  } catch {
    // Silently fail on initial load
  }
  
  // Listen for device events
  const handler = (ev: Event) => {
    const detail = (ev as CustomEvent).detail as { type: string; device?: Device; deviceId?: string };
    if (!detail) return;
    if (detail.type === 'connected') {
      const d = detail.device as Device | undefined;
      message.success(`${t('devices.deviceConnected')}: ${d?.name ?? d?.id ?? ''}`);
    } else if (detail.type === 'disconnected') {
      message.success(`${t('devices.deviceDisconnected')}: ${detail.deviceId ?? ''}`);
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
    <div class="device-list-content">
      <div class="list-header">
        <n-button size="small" @click="handleRefresh">
          <template #icon>
            <RefreshIcon />
          </template>
          {{ t('common.refresh') }}
        </n-button>
        <span class="device-count" v-if="store.devices?.length">
          {{ store.devices.length }} {{ store.devices.length === 1 ? 'device' : 'devices' }}
        </span>
      </div>
      
      <n-data-table
        v-if="store.devices?.length > 0"
        :columns="columns"
        :data="store.devices"
        :row-key="(row: Device) => row.id"
        :checked-row-keys="selectedKeys"
        @update:checked-row-keys="updateSelectedKeys"
        :single-line="false"
        :max-height="280"
        size="small"
        striped
      />
      
      <div v-else class="empty-state">
        <n-empty :description="t('devices.noDevices')">
          <template #extra>
            <div class="empty-hint">
              <p>{{ t('devices.connectHint') }}</p>
              <n-button size="small" type="primary" @click="handleRefresh">
                {{ t('common.refresh') }}
              </n-button>
            </div>
          </template>
        </n-empty>
      </div>
    </div>
  </n-spin>
</template>

<style scoped>
.device-list-content {
  min-height: 200px;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.device-count {
  font-size: 12px;
  opacity: 0.6;
  padding: 4px 8px;
  border-radius: 4px;
}

.empty-state {
  padding: 20px;
}

.empty-hint {
  text-align: center;
  margin-top: 12px;
}

.empty-hint p {
  margin: 0 0 12px;
  font-size: 13px;
  opacity: 0.6;
}
</style>
