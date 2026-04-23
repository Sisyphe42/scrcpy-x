# Issues

- [ ] Ensure TS runtime compatibility for tauri listen in all environments (tests may run in non-TAURI contexts).
- [ ] Validate that device-update CustomEvent payloads are always well-formed across Rust backend changes.
- [ ] Consider unlistening on app shutdown; current approach leaks listeners in long-running apps.
- [ ] Add tests to verify that disconnect during active session triggers session status update.
