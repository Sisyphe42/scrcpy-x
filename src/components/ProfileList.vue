<template>
  <div class="profile-list-container">
    <div v-if="profileList.length > 0" class="profile-cards">
      <div 
        v-for="p in profileList" 
        :key="p.name" 
        class="profile-card"
        :class="{ selected: selectedProfile === p.name, editing: editingProfile === p.name }"
      >
        <!-- Normal view -->
        <div v-if="editingProfile !== p.name" class="profile-compact" @click="selectProfile(p.name)">
          <div class="profile-header">
            <div class="profile-info">
              <n-radio :checked="selectedProfile === p.name" @click.stop />
              <span class="profile-name">{{ p.name }}</span>
              <n-tag v-if="p.is_default" type="success" size="small" class="default-tag">
                {{ t('profiles.defaultProfile') }}
              </n-tag>
            </div>
            <div class="profile-actions" @click.stop>
              <n-button size="tiny" quaternary @click.stop="startEdit(p)">
                <template #icon><CreateOutline /></template>
              </n-button>
              <n-button size="tiny" quaternary @click.stop="setDefault(p)">
                <template #icon><Star :style="{ opacity: p.is_default ? 1 : 0.4 }" /></template>
              </n-button>
              <n-button size="tiny" quaternary @click.stop="duplicateProfile(p)">
                <template #icon><CopyOutline /></template>
              </n-button>
              <n-button size="tiny" quaternary @click.stop="deleteProfile(p)">
                <template #icon><TrashOutline /></template>
              </n-button>
            </div>
          </div>
          <div class="profile-summary">
            <n-ellipsis :line-clamp="1" :tooltip="{ width: 400 }">
              {{ getOptionsSummary(p.options) }}
            </n-ellipsis>
          </div>
        </div>
        
        <!-- Editing view (just name) -->
        <div v-else class="profile-editing">
          <n-input 
            v-model:value="editingName" 
            size="small"
            :placeholder="t('profiles.namePlaceholder')"
            class="name-input"
            @keyup.enter="saveEdit"
            @keyup.escape="cancelEdit"
          />
          <div class="edit-actions">
            <n-button size="tiny" type="primary" @click="saveEdit">
              <template #icon><Checkmark /></template>
            </n-button>
            <n-button size="tiny" @click="cancelEdit">
              <template #icon><Close /></template>
            </n-button>
          </div>
        </div>
      </div>
    </div>
    
    <div v-else class="empty-state">
      <n-empty :description="t('profiles.noProfiles')">
        <template #extra>
          <n-button size="small" type="primary" @click="createNewProfile" style="margin-top: 12px;">
            {{ t('profiles.newProfile') }}
          </n-button>
        </template>
      </n-empty>
    </div>
    
    <n-button 
      v-if="profileList.length > 0"
      size="small" 
      type="primary"
      ghost
      class="create-btn" 
      @click="createNewProfile"
    >
      <template #icon><Add /></template>
      {{ t('profiles.newProfile') }}
    </n-button>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { NTag, NButton, NEllipsis, NEmpty, NInput, NRadio, useMessage } from 'naive-ui';
import { 
  Add as Add, 
  CreateOutline, 
  CopyOutline, 
  TrashOutline,
  Star,
  Checkmark,
  Close
} from '@vicons/ionicons5';
import { useProfileStore } from '../stores/profileStore';
import type { Profile } from '../types/profile';
import type { SessionOptions } from '../types/session';
import { getProfiles, deleteProfile as apiDeleteProfile, saveProfile as apiSaveProfile } from '../api/profiles';

const { t } = useI18n();
const store = useProfileStore();
const message = useMessage();

const profileList = computed(() => store.profiles);

// Selected profile (for applying to session options)
const selectedProfile = ref<string | null>(null);

// Editing state (just for renaming)
const editingProfile = ref<string | null>(null);
const editingName = ref('');

// Sync selected profile with store's active profile
watch(() => store.activeProfileName, (name) => {
  if (name) {
    selectedProfile.value = name;
  }
}, { immediate: true });

onMounted(async () => {
  try {
    await getProfiles();
    // Select default profile on mount
    if (store.defaultProfile) {
      selectProfile(store.defaultProfile.name);
    }
  } catch (err) {
    message.error(t('common.error'));
  }
});

function selectProfile(name: string) {
  selectedProfile.value = name;
  store.setActiveProfile(name);
}

function startEdit(p: Profile) {
  editingProfile.value = p.name;
  editingName.value = p.name;
}

function cancelEdit() {
  editingProfile.value = null;
  editingName.value = '';
}

async function saveEdit() {
  if (!editingName.value.trim()) {
    message.warning(t('profiles.nameRequired'));
    return;
  }
  
  const originalName = editingProfile.value;
  const profile = profileList.value.find(p => p.name === originalName);
  
  if (!profile) {
    cancelEdit();
    return;
  }
  
  const updated: Profile = {
    ...profile,
    name: editingName.value.trim()
  };
  
  try {
    // If name changed, delete old profile first
    if (originalName && originalName !== updated.name) {
      await apiDeleteProfile(originalName);
    }
    
    await apiSaveProfile(updated);
    
    // Update selection if this was the selected profile
    if (selectedProfile.value === originalName) {
      selectProfile(updated.name);
    }
    
    cancelEdit();
    message.success(t('profiles.profileSaved'));
  } catch (err) {
    message.error(t('profiles.saveFailed'));
  }
}

async function createNewProfile() {
  const newProfile: Profile = {
    name: `Profile ${profileList.value.length + 1}`,
    is_default: profileList.value.length === 0,
    options: {}
  };
  
  try {
    await apiSaveProfile(newProfile);
    selectProfile(newProfile.name);
    message.success(t('profiles.profileSaved'));
  } catch (err) {
    message.error(t('profiles.saveFailed'));
  }
}

async function deleteProfile(p: Profile) {
  try {
    await apiDeleteProfile(p.name);
    if (selectedProfile.value === p.name) {
      selectedProfile.value = null;
    }
    message.success(t('profiles.profileDeleted'));
  } catch (err) {
    message.error(t('profiles.deleteFailed'));
  }
}

async function duplicateProfile(p: Profile) {
  const dup: Profile = { 
    ...p, 
    options: { ...p.options }, 
    name: `${p.name} (${t('profiles.duplicateProfile').toLowerCase()})`, 
    is_default: false 
  };
  
  try {
    await apiSaveProfile(dup);
    message.success(t('profiles.profileDuplicated'));
  } catch (err) {
    message.error(t('profiles.saveFailed'));
  }
}

async function setDefault(p: Profile) {
  const updated: Profile = { ...p, is_default: !p.is_default };
  
  try {
    await apiSaveProfile(updated);
  } catch (err) {
    message.error(t('profiles.saveFailed'));
  }
}

function getOptionsSummary(opts: SessionOptions | undefined): string {
  if (!opts || Object.keys(opts).length === 0) {
    return t('profiles.noOptions');
  }
  
  const parts: string[] = [];
  const s = (key: string) => t(`profiles.summary.${key}`);
  
  if (opts.videoCodec) parts.push(`${s('codec')}: ${opts.videoCodec}`);
  if (opts.videoBitrate) parts.push(`${s('bitrate')}: ${opts.videoBitrate}Mbps`);
  if (opts.maxSize) parts.push(`${s('max')}: ${opts.maxSize}px`);
  if (opts.maxFps) parts.push(`${opts.maxFps} FPS`);
  if (opts.crop) parts.push(`${s('crop')}: ${opts.crop}`);
  if (opts.audio === false) parts.push(`${s('audio')}: ${s('off')}`);
  else if (opts.audioCodec) parts.push(`${s('audio')}: ${opts.audioCodec}`);
  if (opts.record) parts.push(`${s('recording')}: ${opts.record}`);
  if (opts.fullscreen) parts.push(s('fullscreen'));
  if (opts.alwaysOnTop) parts.push(s('alwaysOnTop'));
  if (opts.borderless) parts.push(s('borderless'));
  if (opts.turnScreenOff) parts.push(s('screenOff'));
  if (opts.stayAwake) parts.push(s('stayAwake'));
  if (opts.showTouches) parts.push(s('showTouches'));
  if (opts.keyboard) parts.push(`${s('keyboard')}: ${opts.keyboard}`);
  if (opts.mouse) parts.push(`${s('mouse')}: ${opts.mouse}`);
  if (opts.shortcutMod) parts.push(`${s('mod')}: ${opts.shortcutMod}`);
  if (opts.tcpip) parts.push(`${s('tcpip')}: ${opts.tcpip}`);
  
  return parts.length > 0 ? parts.join(' • ') : t('profiles.defaultOptions');
}
</script>

<style scoped>
.profile-list-container { 
  min-height: 200px;
}

.profile-cards {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.profile-card {
  border: 1px solid var(--n-border-color, rgba(0, 0, 0, 0.1));
  border-radius: 8px;
  transition: all 0.2s ease;
  overflow: hidden;
}

.profile-card:hover {
  border-color: #18a058;
}

.profile-card.selected {
  border-color: #18a058;
  box-shadow: 0 2px 8px rgba(24, 160, 88, 0.1);
}

.profile-compact {
  padding: 10px 12px;
  cursor: pointer;
}

.profile-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.profile-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.profile-name {
  font-size: 14px;
  font-weight: 600;
}

.default-tag {
  font-size: 11px;
}

.profile-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.profile-card:hover .profile-actions,
.profile-card.selected .profile-actions {
  opacity: 1;
}

.profile-summary {
  margin-top: 6px;
  margin-left: 24px;
  font-size: 11px;
  opacity: 0.6;
}

/* Editing view */
.profile-editing {
  padding: 10px 12px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.name-input {
  flex: 1;
}

.edit-actions {
  display: flex;
  gap: 4px;
}

.empty-state {
  padding: 20px;
  text-align: center;
}

.create-btn {
  margin-top: 12px;
  width: 100%;
}
</style>
