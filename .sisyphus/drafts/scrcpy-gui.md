# Draft: scrcpy-x GUI Application

## Requirements (confirmed)
- Build a GUI app for scrcpy (screen mirroring tool)
- Cross-platform: Windows, macOS, Linux
- Tech stack: Tauri + Vue3 + Naive UI
- Two main UI components:
  1. Settings/Launch UI - configure and start scrcpy sessions
  2. Device Operation UI - interact with running phone interface

## Technical Decisions
- [CONFIRMED] App name: ScrcpyX
- [CONFIRMED] Feature scope: Curated Essential Set (all 7 categories)
  - Connection: serial, TCP/IP, port range
  - Video Quality: codec, bitrate, max-size, max-fps, crop
  - Audio: enable/disable, codec, bitrate, source
  - Recording: record to file, format
  - Window: fullscreen, always-on-top, borderless, position/size
  - Device Behavior: turn screen off, stay awake, show touches
  - Input Control: keyboard/mouse mode, shortcut modifier
- [CONFIRMED] Integration: Subprocess + Window Management (separate windows)
- [CONFIRMED] Device management: Multi-device support
- [CONFIRMED] Configuration: Named Profiles + Auto-save last settings
- [CONFIRMED] ADB: Use system PATH first, fallback to bundled, user can configure manually
- [CONFIRMED] scrcpy binary: PATH + Fallback to bundled
- [CONFIRMED] Testing: TDD (Vitest + Tauri E2E)
- [CONFIRMED] Device controls: Essential (rotate, fullscreen, screenshot, screen off/on, volume, back/home/recents)
- [CONFIRMED] Languages: TypeScript (Vue) + Rust (Tauri)
- [CONFIRMED] i18n: English only with i18n structure ready

## Research Findings
- **scrcpy location**: D:\projects\scrcpy (forked repo)
- **scrcpy-x location**: D:\projects\scrcpy-x (empty - fresh start)
- **scrcpy tech stack**: C application with SDL UI, Meson build, Gradle for Android server
- **CLI options**: ~100+ options covering:
  - Connection: serial, tcpip, port, tunnel
  - Video: codec (h264/h265/av1), bitrate, max-size, max-fps, crop, orientation
  - Audio: codec (opus/aac/flac/raw), bitrate, source, buffer
  - Recording: format (mp4/mkv/etc), filename
  - Camera: id, size, facing, fps
  - Input: keyboard/mouse/gamepad modes (sdk/uhid/aoa)
  - Window: title, position, size, fullscreen, borderless
  - Device: display-id, turn-screen-off, stay-awake
  - Virtual display: new-display, start-app
- **Environment variables**: ADB path, ANDROID_SERIAL, SCRCPY_SERVER_PATH
- **Exit codes**: 0 (normal), 1 (start failure), 2 (device disconnected)

## Open Questions
1. ~~Which scrcpy features to expose in GUI?~~ → All 7 categories curated
2. ~~How to integrate with scrcpy?~~ → Subprocess + Window Management
3. ~~Single device or multi-device?~~ → Multi-device
4. ~~Should the "device operation UI" embed the scrcpy window?~~ → Separate windows
5. ~~Configuration persistence?~~ → Named Profiles + Auto-save
6. ~~Test strategy?~~ → TDD
7. ~~ADB handling?~~ → PATH + bundled fallback + user config
8. ~~scrcpy binary location?~~ → PATH + bundled fallback
9. ~~Device control UI features?~~ → Essential controls
10. ~~Programming language?~~ → TypeScript + Rust
11. ~~i18n?~~ → English only, i18n-ready
12. ~~App name?~~ → ScrcpyX

## Scope Boundaries
- INCLUDE: 
  - Tauri project setup
  - Vue3 + Naive UI frontend
  - Settings/launch UI
  - Device operation UI
  - Cross-platform support (Win/Mac/Linux)
- EXCLUDE: 
  - Modifying scrcpy source code
  - Building scrcpy from source (assume pre-built binary available)
