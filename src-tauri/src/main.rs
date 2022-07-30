use app::sample;
use app::voice;
use tauri_plugin_store::PluginBuilder;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .invoke_handler(tauri::generate_handler![
            sample::sample_fn,
            voice::generate_query
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
