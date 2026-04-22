import { defineConfig } from 'wdio';

export const config: WebdriverIO.Config = defineConfig({
  //
  // ====================
  // Runner Configuration
  // ====================
  runner: 'local',
  autoCompileOpts: {
    tsNodeOpts: {
      transpileOnly: true,
    },
  },
  //
  // ==================
  // Specify Test Files
  // ==================
  specs: ['./e2e/**/*.test.ts'],
  //
  // ============
  // Capabilities
  // ============
  capabilities: [{
    browserName: 'chrome',
    'goog:chromeOptions': {
      args: ['--headless', '--disable-gpu'],
    },
  }],
  //
  // ===================
  // Test Configurations
  // ===================
  logLevel: 'info',
  bail: 0,
  baseUrl: 'http://localhost:1420',
  waitforTimeout: 10000,
  connectionRetryTimeout: 120000,
  connectionRetryCount: 3,
  //
  // Services
  // ========
  services: [],
  //
  // Framework
  // =========
  framework: 'mocha',
  reporters: ['spec'],
  mochaOpts: {
    ui: 'bdd',
    timeout: 60000,
  },
});
