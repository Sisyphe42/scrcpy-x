import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Session } from '../types';

const SESSIONS_KEY = 'scrcpyx-sessions';
const ACTIVE_SESSION_KEY = 'scrcpyx-activeSession';

function loadSessionsFromStorage(): Session[] {
  try {
    const raw = typeof localStorage !== 'undefined' ? localStorage.getItem(SESSIONS_KEY) : null;
    if (raw) return JSON.parse(raw) as Session[];
  } catch {
    // ignore
  }
  return [];
}

function persistSessions(sessions: Session[]) {
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(SESSIONS_KEY, JSON.stringify(sessions));
    }
  } catch {
    // ignore
  }
}

function loadActiveSessionId(): string | null {
  try {
    const raw = typeof localStorage !== 'undefined' ? localStorage.getItem(ACTIVE_SESSION_KEY) : null;
    return raw ?? null;
  } catch {
    return null;
  }
}

function persistActiveSessionId(id: string | null) {
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(ACTIVE_SESSION_KEY, id ?? '');
    }
  } catch {
    // ignore
  }
}

export const useSessionStore = defineStore('session', () => {
  const sessions = ref<Session[]>(loadSessionsFromStorage());
  const activeSessionId = ref<string | null>(loadActiveSessionId());

  const activeSession = computed<Session | null>(() => {
    return sessions.value.find(s => s.id === activeSessionId.value) ?? null;
  });
  const runningSessions = computed(() => sessions.value.filter(s => s.status === 'Running'));

  function addSession(s: Session) {
    sessions.value.push(s);
    activeSessionId.value = s.id;
    persistSessions(sessions.value);
    persistActiveSessionId(s.id);
  }

  function removeSession(id: string) {
    sessions.value = sessions.value.filter(s => s.id !== id);
    if (activeSessionId.value === id) {
      activeSessionId.value = null;
      persistActiveSessionId(null);
    }
    persistSessions(sessions.value);
  }

  function updateSessionStatus(id: string, status: Session['status'], error?: string) {
    const idx = sessions.value.findIndex(s => s.id === id);
    if (idx >= 0) {
      sessions.value[idx] = { ...sessions.value[idx], status, error } as Session;
      persistSessions(sessions.value);
    }
  }

  return {
    sessions,
    activeSessionId,
    activeSession,
    runningSessions,
    addSession,
    removeSession,
    updateSessionStatus,
  };
});
