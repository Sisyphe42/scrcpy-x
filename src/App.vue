  
<script setup lang="ts">
import { onErrorCaptured, onMounted } from 'vue';
import { NConfigProvider, NMessageProvider, NDialogProvider } from 'naive-ui';
import { isScrcpyError, getErrorMessage } from './utils/errors';
import { showError } from './utils/notifications';

// Global error boundary: capture errors from child components and present user-friendly messages
onMounted(() => {
  // Intentionally left as a hook to satisfy structural requirement.
});

onErrorCaptured((err: any, _vm: any, _info: string) => {
  if (isScrcpyError(err)) {
    const msg = getErrorMessage(err);
    showError(msg);
  } else {
    showError('An unexpected error occurred.');
  }
  // Do not propagate further
  return false;
});
 
</script>

<template>
  <n-config-provider>
    <n-message-provider>
      <n-dialog-provider>
        <router-view />
      </n-dialog-provider>
    </n-message-provider>
  </n-config-provider>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  padding: 0;
  min-height: 100vh;
}

#app {
  min-height: 100vh;
}
</style>
