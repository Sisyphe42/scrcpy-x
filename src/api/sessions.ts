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

// Command wrappers for device/session control (backend must implement corresponding Tauri commands)
export async function sendKeyEvent(sessionId: string, keyCode: string): Promise<void> {
  try {
    await invoke<void>('send_key_event', { sessionId, keyCode });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to send key event: ${msg}`);
  }
}

export async function takeScreenshot(sessionId: string): Promise<void> {
  try {
    await invoke<void>('take_screenshot', { sessionId });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to take screenshot: ${msg}`);
  }
}

export async function setRotation(sessionId: string, rotation: number): Promise<void> {
  try {
    await invoke<void>('set_rotation', { sessionId, rotation });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to set rotation: ${msg}`);
  }
}

export async function setVolume(sessionId: string, volume: number): Promise<void> {
  try {
    await invoke<void>('set_volume', { sessionId, volume });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to set volume: ${msg}`);
  }
}

export async function turnScreenOn(sessionId: string): Promise<void> {
  try {
    await invoke<void>('turn_screen_on', { sessionId });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to turn screen on: ${msg}`);
  }
}

export async function turnScreenOff(sessionId: string): Promise<void> {
  try {
    await invoke<void>('turn_screen_off', { sessionId });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to turn screen off: ${msg}`);
  }
}
