use std::{thread, time::Duration};
use tauri::{
    Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};

const CHAT_W: u32 = 310;
const CHAT_H: u32 = 360;
const GAP: i32 = 2;

fn ensure_chat(app: &tauri::AppHandle) -> Result<WebviewWindow, String> {
    if let Some(window) = app.get_webview_window("chat") {
        return Ok(window);
    }

    WebviewWindowBuilder::new(
        app,
        "chat",
        WebviewUrl::App("index.html?window=chat".into()),
    )
    .title("Kelly")
    .inner_size(CHAT_W as f64, CHAT_H as f64)
    .resizable(false)
    .maximizable(false)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .shadow(false)
    .visible(false)
    .build()
    .map_err(|error| error.to_string())
}

fn place_chat(app: &tauri::AppHandle, focus: bool) -> Result<(), String> {
    let pet = app
        .get_webview_window("main")
        .ok_or("main window not found")?;
    let pet_position = pet.outer_position().map_err(|error| error.to_string())?;
    let pet_size = pet.outer_size().map_err(|error| error.to_string())?;
    let monitor = pet
        .current_monitor()
        .map_err(|error| error.to_string())?
        .ok_or("monitor not found")?;

    let monitor_position = *monitor.position();
    let monitor_size = *monitor.size();
    let min_x = monitor_position.x;
    let max_x = monitor_position.x + monitor_size.width as i32 - CHAT_W as i32;
    let min_y = monitor_position.y;
    let max_y = monitor_position.y + monitor_size.height as i32 - CHAT_H as i32;

    let left = pet_position.x - CHAT_W as i32 - GAP;
    let right = pet_position.x + pet_size.width as i32 + GAP;
    let above = pet_position.y - CHAT_H as i32 - GAP;
    let below = pet_position.y + pet_size.height as i32 + GAP;

    let (x, y, side) = if right <= max_x {
        (right, pet_position.y.clamp(min_y, max_y), "right")
    } else if left >= min_x {
        (left, pet_position.y.clamp(min_y, max_y), "left")
    } else if below <= max_y {
        (pet_position.x.clamp(min_x, max_x), below, "bottom")
    } else {
        (
            pet_position.x.clamp(min_x, max_x),
            above.max(min_y),
            "top",
        )
    };

    let chat = ensure_chat(app)?;
    chat.set_position(PhysicalPosition::new(x, y))
        .map_err(|error| error.to_string())?;
    let _ = chat.emit("bubble-side", side);
    chat.show().map_err(|error| error.to_string())?;
    if focus {
        chat.set_focus().map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn start_chat_follower(app: tauri::AppHandle) {
    thread::spawn(move || {
        let mut last_position: Option<(i32, i32)> = None;

        loop {
            thread::sleep(Duration::from_millis(45));

            let Some(pet) = app.get_webview_window("main") else {
                break;
            };
            if !pet.is_visible().unwrap_or(false) {
                continue;
            }

            let Ok(position) = pet.outer_position() else {
                continue;
            };
            let current = (position.x, position.y);

            if last_position != Some(current) {
                last_position = Some(current);
                if let Some(chat) = app.get_webview_window("chat") {
                    if chat.is_visible().unwrap_or(false) {
                        let _ = place_chat(&app, false);
                    }
                }
            }
        }
    });
}

fn toggle_chat_sync(app: &tauri::AppHandle) -> Result<(), String> {
    let chat = ensure_chat(app)?;
    if chat.is_visible().unwrap_or(false) {
        chat.hide().map_err(|error| error.to_string())
    } else {
        place_chat(app, true)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            use tauri::menu::{MenuBuilder, MenuItemBuilder};
            use tauri::tray::TrayIconBuilder;

            if let Some(main) = app.get_webview_window("main") {
                let _ = main.set_size(PhysicalSize::new(150, 190));
                let _ = main.set_maximizable(false);
            }

            if let Err(error) = ensure_chat(&app.handle()) {
                eprintln!("failed to pre-create chat window: {error}");
            }
            start_chat_follower(app.handle().clone());

            let companion = MenuItemBuilder::with_id("companion", "Kelly")
                .enabled(false)
                .build(app)?;
            let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
            let talk = MenuItemBuilder::with_id("talk", "Talk").build(app)?;
            let settings = MenuItemBuilder::with_id("settings", "Settings").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "Exit KENRI").build(app)?;
            let menu = MenuBuilder::new(app)
                .items(&[&companion, &show, &talk, &settings, &quit])
                .build()?;

            let mut tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Kelly — KENRI Desktop Companion");
            if let Some(icon) = app.default_window_icon() {
                tray = tray.icon(icon.clone());
            }

            tray.on_menu_event(|app, event| match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "talk" => {
                    let _ = toggle_chat_sync(app);
                }
                "settings" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = app.emit("open-settings", ());
                    }
                }
                "quit" => app.exit(0),
                _ => {}
            })
            .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![show_chat, hide_chat, toggle_chat])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running KENRI Desktop Companion");
}

#[tauri::command]
async fn show_chat(app: tauri::AppHandle) -> Result<(), String> {
    place_chat(&app, true)
}

#[tauri::command]
async fn hide_chat(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(chat) = app.get_webview_window("chat") {
        chat.hide().map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[tauri::command]
async fn toggle_chat(app: tauri::AppHandle) -> Result<(), String> {
    toggle_chat_sync(&app)
}
