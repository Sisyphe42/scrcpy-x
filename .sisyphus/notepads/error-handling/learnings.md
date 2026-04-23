Summary of learnings from implementing error handling and notifications:
- Created a lightweight error boundary using Vue 3's onErrorCaptured to convert backend ScrcpyError codes into user-friendly messages via Naive UI notifications.
- Implemented a small errors.ts type system to model backend errors and provide safe, user-friendly mappings for UI.
- Built a notifications.ts wrapper around Naive UI's useMessage to standardize showXxx helpers and support optional retry actions via onClick callbacks.
- Bound a global retry signal scrcpy-retry to re-enter recovery flow where appropriate, enabling recoverable errors to offer a retry prompt.
- Ensured raw backend messages are not exposed; messages are always user-friendly.

Next steps:
- Add unit tests for error mapping functions and notification helpers.
- Wire an actual retry mechanism to trigger mirroring restart from the UI.
