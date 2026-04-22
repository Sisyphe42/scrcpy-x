/**
 * Profile information for UI and IPC.
 */
import { SessionOptions } from './session';

export interface Profile {
  name: string;
  isDefault: boolean;
  options: SessionOptions;
}
