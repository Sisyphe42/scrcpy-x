<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { useI18n } from 'vue-i18n';
import { NLayout, NLayoutSider, NLayoutContent, NMenu } from 'naive-ui';
import type { MenuOption } from 'naive-ui';

const router = useRouter();
const route = useRoute();
const { t } = useI18n();
const collapsed = ref(false);

const menuOptions: MenuOption[] = [
  { label: t('nav.devices'), key: 'devices' },
  { label: t('nav.profiles'), key: 'profiles' },
  { label: t('nav.settings'), key: 'settings' },
];

const activeKey = computed(() => route.name as string);

function handleMenuSelect(key: string) {
  router.push({ name: key });
}
</script>

<template>
  <n-layout has-sider style="height: 100vh;">
    <n-layout-sider
      bordered
      collapse-mode="width"
      :collapsed-width="64"
      :width="200"
      :collapsed="collapsed"
      show-trigger
      @collapse="collapsed = true"
      @expand="collapsed = false"
    >
      <n-menu
        :options="menuOptions"
        :value="activeKey"
        @update:value="handleMenuSelect"
      />
    </n-layout-sider>
    <n-layout-content>
      <router-view />
    </n-layout-content>
  </n-layout>
</template>
