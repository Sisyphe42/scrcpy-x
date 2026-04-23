Decision log for error handling integration:
- Use a dedicated errors.ts to model backend error codes and provide a stable API for UI mapping.
- Implement a global error boundary at App.vue using Vue 3's onErrorCaptured to standardize error handling.
- Create a notifications.ts wrapper for Naive UI to ensure consistent UX across errors, warnings, and infos.
- Map known backend errors to user-friendly messages, keep a recoverability table to determine if a retry is appropriate, and surface retry actions when applicable.
- Do not expose raw backend error messages; always present a friendly, actionable message.
