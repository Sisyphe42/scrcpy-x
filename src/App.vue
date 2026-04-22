  
<script setup lang="ts">
import { onErrorCaptured, onMounted, onUnmounted } from 'vue';
import { NConfigProvider, NMessageProvider, NDialogProvider } from 'naive-ui';
import { isScrcpyError, getErrorMessage } from './utils/errors';
import { showError } from './utils/notifications';
import { handleKeyboard } from './utils/shortcuts';

// Global error boundary: capture errors from child components and present user-friendly messages
onMounted(() => {
  // Register global keyboard shortcuts
  window.addEventListener('keydown', handleKeyboard);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyboard);
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
