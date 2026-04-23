<template>
  <n-input
    v-model:value="text"
    type="textarea"
    :rows="6"
    placeholder='{"key":"value"}'
  />
</template>

<script lang="ts">
import { defineComponent, ref, watch } from 'vue';
import { NInput } from 'naive-ui';
export default defineComponent({
  name: 'SessionOptionsForm',
  components: { NInput },
  props: {
    options: {
      type: Object as () => Record<string, unknown>,
      default: () => ({}),
    },
  },
  emits: ['update'],
  setup(props, { emit }) {
    const text = ref(JSON.stringify(props.options ?? {}, null, 2));

    watch(text, (val) => {
      try {
        const parsed = JSON.parse(val);
        emit('update', parsed);
      } catch {
        // ignore invalid JSON during typing
      }
    });

    return { text };
  },
});
</script>
