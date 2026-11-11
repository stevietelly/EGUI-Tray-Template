# EGUI Tray Template

A lightweight Rust template for building **system-tray applications with a native egui window**.

This project is a starting point for applications that live in the system tray by default and show a full UI on demand — think gameplay capture tools, background recorders, or any "always available, rarely visible" utility.

## Features

- **Tray-first lifecycle** — launches straight into the tray. The main window starts hidden and appears only when the user opens it.
- **Tray menu** — Open / Hide / Quit actions that communicate directly with egui's `Context`.
- **Close-to-tray** — clicking the window's X hides it instead of quitting. Only the tray's **Quit** actually exits.
- **Renderer fallback** — tries OpenGL first, then falls back to wgpu with the DX12 backend forced. Avoids the Vulkan access-violation crash common on older Intel iGPUs.
- **Configurable via env var** — `FTHWT_RENDERER=wgpu` reverses the renderer order.

## Architecture

```
src/
├── main.rs            # Registers modules, calls app::run()
├── app/
│   ├── mod.rs         # Renderer fallback chain, NativeOptions, app creation
│   └── window.rs      # egui App implementation
├── tray/
│   ├── mod.rs         # Tray struct owning the TrayIcon
│   └── menu.rs        # Menu builder + event handler
└── log.rs             # Timestamped logging
```

The tray callback sends `egui::ViewportCommand`s directly through a shared `egui::Context` — no separate event channel or `EventLoopProxy` needed.

## Quick start

```bash
cargo run
```

The app starts in the tray. Right-click the icon to open, hide, or quit.

## Configuration

| Variable | Values | Purpose |
|---|---|---|
| `FTHWT_RENDERER` | `wgpu` | Reverse the renderer order (wgpu first, glow as fallback) |

## Building your own app

1. **Rename the crate** in `Cargo.toml`.
2. **Swap the icon** — replace `create_placeholder_icon()` in `src/tray/mod.rs` with your asset (or load it from `assets/`).
3. **Add your UI** in `src/app/window.rs`.
4. **Hook gameplay capture** — `windows-capture` is already a dependency.

## License

MIT OR Apache-2.0