use crate::logs;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let toggle_item = MenuItem::with_id(app, "toggle", "Toggle Window", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit PassClip", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&toggle_item, &quit_item])?;

    let icon = app
        .default_window_icon()
        .cloned()
        .ok_or("default window icon not configured")?;

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "toggle" => {
                logs::log(app, "DEBUG", "Tray menu item clicked: Toggle Window");
                toggle_window(app);
            }
            "quit" => {
                logs::log(app, "INFO", "Tray menu item clicked: Quit PassClip");
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                logs::log(tray.app_handle(), "DEBUG", "Tray icon left clicked");
                toggle_window(tray.app_handle());
            }
        })
        .build(app)?;

    logs::log(app, "INFO", "System tray icon initialized");
    Ok(())
}

fn toggle_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            logs::log(app, "DEBUG", "Hiding main window to tray");
            let _ = window.hide();
        } else {
            logs::log(app, "DEBUG", "Showing and focusing main window");
            let _ = window.show();
            let _ = window.set_focus();
        }
    } else {
        logs::log(app, "ERROR", "Main window instance not found when toggling");
    }
}