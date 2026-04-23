# Decisions

- Adopt event-driven device discovery via Tauri's listen API and sync with Pinia store.
- Use a DOM CustomEvent (device-update) to propagate device changes to UI for toasts without coupling the UI to store internals.
- Implement auto-refresh on app focus to keep device list fresh without polling.
- On device disconnect, propagate a sessionStatus update to stop an active session and surface an error message for the user.
