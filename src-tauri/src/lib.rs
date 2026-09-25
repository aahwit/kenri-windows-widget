use tauri::{Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const CHAT_W:u32=310; const CHAT_H:u32=360; const SETTINGS_W:u32=430; const SETTINGS_H:u32=700; const GAP:i32=2;

fn ensure_panel(app:&tauri::AppHandle)->tauri::Result<WebviewWindow>{
 if let Some(w)=app.get_webview_window("panel"){return Ok(w)}
 WebviewWindowBuilder::new(app,"panel",WebviewUrl::App("index.html?window=panel".into()))
  .title("Kelly").inner_size(CHAT_W as f64,CHAT_H as f64).resizable(false).maximizable(false)
  .decorations(false).transparent(true).always_on_top(true).shadow(false).visible(false).build()
}

fn place_panel(app:&tauri::AppHandle,width:u32,height:u32,focus:bool)->tauri::Result<()> {
 let Some(pet)=app.get_webview_window("main") else{return Ok(())};
 let panel=ensure_panel(app)?; panel.set_size(PhysicalSize::new(width,height))?;
 let p=pet.outer_position()?; let s=pet.outer_size()?; let Some(mon)=pet.current_monitor()? else{return Ok(())};
 let mp=*mon.position(); let ms=*mon.size(); let min_x=mp.x; let max_x=mp.x+ms.width as i32-width as i32;
 let min_y=mp.y; let max_y=mp.y+ms.height as i32-height as i32; let left=p.x-width as i32-GAP;
 let right=p.x+s.width as i32+GAP; let above=p.y-height as i32-GAP; let below=p.y+s.height as i32+GAP;
 let(x,y,side)=if right<=max_x{(right,p.y.clamp(min_y,max_y),"right")}else if left>=min_x{(left,p.y.clamp(min_y,max_y),"left")}else if below<=max_y{(p.x.clamp(min_x,max_x),below,"bottom")}else{(p.x.clamp(min_x,max_x),above.max(min_y),"top")};
 panel.set_position(PhysicalPosition::new(x,y))?; let _=panel.emit("bubble-side",side); panel.show()?; if focus{panel.set_focus()?} Ok(())
}

fn open_mode(app:&tauri::AppHandle,mode:&str,focus:bool)->tauri::Result<()> {
 let panel=ensure_panel(app)?; let _=panel.emit("panel-mode",mode);
 match mode{"settings"=>place_panel(app,SETTINGS_W,SETTINGS_H,focus),_=>place_panel(app,CHAT_W,CHAT_H,focus)}
}

fn toggle_mode(app:&tauri::AppHandle,mode:&str)->tauri::Result<()> {
 let panel=ensure_panel(app)?;
 if panel.is_visible().unwrap_or(false){let _=panel.emit("panel-mode",mode);panel.hide()}else{open_mode(app,mode,true)}
}

#[cfg_attr(mobile,tauri::mobile_entry_point)]
pub fn run(){tauri::Builder::default().setup(|app|{
 use tauri::menu::{MenuBuilder,MenuItemBuilder};use tauri::tray::TrayIconBuilder;
 if let Some(main)=app.get_webview_window("main"){let _=main.set_size(PhysicalSize::new(150,190));let _=main.set_resizable(false);let _=main.set_maximizable(false);}
 if let Err(e)=ensure_panel(&app.handle()){eprintln!("panel init failed: {e}");}
 let companion=MenuItemBuilder::with_id("companion","Kelly").enabled(false).build(app)?;let show=MenuItemBuilder::with_id("show","Show").build(app)?;let talk=MenuItemBuilder::with_id("talk","Talk").build(app)?;let settings=MenuItemBuilder::with_id("settings","Settings").build(app)?;let quit=MenuItemBuilder::with_id("quit","Exit KENRI").build(app)?;
 let menu=MenuBuilder::new(app).items(&[&companion,&show,&talk,&settings,&quit]).build()?;let mut tray=TrayIconBuilder::new().menu(&menu).tooltip("Kelly — KENRI Desktop Companion");if let Some(icon)=app.default_window_icon(){tray=tray.icon(icon.clone());}
 tray.on_menu_event(|app,event|match event.id.as_ref(){"show"=>if let Some(w)=app.get_webview_window("main"){let _=w.show();let _=w.set_focus();},"talk"=>{if let Some(w)=app.get_webview_window("main"){let _=w.show();}let _=toggle_mode(app,"chat");},"settings"=>{if let Some(w)=app.get_webview_window("main"){let _=w.show();}let _=toggle_mode(app,"settings");},"quit"=>app.exit(0),_=>{}}).build(app)?;Ok(())
 }).invoke_handler(tauri::generate_handler![show_chat,hide_panel,toggle_chat,show_settings]).on_window_event(|window,event|match event{
  tauri::WindowEvent::Moved(_)=>{if window.label()=="main"{if let Some(panel)=window.app_handle().get_webview_window("panel"){if panel.is_visible().unwrap_or(false){let _=panel.emit("reposition-request",());}}}},
  tauri::WindowEvent::CloseRequested{api,..}=>{api.prevent_close();let _=window.hide();},_=>{}
 }).run(tauri::generate_context!()).expect("error while running KENRI Desktop Companion");}

#[tauri::command]fn show_chat(app:tauri::AppHandle)->Result<(),String>{open_mode(&app,"chat",true).map_err(|e|e.to_string())}
#[tauri::command]fn hide_panel(app:tauri::AppHandle)->Result<(),String>{if let Some(panel)=app.get_webview_window("panel"){panel.hide().map_err(|e|e.to_string())?}Ok(())}
#[tauri::command]fn toggle_chat(app:tauri::AppHandle)->Result<(),String>{toggle_mode(&app,"chat").map_err(|e|e.to_string())}
#[tauri::command]fn show_settings(app:tauri::AppHandle)->Result<(),String>{open_mode(&app,"settings",true).map_err(|e|e.to_string())}
