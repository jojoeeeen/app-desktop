use tauri_plugin_store::PluginBuilder;
use app::sample;

#[cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  tauri::Builder::default()
    .plugin(PluginBuilder::default().build())
    .invoke_handler(tauri::generate_handler![sample::sample_fn])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
