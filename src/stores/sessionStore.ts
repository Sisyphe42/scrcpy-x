import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { Session } from '../types';
import { listen } from '@tauri-apps/api/event';

const SESSIONS_KEY = 'scrcpyx-sessions';
const ACTIVE_SESSION_KEY = 'scrcpyx-activeSession';

// Sessions are ephemeral - always clear stale data on startup
// (backend processes don't survive app restart)
function clearStaleSessionData() {
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.removeItem(SESSIONS_KEY);
      localStorage.removeItem(ACTIVE_SESSION_KEY);
    }
  } catch {
    // ignore
  }
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

function persistActiveSessionId(id: string | null) {
  try {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem(ACTIVE_SESSION_KEY, id ?? '');
    }
  } catch {
    // ignore
  }
}

// Clear stale sessions on module load
clearStaleSessionData();

export const useSessionStore = defineStore('session', () => {
  const sessions = ref<Session[]>([]);
  const activeSessionId = ref<string | null>(null);

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

  function setSessions(newSessions: Session[]) {
    sessions.value = newSessions;
    // Attempt to keep a reasonable active session if the previous one is still present
    if (activeSessionId.value) {
      const exists = newSessions.find(s => s.id === activeSessionId.value);
      if (!exists) {
        activeSessionId.value = null;
      }
    }
    persistSessions(newSessions);
  }

  function setActiveSession(id: string) {
    activeSessionId.value = id;
    persistActiveSessionId(id);
  }

  // Initialize event listeners for session events
  (async () => {
    try {
      void await listen('session-started', (event) => {
        const session = event.payload as Session;
        // Only add if not already present (avoid duplicates from manual + event)
        if (!sessions.value.find(s => s.id === session.id)) {
          addSession(session);
        }
      });
      void await listen('session-ended', (event) => {
        const payload = event.payload as { sessionId: string; reason: string };
        removeSession(payload.sessionId);
      });
      void await listen('session-error', (event) => {
        const payload = event.payload as { sessionId: string; error: string };
        updateSessionStatus(payload.sessionId, 'Error', payload.error);
      });
    } catch {
      // Event system not available (e.g., during tests)
    }
  })();

  // Cleanup on app close
  if (typeof window !== 'undefined') {
    window.addEventListener('beforeunload', () => {
      // Sessions should be cleaned up by backend
    });
  }

  return {
    sessions,
    activeSessionId,
    activeSession,
    runningSessions,
    addSession,
    removeSession,
    updateSessionStatus,
    setSessions,
    setActiveSession,
  };
});
