<template>
  <div class="home-view">
    <n-card title="ScrcpyX" class="main-card">
      <n-space vertical>
        <n-text>Welcome to ScrcpyX - GUI for scrcpy</n-text>
        <n-text depth="3">Connect your Android device and click Launch to start mirroring</n-text>
      </n-space>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { NCard, NSpace, NText } from 'naive-ui';
</script>

<style scoped>
.home-view {
  padding: 20px;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
}

.main-card {
  max-width: 600px;
  width: 100%;
}
</style>
