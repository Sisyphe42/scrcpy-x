import { invoke } from '@tauri-apps/api/core';

/**
 * Open the floating device control panel.
 * If already open, focuses the existing window.
 */
export async function openFloatingPanel(): Promise<void> {
  try {
    await invoke<void>('open_floating_panel');
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to open floating panel: ${msg}`);
  }
}

/**
 * Close the floating device control panel.
 */
export async function closeFloatingPanel(): Promise<void> {
  try {
    await invoke<void>('close_floating_panel');
  } catch (err) {
    const msg = err instanceof Error ? err.message : String(err);
    throw new Error(`Failed to close floating panel: ${msg}`);
  }
}
