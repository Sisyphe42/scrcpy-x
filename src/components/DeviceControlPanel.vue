<template>
  <div class="device-control-panel" v-if="store.activeSession">
    <n-space vertical size="md" class="panel" style="width: 100%;">
      <!-- Session status header -->
      <div class="panel-header" style="width: 100%; display: flex; align-items: center; gap: 8px;">
        <span style="font-weight: 600;">Session</span>
        <n-tag :type="tagType" size="small">{{ statusText }}</n-tag>
        <span style="margin-left: auto; font-size: 12px; color: var(--naive-text-color-secondary);">ID: {{ store.activeSession.id }}</span>
      </div>

      <!-- Two-column control layout -->
      <div class="panel-grid" style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
        <!-- Rotation / Fullscreen / Screenshot -->
        <div class="panel-card" style="padding: 12px; border: 1px solid var(--naive-border-color); border-radius: 8px;">
          <n-space vertical size="xs">
            <div class="section-title" style="font-size: 14px; font-weight: 600; color: var(--naive-text-color-secondary);">Rotation</div>
            <div class="rotation-buttons" style="display:flex; flex-wrap: wrap; gap: 6px;">
              <n-button size="small" @click="rotate(0)" :loading="loading.rotate">0°</n-button>
              <n-button size="small" @click="rotate(90)" :loading="loading.rotate">90°</n-button>
              <n-button size="small" @click="rotate(180)" :loading="loading.rotate">180°</n-button>
              <n-button size="small" @click="rotate(270)" :loading="loading.rotate">270°</n-button>
            </div>
            <n-button size="small" @click="toggleFullscreen" :loading="loading.fullscreen" style="margin-top: 6px;">Fullscreen</n-button>
            <n-button size="small" @click="takeScreenshot" :loading="loading.screenshot" style="margin-top: 6px;">Screenshot</n-button>
          </n-space>
        </div>

        <!-- Volume + Navigation + Screen On/Off -->
        <div class="panel-card" style="padding: 12px; border: 1px solid var(--naive-border-color); border-radius: 8px;">
          <n-space vertical size="xs">
            <div class="section-title" style="font-size: 14px; font-weight: 600; color: var(--naive-text-color-secondary);">Volume</div>
            <n-slider v-model:value="volume" :min="0" :max="100" :step="1" />
            <div style="display:flex; gap:8px; align-items:center; margin-top:6px;">
              <span style="min-width: 60px; text-align:right; opacity:.75;">{{ volume }}%</span>
            </div>
            <div style="display:flex; gap:8px; align-items:center; margin-top:6px;">
              <n-button size="small" @click="pressKeyCode('BACK')" :loading="loading.nav">Back</n-button>
              <n-button size="small" @click="pressKeyCode('HOME')" :loading="loading.nav">Home</n-button>
              <n-button size="small" @click="pressKeyCode('APP_SWITCH')" :loading="loading.nav">Recents</n-button>
            </div>
            <div style="display:flex; gap:8px; align-items:center; margin-top:6px;">
              <n-button size="small" @click="turnOffScreen" :loading="loading.screenOff">Screen Off</n-button>
              <n-button size="small" @click="turnOnScreen" :loading="loading.screenOn">Screen On</n-button>
            </div>
          </n-space>
        </div>
      </div>
    </n-space>
  </div>

  <div v-else class="no-session" style="padding: 16px; color: var(--naive-text-color-secondary);">
    No active session. Please connect to a device.
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, reactive } from 'vue'
import { useSessionStore } from '../stores/sessionStore'
import { sendKeyEvent, takeScreenshot, setRotation, setVolume, turnScreenOn, turnScreenOff } from '../api/sessions'

// Global store to access active session information
const store = useSessionStore()

// Current session identifier
const sessionId = computed<string>(() => store.activeSession?.id ?? '')

const statusText = computed<string>(() => store.activeSession?.status ?? 'Disconnected')
const tagType = computed<'default'|'success'|'info'|'warning'|'error'>(() => {
  const s = store.activeSession?.status
  switch (s) {
    case 'Running': return 'success'
    case 'Starting': return 'info'
    case 'Stopped': return 'default'
    case 'Error': return 'error'
    default: return 'default'
  }
})

// Small in-component loading indicators for async actions
const loading = reactive({
  rotate: false,
  fullscreen: false,
  screenshot: false,
  nav: false,
  screenOff: false,
  screenOn: false,
})

function setLoading(key: keyof typeof loading, value: boolean) {
  loading[key] = value
}

async function rotate(angle: number) {
  if (!sessionId.value) return
  setLoading('rotate', true)
  try {
    await setRotation(sessionId.value, angle)
  } finally {
    setLoading('rotate', false)
  }
}

async function toggleFullscreen() {
  if (!sessionId.value) return
  setLoading('fullscreen', true)
  try {
    // Use F11 as a generic fullscreen toggle on the device/scrcpy window
    await sendKeyEvent(sessionId.value, 'F11')
  } finally {
    setLoading('fullscreen', false)
  }
}

async function takeScreenshot() {
  if (!sessionId.value) return
  setLoading('screenshot', true)
  try {
    await takeScreenshot(sessionId.value)
  } finally {
    setLoading('screenshot', false)
  }
}

// Volume control via slider changes
const volume = ref<number>(50)
watch(volume, async (newVal) => {
  if (!sessionId.value) return
  try {
    await setVolume(sessionId.value, newVal)
  } catch {
    // ignore non-blocking errors
  }
})

async function pressKeyCode(code: string) {
  if (!sessionId.value) return
  setLoading('nav', true)
  try {
    await sendKeyEvent(sessionId.value, code)
  } finally {
    setLoading('nav', false)
  }
}

async function turnOffScreen() {
  if (!sessionId.value) return
  setLoading('screenOff', true)
  try {
    await turnScreenOff(sessionId.value)
  } finally {
    setLoading('screenOff', false)
  }
}

async function turnOnScreen() {
  if (!sessionId.value) return
  setLoading('screenOn', true)
  try {
    await turnScreenOn(sessionId.value)
  } finally {
    setLoading('screenOn', false)
  }
}
</script>

<style scoped>
.device-control-panel { padding: 12px; }
.panel-header { display: flex; align-items: center; gap: 8px; }
.panel-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; }
.section-title { font-weight: 600; color: var(--naive-text-color-secondary); }
.no-session { padding: 16px; color: var(--naive-text-color-secondary); }
</style>
