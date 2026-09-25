use tauri::{Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const CHAT_W:u32=310; const CHAT_H:u32=360; const SETTINGS_W:u32=430; const SETTINGS_H:u32=700; const GAP:i32=2;

fn ensure_window(app:&tauri::AppHandle,label:&str,url:&str,w:u32,h:u32)->tauri::Result<WebviewWindow>{
 if let Some(win)=app.get_webview_window(label){return Ok(win)}
 WebviewWindowBuilder::new(app,label,WebviewUrl::App(url.into())).title("Kelly")
  .inner_size(w as f64,h as f64).resizable(false).maximizable(false).decorations(false)
  .transparent(true).always_on_top(true).shadow(false).visible(false).build()
}
fn ensure_chat(app:&tauri::AppHandle)->tauri::Result<WebviewWindow>{ensure_window(app,"chat","index.html?window=chat",CHAT_W,CHAT_H)}
fn ensure_settings(app:&tauri::AppHandle)->tauri::Result<WebviewWindow>{ensure_window(app,"settings","index.html?window=settings",SETTINGS_W,SETTINGS_H)}

fn calc_position(p:PhysicalPosition<i32>,pet:PhysicalSize<u32>,mp:PhysicalPosition<i32>,ms:PhysicalSize<u32>,w:u32,h:u32,slot:i32)->(i32,i32,&'static str){
 let min_x=mp.x;let max_x=mp.x+ms.width as i32-w as i32;let min_y=mp.y;let max_y=mp.y+ms.height as i32-h as i32;
 let right=p.x+pet.width as i32+GAP;let left=p.x-w as i32-GAP;let below=p.y+pet.height as i32+GAP;let above=p.y-h as i32-GAP;
 let offset=slot*26;
 if right<=max_x{(right,(p.y+offset).clamp(min_y,max_y),"right")}
 else if left>=min_x{(left,(p.y+offset).clamp(min_y,max_y),"left")}
 else if below<=max_y{((p.x+offset).clamp(min_x,max_x),below,"bottom")}
 else{((p.x+offset).clamp(min_x,max_x),above.max(min_y),"top")}
}

fn place_one(app:&tauri::AppHandle,label:&str,w:u32,h:u32,slot:i32,focus:bool)->tauri::Result<()> {
 let Some(pet)=app.get_webview_window("main") else{return Ok(())};let p=pet.outer_position()?;let ps=pet.outer_size()?;let Some(mon)=pet.current_monitor()? else{return Ok(())};
 let(x,y,side)=calc_position(p,ps,*mon.position(),*mon.size(),w,h,slot);let win=if label=="chat"{ensure_chat(app)?}else{ensure_settings(app)?};
 win.set_position(PhysicalPosition::new(x,y))?;let _=win.emit("bubble-side",side);win.show()?;if focus{win.set_focus()?}Ok(())
}
fn reposition_open(app:&tauri::AppHandle){let chat_open=app.get_webview_window("chat").map(|w|w.is_visible().unwrap_or(false)).unwrap_or(false);let settings_open=app.get_webview_window("settings").map(|w|w.is_visible().unwrap_or(false)).unwrap_or(false);if chat_open{let _=place_one(app,"chat",CHAT_W,CHAT_H,0,false);}if settings_open{let _=place_one(app,"settings",SETTINGS_W,SETTINGS_H,if chat_open{1}else{0},false);}}
fn toggle(app:&tauri::AppHandle,label:&str)->tauri::Result<()> {let win=if label=="chat"{ensure_chat(app)?}else{ensure_settings(app)?};if win.is_visible().unwrap_or(false){win.hide()}else if label=="chat"{place_one(app,"chat",CHAT_W,CHAT_H,0,true)}else{let chat_open=app.get_webview_window("chat").map(|w|w.is_visible().unwrap_or(false)).unwrap_or(false);place_one(app,"settings",SETTINGS_W,SETTINGS_H,if chat_open{1}else{0},true)}}

#[cfg_attr(mobile,tauri::mobile_entry_point)]pub fn run(){tauri::Builder::default().setup(|app|{
 use tauri::menu::{MenuBuilder,MenuItemBuilder};use tauri::tray::TrayIconBuilder;if let Some(main)=app.get_webview_window("main"){let _=main.set_size(PhysicalSize::new(150,190));let _=main.set_resizable(false);let _=main.set_maximizable(false);}let _=ensure_chat(&app.handle());let _=ensure_settings(&app.handle());
 let companion=MenuItemBuilder::with_id("companion","Kelly").enabled(false).build(app)?;let show=MenuItemBuilder::with_id("show","Show").build(app)?;let talk=MenuItemBuilder::with_id("talk","Talk").build(app)?;let settings=MenuItemBuilder::with_id("settings","Settings").build(app)?;let quit=MenuItemBuilder::with_id("quit","Exit KENRI").build(app)?;let menu=MenuBuilder::new(app).items(&[&companion,&show,&talk,&settings,&quit]).build()?;let mut tray=TrayIconBuilder::new().menu(&menu).tooltip("Kelly — KENRI Desktop Companion");if let Some(icon)=app.default_window_icon(){tray=tray.icon(icon.clone());}tray.on_menu_event(|app,event|match event.id.as_ref(){"show"=>if let Some(w)=app.get_webview_window("main"){let _=w.show();let _=w.set_focus();},"talk"=>{let _=toggle(app,"chat");},"settings"=>{let _=toggle(app,"settings");},"quit"=>app.exit(0),_=>{}}).build(app)?;Ok(())
 }).invoke_handler(tauri::generate_handler![toggle_chat,toggle_settings,hide_chat,hide_settings,restart_pet]).on_window_event(|window,event|match event{tauri::WindowEvent::Moved(_)=>{if window.label()=="main"{reposition_open(window.app_handle());}},tauri::WindowEvent::CloseRequested{api,..}=>{api.prevent_close();let _=window.hide();},_=>{}}).run(tauri::generate_context!()).expect("error while running KENRI Desktop Companion");}

#[tauri::command]fn toggle_chat(app:tauri::AppHandle)->Result<(),String>{toggle(&app,"chat").map_err(|e|e.to_string())}
#[tauri::command]fn toggle_settings(app:tauri::AppHandle)->Result<(),String>{toggle(&app,"settings").map_err(|e|e.to_string())}
#[tauri::command]fn hide_chat(app:tauri::AppHandle)->Result<(),String>{if let Some(w)=app.get_webview_window("chat"){w.hide().map_err(|e|e.to_string())?}Ok(())}
#[tauri::command]fn hide_settings(app:tauri::AppHandle)->Result<(),String>{if let Some(w)=app.get_webview_window("settings"){w.hide().map_err(|e|e.to_string())?}Ok(())}
#[tauri::command]fn restart_pet(app:tauri::AppHandle){app.restart();}
