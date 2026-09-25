use tauri::{Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const CHAT_W: u32 = 310;
const CHAT_H: u32 = 360;
const GAP: i32 = 6;

fn ensure_chat(app: &tauri::AppHandle) -> Result<WebviewWindow, String> {
  if let Some(window) = app.get_webview_window("chat") { return Ok(window); }
  WebviewWindowBuilder::new(app, "chat", WebviewUrl::App("index.html?window=chat".into()))
    .title("Kelly").inner_size(CHAT_W as f64, CHAT_H as f64)
    .resizable(false).decorations(false).transparent(true).always_on_top(true)
    .shadow(false).visible(false).build().map_err(|e| e.to_string())
}

fn place_chat(app: &tauri::AppHandle) -> Result<(), String> {
  let pet = app.get_webview_window("main").ok_or("main window not found")?;
  let p = pet.outer_position().map_err(|e| e.to_string())?;
  let s = pet.outer_size().map_err(|e| e.to_string())?;
  let mon = pet.current_monitor().map_err(|e| e.to_string())?.ok_or("monitor not found")?;
  let mp=*mon.position(); let ms=*mon.size();
  let min_x=mp.x; let max_x=mp.x+ms.width as i32-CHAT_W as i32;
  let min_y=mp.y; let max_y=mp.y+ms.height as i32-CHAT_H as i32;
  let left=p.x-CHAT_W as i32-GAP; let right=p.x+s.width as i32+GAP;
  let above=p.y-CHAT_H as i32-GAP; let below=p.y+s.height as i32+GAP;
  let (x,y,side)=if right<=max_x{(right,p.y.clamp(min_y,max_y),"right")}else if left>=min_x{(left,p.y.clamp(min_y,max_y),"left")}else if below<=max_y{(p.x.clamp(min_x,max_x),below,"bottom")}else{(p.x.clamp(min_x,max_x),above.max(min_y),"top")};
  let chat=ensure_chat(app)?;
  chat.set_position(PhysicalPosition::new(x,y)).map_err(|e| e.to_string())?;
  let _=chat.emit("bubble-side",side);
  chat.show().map_err(|e| e.to_string())?;
  chat.set_focus().map_err(|e| e.to_string())?;
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(){
 tauri::Builder::default().setup(|app|{
  use tauri::menu::{MenuBuilder,MenuItemBuilder}; use tauri::tray::TrayIconBuilder;
  if let Some(main)=app.get_webview_window("main"){let _=main.set_size(PhysicalSize::new(150,190));}
  // Pre-create the hidden chat webview during setup. Keep setup alive even if this optional
  // window cannot be created; show_chat will return the actual error instead of killing startup.
  if let Err(error)=ensure_chat(&app.handle()) { eprintln!("failed to pre-create chat window: {error}"); }
  let companion=MenuItemBuilder::with_id("companion","Kelly").enabled(false).build(app)?;
  let show=MenuItemBuilder::with_id("show","Show").build(app)?;let talk=MenuItemBuilder::with_id("talk","Talk").build(app)?;let settings=MenuItemBuilder::with_id("settings","Settings").build(app)?;let quit=MenuItemBuilder::with_id("quit","Exit KENRI").build(app)?;
  let menu=MenuBuilder::new(app).items(&[&companion,&show,&talk,&settings,&quit]).build()?;let mut tray=TrayIconBuilder::new().menu(&menu).tooltip("Kelly — KENRI Desktop Companion");if let Some(icon)=app.default_window_icon(){tray=tray.icon(icon.clone());}
  tray.on_menu_event(|app,event|match event.id.as_ref(){"show"=>if let Some(w)=app.get_webview_window("main"){let _=w.show();let _=w.set_focus();},"talk"=>{if let Some(w)=app.get_webview_window("main"){let _=w.show();}if let Err(error)=place_chat(app){eprintln!("failed to show chat window: {error}");}},"settings"=>if let Some(w)=app.get_webview_window("main"){let _=w.show();let _=w.set_focus();let _=app.emit("open-settings",());},"quit"=>app.exit(0),_=>{}}).build(app)?;Ok(())
 }).invoke_handler(tauri::generate_handler![show_chat,hide_chat]).on_window_event(|window,event|{if let tauri::WindowEvent::CloseRequested{api,..}=event{api.prevent_close();let _=window.hide();}}).run(tauri::generate_context!()).expect("error while running KENRI Desktop Companion");
}
#[tauri::command] async fn show_chat(app:tauri::AppHandle)->Result<(),String>{place_chat(&app)}
#[tauri::command] async fn hide_chat(app:tauri::AppHandle)->Result<(),String>{if let Some(chat)=app.get_webview_window("chat"){chat.hide().map_err(|e|e.to_string())?;}Ok(())}
