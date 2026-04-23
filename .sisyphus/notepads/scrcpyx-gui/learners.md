## Learnings from Task: Connect Stores to Backend Commands
- Scanned existing Pinia stores: deviceStore.ts, sessionStore.ts, profileStore.ts, settingsStore.ts to align types and APIs.
- Implemented Tauri API wrappers under src/api/ for devices, sessions, profiles, and settings using @tauri-apps/api/core invoke with proper generics.
- Added a bulk loader in sessionStore.ts (setSessions) to support syncing sessions from backend APIs.
- Ensured wrappers update corresponding stores after successful backend calls (setDevices, addSession, loadProfiles, updateSettings).
- Exported wrappers via src/api/index.ts for centralized import.
- Verified type usage against src/types (Device, Session, Profile, AppSettings, SessionOptions).
- Next steps: run TS compile, ensure tests/build pass, and verify runtime behavior with actual Tauri commands.
