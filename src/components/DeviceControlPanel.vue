<template>
  <div class="device-control-panel" v-if="store.activeSession" tabindex="0">
    <div class="panel">
      <!-- Session status header -->
      <div class="panel-header">
        <span style="font-weight: 600;">{{ t('controls.title') }}</span>
        <n-tag :type="tagType" size="small">{{ statusText }}</n-tag>
        <span class="session-id">ID: {{ store.activeSession.id }}</span>
      </div>

      <!-- Shortcuts help -->
      <div class="shortcuts-help">
        {{ t('controls.shortcuts') }}
      </div>

      <!-- Two-column control layout -->
      <div class="panel-grid">
        <!-- Rotation / Fullscreen / Screenshot -->
        <div class="panel-card">
          <div class="section-title">{{ t('controls.rotation') }}</div>
          <div class="rotation-buttons">
            <n-button size="small" @click="rotate(0)" :loading="loading.rotate">0°</n-button>
            <n-button size="small" @click="rotate(90)" :loading="loading.rotate">90°</n-button>
            <n-button size="small" @click="rotate(180)" :loading="loading.rotate">180°</n-button>
            <n-button size="small" @click="rotate(270)" :loading="loading.rotate">270°</n-button>
          </div>
          <div class="button-row">
            <n-button size="small" @click="toggleFullscreen" :loading="loading.fullscreen">{{ t('controls.fullscreen') }}</n-button>
            <n-button size="small" @click="doScreenshot" :loading="loading.screenshot">{{ t('controls.screenshot') }}</n-button>
          </div>
        </div>

        <!-- Volume + Navigation + Screen On/Off -->
        <div class="panel-card">
          <div class="section-title">{{ t('controls.volume') }}</div>
          <n-slider v-model:value="volume" :min="0" :max="100" :step="1" />
          <div class="volume-display">{{ volume }}%</div>
          <div class="button-row">
            <n-button size="small" @click="pressKeyCode('BACK')" :loading="loading.nav">{{ t('controls.back') }}</n-button>
            <n-button size="small" @click="pressKeyCode('HOME')" :loading="loading.nav">{{ t('controls.home') }}</n-button>
            <n-button size="small" @click="pressKeyCode('APP_SWITCH')" :loading="loading.nav">{{ t('controls.recents') }}</n-button>
          </div>
          <div class="button-row">
            <n-button size="small" @click="turnOffScreen" :loading="loading.screenOff">{{ t('controls.screenOff') }}</n-button>
            <n-button size="small" @click="turnOnScreen" :loading="loading.screenOn">{{ t('controls.screenOn') }}</n-button>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div v-else class="no-session">
    {{ t('controls.noSession') }}
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, reactive, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useSessionStore } from '../stores/sessionStore'
import { useMessage } from 'naive-ui'
import { sendKeyEvent, takeScreenshot as apiTakeScreenshot, setRotation, setVolume, turnScreenOn, turnScreenOff } from '../api/sessions'

const { t } = useI18n()
const store = useSessionStore()
const message = useMessage()

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
    message.success(`Rotated to ${angle}°`)
  } catch (err) {
    message.error(`Failed to rotate: ${err}`)
  } finally {
    setLoading('rotate', false)
  }
}

async function toggleFullscreen() {
  if (!sessionId.value) return
  setLoading('fullscreen', true)
  try {
    await sendKeyEvent(sessionId.value, 'F11')
  } catch (err) {
    message.error(`Failed to toggle fullscreen: ${err}`)
  } finally {
    setLoading('fullscreen', false)
  }
}

async function doScreenshot() {
  if (!sessionId.value) return
  setLoading('screenshot', true)
  try {
    const path = await apiTakeScreenshot(sessionId.value)
    message.success(`Screenshot saved: ${path || 'OK'}`)
  } catch (err) {
    message.error(`Failed to take screenshot: ${err}`)
  } finally {
    setLoading('screenshot', false)
  }
}

const volume = ref<number>(50)
watch(volume, async (newVal) => {
  if (!sessionId.value) return
  try {
    await setVolume(sessionId.value, newVal)
  } catch {
    // Volume changes are non-critical, don't show error toasts
  }
})

async function pressKeyCode(code: string) {
  if (!sessionId.value) return
  setLoading('nav', true)
  try {
    await sendKeyEvent(sessionId.value, code)
  } catch (err) {
    message.error(`Failed to send ${code}: ${err}`)
  } finally {
    setLoading('nav', false)
  }
}

async function turnOffScreen() {
  if (!sessionId.value) return
  setLoading('screenOff', true)
  try {
    await turnScreenOff(sessionId.value)
    message.success('Screen turned off')
  } catch (err) {
    message.error(`Failed to turn off screen: ${err}`)
  } finally {
    setLoading('screenOff', false)
  }
}

async function turnOnScreen() {
  if (!sessionId.value) return
  setLoading('screenOn', true)
  try {
    await turnScreenOn(sessionId.value)
    message.success('Screen turned on')
  } catch (err) {
    message.error(`Failed to turn on screen: ${err}`)
  } finally {
    setLoading('screenOn', false)
  }
}

// Keyboard shortcuts
function handleKeydown(e: KeyboardEvent) {
  if (!sessionId.value) return
  // Ignore if typing in an input
  if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return
  
  // Ctrl/Cmd + S = Screenshot
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault()
    doScreenshot()
    return
  }
  
  // Ctrl/Cmd + F = Fullscreen
  if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
    e.preventDefault()
    toggleFullscreen()
    return
  }
  
  // Arrow keys for rotation (with Alt)
  if (e.altKey) {
    switch (e.key) {
      case 'ArrowUp':
        e.preventDefault()
        rotate(0)
        break
      case 'ArrowRight':
        e.preventDefault()
        rotate(90)
        break
      case 'ArrowDown':
        e.preventDefault()
        rotate(180)
        break
      case 'ArrowLeft':
        e.preventDefault()
        rotate(270)
        break
    }
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.device-control-panel {
  padding: 12px;
}

.panel {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.session-id {
  margin-left: auto;
  font-size: 12px;
}

.shortcuts-help {
  font-size: 11px;
  opacity: 0.6;
  padding: 4px 8px;
  border-radius: 4px;
}

.panel-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.panel-card {
  padding: 12px;
  border: 1px solid rgba(0,0,0,0.08);
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
}

.rotation-buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.button-row {
  display: flex;
  gap: 6px;
  margin-top: 4px;
}

.volume-display {
  text-align: right;
  opacity: 0.75;
  font-size: 12px;
}

.no-session {
  padding: 16px;
}
</style>
