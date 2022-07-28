use std::collections::HashMap;
use std::error::Error;

// XXX: 環境変数から読めるようにする
static BASEURL: &str = "http://localhost:50021";

async fn get_query(speaker: i64, text: String) -> Result<String, Box<dyn Error>> {
    let mut params = HashMap::new();
    params.insert("speaker", speaker.to_string());
    params.insert("text", text);
    let url = reqwest::Url::parse_with_params(&format!("{}/audio_query", BASEURL), params).unwrap();
    let client = reqwest::Client::new();
    let query = client.post(url).send().await?.text().await?;
    let query_text = serde_json::to_string(&query).unwrap();
    Ok(query_text)
}

#[tauri::command]
pub async fn generate_query(speaker: i64, text: String) -> Result<String, String> {
    let ret: Result<String, Box<dyn Error>> = get_query(speaker, text).await;
    match ret {
        Ok(query) => {
            println!("{:?}", query);
            Ok(query.to_string())
        }
        Err(msg) => {
            Err(msg.to_string())
        }
    }
}
