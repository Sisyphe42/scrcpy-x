<template>
  <div class="profile-management">
    <n-list bordered class="profile-list" style="max-width: 780px;">
      <n-list-item v-for="p in profiles" :key="p.id" style="display:flex; align-items:center; justify-content: space-between;">
        <div class="profile-row" style="display:flex; align-items:center; gap:12px;">
          <span class="name" style="font-weight:600;">{{ p.name }}</span>
          <n-tag v-if="p.is_default" type="success" size="small" style="margin-left:6px;">Default</n-tag>
        </div>
        <div class="actions" style="display:flex; gap:6px;">
          <n-button size="small" @click="editProfile(p)">Edit</n-button>
          <n-button size="small" @click="duplicateProfile(p)">Duplicate</n-button>
          <n-button size="small" danger @click="deleteProfile(p)">Delete</n-button>
        </div>
      </n-list-item>
    </n-list>
    <n-button style="margin-top:12px" @click="createProfile">New Profile</n-button>

    <ProfileEditor
      :modelValue="editingProfile"
      :visible="editorVisible"
      @update:visible="val => editorVisible = val"
      @save="onEditorSave"
    />
  </div>
 </template>

<script lang="ts">
import { defineComponent, ref, onMounted } from 'vue';
import { NList, NListItem, NTag, NButton } from 'naive-ui';
import { useProfileStore } from '@/stores/profile';
import type { Profile } from '@/types/profile';
import ProfileEditor from './ProfileEditor.vue';

export default defineComponent({
  name: 'ProfileList',
  components: { NList, NListItem, NTag, NButton, ProfileEditor },
  setup() {
    const store = useProfileStore();
    const profiles = ref<Profile[]>([]);
    const editorVisible = ref(false);
    const editingProfile = ref<Profile>({ id: '', name: '', is_default: false, options: {} });

    onMounted(async () => {
      profiles.value = await store.getProfiles();
    });

    const editProfile = (p: Profile) => {
      editingProfile.value = { ...p };
      editorVisible.value = true;
    };

    const createProfile = () => {
      editingProfile.value = { id: '', name: '', is_default: false, options: {} };
      editorVisible.value = true;
    };

    const onEditorSave = async (updated: Profile) => {
      if (updated.id && updated.id.trim() !== '') {
        await store.updateProfile(updated);
      } else {
        await store.saveProfile(updated);
      }
      profiles.value = await store.getProfiles();
    };

    const deleteProfile = async (p: Profile) => {
      await store.deleteProfile(p.id);
      profiles.value = await store.getProfiles();
    };

    const duplicateProfile = async (p: Profile) => {
      await store.duplicateProfile(p.id);
      profiles.value = await store.getProfiles();
    };

    return { profiles, editorVisible, editingProfile, editProfile, createProfile, onEditorSave, deleteProfile, duplicateProfile };
  },
});
</script>

<style scoped>
.profile-management { padding: 16px; }
.profile-list { max-width: 780px; }
.profile-row { display: flex; align-items: center; }
.name { font-size: 14px; }
.actions { display: flex; gap: 6px; }
</style>
