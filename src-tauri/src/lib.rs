#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      use tauri::menu::{MenuBuilder, MenuItemBuilder};
      use tauri::tray::TrayIconBuilder;
      use tauri::{Emitter, Manager};

      let companion = MenuItemBuilder::with_id("companion", "Kelly").enabled(false).build(app)?;
      let show = MenuItemBuilder::with_id("show", "Show").build(app)?;
      let talk = MenuItemBuilder::with_id("talk", "Talk").build(app)?;
      let settings = MenuItemBuilder::with_id("settings", "Settings").build(app)?;
      let quit = MenuItemBuilder::with_id("quit", "Exit KENRI").build(app)?;
      let menu = MenuBuilder::new(app).items(&[&companion, &show, &talk, &settings, &quit]).build()?;
      let mut tray = TrayIconBuilder::new().menu(&menu).tooltip("Kelly — KENRI Desktop Companion");
      if let Some(icon) = app.default_window_icon() { tray = tray.icon(icon.clone()); }
      tray.on_menu_event(|app, event| match event.id.as_ref() {
        "show" => if let Some(window) = app.get_webview_window("main") { let _ = window.show(); let _ = window.set_focus(); },
        "talk" => if let Some(window) = app.get_webview_window("main") { let _ = window.show(); let _ = window.set_focus(); let _ = app.emit("open-chat", ()); },
        "settings" => if let Some(window) = app.get_webview_window("main") { let _ = window.show(); let _ = window.set_focus(); let _ = app.emit("open-settings", ()); },
        "quit" => app.exit(0),
        _ => {}
      }).build(app)?;
      Ok(())
    })
    .on_window_event(|window, event| {
      if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        let _ = window.hide();
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running KENRI Desktop Companion");
}
