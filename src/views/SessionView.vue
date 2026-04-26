<script setup lang="ts">
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useMessage, NTag, NButton, NAlert, NIcon } from 'naive-ui';
import { DesktopOutline } from '@vicons/ionicons5';
import { useSessionStore } from '../stores/sessionStore';
import { stopSession } from '../api/sessions';
import DeviceControlPanel from '../components/DeviceControlPanel.vue';
import type { Session } from '../types';

const { t } = useI18n();
const message = useMessage();
const store = useSessionStore();
const stopErrors = ref<Record<string, string>>({});

const activeSessions = computed(() => store.sessions.filter((s: Session) => s.status !== 'Stopped'));
const selectedSessionId = ref<string | null>(store.activeSession?.id ?? null);

const selectedSession = computed<Session | null>(() => {
  if (!selectedSessionId.value) return null;
  return store.sessions.find(s => s.id === selectedSessionId.value) ?? null;
});

function selectSession(s: Session) {
  selectedSessionId.value = s.id;
  store.setActiveSession(s.id);
}

async function onStop(sessionId: string) {
  stopErrors.value[sessionId] = '';
  try {
    await stopSession(sessionId);
    message.success(t('session.stopped'));
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    stopErrors.value[sessionId] = msg;
    message.error(msg);
  }
}

function tagType(status: Session['status']): 'default' | 'success' | 'info' | 'warning' | 'error' {
  switch (status) {
    case 'Running':
    case 'Starting':
      return 'success';
    case 'Error':
      return 'error';
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

<template>
  <div class="session-view">
    <div class="page-header">
      <h2 class="page-title">{{ t('session.title') }}</h2>
      <p class="page-subtitle">{{ t('session.subtitle') }}</p>
    </div>

    <!-- Session list -->
    <div v-if="activeSessions.length > 0" class="session-grid">
      <div
        v-for="session in activeSessions"
        :key="session.id"
        class="session-card"
        :class="{ selected: selectedSessionId === session.id }"
        @click="selectSession(session)"
      >
        <div class="session-card-header">
          <div class="session-card-icon">
            <n-icon :size="24"><DesktopOutline /></n-icon>
          </div>
          <div class="session-card-info">
            <div class="session-device-name">{{ session.deviceId }}</div>
            <n-tag :type="tagType(session.status)" size="small">{{ getStatusText(session.status) }}</n-tag>
          </div>
        </div>

        <div class="session-meta">
          <span>{{ t('session.duration') }}: {{ durationOf(session) }}</span>
        </div>

        <div v-if="session.error" class="session-error">
          <n-alert type="error" closable>
            {{ session.error }}
          </n-alert>
        </div>

        <div v-if="stopErrors[session.id]" class="session-error">
          <n-alert type="error" closable>
            {{ stopErrors[session.id] }}
          </n-alert>
        </div>

        <div class="session-actions">
          <n-button
            v-if="session.status === 'Running'"
            size="small"
            type="error"
            @click.stop="onStop(session.id)"
          >{{ t('session.stop') }}</n-button>
        </div>
      </div>
    </div>

    <div v-else class="empty-state">
      <n-icon :size="48" class="empty-icon"><DesktopOutline /></n-icon>
      <p>{{ t('session.noSessions') }}</p>
    </div>

    <!-- Device Control Panel for selected session -->
    <div v-if="selectedSession && selectedSession.status === 'Running'" class="control-section">
      <h3 class="section-title">{{ t('controls.title') }}</h3>
      <DeviceControlPanel />
    </div>
  </div>
</template>

<style scoped>
.session-view {
  padding: 20px;
  width: 100%;
  box-sizing: border-box;
}

.page-header {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: 16px;
}

.page-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.page-subtitle {
  margin: 0;
  font-size: 13px;
  opacity: 0.6;
}

.session-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 12px;
}

.session-card {
  border: 1px solid rgba(0, 0, 0, 0.08);
  border-radius: 12px;
  padding: 16px;
  cursor: pointer;
  transition: border-color 0.2s ease, box-shadow 0.2s ease;
}

.session-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
}

.session-card.selected {
  border-color: #18a058;
  box-shadow: 0 4px 12px rgba(24, 160, 88, 0.12);
}

.session-card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 8px;
}

.session-card-icon {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  background: rgba(24, 160, 88, 0.08);
  display: flex;
  align-items: center;
  justify-content: center;
  color: #18a058;
  flex-shrink: 0;
}

.session-card-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.session-device-name {
  font-weight: 600;
  font-size: 14px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.session-meta {
  font-size: 12px;
  opacity: 0.6;
  margin-bottom: 8px;
}

.session-error {
  margin-top: 8px;
}

.session-actions {
  margin-top: 8px;
  display: flex;
  gap: 8px;
}

.control-section {
  margin-top: 24px;
}

.section-title {
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 12px;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  opacity: 0.6;
}

.empty-icon {
  margin-bottom: 12px;
  opacity: 0.3;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}
</style>
