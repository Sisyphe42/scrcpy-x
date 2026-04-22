/**
 * Device types for IPC with Rust backend.
 * Mirrors Rust Device and DeviceStatus with camelCase TS properties.
 */
export type DeviceStatus = 'Online' | 'Offline' | 'Unauthorized';

export interface Device {
  /** Device ID (serial) */
  id: string;
  /** Device name/model */
  name: string;
  /** Device model */
  model: string;
  /** Device connection status */
  status: DeviceStatus;
}
