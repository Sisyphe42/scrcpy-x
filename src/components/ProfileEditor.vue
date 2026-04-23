<template>
  <n-modal :show="visible" @update:show="v => emit('update:visible', v)" title="Profile Editor" width="560">
    <div style="padding: 16px;">
      <n-form label-align="left" label-width="120">
        <n-form-item label="Name">
          <n-input v-model:value="localProfile.name" placeholder="Profile name" />
        </n-form-item>
        <n-form-item label="Set as Default">
          <n-switch v-model:value="localProfile.is_default" />
        </n-form-item>
        <n-form-item label="Options">
          <SessionOptionsForm :options="localProfile.options" @update="opts => localProfile.options = opts" />
        </n-form-item>
      </n-form>
    </div>
    <template #footer>
      <n-button size="small" @click="onCancel">Cancel</n-button>
      <n-button size="small" type="primary" @click="onSave" style="margin-left: 8px">Save</n-button>
    </template>
  </n-modal>
 </template>

<script lang="ts">
import { defineComponent, ref, watch, toRef } from 'vue';
import { NModal, NForm, NFormItem, NInput, NSwitch, NButton } from 'naive-ui';
import type { Profile } from '../types/profile';
import SessionOptionsForm from './SessionOptionsForm.vue';

export default defineComponent({
  name: 'ProfileEditor',
  components: { NModal, NForm, NFormItem, NInput, NSwitch, NButton, SessionOptionsForm },
  props: {
    modelValue: { type: Object as () => Profile, required: true },
    visible: { type: Boolean, required: true },
  },
  emits: ['update:visible', 'save'],
  setup(props, { emit }) {
    const localProfile = ref<Profile>({ ...props.modelValue });
    const visible = toRef(props, 'visible');

    // React to external model changes
    watch(
      () => props.modelValue,
      (newVal) => {
        localProfile.value = { ...newVal };
      },
      { immediate: true }
    );

    // Keep localProfile in sync when modal is closed without saving
    watch(visible, (val) => {
      if (!val) {
        localProfile.value = { ...props.modelValue };
      }
    });

    const onCancel = () => emit('update:visible', false);
    const onSave = () => {
      emit('save', { ...localProfile.value });
      emit('update:visible', false);
    };

    return { localProfile, visible, onCancel, onSave, emit };
  },
});
</script>

<style scoped>
</style>
