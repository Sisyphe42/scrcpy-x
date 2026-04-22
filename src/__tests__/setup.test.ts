import { describe, it, expect } from 'vitest';

describe('setup', () => {
  it('should pass basic test', () => {
    expect(true).toBe(true);
  });

  it('should have correct app name', () => {
    const appName = 'ScrcpyX';
    expect(appName).toBe('ScrcpyX');
  });
});
