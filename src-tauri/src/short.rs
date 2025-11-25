use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{
    Builder, Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
};

pub fn register_shortcuts(app_handle: &AppHandle) -> tauri::Result<()> {
    println!("Registering global shortcuts...");

    // ---------------- Ctrl+Shift+C ----------------
    let toggle_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyH);

    let handle_clone = app_handle.clone();
    app_handle.plugin(
        Builder::new()
            .with_handler(move |app, shortcut, event| {
                if shortcut == &toggle_shortcut {
                    if event.state() == ShortcutState::Pressed {
                        if let Some(win) = app.get_webview_window("main") {
                            if win.is_visible().unwrap_or(false) {
                                let _ = win.hide();
                            } else {
                                let _ = win.show();
                                let _ = win.set_focus();
                            }
                        }
                        println!("Ctrl+Shift+C pressed!");
                    }
                }
            })
            .build(),
    )?;

    // ---------------- Ctrl+Shift+V ----------------
    let paste_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyV);

    let handle_clone_v = app_handle.clone();
    app_handle.plugin(
        Builder::new()
            .with_handler(move |app, shortcut, event| {
                if shortcut == &paste_shortcut {
                    if event.state() == ShortcutState::Pressed {
                        println!("Ctrl+Shift+V pressed!");
                        // You can add custom logic here
                    }
                }
            })
            .build(),
    )?;

    Ok(())
}
