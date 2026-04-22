export type ScrcpyErrorCode =
  | 'BinaryNotFound'
  | 'DeviceNotFound'
  | 'SessionNotFound'
  | 'ADBError'
  | 'ScrcpyError'
  | 'IoError';

export interface ScrcpyError {
  code: ScrcpyErrorCode;
  message?: string;
}

export function isScrcpyError(err: any): err is ScrcpyError {
  return !!err && typeof err === 'object' && 'code' in err;
}

export function getErrorMessage(err: ScrcpyError): string {
  switch (err.code) {
    case 'BinaryNotFound':
      return 'ADB or scrcpy not found. Please install or configure paths in Settings.';
    case 'DeviceNotFound':
      return 'Device disconnected. Please reconnect and try again.';
    case 'SessionNotFound':
      return 'Failed to start mirroring. Check device connection and try again.';
    case 'ADBError':
      // Fall through to a generic message if specific text is not provided
      break;
    case 'ScrcpyError':
    case 'IoError':
      // Common recoverable I/O issues will be mapped below
      break;
  }
  // Fallback generic message to avoid exposing raw backend errors
  return 'An unexpected error occurred.';
}

export function isRecoverable(err: any): boolean {
  if (!isScrcpyError(err)) return false;
  const recoverable: ScrcpyErrorCode[] = ['DeviceNotFound', 'SessionNotFound', 'ADBError', 'IoError'];
  return recoverable.includes(err.code);
}

export function getRetryAction(err: ScrcpyError): string {
  switch (err.code) {
    case 'DeviceNotFound':
      return 'Reconnect the device and try again.';
    case 'SessionNotFound':
      return 'Retry mirroring.';
    case 'ADBError':
      return 'Check ADB setup and retry.';
    case 'IoError':
      return 'Retry the operation.';
    case 'BinaryNotFound':
    case 'ScrcpyError':
    default:
      return 'Retry.';
  }
}
