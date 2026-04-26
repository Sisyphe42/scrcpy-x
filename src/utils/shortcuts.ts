import { refreshDevices } from '../api/devices';
import { takeScreenshot, sendKeyEvent } from '../api/sessions';
import { useSessionStore } from '../stores/sessionStore';

// Lazy router reference — set by App.vue on mount
let _router: any = null;
export function setRouter(router: any) {
  _router = router;
}

export interface Shortcut {
  key: string;
  ctrl: boolean;
  shift?: boolean;
  action: () => void | Promise<void>;
  requiresSession?: boolean;
  description: string;
}

// Page navigation order for Ctrl+Tab cycling
const PAGE_ORDER = ['launch', 'sessions', 'settings'];

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
      const sessionStore = useSessionStore();
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
      const sessionStore = useSessionStore();
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
      const sessionStore = useSessionStore();
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
      const sessionStore = useSessionStore();
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
      const sessionStore = useSessionStore();
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

  // Ctrl+Tab: cycle to next page, Ctrl+Shift+Tab: previous
  if (event.ctrlKey && event.key === 'Tab') {
    event.preventDefault();
    if (_router) {
      const currentRoute = _router.currentRoute?.value?.name as string;
      const currentIdx = PAGE_ORDER.indexOf(currentRoute);
      const nextIdx = event.shiftKey
        ? (currentIdx - 1 + PAGE_ORDER.length) % PAGE_ORDER.length
        : (currentIdx + 1) % PAGE_ORDER.length;
      _router.push({ name: PAGE_ORDER[nextIdx] });
    }
    return true;
  }

  const sessionStore = useSessionStore();
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
