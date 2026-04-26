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

export interface AppSettings {
  lastProfile?: string;
  windowBounds?: WindowBounds;
  binaryPaths: BinaryPaths;
  theme: string;
  maxSessions: number;
  language?: string;
}
