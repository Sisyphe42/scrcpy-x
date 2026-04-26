<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useMessage } from 'naive-ui';
import { Play, PhonePortraitOutline, FolderOutline, SettingsOutline } from '@vicons/ionicons5';
import { useDeviceStore } from '../stores/deviceStore';
import { useProfileStore } from '../stores/profileStore';
import { useSettingsStore } from '../stores/settingsStore';
import { launchSession } from '../api/sessions';
import DeviceList from '../components/DeviceList.vue';
import ProfileList from '../components/ProfileList.vue';
import SessionOptionsForm from '../components/SessionOptionsForm.vue';
import type { SessionOptions } from '../types/session';

const { t } = useI18n();
const message = useMessage();
const deviceStore = useDeviceStore();
const profileStore = useProfileStore();
const settingsStore = useSettingsStore();
const isLaunching = ref(false);
const isLaunchingAll = ref(false);

// Current session options (synced with selected profile)
const currentOptions = ref<SessionOptions>({});

// Sync options with active profile
watch(() => profileStore.activeProfile, (profile) => {
  if (profile?.options) {
    currentOptions.value = { ...profile.options };
  }
}, { immediate: true });

// Get all selected device IDs (for multi-device launch)
const selectedDeviceIds = computed(() => {
  const ids: string[] = [];
  if (deviceStore.selectedDeviceId) {
    ids.push(deviceStore.selectedDeviceId);
  }
  return ids;
});

const canLaunch = computed(() => Boolean(deviceStore?.selectedDeviceId));
const canLaunchAll = computed(() => selectedDeviceIds.value.length > 1);

// Single device launch
const launch = async () => {
  if (!deviceStore?.selectedDeviceId) {
    message.warning(t('launch.noDevice'));
    return;
  }
  isLaunching.value = true;
  try {
    const session = await launchSession(deviceStore.selectedDeviceId, currentOptions.value);
    message.success(`${t('launch.sessionStarted')}: ${session.id}`);
  } catch (err: unknown) {
    const msg = err instanceof Error ? err.message : String(err);
    message.error(`${t('launch.launchFailed')}: ${msg}`);
  } finally {
    isLaunching.value = false;
  }
};

// Multi-device launch
const launchAll = async () => {
  const deviceIds = selectedDeviceIds.value;
  if (deviceIds.length === 0) {
    message.warning(t('launch.noDevice'));
    return;
  }
  
  const maxSessions = settingsStore.settings.maxSessions || 5;
  if (deviceIds.length > maxSessions) {
    message.warning(`${t('launch.tooManyDevices')} ${maxSessions}`);
    return;
  }
  
  isLaunchingAll.value = true;
  let successCount = 0;
  let failCount = 0;
  
  for (const deviceId of deviceIds) {
    try {
      await launchSession(deviceId, currentOptions.value);
      successCount++;
    } catch {
      failCount++;
    }
  }
  
  isLaunchingAll.value = false;
  
  if (successCount > 0) {
    message.success(`${t('launch.launchedCount')} ${successCount}`);
  }
  if (failCount > 0) {
    message.error(`${t('launch.launchFailedCount')} ${failCount}`);
  }
};
</script>

<template>
  <div class="launch-view">
    <div class="page-header">
      <h2 class="page-title">{{ t('launch.title') }}</h2>
      <p class="page-subtitle">{{ t('launch.subtitle') }}</p>
    </div>
    
    <div class="launch-grid">
      <!-- Step 1: Select Device -->
      <n-card class="grid-card step-card" size="small">
        <template #header>
          <div class="card-header">
            <n-icon class="step-icon" :component="PhonePortraitOutline" />
            <span class="card-title">{{ t('launch.panelDevices') }}</span>
          </div>
        </template>
        <DeviceList />
      </n-card>

      <!-- Step 2: Select Profile -->
      <n-card class="grid-card step-card" size="small">
        <template #header>
          <div class="card-header">
            <n-icon class="step-icon" :component="FolderOutline" />
            <span class="card-title">{{ t('launch.panelProfiles') }}</span>
          </div>
        </template>
        <ProfileList />
      </n-card>

      <!-- Step 3: Configure Options -->
      <n-card class="grid-card step-card" size="small">
        <template #header>
          <div class="card-header">
            <n-icon class="step-icon" :component="SettingsOutline" />
            <span class="card-title">{{ t('launch.panelOptions') }}</span>
          </div>
        </template>
        <SessionOptionsForm v-model="currentOptions" />
      </n-card>
    </div>

    <!-- Launch Actions -->
    <div class="launch-actions">
      <div class="action-right">
        <n-button 
          type="primary" 
          size="large" 
          :loading="isLaunching" 
          :disabled="!canLaunch" 
          @click="launch"
        >
          <template #icon>
            <Play />
          </template>
          {{ t('launch.launchButton') }}
        </n-button>
        <n-button
          type="primary"
          size="large"
          :loading="isLaunchingAll"
          :disabled="!canLaunchAll"
          @click="launchAll"
        >
          {{ t('launch.launchAllButton', { count: selectedDeviceIds.length }) }}
        </n-button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.launch-view { 
  padding: 16px; 
  box-sizing: border-box;
  width: 100%;
}

/* Page header */
.page-header {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: 16px;
}

.page-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.page-subtitle {
  margin: 0;
  font-size: 13px;
  opacity: 0.6;
}

/* Default: 3 columns on large screens */
.launch-grid { 
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
  min-height: 400px;
  width: 100%;
  margin-bottom: 20px;
}

.grid-card {
  overflow: auto;
  min-height: 300px;
}

/* Step card styling */
.step-card {
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.step-card:hover {
  box-shadow: 0 4px 12px rgba(24, 160, 88, 0.1);
}

.card-header {
  display: flex;
  align-items: center;
  gap: 10px;
}

.step-icon {
  font-size: 20px;
  color: #18a058;
}

.card-title {
  font-weight: 600;
  font-size: 15px;
}

/* Action bar */
.launch-actions {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20px;
  border-radius: 12px;
  gap: 16px;
}

.action-right {
  display: flex;
  gap: 12px;
}

/* Responsive breakpoints */
/* Medium screens: 2 columns */
@media (max-width: 1200px) {
  .launch-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .grid-card:last-child {
    grid-column: span 2;
  }
}

/* Small screens: 1 column */
@media (max-width: 800px) {
  .launch-view {
    padding: 12px;
  }
  
  .launch-grid {
    grid-template-columns: 1fr;
    min-height: auto;
    gap: 16px;
  }
  
  .grid-card {
    min-height: 250px;
  }
  
  .grid-card:last-child {
    grid-column: span 1;
  }
  
  .launch-actions {
    flex-direction: column;
    align-items: stretch;
    padding: 16px;
  }
  
  .action-right {
    justify-content: center;
    width: 100%;
  }
}

/* Very small screens */
@media (max-width: 500px) {
  .launch-view {
    padding: 8px;
  }
  
  .action-right {
    flex-direction: column;
  }
  
  .action-right .n-button {
    width: 100%;
  }
}
</style>
