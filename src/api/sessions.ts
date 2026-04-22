import { invoke } from '@tauri-apps/api/core';
import type { Session, SessionOptions } from '../types';
import { useSessionStore } from '../stores/sessionStore';

// Launch a new session for a device with given options
export async function launchSession(deviceId: string, options: SessionOptions): Promise<Session> {
  try {
    const session = await invoke<Session>('launch_session', { deviceId, options });
    const store = useSessionStore();
    if (session) {
      store.addSession(session);
    }
    return session;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to launch session: ${msg}`);
  }
}

// Stop a given session
export async function stopSession(sessionId: string): Promise<void> {
  try {
    await invoke<void>('stop_session', { sessionId });
    const store = useSessionStore();
    store.removeSession(sessionId);
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to stop session: ${msg}`);
  }
}

// Retrieve all sessions and sync store
export async function getSessions(): Promise<Session[]> {
  try {
    const sessions = await invoke<Session[]>('get_sessions');
    const store = useSessionStore();
    if (Array.isArray(sessions)) {
      store.setSessions(sessions);
    }
    return sessions;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to fetch sessions: ${msg}`);
  }
}
