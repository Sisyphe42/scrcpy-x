import { invoke } from '@tauri-apps/api/core';
import type { Session, SessionOptions } from '../types';
import { useSessionStore } from '../stores/sessionStore';

// Android KeyEvent codes (subset used by UI)
const KEYCODE_MAP: Record<string, number> = {
  'BACK': 4,
  'HOME': 3,
  'APP_SWITCH': 187,
  'MENU': 82,
  'POWER': 26,
  'VOLUME_UP': 24,
  'VOLUME_DOWN': 25,
  'MUTE': 164,
  'MEDIA_PLAY_PAUSE': 85,
  'MEDIA_STOP': 86,
  'MEDIA_NEXT': 87,
  'MEDIA_PREVIOUS': 88,
  'ENTER': 66,
  'DPAD_UP': 19,
  'DPAD_DOWN': 20,
  'DPAD_LEFT': 21,
  'DPAD_RIGHT': 22,
  'F11': 0, // Special: handled by scrcpy fullscreen shortcut, not ADB
};

// Rotation degrees to Android orientation (0=portrait, 1=landscape, 2=reverse portrait, 3=reverse landscape)
function degreesToOrientation(degrees: number): number {
  switch (degrees) {
    case 0: return 0;
    case 90: return 1;
    case 180: return 2;
    case 270: return 3;
    default: return Math.round(degrees / 90) % 4;
  }
}

// Volume percentage (0-100) to ADB level (0-15)
function volumeToLevel(percent: number): number {
  return Math.round((percent / 100) * 15);
}

// Get deviceId from sessionId via session store
function getDeviceIdFromSession(sessionId: string): string | null {
  const store = useSessionStore();
  const session = store.sessions.find(s => s.id === sessionId);
  return session?.deviceId ?? null;
}

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

/**
 * Send a key event to a device.
 * @param sessionId - The session ID
 * @param keyCode - Key code string (e.g., 'BACK', 'HOME', 'APP_SWITCH') or Android keycode number
 */
export async function sendKeyEvent(sessionId: string, keyCode: string | number): Promise<void> {
  const deviceId = getDeviceIdFromSession(sessionId);
  if (!deviceId) {
    throw new Error(`Session ${sessionId} not found or has no device`);
  }
  
  // Convert string keyCode to Android keycode
  let keycode: number;
  if (typeof keyCode === 'string') {
    keycode = KEYCODE_MAP[keyCode] ?? parseInt(keyCode, 10);
    if (isNaN(keycode)) {
      throw new Error(`Unknown key code: ${keyCode}`);
    }
  } else {
    keycode = keyCode;
  }
  
  // Special case: F11 is fullscreen shortcut, not an ADB key
  if (keyCode === 'F11') {
    // scrcpy handles fullscreen via keyboard shortcuts, not ADB
    // This is a no-op for now - fullscreen should be set in session options
    return;
  }
  
  try {
    await invoke<void>('send_key_event', { deviceId: deviceId, keycode });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to send key event: ${msg}`);
  }
}

/**
 * Take a screenshot from a device.
 * @param sessionId - The session ID
 * @returns Path to saved screenshot file
 */
export async function takeScreenshot(sessionId: string): Promise<string> {
  const deviceId = getDeviceIdFromSession(sessionId);
  if (!deviceId) {
    throw new Error(`Session ${sessionId} not found or has no device`);
  }
  
  try {
    const path = await invoke<string>('take_screenshot', { deviceId: deviceId });
    return path;
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to take screenshot: ${msg}`);
  }
}

/**
 * Set screen rotation.
 * @param sessionId - The session ID
 * @param degrees - Rotation in degrees (0, 90, 180, 270)
 */
export async function setRotation(sessionId: string, degrees: number): Promise<void> {
  const deviceId = getDeviceIdFromSession(sessionId);
  if (!deviceId) {
    throw new Error(`Session ${sessionId} not found or has no device`);
  }
  
  const orientation = degreesToOrientation(degrees);
  
  try {
    await invoke<void>('set_rotation', { deviceId: deviceId, orientation });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to set rotation: ${msg}`);
  }
}

/**
 * Set media volume.
 * @param sessionId - The session ID
 * @param percent - Volume percentage (0-100)
 */
export async function setVolume(sessionId: string, percent: number): Promise<void> {
  const deviceId = getDeviceIdFromSession(sessionId);
  if (!deviceId) {
    throw new Error(`Session ${sessionId} not found or has no device`);
  }
  
  const level = volumeToLevel(percent);
  
  try {
    await invoke<void>('set_volume', { deviceId: deviceId, level });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to set volume: ${msg}`);
  }
}

/**
 * Turn screen on.
 * @param sessionId - The session ID
 */
export async function turnScreenOn(sessionId: string): Promise<void> {
  const deviceId = getDeviceIdFromSession(sessionId);
  if (!deviceId) {
    throw new Error(`Session ${sessionId} not found or has no device`);
  }
  
  try {
    await invoke<void>('turn_screen_on', { deviceId: deviceId });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to turn screen on: ${msg}`);
  }
}

/**
 * Turn screen off.
 * @param sessionId - The session ID
 */
export async function turnScreenOff(sessionId: string): Promise<void> {
  const deviceId = getDeviceIdFromSession(sessionId);
  if (!deviceId) {
    throw new Error(`Session ${sessionId} not found or has no device`);
  }
  
  try {
    await invoke<void>('turn_screen_off', { deviceId: deviceId });
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to turn screen off: ${msg}`);
  }
}
