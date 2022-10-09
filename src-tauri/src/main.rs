#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::api::process::{Command, CommandEvent};
use tauri::Manager;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let splashscreen_window = app.get_window("splashscreen").unwrap();
      let main_window = app.get_window("main").unwrap();

      let (mut rx, _) = Command::new_sidecar("server")
        .expect("failed to create sidecar")
        .spawn()
        .expect("Failed to spawn server");

      tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
          if let CommandEvent::Stdout(line) = event {
            if line.contains("Network: ") {
              let tokens: Vec<&str> = line.split(":").collect();
              splashscreen_window.close().unwrap();
              main_window.eval(&format!("window.location.replace('http://localhost:{}')", tokens[3]));
              main_window.show().unwrap();
            }
          }
        }
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running application");
}
