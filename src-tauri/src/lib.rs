mod proxy;
mod tray;

#[tauri::command]
fn switch_proxy(mode: &str) -> () {
    proxy::change_proxy(mode);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![switch_proxy])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
