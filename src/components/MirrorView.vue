<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSessionStore } from '../stores/sessionStore'
import { invoke } from '@tauri-apps/api/core'

const { t } = useI18n()
const store = useSessionStore()

const frameSrc = ref<string>('')
const loading = ref(false)
const error = ref<string | null>(null)
const fps = ref(1) // Default 1 FPS for screencap polling
let pollTimer: ReturnType<typeof setInterval> | null = null

const deviceId = computed(() => store.activeSession?.deviceId ?? '')
const hasActiveSession = computed(() => !!store.activeSession && store.activeSession.status === 'Running')

async function captureFrame() {
  if (!deviceId.value) return
  loading.value = true
  try {
    const dataUri = await invoke<string>('screencap_base64', { deviceId: deviceId.value })
    frameSrc.value = dataUri
    error.value = null
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err)
    error.value = msg
  } finally {
    loading.value = false
  }
}

function startPolling() {
  stopPolling()
  if (!hasActiveSession.value) return
  captureFrame() // Immediate first capture
  pollTimer = setInterval(captureFrame, Math.max(200, Math.round(1000 / fps.value)))
}

function stopPolling() {
  if (pollTimer) {
    clearInterval(pollTimer)
    pollTimer = null
  }
}

watch(hasActiveSession, (active) => {
  if (active) {
    startPolling()
  } else {
    stopPolling()
    frameSrc.value = ''
  }
})

watch(fps, () => {
  if (hasActiveSession.value) {
    startPolling()
  }
})

onMounted(() => {
  if (hasActiveSession.value) {
    startPolling()
  }
})

onUnmounted(() => {
  stopPolling()
})
</script>

<template>
  <div class="mirror-view">
    <div class="mirror-header">
      <span class="mirror-title">{{ t('mirror.title') }}</span>
      <div class="mirror-controls">
        <label class="fps-control">
          {{ t('mirror.fps') }}:
          <input type="range" v-model.number="fps" min="0.5" max="4" step="0.5" class="fps-slider" />
          <span class="fps-value">{{ fps }} fps</span>
        </label>
      </div>
    </div>

    <div class="mirror-frame-container" v-if="hasActiveSession">
      <img
        v-if="frameSrc"
        :src="frameSrc"
        alt="Device Screen"
        class="mirror-frame"
        :class="{ loading }"
      />
      <div v-if="!frameSrc && !error" class="mirror-placeholder">
        {{ loading ? t('common.loading') : t('mirror.connecting') }}
      </div>
      <div v-if="error" class="mirror-error">
        {{ error }}
      </div>
    </div>

    <div v-else class="mirror-no-session">
      {{ t('controls.noSession') }}
    </div>
  </div>
</template>

<style scoped>
.mirror-view {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.mirror-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.mirror-title {
  font-weight: 600;
  font-size: 14px;
}

.mirror-controls {
  display: flex;
  align-items: center;
  gap: 8px;
}

.fps-control {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  opacity: 0.7;
}

.fps-slider {
  width: 60px;
}

.fps-value {
  min-width: 50px;
  text-align: right;
}

.mirror-frame-container {
  position: relative;
  width: 100%;
  background: #000;
  border-radius: 8px;
  overflow: hidden;
  aspect-ratio: 9 / 16;
  display: flex;
  align-items: center;
  justify-content: center;
}

.mirror-frame {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  transition: opacity 0.15s ease;
}

.mirror-frame.loading {
  opacity: 0.7;
}

.mirror-placeholder,
.mirror-error {
  color: rgba(255, 255, 255, 0.6);
  font-size: 13px;
  text-align: center;
  padding: 20px;
}

.mirror-error {
  color: #e88080;
}

.mirror-no-session {
  text-align: center;
  padding: 20px;
  opacity: 0.6;
  font-size: 13px;
}
</style>
