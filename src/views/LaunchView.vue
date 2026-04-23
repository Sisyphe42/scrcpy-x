<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useDeviceStore } from '../stores/deviceStore';
import { useProfileStore } from '../stores/profileStore';
import { launchSession } from '../api/sessions';
import DeviceList from '../components/DeviceList.vue';
import SessionOptionsForm from '../components/SessionOptionsForm.vue';
import ProfileList from '../components/ProfileList.vue';

type SessionOptions = any;

const deviceStore = useDeviceStore();
const profileStore = useProfileStore();
const isLaunching = ref(false);

// Quick-profile dropdown (UI only; wired-in later)
const quickProfile = ref<string>('');
const quickProfileOptions = [
  { label: 'Default', value: 'default' },
  { label: 'Speed', value: 'speed' },
  { label: 'Quality', value: 'quality' }
];

// Simple status message for user feedback
const status = ref<{ type: 'idle'|'success'|'error', text: string }>({ type: 'idle', text: '' });
function showStatus(text: string, type: 'success'|'error' = 'success') {
  status.value = { type, text };
  setTimeout(() => { status.value = { type: 'idle', text: '' }; }, 4000);
}

const canLaunch = computed(() => Boolean(deviceStore?.selectedDeviceId));

const launch = async () => {
  if (!deviceStore?.selectedDeviceId) {
    showStatus('Please select a device before launching.', 'error');
    return;
  }
  isLaunching.value = true;
  try {
    const options: SessionOptions = (profileStore?.activeProfile as any)?.options ?? {};
    await launchSession(deviceStore.selectedDeviceId, options);
    showStatus('Launch started.', 'success');
  } catch (err: any) {
    const msg = err?.message ?? String(err);
    showStatus(`Launch failed: ${msg}`, 'error');
  } finally {
    isLaunching.value = false;
  }
};

// Optional: bind quickProfile changes to future profile store integration
watch(quickProfile, (newVal) => {
  if (newVal) {
    profileStore.setActiveProfile(newVal);
  }
});
</script>

<template>
  <n-space vertical class="launch-view" style="width: 100%;">
    <n-card title="Launch View" bordered style="width: 100%;">
      <div class="launch-grid" style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;">
        <DeviceList />
        <SessionOptionsForm />
        <ProfileList />
      </div>

      <div style="display:flex; justify-content: space-between; align-items:center; margin-top: 12px;">
        <div style="display:flex; align-items:center; gap: 12px;">
          <n-select v-model:value="quickProfile" :options="quickProfileOptions" placeholder="Quick profile" style="width: 240px" />
        </div>
        <div style="display:flex; gap:8px;">
          <n-button type="primary" :loading="isLaunching" :disabled="!canLaunch" @click="launch">
            Launch
          </n-button>
          <n-button disabled>Launch All Selected</n-button>
        </div>
      </div>
      <div v-if="status.text" :class="['status', status.type]" style="margin-top: 12px;">
        {{ status.text }}
      </div>
    </n-card>
  </n-space>
</template>

<style scoped>
.launch-view { padding: 16px; }
.status { font-size: 14px; padding: 8px; border-radius: 6px; }
.status.success { color: #0f5132; background: #d1e7dd; }
.status.error { color: #842029; background: #f8d7da; }
.launch-grid { /* grid defined in template; no additional styles */ }
</style>
