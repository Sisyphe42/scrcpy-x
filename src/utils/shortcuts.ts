import { useSessionStore } from '../stores/sessionStore';
import { refreshDevices } from '../api/devices';
import { takeScreenshot, sendKeyEvent } from '../api/sessions';

export interface Shortcut {
  key: string;
  ctrl: boolean;
  shift?: boolean;
  action: () => void | Promise<void>;
  requiresSession?: boolean;
  description: string;
}

const sessionStore = useSessionStore();

export const shortcuts: Record<string, Shortcut> = {
  refresh: {
    key: 'r',
    ctrl: true,
    action: async () => { await refreshDevices(); },
    description: 'Refresh devices',
  },
  screenshot: {
    key: 's',
    ctrl: true,
    action: async () => {
      const session = sessionStore.activeSession;
      if (session) {
        await takeScreenshot(session.id);
      }
    },
    requiresSession: true,
    description: 'Take screenshot',
  },
  fullscreen: {
    key: 'f',
    ctrl: false,
    action: async () => {
      const session = sessionStore.activeSession;
      if (session) {
        await sendKeyEvent(session.id, 'F11');
      }
    },
    requiresSession: true,
    description: 'Toggle fullscreen',
  },
  screenOff: {
    key: 'o',
    ctrl: false,
    action: async () => {
      const session = sessionStore.activeSession;
      if (session) {
        await sendKeyEvent(session.id, 'POWER');
      }
    },
    requiresSession: true,
    description: 'Turn screen off',
  },
  back: {
    key: 'b',
    ctrl: false,
    action: async () => {
      const session = sessionStore.activeSession;
      if (session) {
        await sendKeyEvent(session.id, 'BACK');
      }
    },
    requiresSession: true,
    description: 'Go back',
  },
  home: {
    key: 'h',
    ctrl: false,
    action: async () => {
      const session = sessionStore.activeSession;
      if (session) {
        await sendKeyEvent(session.id, 'HOME');
      }
    },
    requiresSession: true,
    description: 'Go home',
  },
};

export function handleKeyboard(event: KeyboardEvent): boolean {
  // Don't intercept when typing in inputs
  const target = event.target as HTMLElement;
  if (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA' || target.isContentEditable) {
    return false;
  }

  const hasActiveSession = !!sessionStore.activeSession;

  for (const shortcut of Object.values(shortcuts)) {
    const keyMatch = event.key.toLowerCase() === shortcut.key.toLowerCase();
    const ctrlMatch = event.ctrlKey === shortcut.ctrl;
    const shiftMatch = shortcut.shift ? event.shiftKey === shortcut.shift : true;

    if (keyMatch && ctrlMatch && shiftMatch) {
      if (shortcut.requiresSession && !hasActiveSession) {
        continue; // Skip shortcuts that require a session
      }
      event.preventDefault();
      void shortcut.action();
      return true;
    }
  }
  return false;
}

export function getShortcutHint(name: string): string {
  const shortcut = shortcuts[name];
  if (!shortcut) return '';
  const parts: string[] = [];
  if (shortcut.ctrl) parts.push('Ctrl');
  if (shortcut.shift) parts.push('Shift');
  parts.push(shortcut.key.toUpperCase());
  return parts.join('+');
}
