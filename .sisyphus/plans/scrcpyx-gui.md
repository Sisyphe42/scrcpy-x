# ScrcpyX - GUI Application for scrcpy

## Working Rules

- **Commit requires user review**: Never commit without explicit user approval. Present changes for review first.
- **Commit message with review request**: When requesting review for commit, always include the commit message in the request. Format: "Review needed for commit: [commit message]"

## Current Status

All initial implementation tasks (Wave 1-7) are complete. The app is functional with:
- Device discovery, session management, profile CRUD
- Settings, i18n (EN + ZH), dark mode
- Launch / Sessions / Settings pages with sidebar navigation

## Architecture

- **Frontend**: Vue 3 + TypeScript + Naive UI + Vue Router + Pinia
- **Backend**: Rust (Tauri)
- **Build**: Vite + Tauri CLI
- **Testing**: Vitest (unit) + WebdriverIO (E2E)

## Key Files

| Path | Purpose |
|------|---------|
| `src/views/LaunchView.vue` | Main launch page (devices, profiles, options) |
| `src/views/SessionView.vue` | Active sessions + device controls |
| `src/views/SettingsView.vue` | App settings |
| `src/layouts/MainLayout.vue` | Sidebar navigation layout |
| `src/components/SessionOptionsForm.vue` | Scrcpy options form (7 categories) |
| `src/stores/sessionStore.ts` | Session state management |
| `src-tauri/src/session/manager.rs` | Rust session/subprocess management |
| `src-tauri/src/commands/session.rs` | Tauri command wrappers |

## TODOs - Current Focus

### 高优先级
- [x] 实现浮动设备控制面板：独立 Tauri 窗口，绑定 scrcpy 窗口，支持旋转、截图、音量等快捷操作
- [x] 应用内投屏视图：内嵌 scrcpy 画面，支持实时显示手机屏幕
- [x] 监测 scrcpy 进程异常退出，自动更新会话状态
- [x] 修复设备控制无效，排查 deviceId 传递及 ADB 权限问题
- [x] 修复会话时长不正确显示，前后端同步启动时间

### 中优先级
- [x] 设置页 UI 更新，统一 top title + icon 卡片布局
- [x] 截图设置：支持自定义文件名、保存路径及保存到剪贴板
- [x] Ctrl+Tab 快捷键切换页面

### 低优先级
- [x] 关于页卡片：展示 GitHub 链接，感谢 scrcpy 等

---

## 待完善
- 截图设置：前端已存储但尚未接线到后端 take_screenshot（需重新实现 filename/path 传递）
- 设置页：UI 已有基础卡片布局，需进一步完善细节和交互
- turn_screen_on/off：需修正 dumpsys 解析逻辑（避免 shell 管道传递问题）
- 浮动面板：需验证 Tauri v2 多窗口权限配置

## 已完成（需回归验证）
- Pinia 状态管理完善
- 会话管理页面分离，Sidebar 新增会话菜单项
- 基础设备、会话、配置文件管理界面实现
- 进程退出监测：后台线程监控 scrcpy 进程，自动更新会话状态并 emit 事件到前端
- 会话时长显示：Rust Session 新增 startedAt 字段，前端实时计时器每秒刷新
- 设备控制修复：turn_screen_on/off 改为检测屏幕状态后再发送 power key，避免误触
- 事件系统：session-started/session-ended/session-error 事件从 Rust → 前端完整打通
