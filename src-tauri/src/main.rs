#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::Manager;

mod commands;

#[derive(Clone, serde::Serialize)]
struct ExamplePayload {
    message: String
}

fn main() {
	tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.emit(
                "example-event",
                ExamplePayload { message: "How do events work?".into() }
            ).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::wink,
            commands::get_uuids,
            commands::list_totp,
            commands::register_totp,
            commands::calculate_totp,
            commands::update::update_device,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
