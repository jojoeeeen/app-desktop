#[tauri::command]
pub async fn sample_fn() -> Result<String, String> {
    let msg = "Rustの関数実行に成功";
    println!("{}", msg);
    Ok(msg.to_string())
}
