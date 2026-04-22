import { invoke } from '@tauri-apps/api/core';
import type { Profile } from '../types';
import { useProfileStore } from '../stores/profileStore';

// Get all profiles from backend
export async function getProfiles(): Promise<Profile[]> {
  try {
    const profiles = await invoke<Profile[]>('get_profiles');
    const store = useProfileStore();
    if (Array.isArray(profiles)) {
      store.loadProfiles(profiles);
    }
    return profiles;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to fetch profiles: ${msg}`);
  }
}

// Get a single profile by name
export async function getProfile(name: string): Promise<Profile | null> {
  try {
    const profile = await invoke<Profile | null>('get_profile', { name });
    return profile;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to fetch profile '${name}': ${msg}`);
  }
}

// Save or update a profile
export async function saveProfile(profile: Profile): Promise<void> {
  try {
    await invoke<void>('save_profile', { profile });
    const store = useProfileStore();
    store.saveProfile(profile);
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to save profile '${profile.name}': ${msg}`);
  }
}

// Delete a profile by name
export async function deleteProfile(name: string): Promise<void> {
  try {
    await invoke<void>('delete_profile', { name });
    const store = useProfileStore();
    store.deleteProfile(name);
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to delete profile '${name}': ${msg}`);
  }
}

// Set a default profile by name
export async function setDefaultProfile(name: string): Promise<void> {
  try {
    await invoke<void>('set_default_profile', { name });
    const store = useProfileStore();
    store.setActiveProfile(name);
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to set default profile '${name}': ${msg}`);
  }
}
