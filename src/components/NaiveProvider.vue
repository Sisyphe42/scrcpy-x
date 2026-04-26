<script setup lang="ts">
import { useMessage } from 'naive-ui';
import { setMessageInstance } from '../utils/notifications';
import { onMounted, onUnmounted } from 'vue';

const message = useMessage();

onMounted(() => {
  setMessageInstance(message);
});

onUnmounted(() => {
  setMessageInstance(null as any);
});
</script>

<template>
  <slot />
</template>
