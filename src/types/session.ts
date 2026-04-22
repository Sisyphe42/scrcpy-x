/**
 * Session related types for IPC with Rust backend.
 * CamelCase property names correspond to Rust snake_case via serde.
 */
export type SessionStatus = 'Starting' | 'Running' | 'Stopped' | 'Error';

export interface SessionOptions {
  serial?: string;
  tcpip?: string;
  portRange?: string;
  videoCodec?: string;
  videoBitrate?: number;
  maxSize?: number;
  maxFps?: number;
  crop?: string;
  audio?: boolean;
  audioCodec?: string;
  audioBitrate?: number;
  audioSource?: string;
  record?: string;
  recordFormat?: string;
  fullscreen?: boolean;
  alwaysOnTop?: boolean;
  borderless?: boolean;
  windowTitle?: string;
  windowX?: number;
  windowY?: number;
  windowWidth?: number;
  windowHeight?: number;
  turnScreenOff?: boolean;
  stayAwake?: boolean;
  showTouches?: boolean;
  keyboard?: string;
  mouse?: string;
  gamepad?: string;
  shortcutMod?: string;
  extra?: Record<string, string>;
}

export interface Session {
  id: string;
  deviceId: string;
  status: SessionStatus;
  options: SessionOptions;
  error?: string;
}
