// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, GlobalShortcutManager, WindowEvent};
use tauri::{Manager, WindowBuilder, WindowUrl};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            let wa = app.handle();
            let main_window = wa.get_window("main").unwrap();
            let search_shortcut = "Shift+Space";
            let mut shortcut = app.global_shortcut_manager();

            // 创建窗口-----------------------------
            let search_win = WindowBuilder::new(
                app,
                "search",
                WindowUrl::External("http://yjmyzz.cnblogs.com/".parse().unwrap()),
            )
            .title("菩提村下的杨过")
            .inner_size(640.0, 480.0)
            .position(50.0, 100.0)
            .visible(false)
            .build()?;

            search_win.on_window_event(move |event| {
                if let WindowEvent::Focused(is_focused) = event {
                    if !is_focused {
                        main_window.get_window("search").unwrap().hide().unwrap();
                    }
                }
            });

            shortcut
                .register(search_shortcut, move || {
                    search_win.show().unwrap();
                    search_win.set_always_on_top(true).unwrap();
                    search_win.set_focus().unwrap();
                })
                .unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(context)
        .expect("error while running tauri application");
}
