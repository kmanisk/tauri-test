mod short;

use crate::short::register_shortcuts;
use tauri::{
    menu::{Menu, MenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, // Needed for `get_webview_window`
};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// ---------------- Commands ----------------
#[tauri::command]
fn get_clipboard(app: tauri::AppHandle) -> String {
    app.clipboard().read_text().unwrap_or_else(|_| "".into())
}

#[tauri::command]
fn write_clipboard(app: tauri::AppHandle, text: String) {
    let _ = app.clipboard().write_text(text);
}

// ---------------- Run ----------------
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Initialize plugins
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        // Setup
        .setup(|app| {
            // Register global shortcuts
            register_shortcuts(&app.handle())?;
            Ok(())
        })
        // ---------------- Menu Bar (Commented Out) ----------------
        /*
        .setup(|app| {
            // File menu
            let file_open = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
            let file_exit = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;
            let file_menu = Submenu::with_items(app, "File", true, &[&file_open, &file_exit])?;
            // Edit menu
            let edit_cut = MenuItem::with_id(app, "cut", "Cut", true, None::<&str>)?;
            let edit_copy = MenuItem::with_id(app, "copy", "Copy", true, None::<&str>)?;
            let edit_paste = MenuItem::with_id(app, "paste", "Paste", true, None::<&str>)?;
            let edit_menu = Submenu::with_items(app, "Edit", true, &[&edit_cut, &edit_copy, &edit_paste])?;
            // Help menu
            let help_docs = MenuItem::with_id(app, "docs", "Documentation", true, None::<&str>)?;
            let help_issue = MenuItem::with_id(app, "issue", "Report Issue", true, None::<&str>)?;
            let help_menu = Submenu::with_items(app, "Help", true, &[&help_docs, &help_issue])?;
            // About menu
            let about_item = MenuItem::with_id(app, "about", "About This App", true, None::<&str>)?;
            let about_menu = Submenu::with_items(app, "About", true, &[&about_item])?;
            // Clipboard menu
            let clip_read = MenuItem::with_id(app, "clip_read", "Show Clipboard", true, None::<&str>)?;
            let clip_menu = Submenu::with_items(app, "Clipboard", true, &[&clip_read])?;
            // Root menu
            let window_menu = Menu::with_items(app, &[&file_menu, &edit_menu, &help_menu, &about_menu, &clip_menu])?;
            app.set_menu(window_menu)?;
            Ok(())
        })
        */
        // ---------------- System Tray ----------------
        .setup(|app| {
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "Hide", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;

            TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .on_menu_event(|app, event| match event.id.0.as_str() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                        }
                    }
                    "hide" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.hide();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.unminimize();
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_clipboard, write_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
