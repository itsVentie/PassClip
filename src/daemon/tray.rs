use crate::daemon::monitor::MONITOR_PAUSED;
use log::info;
use std::sync::atomic::Ordering;
use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIconBuilder};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

pub fn spawn_tray() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let event_loop = EventLoopBuilder::new().build()?;

    let toggle_item = CheckMenuItem::new("Pause Monitoring", true, false, None);
    let open_config_item = MenuItem::new("Open Config", true, None);
    let exit_item = MenuItem::new("Exit PassClip", true, None);

    let tray_menu = Menu::new();
    tray_menu.append(&toggle_item)?;
    tray_menu.append(&open_config_item)?;
    tray_menu.append(&PredefinedMenuItem::separator())?;
    tray_menu.append(&exit_item)?;

    let icon = create_default_icon();

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("PassClip Daemon")
        .with_icon(icon)
        .build()?;

    let menu_channel = MenuEvent::receiver();

    event_loop.run(move |_event, elwt| {
        elwt.set_control_flow(ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(100),
        ));

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == toggle_item.id() {
                let is_paused = toggle_item.is_checked();
                MONITOR_PAUSED.store(is_paused, Ordering::SeqCst);
                info!("Clipboard monitoring toggled. Paused: {}", is_paused);
            } else if event.id == open_config_item.id() {
                let _ = open::that("passclip.toml");
            } else if event.id == exit_item.id() {
                info!("Exit requested from system tray. Shutting down PassClip daemon...");
                elwt.exit();
                std::process::exit(0);
            }
        }
    })?;

    Ok(())
}

fn create_default_icon() -> Icon {
    let width = 16;
    let height = 16;
    let mut buffer = Vec::with_capacity((width * height * 4) as usize);

    for _ in 0..(width * height) {
        buffer.extend_from_slice(&[0, 200, 100, 255]);
    }

    Icon::from_rgba(buffer, width, height).expect("Failed to create tray icon")
}
