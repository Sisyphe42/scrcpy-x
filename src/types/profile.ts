import type { SessionOptions } from './session';

export interface Profile {
  name: string;
  is_default: boolean;
  options: SessionOptions;
}
