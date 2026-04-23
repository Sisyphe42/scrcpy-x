<template>
  <div class="profile-management">
    <n-list bordered class="profile-list" style="max-width: 780px;">
      <n-list-item v-for="p in profileList" :key="p.name" style="display:flex; align-items:center; justify-content: space-between;">
        <div class="profile-row" style="display:flex; align-items:center; gap:12px;">
          <span class="name" style="font-weight:600;">{{ p.name }}</span>
          <n-tag v-if="p.is_default" type="success" size="small" style="margin-left:6px;">Default</n-tag>
        </div>
        <div class="actions" style="display:flex; gap:6px;">
          <n-button size="small" @click="editProfile(p)">Edit</n-button>
          <n-button size="small" @click="duplicateProfile(p)">Duplicate</n-button>
          <n-button size="small" @click="deleteProfile(p)">Delete</n-button>
        </div>
      </n-list-item>
    </n-list>
    <n-button style="margin-top:12px" @click="createProfile">New Profile</n-button>

    <ProfileEditor
      :modelValue="editingProfile"
      :visible="editorVisible"
      @update:visible="(val: boolean) => editorVisible = val"
      @save="onEditorSave"
    />
  </div>
 </template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { NList, NListItem, NTag, NButton } from 'naive-ui';
import { useProfileStore } from '../stores/profileStore';
import type { Profile } from '../types/profile';
import ProfileEditor from './ProfileEditor.vue';

const store = useProfileStore();

const profileList = computed(() => store.profiles);

const editorVisible = ref(false);
const editingProfile = ref<Profile>({ id: '', name: '', is_default: false, options: {} });

function editProfile(p: Profile) {
  editingProfile.value = { ...p };
  editorVisible.value = true;
}

function createProfile() {
  editingProfile.value = { id: '', name: '', is_default: false, options: {} };
  editorVisible.value = true;
}

function onEditorSave(updated: Profile) {
  store.saveProfile(updated);
}

function deleteProfile(p: Profile) {
  store.deleteProfile(p.name);
}

function duplicateProfile(p: Profile) {
  const dup: Profile = { ...p, id: '', name: p.name + ' (copy)', is_default: false };
  store.saveProfile(dup);
}
</script>

<style scoped>
.profile-management { padding: 16px; }
.profile-list { max-width: 780px; }
.profile-row { display: flex; align-items: center; }
.name { font-size: 14px; }
.actions { display: flex; gap: 6px; }
</style>
