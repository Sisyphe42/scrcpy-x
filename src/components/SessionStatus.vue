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
          <div class="device-name">Device: {{ session.deviceId }}</div>
          <n-tag :type="tagType(session.status)" class="status-tag">{{ session.status }}</n-tag>
        </div>

        <div class="session-meta">
          <span class="label">Duration:</span>
          <span class="value">{{ durationOf(session) }}</span>
        </div>

        <div class="session-actions">
          <n-button
            v-if="session.status === 'Running'"
            size="small"
            type="error"
            @click.stop="() => onStop(session.id)"
          >Stop</n-button>
        </div>

        <!-- Inline error if the session has an error message -->
        <div v-if="session.error" class="session-error">
          <n-alert type="error" :title="session.error" show-icon />
        </div>

        <!-- Per-session stop error (from API call) -->
        <div v-if="stopErrors[session.id]" class="session-stop-error">
          <n-alert type="error" :title="stopErrors[session.id]" show-icon />
        </div>
      </div>
    </div>
    <div v-else class="empty-state">
      No active sessions.
    </div>
  </section>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { Session } from '../types';
import { stopSession } from '../api/sessions';
import { useSessionStore } from '../stores/sessionStore';

// Events
const emit = defineEmits<{ (e: 'select', session: Session): void }>();

// Access the session store
const store = useSessionStore();

// Access the raw sessions array from the store via a computed wrapper
const sessionsList = computed(() => store.sessions.value);

// Show all sessions that are not yet stopped (active sessions)
const activeSessions = computed(() => sessionsList.value.filter(s => s.status !== 'Stopped'));

// Local per-session stop error messages
const stopErrors = ref<Record<string, string>>({});

function selectSession(s: Session) {
  emit('select', s);
}

async function onStop(sessionId: string) {
  // Clear previous error for this session
  stopErrors.value[sessionId] = '';
  try {
    await stopSession(sessionId);
    // On success, the session store will remove the session; UI updates automatically
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

// Duration helper: if a startedAt timestamp is provided by the backend, show HH:MM:SS
function durationOf(s: Session): string {
  const startedAt = (s as any).startedAt;
  if (!startedAt) return '—';
  const diff = Date.now() - new Date(startedAt).getTime();
  const totalSec = Math.floor(diff / 1000);
  const h = Math.floor(totalSec / 3600);
  const m = Math.floor((totalSec % 3600) / 60);
  const sec = totalSec % 60;
  const pad = (n: number) => String(n).padStart(2, '0');
  return `${pad(h)}:${pad(m)}:${pad(sec)}`;
}
</script>

<style scoped>
.session-status {
  padding: 12px;
}
.session-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: 12px;
}
.session-card {
  border: 1px solid rgba(0,0,0,0.08);
  border-radius: 12px;
  padding: 12px;
  background: linear-gradient(135deg, rgba(255,255,255,0.92), rgba(255,255,255,0.85));
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}
.session-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 18px rgba(0,0,0,0.08);
}
.session-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 6px;
}
.device-name {
  font-weight: 600;
  font-size: 13px;
  color: #333;
}
.status-tag {
  margin-left: 8px;
}
.session-meta {
  color: #555;
  font-size: 12px;
  margin: 6px 0 8px;
  display: flex;
  gap: 6px;
  align-items: center;
}
.session-actions {
  display: flex;
  gap: 8px;
  margin-top: 6px;
}
.session-error {
  margin-top: 6px;
}
.session-stop-error {
  margin-top: 6px;
}
.empty-state {
  color: #666;
  text-align: center;
  padding: 20px;
}
</style>
