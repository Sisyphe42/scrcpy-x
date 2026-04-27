/**
 * Application settings and paths configuration (frontend side).
 */
export interface WindowBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface BinaryPaths {
  adb?: string;
  scrcpy?: string;
}

export type BinarySource = 'auto' | 'bundled' | 'custom';

export interface BinarySourceConfig {
  adbSource: BinarySource;
  scrcpySource: BinarySource;
  adbPath?: string;
  scrcpyPath?: string;
}

export interface AppSettings {
  lastProfile?: string;
  windowBounds?: WindowBounds;
  binaryConfig: BinarySourceConfig;
  theme: string;
  maxSessions: number;
  language?: string;
  // Screenshot settings
  screenshotFilename?: string;
  screenshotPath?: string;
  screenshotClipboard?: boolean;
  screenshotFormat?: string;
}
