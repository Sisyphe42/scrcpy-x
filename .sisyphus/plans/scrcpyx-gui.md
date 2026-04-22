# ScrcpyX - GUI Application for scrcpy

## TL;DR
> **Summary**: Build a cross-platform GUI wrapper for scrcpy using Tauri + Vue3 + Naive UI with multi-device support, profile management, and essential device controls.
> **Deliverables**: Desktop app (Win/Mac/Linux) that manages scrcpy sessions with settings UI and device operation panel.
> **Effort**: Large
> **Parallel**: YES - 7 waves
> **Critical Path**: Project Setup → Core Backend → Device Discovery → UI Components → Integration → Testing

## Context
### Original Request
Build a GUI app for scrcpy (screen mirroring tool) with:
- Cross-platform support (Windows, macOS, Linux)
- Tech stack: Tauri + Vue3 + Naive UI
- Two main UIs: Settings/Launch UI and Device Operation UI

### Interview Summary
- **App Name**: ScrcpyX
- **Integration**: Subprocess management (launch scrcpy as separate process, manage its window)
- **Device Support**: Multi-device with concurrent sessions
- **Feature Scope**: Curated essential options from 7 categories
- **Configuration**: Named profiles + auto-save last settings
- **Binary Handling**: PATH-first resolution, bundled fallback
- **Testing**: TDD with Vitest + Tauri E2E
- **Languages**: TypeScript (frontend) + Rust (backend)
- **i18n**: English only, i18n-ready structure

### Metis Review (gaps addressed)
- **Lifecycle management**: Robust subprocess cleanup on exit/disconnect
- **Hot-plug handling**: Real-time device list updates
- **Error surfacing**: Toast notifications with actionable messages
- **Profile schema**: JSON storage in app data directory
- **Process limits**: Configurable max concurrent sessions
- **Atomic writes**: Prevent profile corruption
- **Permission handling**: Guide users through macOS/Windows prompts

## Work Objectives
### Core Objective
Create a user-friendly GUI that wraps scrcpy's CLI functionality, enabling users to:
1. Discover and select connected Android devices
2. Configure scrcpy options via intuitive UI (not CLI)
3. Launch and manage multiple device sessions
4. Control devices with essential actions (rotate, screenshot, etc.)
5. Save and load configuration profiles

### Deliverables
1. Tauri application with Rust backend
2. Vue3 + Naive UI frontend with two main views
3. Profile management system
4. Device discovery and session management
5. Cross-platform packaging (Win/Mac/Linux)
6. Test suite (Vitest unit + Tauri E2E)

### Definition of Done
- [ ] `npm run tauri build` produces installers for all 3 platforms
- [ ] All Vitest tests pass: `npm run test`
- [ ] All E2E tests pass: `npm run test:e2e`
- [ ] Can detect connected devices via ADB
- [ ] Can launch scrcpy session with configured options
- [ ] Profiles persist across app restarts
- [ ] Multi-device sessions work concurrently
- [ ] All subprocesses terminate on app exit

### Must Have
- Device discovery (ADB integration)
- Profile CRUD operations
- Session lifecycle management (start/stop/cleanup)
- Essential device controls
- Cross-platform binary resolution
- Error handling with user-friendly messages

### Must NOT Have (guardrails)
- DO NOT modify scrcpy source code
- DO NOT spawn unmanaged background processes
- DO NOT expose dangerous CLI options by default
- DO NOT bypass ADB authentication/security
- DO NOT hardcode platform-specific paths
- DO NOT leave orphan processes on exit

## Verification Strategy
> ZERO HUMAN INTERVENTION - all verification is agent-executed.
- Test decision: TDD (write tests first, then implementation)
- Framework: Vitest (unit) + Tauri WebDriver (E2E)
- QA policy: Every task has agent-executed scenarios
- Evidence: .sisyphus/evidence/task-{N}-{slug}.{ext}

## Execution Strategy
### Parallel Execution Waves
> Target: 5-8 tasks per wave.

**Wave 1: Foundation** (5 tasks)
- Project scaffolding, config files, test setup

**Wave 2: Core Backend** (6 tasks)
- Rust commands, subprocess management, ADB integration

**Wave 3: Data Layer** (5 tasks)
- Profile storage, settings persistence, state management

**Wave 4: UI Components** (7 tasks)
- Vue components for settings, device list, controls

**Wave 5: Integration** (5 tasks)
- Connect UI to backend, IPC layer

**Wave 6: Features** (6 tasks)
- Profile management, multi-device, device controls

**Wave 7: Packaging & Polish** (5 tasks)
- Build config, error handling, E2E tests, documentation

### Dependency Matrix
```
Wave 1 (Foundation) → all subsequent waves
Wave 2 (Core Backend) → Wave 5 (Integration)
Wave 3 (Data Layer) → Wave 5 (Integration)
Wave 4 (UI Components) → Wave 5 (Integration)
Wave 5 (Integration) → Wave 6 (Features)
Wave 6 (Features) → Wave 7 (Packaging)
```

### Agent Dispatch Summary
- Wave 1: 5 quick tasks (project setup)
- Wave 2: 6 deep tasks (Rust backend)
- Wave 3: 5 deep tasks (data/storage)
- Wave 4: 6 visual-engineering tasks (Vue components)
- Wave 5: 5 deep tasks (IPC integration)
- Wave 6: 6 unspecified-high tasks (feature completion)
- Wave 7: 5 unspecified-high tasks (polish/package)

## TODOs
> Implementation + Test = ONE task. Never separate.
> EVERY task MUST have: Agent Profile + Parallelization + QA Scenarios.

### Wave 1: Foundation

- [x] 1. Initialize Tauri + Vue3 Project

  **What to do**:
  - Run `npm create tauri-app@latest` with Vue + TypeScript template
  - Configure project name as "scrcpyx"
  - Set up directory structure: `src/` (Vue), `src-tauri/` (Rust)
  - Add Naive UI dependency: `npm install naive-ui`
  - Add Vue Router: `npm install vue-router@4`
  - Add Pinia for state: `npm install pinia`
  - Configure TypeScript strict mode in `tsconfig.json`

  **Must NOT do**:
  - Do not add unnecessary UI libraries
  - Do not skip TypeScript configuration

  **Recommended Agent Profile**:
  - Category: `quick` - Reason: Standard scaffolding, well-documented process
  - Skills: [] - No special skills needed
  - Omitted: [] - None

  **Parallelization**: Can Parallel: NO | Wave 1 | Blocks: 2-5 | Blocked By: none

  **References**:
  - External: https://tauri.app/v1/guides/getting-started/setup/
  - External: https://www.naiveui.com/en-US/os-theme/docs/start

  **Acceptance Criteria**:
  - [ ] `npm run tauri dev` starts the app successfully
  - [ ] `src/main.ts` imports Naive UI and Pinia
  - [ ] TypeScript compiles without errors

  **QA Scenarios**:
  ```
  Scenario: Dev server starts
    Tool: Bash
    Steps: cd D:\projects\scrcpy-x && npm run tauri dev -- --exit
    Expected: Process exits with code 0, no TypeScript errors
    Evidence: .sisyphus/evidence/task-1-dev-start.log
  ```

  **Commit**: YES | Message: `chore: initialize tauri + vue3 project` | Files: all new files

- [x] 2. Configure Vitest for Unit Testing

  **What to do**:
  - Install Vitest: `npm install -D vitest @vue/test-utils happy-dom`
  - Create `vitest.config.ts` with Vue plugin support
  - Add test script to `package.json`: `"test": "vitest"`
  - Create `src/__tests__/setup.test.ts` as placeholder
  - Configure coverage reporting

  **Must NOT do**:
  - Do not use Jest (use Vitest for Vite compatibility)

  **Recommended Agent Profile**:
  - Category: `quick` - Reason: Standard test setup
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 1 | Blocks: all test-dependent tasks | Blocked By: 1

  **References**:
  - External: https://vitest.dev/guide/

  **Acceptance Criteria**:
  - [ ] `npm run test` executes without errors
  - [ ] Placeholder test passes
  - [ ] Coverage command works: `npm run test -- --coverage`

  **QA Scenarios**:
  ```
  Scenario: Tests run successfully
    Tool: Bash
    Steps: cd D:\projects\scrcpy-x && npm run test -- --run
    Expected: All tests pass, exit code 0
    Evidence: .sisyphus/evidence/task-2-vitest.log
  ```

  **Commit**: YES | Message: `test: configure vitest for unit testing` | Files: vitest.config.ts, package.json, src/__tests__/*

- [x] 3. Configure Tauri E2E Testing

  **What to do**:
  - Install Tauri WebDriver: `npm install -D @tauri-apps/tauri-driver webdriverio`
  - Create `e2e/` directory for E2E tests
  - Create `e2e/setup.test.ts` as placeholder
  - Add E2E script: `"test:e2e": "tauri-driver && wdio run ./e2e/wdio.conf.ts"`
  - Create `e2e/wdio.conf.ts` with Tauri capabilities

  **Must NOT do**:
  - Do not use Playwright (Tauri has native WebDriver support)

  **Recommended Agent Profile**:
  - Category: `quick` - Reason: Standard E2E setup
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 1 | Blocks: Wave 5+ | Blocked By: 1

  **References**:
  - External: https://tauri.app/v1/guides/testing/webdriver/

  **Acceptance Criteria**:
  - [ ] `e2e/` directory exists with config
  - [ ] Placeholder E2E test structure ready

  **QA Scenarios**:
  ```
  Scenario: E2E test structure exists
    Tool: Bash
    Steps: cd D:\projects\scrcpy-x && Test-Path e2e/wdio.conf.ts
    Expected: Returns true
    Evidence: .sisyphus/evidence/task-3-e2e-setup.txt
  ```

  **Commit**: YES | Message: `test: configure tauri e2e testing` | Files: e2e/*, package.json

- [x] 4. Set Up i18n Infrastructure

  **What to do**:
  - Install vue-i18n: `npm install vue-i18n@9`
  - Create `src/locales/en.ts` with English translations
  - Create `src/locales/index.ts` to export i18n instance
  - Configure i18n in `src/main.ts`
  - Add basic translation keys: `app.title`, `common.save`, `common.cancel`

  **Must NOT do**:
  - Do not add translations beyond English (MVP scope)

  **Recommended Agent Profile**:
  - Category: `quick` - Reason: Standard i18n setup
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: UI components | Blocked By: 1

  **References**:
  - External: https://vue-i18n.intlify.dev/guide/

  **Acceptance Criteria**:
  - [ ] `src/locales/en.ts` exists with translation keys
  - [ ] i18n configured in main.ts
  - [ ] `$t('app.title')` works in components

  **QA Scenarios**:
  ```
  Scenario: i18n initializes correctly
    Tool: Vitest
    Steps: Import i18n from src/locales/index.ts, call i18n.global.t('app.title')
    Expected: Returns 'ScrcpyX' (or app title)
    Evidence: .sisyphus/evidence/task-4-i18n.test.ts
  ```

  **Commit**: YES | Message: `feat: set up i18n infrastructure` | Files: src/locales/*, src/main.ts

- [x] 5. Create Project Documentation

  **What to do**:
  - Create `README.md` with project overview, setup instructions
  - Document the tech stack and architecture decisions
  - Add development setup steps
  - Document available npm scripts
  - Create `.env.example` for environment variables

  **Must NOT do**:
  - Do not create extensive API documentation yet (defer to implementation)

  **Recommended Agent Profile**:
  - Category: `writing` - Reason: Documentation task
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 1 | Blocks: none | Blocked By: none

  **References**:
  - Pattern: `D:\projects\scrcpy\README.md` - Reference for documentation style

  **Acceptance Criteria**:
  - [ ] README.md exists with setup instructions
  - [ ] `.env.example` lists required environment variables

  **QA Scenarios**:
  ```
  Scenario: Documentation is complete
    Tool: Bash
    Steps: cd D:\projects\scrcpy-x && Test-Path README.md && Test-Path .env.example
    Expected: Both files exist
    Evidence: .sisyphus/evidence/task-5-docs.txt
  ```

  **Commit**: YES | Message: `docs: add project documentation` | Files: README.md, .env.example

### Wave 2: Core Backend (Rust)

- [x] 6. Define Tauri Command Structure

  **What to do**:
  - Create `src-tauri/src/commands/` directory
  - Create `src-tauri/src/commands/mod.rs` to export all commands
  - Create placeholder commands: `get_devices`, `launch_session`, `stop_session`, `get_profiles`
  - Register commands in `src-tauri/src/main.rs`
  - Define error types in `src-tauri/src/error.rs`

  **Must NOT do**:
  - Do not implement command logic yet (just structure)

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: Requires Rust architecture understanding
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: 7-11 | Blocked By: 1

  **References**:
  - External: https://tauri.app/v1/guides/features/command/
  - Pattern: `D:\projects\scrcpy\app\src\cli.c` - Reference for command patterns

  **Acceptance Criteria**:
  - [ ] Commands compile without errors
  - [ ] `cargo build` in src-tauri succeeds
  - [ ] Commands are registered in main.rs

  **QA Scenarios**:
  ```
  Scenario: Rust project compiles
    Tool: Bash
    Steps: cd D:\projects\scrcpy-x\src-tauri && cargo build
    Expected: Exit code 0, no compilation errors
    Evidence: .sisyphus/evidence/task-6-rust-build.log
  ```

  **Commit**: YES | Message: `feat(backend): define tauri command structure` | Files: src-tauri/src/commands/*, src-tauri/src/error.rs, src-tauri/src/main.rs

- [x] 7. Implement Binary Resolution (ADB & scrcpy)

  **What to do**:
  - Create `src-tauri/src/binaries/mod.rs`
  - Implement `find_adb()` - check PATH, then bundled fallback
  - Implement `find_scrcpy()` - check PATH, then bundled fallback
  - Create `src-tauri/src/binaries/bundled.rs` for bundled binary paths
  - Add binary path configuration to settings
  - Implement version check for found binaries

  **Must NOT do**:
  - Do not hardcode absolute paths
  - Do not bundle actual binaries yet (structure only)

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: Cross-platform path handling complexity
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: 8-10 | Blocked By: 6

  **References**:
  - External: https://doc.rust-lang.org/std/env/fn.var.html - Environment variables
  - External: https://crates.io/crates/which - For PATH resolution

  **Acceptance Criteria**:
  - [ ] `find_adb()` returns Ok(path) or Err(BinaryNotFound)
  - [ ] `find_scrcpy()` returns Ok(path) or Err(BinaryNotFound)
  - [ ] Unit tests pass for both functions

  **QA Scenarios**:
  ```
  Scenario: Binary resolution finds system ADB
    Tool: Vitest (call Tauri command)
    Steps: Invoke get_adb_path command, assert path exists
    Expected: Returns valid path string
    Evidence: .sisyphus/evidence/task-7-adb-path.test.ts

  Scenario: Binary resolution handles missing scrcpy
    Tool: Vitest
    Steps: Temporarily remove scrcpy from PATH mock, invoke get_scrcpy_path
    Expected: Returns error with actionable message
    Evidence: .sisyphus/evidence/task-7-scrcpy-missing.test.ts
  ```

  **Commit**: YES | Message: `feat(backend): implement binary resolution` | Files: src-tauri/src/binaries/*

- [x] 8. Implement Device Discovery (ADB Integration)

  **What to do**:
  - Create `src-tauri/src/device/mod.rs`
  - Implement `Device` struct with fields: id, name, model, status
  - Implement `get_devices()` command - runs `adb devices -l`
  - Parse ADB output to extract device info
  - Implement `DeviceWatcher` for real-time updates (polling-based)
  - Handle ADB server not running scenario

  **Must NOT do**:
  - Do not block main thread with ADB calls (use async)
  - Do not assume ADB is always available

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: Complex subprocess parsing, async handling
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: 14, 18 | Blocked By: 7

  **References**:
  - Pattern: `D:\projects\scrcpy\app\src\adb\*` - ADB interaction patterns
  - External: https://tauri.app/v1/guides/features/command/#async-commands

  **Acceptance Criteria**:
  - [ ] `get_devices()` returns list of Device structs
  - [ ] Handles "no devices" gracefully
  - [ ] Handles ADB not found error
  - [ ] Unit tests for ADB output parsing

  **QA Scenarios**:
  ```
  Scenario: Device discovery works with emulator
    Tool: Bash + Vitest
    Steps: Start Android emulator, invoke get_devices command
    Expected: Returns list with emulator-5554 device
    Evidence: .sisyphus/evidence/task-8-device-discovery.test.ts

  Scenario: Device discovery handles no devices
    Tool: Vitest
    Steps: Ensure no devices connected, invoke get_devices
    Expected: Returns empty list (not error)
    Evidence: .sisyphus/evidence/task-8-no-devices.test.ts
  ```

  **Commit**: YES | Message: `feat(backend): implement device discovery` | Files: src-tauri/src/device/*

- [x] 9. Implement Session Management (scrcpy Subprocess)

  **What to do**:
  - Create `src-tauri/src/session/mod.rs`
  - Implement `Session` struct with fields: id, device_id, process_handle, status
  - Implement `launch_session(device_id, options)` command
  - Build scrcpy CLI args from options struct
  - Spawn scrcpy process with proper stdio handling
  - Implement `stop_session(session_id)` - kill process gracefully
  - Implement `SessionManager` to track all active sessions
  - Handle process exit events (crash, disconnect)

  **Must NOT do**:
  - Do not use `kill -9` as first attempt (try graceful shutdown)
  - Do not leak process handles

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: Complex lifecycle management, error handling
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 2 | Blocks: 17, 19-21 | Blocked By: 7

  **References**:
  - Pattern: `D:\projects\scrcpy\app\src\main.c` - scrcpy startup patterns
  - External: https://doc.rust-lang.org/std/process/struct.Command.html

  **Acceptance Criteria**:
  - [ ] `launch_session()` spawns scrcpy process
  - [ ] Process arguments correctly formatted from options
  - [ ] `stop_session()` terminates process cleanly
  - [ ] Session status updates on process exit
  - [ ] All processes killed on app shutdown

  **QA Scenarios**:
  ```
  Scenario: Launch session creates process
    Tool: Vitest
    Steps: Mock device, invoke launch_session with minimal options
    Expected: Returns session_id, process exists
    Evidence: .sisyphus/evidence/task-9-launch-session.test.ts

  Scenario: Stop session terminates process
    Tool: Vitest
    Steps: Launch session, then invoke stop_session
    Expected: Process terminated, session removed
    Evidence: .sisyphus/evidence/task-9-stop-session.test.ts

  Scenario: Orphan cleanup on app exit
    Tool: Bash
    Steps: Start app, launch session, force-close app, check for orphan processes
    Expected: No scrcpy processes running
    Evidence: .sisyphus/evidence/task-9-orphan-cleanup.log
  ```

  **Commit**: YES | Message: `feat(backend): implement session management` | Files: src-tauri/src/session/*

- [x] 10. Implement Device Control Commands

  **What to do**:
  - Create `src-tauri/src/controls/mod.rs`
  - Implement `send_key_event(device_id, keycode)` - via ADB
  - Implement `take_screenshot(device_id)` - via ADB shell screencap
  - Implement `set_rotation(device_id, orientation)` - via ADB
  - Implement `set_volume(device_id, level)` - via ADB
  - Implement `turn_screen_on/off(device_id)` - via ADB
  - Map to scrcpy shortcuts where applicable

  **Must NOT do**:
  - Do not implement complex gestures (out of scope)

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: ADB command knowledge required
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: 21 | Blocked By: 7

  **References**:
  - Pattern: `D:\projects\scrcpy\app\src\controller.c` - Control message patterns
  - External: https://developer.android.com/reference/android/view/KeyEvent

  **Acceptance Criteria**:
  - [ ] `take_screenshot()` returns image path or data
  - [ ] `send_key_event()` executes without error
  - [ ] `set_rotation()` rotates device display

  **QA Scenarios**:
  ```
  Scenario: Screenshot captures device screen
    Tool: Vitest
    Steps: Connect device, invoke take_screenshot
    Expected: Returns PNG data, non-empty
    Evidence: .sisyphus/evidence/task-10-screenshot.test.ts

  Scenario: Key event sent successfully
    Tool: Vitest
    Steps: Invoke send_key_event(device_id, KEYCODE_HOME)
    Expected: Command returns success
    Evidence: .sisyphus/evidence/task-10-key-event.test.ts
  ```

  **Commit**: YES | Message: `feat(backend): implement device control commands` | Files: src-tauri/src/controls/*

- [x] 11. Implement Event Emission System

  **What to do**:
  - Create `src-tauri/src/events/mod.rs`
  - Define event types: `device-connected`, `device-disconnected`, `session-started`, `session-ended`, `session-error`
  - Implement event emission using Tauri's event system
  - Create `emit_event()` helper function
  - Document event payloads in comments

  **Must NOT do**:
  - Do not emit events from unrelated code paths

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: Requires understanding Tauri event system
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 2 | Blocks: Wave 5 | Blocked By: 6

  **References**:
  - External: https://tauri.app/v1/guides/features/events/

  **Acceptance Criteria**:
  - [ ] Event types defined with clear payloads
  - [ ] `emit_event()` compiles and works
  - [ ] Frontend can listen to events (verified in integration)

  **QA Scenarios**:
  ```
  Scenario: Events can be emitted and received
    Tool: Vitest
    Steps: Listen to 'device-connected' event, emit event from Rust
    Expected: Frontend receives event with correct payload
    Evidence: .sisyphus/evidence/task-11-events.test.ts
  ```

  **Commit**: YES | Message: `feat(backend): implement event emission system` | Files: src-tauri/src/events/*

### Wave 3: Data Layer

- [x] 12. Define TypeScript Types and Interfaces

  **What to do**:
  - Create `src/types/` directory
  - Create `src/types/device.ts` - Device, DeviceStatus interfaces
  - Create `src/types/session.ts` - Session, SessionStatus, SessionOptions interfaces
  - Create `src/types/profile.ts` - Profile, ProfileOptions interfaces
  - Create `src/types/settings.ts` - AppSettings, BinaryPaths interfaces
  - Create `src/types/index.ts` - Export all types
  - Ensure types match Rust structs (for IPC serialization)

  **Must NOT do**:
  - Do not use `any` type

  **Recommended Agent Profile**:
  - Category: `quick` - Reason: Type definitions, straightforward
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 3 | Blocks: 13-16, Wave 4 | Blocked By: 1

  **References**:
  - Pattern: `D:\projects\scrcpy\app\src\options.h` - Match scrcpy option types

  **Acceptance Criteria**:
  - [ ] All types exported from index.ts
  - [ ] TypeScript compiles without errors
  - [ ] Types match Rust struct fields

  **QA Scenarios**:
  ```
  Scenario: Types compile correctly
    Tool: Bash
    Steps: cd D:\projects\scrcpy-x && npx tsc --noEmit
    Expected: No errors
    Evidence: .sisyphus/evidence/task-12-types.log
  ```

  **Commit**: YES | Message: `feat(types): define typescript types and interfaces` | Files: src/types/*

- [x] 13. Implement Pinia Stores

  **What to do**:
  - Create `src/stores/` directory
  - Create `src/stores/deviceStore.ts` - Device list, selected device
  - Create `src/stores/sessionStore.ts` - Active sessions, session state
  - Create `src/stores/profileStore.ts` - Profiles, active profile
  - Create `src/stores/settingsStore.ts` - App settings, binary paths
  - Implement getters, actions for each store
  - Add persistence middleware (connect to Tauri storage)

  **Must NOT do**:
  - Do not mix unrelated state in same store

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: State management architecture
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 3 | Blocks: Wave 4 | Blocked By: 12

  **References**:
  - External: https://pinia.vuejs.org/core-concepts/

  **Acceptance Criteria**:
  - [ ] All stores created with proper typing
  - [ ] Stores initialize with default values
  - [ ] Actions are typed correctly

  **QA Scenarios**:
  ```
  Scenario: Stores initialize correctly
    Tool: Vitest
    Steps: Import each store, access state, verify defaults
    Expected: All stores have expected default state
    Evidence: .sisyphus/evidence/task-13-stores.test.ts
  ```

  **Commit**: YES | Message: `feat(store): implement pinia stores` | Files: src/stores/*

- [x] 14. Implement Profile Storage Backend

  **What to do**:
  - Create `src-tauri/src/storage/mod.rs`
  - Implement `Profile` struct matching TypeScript interface
  - Implement `save_profile(profile)` - JSON to app data dir
  - Implement `load_profile(name)` - JSON from app data dir
  - Implement `list_profiles()` - list all saved profiles
  - Implement `delete_profile(name)` - remove profile file
  - Use atomic writes (write to temp, then rename)
  - Register storage commands in Tauri

  **Must NOT do**:
  - Do not store profiles in hardcoded paths
  - Do not overwrite without backup

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: File I/O, cross-platform paths
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 3 | Blocks: 16 | Blocked By: 6

  **References**:
  - External: https://tauri.app/v1/guides/features/appdata/

  **Acceptance Criteria**:
  - [ ] Profiles saved to app data directory
  - [ ] `list_profiles()` returns all profiles
  - [ ] Corrupted profile handled gracefully

  **QA Scenarios**:
  ```
  Scenario: Profile CRUD works
    Tool: Vitest
    Steps: Create profile, save, list, load, delete
    Expected: All operations succeed, data matches
    Evidence: .sisyphus/evidence/task-14-profile-crud.test.ts

  Scenario: Corrupted profile handled
    Tool: Vitest
    Steps: Write invalid JSON to profile file, call load_profile
    Expected: Returns error, doesn't crash
    Evidence: .sisyphus/evidence/task-14-corrupt-profile.test.ts
  ```

  **Commit**: YES | Message: `feat(backend): implement profile storage` | Files: src-tauri/src/storage/*

- [x] 15. Implement Settings Persistence

  **What to do**:
  - Create `src-tauri/src/settings/mod.rs`
  - Implement `AppSettings` struct: last_profile, window_bounds, binary_paths
  - Implement `save_settings(settings)` command
  - Implement `load_settings()` command
  - Auto-save on app close (Tauri close event)
  - Auto-load on app start

  **Must NOT do**:
  - Do not store sensitive data without encryption

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: Cross-platform app data handling
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 3 | Blocks: 16 | Blocked By: 6

  **References**:
  - External: https://tauri.app/v1/guides/features/appdata/

  **Acceptance Criteria**:
  - [ ] Settings persist across app restarts
  - [ ] Default settings used if none exist

  **QA Scenarios**:
  ```
  Scenario: Settings persist across restarts
    Tool: E2E
    Steps: Change setting, close app, reopen app, check setting
    Expected: Setting value preserved
    Evidence: .sisyphus/evidence/task-15-settings-persist.test.ts
  ```

  **Commit**: YES | Message: `feat(backend): implement settings persistence` | Files: src-tauri/src/settings/*

- [x] 16. Connect Stores to Backend Commands

  **What to do**:
  - Create `src/api/` directory for Tauri command wrappers
  - Create `src/api/devices.ts` - wrap get_devices command
  - Create `src/api/sessions.ts` - wrap launch/stop_session commands
  - Create `src/api/profiles.ts` - wrap profile CRUD commands
  - Create `src/api/settings.ts` - wrap settings commands
  - Implement error handling and type conversion
  - Connect stores to API calls

  **Must NOT do**:
  - Do not call Tauri commands directly from components

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: IPC integration, error handling
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 3 | Blocks: Wave 5 | Blocked By: 13, 14, 15

  **References**:
  - External: https://tauri.app/v1/api/js/tauri/#invoke

  **Acceptance Criteria**:
  - [ ] All API wrappers typed correctly
  - [ ] Errors caught and surfaced to UI
  - [ ] Stores update from API responses

  **QA Scenarios**:
  ```
  Scenario: API wrappers work correctly
    Tool: Vitest
    Steps: Mock Tauri invoke, call API wrapper, verify result
    Expected: Returns typed data, handles errors
    Evidence: .sisyphus/evidence/task-16-api-wrappers.test.ts
  ```

  **Commit**: YES | Message: `feat(api): connect stores to backend commands` | Files: src/api/*

### Wave 4: UI Components

- [x] 17. Create App Layout and Navigation

  **What to do**:
  - Create `src/layouts/MainLayout.vue` - main app container
  - Implement sidebar navigation (Devices, Profiles, Settings)
  - Create `src/views/` directory
  - Create `src/views/DevicesView.vue` - device management
  - Create `src/views/ProfilesView.vue` - profile management
  - Create `src/views/SettingsView.vue` - app settings
  - Configure Vue Router with routes

  **Must NOT do**:
  - Do not use complex navigation patterns (keep simple)

  **Recommended Agent Profile**:
  - Category: `visual-engineering` - Reason: UI layout and navigation
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 4 | Blocks: 18-22 | Blocked By: 4, 12

  **References**:
  - External: https://www.naiveui.com/en-US/os-theme/components/layout

  **Acceptance Criteria**:
  - [ ] Navigation between views works
  - [ ] Layout responsive on resize
  - [ ] Active view highlighted in sidebar

  **QA Scenarios**:
  ```
  Scenario: Navigation works
    Tool: E2E
    Steps: Click sidebar items, verify view changes
    Expected: Each click shows correct view
    Evidence: .sisyphus/evidence/task-17-navigation.test.ts
  ```

  **Commit**: YES | Message: `feat(ui): create app layout and navigation` | Files: src/layouts/*, src/views/*, src/router/*

- [x] 18. Create Device List Component

  **What to do**:
  - Create `src/components/DeviceList.vue`
  - Display connected devices in a table/list
  - Show device name, model, status (online/offline)
  - Implement device selection (single/multi-select)
  - Add refresh button to rescan devices
  - Connect to deviceStore
  - Show loading state during scan

  **Must NOT do**:
  - Do not poll devices continuously (use events)

  **Recommended Agent Profile**:
  - Category: `visual-engineering` - Reason: List component with state
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: 23 | Blocked By: 17

  **References**:
  - External: https://www.naiveui.com/en-US/os-theme/components/data-table

  **Acceptance Criteria**:
  - [ ] Devices displayed in list
  - [ ] Selection works (single/multi)
  - [ ] Refresh triggers device scan

  **QA Scenarios**:
  ```
  Scenario: Device list displays devices
    Tool: E2E
    Steps: Connect device, open DevicesView, verify device appears
    Expected: Device shown with correct name/status
    Evidence: .sisyphus/evidence/task-18-device-list.test.ts
  ```

  **Commit**: YES | Message: `feat(ui): create device list component` | Files: src/components/DeviceList.vue

- [x] 19. Create Session Options Form

  **What to do**:
  - Create `src/components/SessionOptionsForm.vue`
  - Group options by category (Video, Audio, Recording, etc.)
  - Use Naive UI form components (n-slider, n-select, n-switch, n-input-number)
  - Implement validation for option values
  - Map form values to SessionOptions type
  - Show tooltips with option descriptions

  **Must NOT do**:
  - Do not expose all 100+ options (curated set only)

  **Recommended Agent Profile**:
  - Category: `visual-engineering` - Reason: Complex form with many fields
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: 23 | Blocked By: 12

  **References**:
  - Pattern: `D:\projects\scrcpy\app\src\cli.c` - Option descriptions
  - External: https://www.naiveui.com/en-US/os-theme/components/form

  **Acceptance Criteria**:
  - [ ] All 7 categories represented
  - [ ] Form validation works
  - [ ] Values map to SessionOptions

  **QA Scenarios**:
  ```
  Scenario: Form validates correctly
    Tool: Vitest
    Steps: Set invalid values, check validation errors
    Expected: Validation messages shown for invalid inputs
    Evidence: .sisyphus/evidence/task-19-form-validation.test.ts
  ```

  **Commit**: YES | Message: `feat(ui): create session options form` | Files: src/components/SessionOptionsForm.vue

- [x] 20. Create Profile Management UI

  **What to do**:
  - Create `src/components/ProfileList.vue` - list saved profiles
  - Create `src/components/ProfileEditor.vue` - edit profile details
  - Implement profile CRUD UI (create, rename, duplicate, delete)
  - Show profile preview (options summary)
  - Connect to profileStore
  - Add "Set as Default" toggle

  **Must NOT do**:
  - Do not allow deleting the last profile

  **Recommended Agent Profile**:
  - Category: `visual-engineering` - Reason: CRUD UI pattern
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: 23 | Blocked By: 17

  **References**:
  - External: https://www.naiveui.com/en-US/os-theme/components/list

  **Acceptance Criteria**:
  - [ ] Profile CRUD operations work
  - [ ] Profile list updates on changes
  - [ ] Default profile highlighted

  **QA Scenarios**:
  ```
  Scenario: Profile CRUD works in UI
    Tool: E2E
    Steps: Create new profile, edit, duplicate, delete
    Expected: All operations succeed, list updates
    Evidence: .sisyphus/evidence/task-20-profile-ui.test.ts
  ```

  **Commit**: YES | Message: `feat(ui): create profile management ui` | Files: src/components/ProfileList.vue, src/components/ProfileEditor.vue

- [x] 21. Create Device Control Panel

  **What to do**:
  - Create `src/components/DeviceControlPanel.vue`
  - Add control buttons: Rotate, Fullscreen, Screenshot, Screen Off/On
  - Add volume controls (slider)
  - Add navigation buttons: Back, Home, Recents
  - Show session status indicator
  - Implement keyboard shortcuts for controls
  - Connect to session control API

  **Must NOT do**:
  - Do not block UI while sending commands

  **Recommended Agent Profile**:
  - Category: `visual-engineering` - Reason: Interactive control panel
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: 25 | Blocked By: 12

  **References**:
  - Pattern: `D:\projects\scrcpy\app\src\shortcut_mod.h` - Shortcut patterns
  - External: https://www.naiveui.com/en-US/os-theme/components/button

  **Acceptance Criteria**:
  - [ ] All control buttons functional
  - [ ] Keyboard shortcuts work
  - [ ] Status indicator updates

  **QA Scenarios**:
  ```
  Scenario: Control buttons send commands
    Tool: E2E
    Steps: Click each control button, verify command sent
    Expected: Commands execute on device
    Evidence: .sisyphus/evidence/task-21-controls.test.ts
  ```

  **Commit**: YES | Message: `feat(ui): create device control panel` | Files: src/components/DeviceControlPanel.vue

- [x] 22. Create Session Status View

  **What to do**:
  - Create `src/components/SessionStatus.vue`
  - Show active sessions in a list/grid
  - Display session info: device name, status, duration
  - Add stop button for each session
  - Show error messages if session failed
  - Implement session selection for multi-device

  **Must NOT do**:
  - Do not show excessive details (keep summary view)

  **Recommended Agent Profile**:
  - Category: `visual-engineering` - Reason: Status display component
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 4 | Blocks: 25 | Blocked By: 12

  **References**:
  - External: https://www.naiveui.com/en-US/os-theme/components/card

  **Acceptance Criteria**:
  - [ ] Active sessions displayed
  - [ ] Stop button terminates session
  - [ ] Errors shown with actionable messages

  **QA Scenarios**:
  ```
  Scenario: Session status updates
    Tool: E2E
    Steps: Launch session, verify status shows, stop session
    Expected: Status updates correctly throughout
    Evidence: .sisyphus/evidence/task-22-session-status.test.ts
  ```

  **Commit**: YES | Message: `feat(ui): create session status view` | Files: src/components/SessionStatus.vue

- [x] 23. Create Launch View (Main Action UI)

  **What to do**:
  - Create `src/views/LaunchView.vue`
  - Combine DeviceList, SessionOptionsForm, ProfileList
  - Add "Launch" button with device selection
  - Show quick profile selector dropdown
  - Implement "Launch All Selected" for multi-device
  - Add confirmation for destructive actions

  **Must NOT do**:
  - Do not allow launch without device selection

  **Recommended Agent Profile**:
  - Category: `visual-engineering` - Reason: Composite view assembly
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 4 | Blocks: Wave 5 | Blocked By: 17, 18, 19, 20

  **References**:
  - External: https://www.naiveui.com/en-US/os-theme/components/space

  **Acceptance Criteria**:
  - [ ] Launch button disabled without device
  - [ ] Profile selection affects options
  - [ ] Multi-device launch works

  **QA Scenarios**:
  ```
  Scenario: Launch flow works
    Tool: E2E
    Steps: Select device, select profile, click Launch
    Expected: Session starts, scrcpy window appears
    Evidence: .sisyphus/evidence/task-23-launch.test.ts
  ```

  **Commit**: YES | Message: `feat(ui): create launch view` | Files: src/views/LaunchView.vue

### Wave 5: Integration

- [x] 24. Connect Device Discovery to UI

  **What to do**:
  - Subscribe to device events in deviceStore
  - Update DeviceList when devices connect/disconnect
  - Implement auto-refresh on app focus
  - Show toast notifications for device events
  - Handle device disconnect during session

  **Must NOT do**:
  - Do not poll continuously (use events)

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: Event-driven integration
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 5 | Blocks: none | Blocked By: 16, 18

  **References**:
  - External: https://tauri.app/v1/api/js/event/

  **Acceptance Criteria**:
  - [ ] Device list updates on connect/disconnect
  - [ ] Toast shows for device events
  - [ ] Session handles device disconnect

  **QA Scenarios**:
  ```
  Scenario: Hot-plug updates UI
    Tool: E2E
    Steps: Connect device, verify list updates, disconnect, verify removal
    Expected: List updates without manual refresh
    Evidence: .sisyphus/evidence/task-24-hotplug.test.ts
  ```

  **Commit**: YES | Message: `feat(integration): connect device discovery to ui` | Files: src/stores/deviceStore.ts, src/components/DeviceList.vue

- [x] 25. Connect Session Management to UI

  **What to do**:
  - Connect Launch button to launch_session API
  - Update SessionStatus on session events
  - Connect DeviceControlPanel to active session
  - Handle session errors with user-friendly messages
  - Implement session cleanup on app close

  **Must NOT do**:
  - Do not leave orphan sessions on error

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: Complex state synchronization
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 5 | Blocks: Wave 6 | Blocked By: 16, 21, 22, 23

  **References**:
  - External: https://tauri.app/v1/api/js/event/

  **Acceptance Criteria**:
  - [ ] Launch creates session
  - [ ] Session status reflects reality
  - [ ] Controls affect correct session

  **QA Scenarios**:
  ```
  Scenario: Session lifecycle works
    Tool: E2E
    Steps: Launch session, verify status, use controls, stop session
    Expected: All operations work, status accurate
    Evidence: .sisyphus/evidence/task-25-session-lifecycle.test.ts
  ```

  **Commit**: YES | Message: `feat(integration): connect session management to ui` | Files: src/stores/sessionStore.ts, src/views/LaunchView.vue

- [x] 26. Connect Profile Management to UI

  **What to do**:
  - Connect ProfileEditor to profile API
  - Auto-save profile on changes (debounced)
  - Load default profile on app start
  - Implement profile import/export (JSON file)
  - Show unsaved changes indicator

  **Must NOT do**:
  - Do not auto-save too frequently (debounce)

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: Persistence integration
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 5 | Blocks: none | Blocked By: 16, 20

  **References**:
  - External: https://tauri.app/v1/api/js/dialog/

  **Acceptance Criteria**:
  - [ ] Profile changes persist
  - [ ] Default profile loads on start
  - [ ] Import/export works

  **QA Scenarios**:
  ```
  Scenario: Profile persistence works
    Tool: E2E
    Steps: Edit profile, close app, reopen, verify changes
    Expected: Profile changes preserved
    Evidence: .sisyphus/evidence/task-26-profile-persist.test.ts
  ```

  **Commit**: YES | Message: `feat(integration): connect profile management to ui` | Files: src/stores/profileStore.ts, src/components/ProfileEditor.vue

- [x] 27. Implement Error Handling and Notifications

  **What to do**:
  - Create `src/utils/notifications.ts` - toast wrapper
  - Create `src/utils/errors.ts` - error type guards
  - Implement global error boundary
  - Map backend errors to user-friendly messages
  - Add retry actions for recoverable errors
  - Implement error logging (local file)

  **Must NOT do**:
  - Do not show raw error messages to users

  **Recommended Agent Profile**:
  - Category: `deep` - Reason: UX-critical error handling
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 5 | Blocks: none | Blocked By: 16

  **References**:
  - External: https://www.naiveui.com/en-US/os-theme/components/message

  **Acceptance Criteria**:
  - [ ] All errors show user-friendly message
  - [ ] Recoverable errors have retry action
  - [ ] Errors logged for debugging

  **QA Scenarios**:
  ```
  Scenario: Error shows friendly message
    Tool: E2E
    Steps: Trigger error (e.g., invalid device), verify toast
    Expected: Toast shows actionable message
    Evidence: .sisyphus/evidence/task-27-error-handling.test.ts
  ```

  **Commit**: YES | Message: `feat(integration): implement error handling and notifications` | Files: src/utils/notifications.ts, src/utils/errors.ts

- [x] 28. Implement Settings View

  **What to do**:
  - Create `src/views/SettingsView.vue` content
  - Add binary path configuration (ADB, scrcpy)
  - Add theme selector (light/dark/system)
  - Add auto-start options
  - Add max concurrent sessions setting
  - Connect to settingsStore and backend

  **Must NOT do**:
  - Do not add settings beyond defined scope

  **Recommended Agent Profile**:
  - Category: `visual-engineering` - Reason: Settings form UI
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 5 | Blocks: none | Blocked By: 17

  **References**:
  - External: https://www.naiveui.com/en-US/os-theme/components/switch

  **Acceptance Criteria**:
  - [ ] Settings persist
  - [ ] Theme toggle works
  - [ ] Binary paths configurable

  **QA Scenarios**:
  ```
  Scenario: Settings persist
    Tool: E2E
    Steps: Change setting, close app, reopen, verify
    Expected: Setting preserved
    Evidence: .sisyphus/evidence/task-28-settings.test.ts
  ```

  **Commit**: YES | Message: `feat(ui): implement settings view` | Files: src/views/SettingsView.vue

### Wave 6: Features

- [x] 29. Implement Multi-Device Session Management

  **What to do**:
  - Enable multi-select in DeviceList
  - Implement "Launch All" for selected devices
  - Track multiple sessions in SessionStatus
  - Implement per-session control panel
  - Add session switcher for control panel
  - Implement max sessions limit

  **Must NOT do**:
  - Do not exceed max concurrent sessions

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Reason: Complex multi-state management
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 6 | Blocks: none | Blocked By: 25

  **References**:
  - External: https://www.naiveui.com/en-US/os-theme/components/tabs

  **Acceptance Criteria**:
  - [ ] Multiple devices can launch
  - [ ] Each session tracked separately
  - [ ] Max limit enforced

  **QA Scenarios**:
  ```
  Scenario: Multi-device launch works
    Tool: E2E
    Steps: Select 2 devices, click Launch All
    Expected: Both sessions start, both tracked
    Evidence: .sisyphus/evidence/task-29-multi-device.test.ts
  ```

  **Commit**: YES | Message: `feat: implement multi-device session management` | Files: src/components/DeviceList.vue, src/stores/sessionStore.ts

- [x] 30. Implement Screenshot Feature

  **What to do**:
  - Add screenshot button to control panel
  - Call take_screenshot API
  - Show preview in notification
  - Implement save to file (with dialog)
  - Add copy to clipboard option
  - Store recent screenshots in session

  **Must NOT do**:
  - Do not block UI during capture

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Reason: Feature with multiple paths
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 6 | Blocks: none | Blocked By: 25

  **References**:
  - External: https://tauri.app/v1/api/js/dialog/

  **Acceptance Criteria**:
  - [ ] Screenshot captures screen
  - [ ] Save dialog works
  - [ ] Copy to clipboard works

  **QA Scenarios**:
  ```
  Scenario: Screenshot works
    Tool: E2E
    Steps: Click screenshot button, verify preview, save
    Expected: Image saved, preview shown
    Evidence: .sisyphus/evidence/task-30-screenshot.test.ts
  ```

  **Commit**: YES | Message: `feat: implement screenshot feature` | Files: src/components/DeviceControlPanel.vue

- [x] 31. Implement Recording Feature

  **What to do**:
  - Add recording controls to session options
  - Implement start/stop recording
  - Show recording indicator in session status
  - Implement recording path configuration
  - Add recording format selector
  - Show recording duration

  **Must NOT do**:
  - Do not allow recording without path

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Reason: Feature with state tracking
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 6 | Blocks: none | Blocked By: 25

  **References**:
  - Pattern: `D:\projects\scrcpy\doc\recording.md` - Recording options

  **Acceptance Criteria**:
  - [ ] Recording starts/stops
  - [ ] File saved to configured path
  - [ ] Duration displayed

  **QA Scenarios**:
  ```
  Scenario: Recording works
    Tool: E2E
    Steps: Start recording, wait, stop, verify file exists
    Expected: Recording file created with content
    Evidence: .sisyphus/evidence/task-31-recording.test.ts
  ```

  **Commit**: YES | Message: `feat: implement recording feature` | Files: src/components/SessionOptionsForm.vue, src/stores/sessionStore.ts

- [x] 32. Implement Keyboard Shortcuts

  **What to do**:
  - Create `src/utils/shortcuts.ts` - shortcut definitions
  - Implement global shortcuts: Ctrl+R (refresh), Ctrl+S (screenshot)
  - Implement session shortcuts: F (fullscreen), O (screen off)
  - Show shortcuts in tooltips
  - Make shortcuts configurable in settings

  **Must NOT do**:
  - Do not override system shortcuts

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Reason: Keyboard event handling
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 6 | Blocks: none | Blocked By: 25

  **References**:
  - Pattern: `D:\projects\scrcpy\app\src\shortcut_mod.h` - Shortcut patterns

  **Acceptance Criteria**:
  - [ ] Shortcuts trigger actions
  - [ ] Shortcuts shown in tooltips
  - [ ] Configurable in settings

  **QA Scenarios**:
  ```
  Scenario: Shortcuts work
    Tool: E2E
    Steps: Press Ctrl+R, verify refresh triggered
    Expected: Action executes
    Evidence: .sisyphus/evidence/task-32-shortcuts.test.ts
  ```

  **Commit**: YES | Message: `feat: implement keyboard shortcuts` | Files: src/utils/shortcuts.ts

- [x] 33. Implement Window State Persistence

  **What to do**:
  - Save window position and size on close
  - Restore window state on open
  - Handle multi-monitor scenarios
  - Implement "Always on Top" toggle
  - Save sidebar collapsed state

  **Must NOT do**:
  - Do not restore off-screen position

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Reason: Platform-specific window handling
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 6 | Blocks: none | Blocked By: 15

  **References**:
  - External: https://tauri.app/v1/api/js/window/

  **Acceptance Criteria**:
  - [ ] Window state persists
  - [ ] Off-screen handled
  - [ ] Always on Top works

  **QA Scenarios**:
  ```
  Scenario: Window state persists
    Tool: E2E
    Steps: Move window, close, reopen, verify position
    Expected: Window in same position
    Evidence: .sisyphus/evidence/task-33-window-state.test.ts
  ```

  **Commit**: YES | Message: `feat: implement window state persistence` | Files: src/stores/settingsStore.ts

- [x] 34. Implement Binary Bundling Setup

  **What to do**:
  - Create `src-tauri/binaries/` directory
  - Add bundling configuration in `tauri.conf.json`
  - Create scripts to download scrcpy/adb binaries
  - Implement version check for bundled binaries
  - Add binary update mechanism
  - Document bundled binary versions

  **Must NOT do**:
  - Do not commit large binaries to git (use CI download)

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Reason: Build configuration, external dependencies
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 6 | Blocks: 35 | Blocked By: 7

  **References**:
  - External: https://tauri.app/v1/guides/building/app-size/
  - Pattern: `D:\projects\scrcpy\doc\windows.md` - Windows binary info

  **Acceptance Criteria**:
  - [ ] Binaries bundled in release build
  - [ ] Version check works
  - [ ] Fallback to bundled works

  **QA Scenarios**:
  ```
  Scenario: Bundled binary works
    Tool: Bash
    Steps: Build release, run on clean machine, verify bundled binary used
    Expected: App works without system scrcpy
    Evidence: .sisyphus/evidence/task-34-bundling.log
  ```

  **Commit**: YES | Message: `feat: implement binary bundling setup` | Files: src-tauri/tauri.conf.json, scripts/*

### Wave 7: Packaging & Polish

- [x] 35. Configure Cross-Platform Builds

  **What to do**:
  - Configure Windows build (NSIS installer)
  - Configure macOS build (DMG, code signing)
  - Configure Linux build (AppImage, deb)
  - Add build scripts for each platform
  - Configure CI/CD for automated builds
  - Add version info to builds

  **Must NOT do**:
  - Do not require manual signing (automate or document)

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Reason: Platform-specific build config
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 7 | Blocks: none | Blocked By: 34

  **References**:
  - External: https://tauri.app/v1/guides/building/

  **Acceptance Criteria**:
  - [ ] Windows installer builds
  - [ ] macOS DMG builds
  - [ ] Linux AppImage builds

  **QA Scenarios**:
  ```
  Scenario: All platforms build
    Tool: Bash
    Steps: npm run tauri build -- --target all
    Expected: Installers for all platforms created
    Evidence: .sisyphus/evidence/task-35-build.log
  ```

  **Commit**: YES | Message: `build: configure cross-platform builds` | Files: src-tauri/tauri.conf.json, .github/workflows/*

- [x] 36. Write Comprehensive E2E Tests

  **What to do**:
  - Write E2E test for full launch flow
  - Write E2E test for profile management
  - Write E2E test for multi-device
  - Write E2E test for device controls
  - Write E2E test for error scenarios
  - Configure CI to run E2E tests

  **Must NOT do**:
  - Do not skip E2E tests for critical paths

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Reason: Test writing
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 7 | Blocks: none | Blocked By: 3

  **References**:
  - External: https://tauri.app/v1/guides/testing/webdriver/

  **Acceptance Criteria**:
  - [ ] All critical flows tested
  - [ ] Tests pass on all platforms
  - [ ] CI runs tests automatically

  **QA Scenarios**:
  ```
  Scenario: E2E tests pass
    Tool: Bash
    Steps: npm run test:e2e
    Expected: All tests pass
    Evidence: .sisyphus/evidence/task-36-e2e.log
  ```

  **Commit**: YES | Message: `test: write comprehensive e2e tests` | Files: e2e/*

- [x] 37. Add Application Icon and Branding

  **What to do**:
  - Create app icon (multiple sizes)
  - Configure icon in tauri.conf.json
  - Add splash screen (if supported)
  - Update app metadata (name, description)
  - Create about dialog
  - Add license information

  **Must NOT do**:
  - Do not use copyrighted icons

  **Recommended Agent Profile**:
  - Category: `visual-engineering` - Reason: Icon and branding
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 7 | Blocks: none | Blocked By: none

  **References**:
  - Pattern: `D:\projects\scrcpy\app\data\icon.svg` - Reference icon

  **Acceptance Criteria**:
  - [ ] Icon shows in taskbar/dock
  - [ ] About dialog shows info
  - [ ] App name correct

  **QA Scenarios**:
  ```
  Scenario: Branding correct
    Tool: E2E
    Steps: Check window title, taskbar icon, about dialog
    Expected: Shows ScrcpyX branding
    Evidence: .sisyphus/evidence/task-37-branding.test.ts
  ```

  **Commit**: YES | Message: `style: add application icon and branding` | Files: src-tauri/icons/*, src/components/AboutDialog.vue

- [x] 38. Write User Documentation

  **What to do**:
  - Update README with user instructions
  - Document all UI features
  - Add troubleshooting section
  - Document keyboard shortcuts
  - Add FAQ for common issues
  - Create getting started guide

  **Must NOT do**:
  - Do not duplicate scrcpy documentation (link to it)

  **Recommended Agent Profile**:
  - Category: `writing` - Reason: Documentation
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: YES | Wave 7 | Blocks: none | Blocked By: none

  **References**:
  - Pattern: `D:\projects\scrcpy\README.md` - Documentation style

  **Acceptance Criteria**:
  - [ ] README complete
  - [ ] Shortcuts documented
  - [ ] FAQ helpful

  **QA Scenarios**:
  ```
  Scenario: Documentation complete
    Tool: Bash
    Steps: Check README sections exist
    Expected: All sections present
    Evidence: .sisyphus/evidence/task-38-docs.txt
  ```

  **Commit**: YES | Message: `docs: write user documentation` | Files: README.md, docs/*

- [x] 39. Final Integration Test and Polish

  **What to do**:
  - Run full test suite
  - Fix any failing tests
  - Performance optimization (lazy loading)
  - Accessibility audit (keyboard nav, ARIA)
  - Code cleanup and refactoring
  - Remove debug code and console logs

  **Must NOT do**:
  - Do not skip failing tests

  **Recommended Agent Profile**:
  - Category: `unspecified-high` - Reason: Final polish
  - Skills: [] - None needed

  **Parallelization**: Can Parallel: NO | Wave 7 | Blocks: none | Blocked By: all previous

  **References**:
  - External: https://www.naiveui.com/en-US/os-theme/docs/accessibility

  **Acceptance Criteria**:
  - [ ] All tests pass
  - [ ] No console errors
  - [ ] Keyboard navigation works

  **QA Scenarios**:
  ```
  Scenario: All tests pass
    Tool: Bash
    Steps: npm run test && npm run test:e2e
    Expected: All tests pass
    Evidence: .sisyphus/evidence/task-39-final.log
  ```

  **Commit**: YES | Message: `chore: final integration test and polish` | Files: various

## Final Verification Wave (MANDATORY — after ALL implementation tasks)
> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.
> **Do NOT auto-proceed after verification. Wait for user's explicit approval before marking work complete.**
> **Never mark F1-F4 as checked before getting user's okay.** Rejection or user feedback -> fix -> re-run -> present again -> wait for okay.

- [x] F1. Plan Compliance Audit — oracle
  - Verify all 39 tasks completed as specified
  - Check no scope creep occurred
  - Confirm all acceptance criteria met

- [x] F2. Code Quality Review — unspecified-high
  - TypeScript strict mode compliance
  - Rust clippy warnings addressed
  - No console.log statements in production code
  - Proper error handling throughout

- [x] F3. Real Manual QA — unspecified-high (+ playwright if UI)
  - Launch app on Windows
  - Connect physical device or emulator
  - Execute full launch flow
  - Test multi-device scenario
  - Test profile management
  - Test all device controls
  - Verify app exit cleans up processes

- [x] F4. Scope Fidelity Check — deep
  - No modifications to scrcpy source
  - No unmanaged subprocess leaks
  - No hardcoded paths
  - All guardrails respected

## Commit Strategy
- Each task commits independently with semantic commit messages
- Format: `type(scope): description`
- Types: feat, fix, docs, style, refactor, test, chore, build
- Squash related commits before merge to main

## Success Criteria
1. **Functional**: App launches, detects devices, starts sessions, controls work
2. **Cross-Platform**: Builds and runs on Windows, macOS, Linux
3. **Quality**: All tests pass, no console errors, accessible
4. **User Experience**: Intuitive UI, helpful error messages, keyboard navigation
5. **Reliability**: No orphan processes, graceful error handling, data persistence works
