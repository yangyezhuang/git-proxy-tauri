#[path = "./proxy.rs"]
mod proxy;

use tauri::{
    menu::{Menu, MenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let default_i = MenuItem::with_id(app, "default_mode", "默认", true, None::<&str>)?;
    let http_i = MenuItem::with_id(app, "http_mode", "http模式", true, None::<&str>)?;
    let socks_i = MenuItem::with_id(app, "socks_mode", "socks模式", true, None::<&str>)?;
    let mode = Submenu::with_id_and_items(app, "File", "代理", true, &[&http_i, &socks_i])?;
    let show_i = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
    let hide_i = MenuItem::with_id(app, "hide", "隐藏", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    // 分割线
    let menu = Menu::with_items(app, &[&default_i, &mode, &show_i, &hide_i, &quit_i])?;

    let _ = TrayIconBuilder::with_id("tray")
        .title("title")
        .tooltip("GitProxy")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "default_mode" => {
                proxy::change_proxy("default");
            }
            "http_mode" => {
                proxy::change_proxy("http");
            }
            "socks_mode" => {
                proxy::change_proxy("socks");
            }
            "show" => {
                let window = app.get_webview_window("main").unwrap();
                let _ = window.show();
            }
            "hide" => {
                let window = app.get_webview_window("main").unwrap();
                let _ = window.hide();
            }
            "quit" => {
                app.exit(0);
            }
            // Add more events here
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app);
    Ok(())
}
