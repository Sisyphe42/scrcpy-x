<template>
  <section class="session-status" aria-label="Session Status">
    <div class="session-grid" v-if="activeSessions.length > 0">
      <div
        v-for="session in activeSessions" 
        :key="session.id" 
        class="session-card"
        @click="() => selectSession(session)"
      >
        <div class="session-header">
          <div class="device-name">{{ t('session.deviceId') }}: {{ session.deviceId }}</div>
          <n-tag :type="tagType(session.status)" class="status-tag">{{ getStatusText(session.status) }}</n-tag>
        </div>

        <div class="session-meta">
          <span class="label">{{ t('session.duration') }}:</span>
          <span class="value">{{ durationOf(session) }}</span>
        </div>

        <div class="session-actions">
          <n-button
            v-if="session.status === 'Running'"
            size="small"
            type="error"
            @click.stop="() => onStop(session.id)"
          >{{ t('session.stop') }}</n-button>
        </div>

        <div v-if="session.error" class="session-error">
          <n-alert type="error" :title="session.error" show-icon />
        </div>

        <div v-if="stopErrors[session.id]" class="session-stop-error">
          <n-alert type="error" :title="stopErrors[session.id]" show-icon />
        </div>
      </div>
    </div>
    <div v-else class="empty-state">
      {{ t('session.noSessions') }}
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue';
import { useI18n } from 'vue-i18n';
import type { Session } from '../types';
import { stopSession } from '../api/sessions';
import { useSessionStore } from '../stores/sessionStore';

const { t } = useI18n();
const emit = defineEmits<{ (e: 'select', session: Session): void }>();
const store = useSessionStore();

const sessionsList = computed(() => store.sessions);
const activeSessions = computed(() => sessionsList.value.filter((s: Session) => s.status !== 'Stopped'));
const stopErrors = ref<Record<string, string>>({});

// Live timer: update every second so duration ticks in real time
const now = ref(Date.now());
let timer: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
  timer = setInterval(() => { now.value = Date.now(); }, 1000);
});
onUnmounted(() => {
  if (timer) { clearInterval(timer); timer = null; }
});

function selectSession(s: Session) {
  emit('select', s);
}

async function onStop(sessionId: string) {
  stopErrors.value[sessionId] = '';
  try {
    await stopSession(sessionId);
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    stopErrors.value[sessionId] = msg;
  }
}

function tagType(status: Session['status']): 'default' | 'success' | 'info' | 'warning' | 'danger' {
  switch (status) {
    case 'Running':
    case 'Starting':
      return 'success';
    case 'Error':
      return 'danger';
    default:
      return 'default';
  }
}

function getStatusText(status: Session['status']): string {
  switch (status) {
    case 'Starting': return t('session.status.starting');
    case 'Running': return t('session.status.running');
    case 'Stopped': return t('session.status.stopped');
    case 'Error': return t('session.status.error');
    default: return status;
  }
}

function durationOf(s: Session): string {
  if (!s.startedAt) return '—';
  const diff = now.value - s.startedAt;
  if (diff < 0) return '—';
  const totalSec = Math.floor(diff / 1000);
  const h = Math.floor(totalSec / 3600);
  const m = Math.floor((totalSec % 3600) / 60);
  const sec = totalSec % 60;
  const pad = (n: number) => String(n).padStart(2, '0');
  return `${pad(h)}:${pad(m)}:${pad(sec)}`;
}
</script>

<style scoped>
.session-status { padding: 12px; }
.session-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(240px, 1fr)); gap: 12px; }
.session-card {
  border: 1px solid rgba(0,0,0,0.08);
  border-radius: 12px;
  padding: 12px;
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}
.session-card:hover { transform: translateY(-2px); box-shadow: 0 8px 18px rgba(0,0,0,0.08); }
.session-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px; }
.device-name { font-weight: 600; font-size: 13px; }
.status-tag { margin-left: 8px; }
.session-meta { font-size: 12px; margin: 6px 0 8px; display: flex; gap: 6px; align-items: center; opacity: 0.7; }
.session-actions { display: flex; gap: 8px; margin-top: 6px; }
.session-error { margin-top: 6px; }
.session-stop-error { margin-top: 6px; }
.empty-state { opacity: 0.6; text-align: center; padding: 20px; }
</style>
