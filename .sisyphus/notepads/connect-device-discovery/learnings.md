# Learnings

- Subscribing to Tauri events from a Pinia store is effective for real-time UI updates.
- Emitting a lightweight DOM event (device-update) from the store allows UI components to react with toasts without coupling to the store internals.
- Auto-refresh on app focus should be lightweight (no polling) to respect resources.
- Handling device disconnect during an active session requires coordinating with sessionStore to mark the session as Stopped with a helpful error.
- Prefer localStorage for simple persistence during development; guarded access ensures compatibility in non-TAURI environments.
