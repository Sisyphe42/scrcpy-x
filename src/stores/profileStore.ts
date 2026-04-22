import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Profile } from '../types';

const PROFILES_KEY = 'scrcpyx-profiles';
const ACTIVE_PROFILE_KEY = 'scrcpyx-activeProfile';

function loadProfilesFromStorage(): Profile[] {
  try {
    const raw = typeof localStorage !== 'undefined' ? localStorage.getItem(PROFILES_KEY) : null;
    if (raw) return JSON.parse(raw) as Profile[];
  } catch {
    // ignore
  }
  return [];
}

function persistProfiles(profiles: Profile[]) {
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(PROFILES_KEY, JSON.stringify(profiles));
    }
  } catch {
    // ignore
  }
}

function loadActiveProfileName(): string | null {
  try {
    const raw = typeof localStorage !== 'undefined' ? localStorage.getItem(ACTIVE_PROFILE_KEY) : null;
    return raw ?? null;
  } catch {
    return null;
  }
}

function persistActiveProfileName(name: string | null) {
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(ACTIVE_PROFILE_KEY, name ?? '');
    }
  } catch {
    // ignore
  }
}

export const useProfileStore = defineStore('profile', () => {
  const profiles = ref<Profile[]>(loadProfilesFromStorage());
  const activeProfileName = ref<string | null>(loadActiveProfileName());

  const activeProfile = computed<Profile | null>(() => {
    if (!activeProfileName.value) return null;
    return profiles.value.find(p => p.name === activeProfileName.value) ?? null;
  });
  const defaultProfile = computed<Profile | null>(() => profiles.value.find(p => p.isDefault) ?? null);

  function loadProfiles(newProfiles: Profile[]) {
    profiles.value = newProfiles;
    persistProfiles(profiles.value);
  }

  function saveProfile(p: Profile) {
    const idx = profiles.value.findIndex(pp => pp.name === p.name);
    if (idx >= 0) {
      profiles.value[idx] = p;
    } else {
      profiles.value.push(p);
    }
    persistProfiles(profiles.value);
  }

  function deleteProfile(name: string) {
    profiles.value = profiles.value.filter(p => p.name !== name);
    if (activeProfileName.value === name) activeProfileName.value = null;
    persistProfiles(profiles.value);
  }

  function setActiveProfile(name: string | null) {
    activeProfileName.value = name;
    persistActiveProfileName(name);
  }

  return {
    profiles,
    activeProfileName,
    activeProfile,
    defaultProfile,
    loadProfiles,
    saveProfile,
    deleteProfile,
    setActiveProfile,
  };
});
