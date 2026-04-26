# ScrcpyX - GUI Application for scrcpy

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
- [ ] 实现浮动设备控制面板：独立 Tauri 窗口，绑定 scrcpy 窗口，支持旋转、截图、音量等快捷操作
- [ ] 应用内投屏视图：内嵌 scrcpy 画面，支持实时显示手机屏幕
- [ ] 监测 scrcpy 进程异常退出，自动更新会话状态
- [ ] 修复设备控制无效，排查 deviceId 传递及 ADB 权限问题
- [ ] 修复会话时长不正确显示，前后端同步启动时间

### 中优先级
- [ ] 设置页 UI 更新，统一 top title + icon 卡片布局
- [ ] 截图设置：支持自定义文件名、保存路径及保存到剪贴板
- [ ] Ctrl+Tab 快捷键切换页面

### 低优先级
- [ ] 关于页卡片：展示 GitHub 链接，感谢 scrcpy 等

---

## 已完成（需回归验证）
- Pinia 状态管理完善
- 会话管理页面分离，Sidebar 新增会话菜单项
- 基础设备、会话、配置文件管理界面实现
