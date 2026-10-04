#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicBool, Ordering};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// true = clicks pass through to the game
static CLICK_THROUGH: AtomicBool = AtomicBool::new(true);

fn toggle_mouse(app: &AppHandle) {
    let next = !CLICK_THROUGH.load(Ordering::SeqCst);
    CLICK_THROUGH.store(next, Ordering::SeqCst);
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.set_ignore_cursor_events(next);
        let _ = w.set_always_on_top(true);
        if !next {
            let _ = w.set_focus();
        }
    }
    if let Some(tray) = app.tray_by_id("ep-tray") {
        let _ = tray.set_tooltip(Some(if next {
            "Extinction Point HUD — PLAYING (click-through). Click to use mouse."
        } else {
            "Extinction Point HUD — MOUSE ON. Click to return to game."
        }));
    }
}

fn main() {
    // Mouse toggle: ` (tilde / Backquote) key, or Ctrl+Alt+M as a backup. Tray icon click also toggles.
    let toggle = Shortcut::new(None, Code::Backquote);
    let toggle_backup = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyM);
    let quit = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyQ);

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if event.state() != ShortcutState::Pressed {
                        return;
                    }
                    if shortcut == &quit {
                        app.exit(0);
                        return;
                    }
                    if shortcut == &toggle || shortcut == &toggle_backup {
                        toggle_mouse(app);
                    }
                })
                .build(),
        )
        .setup(move |app| {
            // Never let one failed hotkey stop the overlay from starting.
            for sc in [toggle, toggle_backup, quit] {
                let _ = app.global_shortcut().register(sc);
            }

            // System tray icon: always works, even if the game swallows hotkeys.
            let toggle_item = MenuItem::with_id(app, "toggle", "Toggle mouse control (`)", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit overlay", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&toggle_item, &quit_item])?;
            let mut tray = TrayIconBuilder::with_id("ep-tray")
                .tooltip("Extinction Point HUD — PLAYING (click-through). Click to use mouse.")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "toggle" => toggle_mouse(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_mouse(tray.app_handle());
                    }
                });
            if let Some(icon) = app.default_window_icon() {
                tray = tray.icon(icon.clone());
            }
            let _ = tray.build(app);

            if let Some(w) = app.get_webview_window("main") {
                // Cover the whole monitor explicitly (Windows ignores fullscreen on transparent borderless windows).
                if let Ok(Some(m)) = w.current_monitor().or_else(|_| w.primary_monitor()) {
                    let pos = *m.position();
                    let size = *m.size();
                    let _ = w.set_position(tauri::PhysicalPosition::new(pos.x, pos.y));
                    let _ = w.set_size(tauri::PhysicalSize::new(size.width, size.height));
                }
                w.set_ignore_cursor_events(true)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to start Extinction Point Overlay");
}
