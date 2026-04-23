# Profile Management Task — Issues
- Potential blocker: Missing profileStore API methods (getProfiles, saveProfile, updateProfile, deleteProfile, duplicateProfile). Ensure the store provides these actions or adjust UI accordingly.
- May need to adjust imports if project uses a different store path (e.g., '@/stores/profile' vs './store/profile').
- SessionOptionsForm JSON handling could fail if user pastes invalid JSON; ensure UX communicates parsing status.
