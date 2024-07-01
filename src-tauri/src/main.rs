#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use scoreboard::coc_models;
use tauri::{Manager, WindowEvent};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            let main_window = app_handle.get_window("main").unwrap();
            main_window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { .. } = event {
                    for window in app_handle.windows().values() {
                        window.close().unwrap();
                    }
                    std::process::exit(0);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            coc_models::login::login,
            coc_models::search::search_clans,
            coc_models::window::open_new_window,
            coc_models::clan_info::get_war,
            coc_models::window::adjust_window_size,
            coc_models::window::close_window,
            coc_models::clan_info::parse_wardata
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// "updater": {
//       "active": true,
//       "endpoints": [
//         "https://drive.google.com/file/d/1H4EGEhbhUIQD2nvfGnjCne7YeAGz0t1F/view?usp=sharing"
//       ],
//       "dialog": true,
//       "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDdEMTQxNzY0M0UyMEI1MTMKUldRVHRTQStaQmNVZldrc0ptTFBOU2RqUThqby9MM2ZITTE1a1NQTVV1NlpqTW42K3czclBPTU0K",
//       "windows": {
//         "installMode": "passive"
//       }
//     },
