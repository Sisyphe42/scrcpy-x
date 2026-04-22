# ScrcpyX

A cross-platform GUI application for [scrcpy](https://github.com/Genymobile/scrcpy) - Android screen mirroring tool.

## Features

- **Cross-platform**: Windows, macOS, Linux
- **Multi-device support**: Manage multiple Android devices simultaneously
- **Profile management**: Save and load configuration profiles
- **User-friendly UI**: Built with Tauri + Vue3 + Naive UI
- **Essential controls**: Rotate, fullscreen, screenshot, screen on/off, volume, navigation

## Tech Stack

- **Frontend**: Vue 3 + TypeScript + Naive UI + Vue Router + Pinia
- **Backend**: Rust (Tauri)
- **Build**: Vite + Tauri CLI
- **Testing**: Vitest (unit) + WebdriverIO (E2E)

## Prerequisites

- Node.js >= 18
- Rust >= 1.70
- Android SDK Platform Tools (ADB)
- scrcpy binary

## Installation

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Development

```bash
# Start development server
npm run tauri dev

# Run unit tests
npm run test

# Run unit tests (single run)
npm run test:run

# Run tests with coverage
npm run test:coverage

# Build frontend only
npm run build
```

## Project Structure

```
scrcpy-x/
├── src/                    # Vue frontend
│   ├── components/         # Vue components
│   ├── views/              # Page views
│   ├── stores/             # Pinia stores
│   ├── types/              # TypeScript types
│   ├── locales/            # i18n translations
│   ├── api/                # Tauri API wrappers
│   └── utils/              # Utility functions
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── commands/       # Tauri commands
│   │   ├── device/         # Device discovery
│   │   ├── session/        # Session management
│   │   └── storage/        # Profile storage
│   └── tauri.conf.json     # Tauri configuration
├── e2e/                    # E2E tests
└── .sisyphus/              # Project planning
```

## Configuration

ScrcpyX uses profiles to store scrcpy configuration options. Each profile can configure:

- **Connection**: Device selection, TCP/IP, port range
- **Video**: Codec, bitrate, max-size, max-fps, crop
- **Audio**: Enable/disable, codec, bitrate, source
- **Recording**: Record to file, format
- **Window**: Fullscreen, always-on-top, borderless, position/size
- **Device**: Turn screen off, stay awake, show touches
- **Input**: Keyboard/mouse mode, shortcut modifier

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [scrcpy](https://github.com/Genymobile/scrcpy) - The underlying screen mirroring tool
- [Tauri](https://tauri.app/) - Cross-platform desktop app framework
- [Naive UI](https://www.naiveui.com/) - Vue 3 UI component library
// This patch adds Profile Management UI components and supporting types.
