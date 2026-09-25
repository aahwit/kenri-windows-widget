use tauri::{Emitter, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const CHAT_W:u32=310; const CHAT_H:u32=360; const GAP:i32=2; const EDGE:i32=8;

fn ensure_chat(app:&tauri::AppHandle)->tauri::Result<WebviewWindow>{
 if let Some(w)=app.get_webview_window("chat"){return Ok(w)}
 WebviewWindowBuilder::new(app,"chat",WebviewUrl::App("index.html?window=chat".into()))
  .title("Kelly").inner_size(CHAT_W as f64,CHAT_H as f64).resizable(false).maximizable(false)
  .decorations(false).transparent(true).always_on_top(true).shadow(false).visible(false).build()
}

fn place_chat(app:&tauri::AppHandle,focus:bool)->tauri::Result<()> {
 let Some(pet)=app.get_webview_window("main") else{return Ok(())};
 let p=pet.outer_position()?; let s=pet.outer_size()?; let Some(mon)=pet.current_monitor()? else{return Ok(())};
 let mp=*mon.position(); let ms=*mon.size(); let min_x=mp.x; let max_x=mp.x+ms.width as i32-CHAT_W as i32;
 let min_y=mp.y; let max_y=mp.y+ms.height as i32-CHAT_H as i32; let left=p.x-CHAT_W as i32-GAP;
 let right=p.x+s.width as i32+GAP; let above=p.y-CHAT_H as i32-GAP; let below=p.y+s.height as i32+GAP;
 let(x,y,side)=if right<=max_x{(right,p.y.clamp(min_y,max_y),"right")}else if left>=min_x{(left,p.y.clamp(min_y,max_y),"left")}else if below<=max_y{(p.x.clamp(min_x,max_x),below,"bottom")}else{(p.x.clamp(min_x,max_x),above.max(min_y),"top")};
 let chat=ensure_chat(app)?; chat.set_position(PhysicalPosition::new(x,y))?; let _=chat.emit("bubble-side",side); chat.show()?; if focus{chat.set_focus()?} Ok(())
}

fn toggle_chat_sync(app:&tauri::AppHandle)->tauri::Result<()> {let chat=ensure_chat(app)?;if chat.is_visible().unwrap_or(false){chat.hide()}else{place_chat(app,true)}}

fn keep_pet_on_screen(window:&tauri::Window,pos:PhysicalPosition<i32>){
 if window.label()!="main"{return} let Ok(size)=window.outer_size()else{return}; let Ok(Some(mon))=window.current_monitor()else{return};
 let mp=*mon.position(); let ms=*mon.size(); let min_x=mp.x+EDGE; let min_y=mp.y+EDGE;
 let max_x=mp.x+ms.width as i32-size.width as i32-EDGE; let max_y=mp.y+ms.height as i32-size.height as i32-EDGE;
 let x=pos.x.clamp(min_x,max_x.max(min_x)); let y=pos.y.clamp(min_y,max_y.max(min_y));
 if x!=pos.x||y!=pos.y{let _=window.set_position(PhysicalPosition::new(x,y));}
}

#[cfg_attr(mobile,tauri::mobile_entry_point)]
pub fn run(){tauri::Builder::default().setup(|app|{
 use tauri::menu::{MenuBuilder,MenuItemBuilder};use tauri::tray::TrayIconBuilder;
 if let Some(main)=app.get_webview_window("main"){let _=main.set_size(PhysicalSize::new(150,190));let _=main.set_resizable(false);let _=main.set_maximizable(false);}
 if let Err(e)=ensure_chat(&app.handle()){eprintln!("chat window init failed: {e}");}
 let companion=MenuItemBuilder::with_id("companion","Kelly").enabled(false).build(app)?;let show=MenuItemBuilder::with_id("show","Show").build(app)?;let talk=MenuItemBuilder::with_id("talk","Talk").build(app)?;let settings=MenuItemBuilder::with_id("settings","Settings").build(app)?;let quit=MenuItemBuilder::with_id("quit","Exit KENRI").build(app)?;
 let menu=MenuBuilder::new(app).items(&[&companion,&show,&talk,&settings,&quit]).build()?;let mut tray=TrayIconBuilder::new().menu(&menu).tooltip("Kelly — KENRI Desktop Companion");if let Some(icon)=app.default_window_icon(){tray=tray.icon(icon.clone());}
 tray.on_menu_event(|app,event|match event.id.as_ref(){"show"=>if let Some(w)=app.get_webview_window("main"){let _=w.show();let _=w.set_focus();},"talk"=>{if let Some(w)=app.get_webview_window("main"){let _=w.show();}let _=toggle_chat_sync(app);},"settings"=>if let Some(w)=app.get_webview_window("main"){let _=w.show();let _=w.set_focus();let _=app.emit("open-settings",());},"quit"=>app.exit(0),_=>{}}).build(app)?;Ok(())
 }).invoke_handler(tauri::generate_handler![show_chat,hide_chat,toggle_chat]).on_window_event(|window,event|match event{
  tauri::WindowEvent::Moved(pos)=>{keep_pet_on_screen(window,*pos);if window.label()=="main"{if let Some(chat)=window.app_handle().get_webview_window("chat"){if chat.is_visible().unwrap_or(false){let _=place_chat(window.app_handle(),false);}}}},
  tauri::WindowEvent::CloseRequested{api,..}=>{api.prevent_close();let _=window.hide();},_=>{}
 }).run(tauri::generate_context!()).expect("error while running KENRI Desktop Companion");}

#[tauri::command]fn show_chat(app:tauri::AppHandle)->Result<(),String>{place_chat(&app,true).map_err(|e|e.to_string())}
#[tauri::command]fn hide_chat(app:tauri::AppHandle)->Result<(),String>{if let Some(chat)=app.get_webview_window("chat"){chat.hide().map_err(|e|e.to_string())?}Ok(())}
#[tauri::command]fn toggle_chat(app:tauri::AppHandle)->Result<(),String>{toggle_chat_sync(&app).map_err(|e|e.to_string())}
