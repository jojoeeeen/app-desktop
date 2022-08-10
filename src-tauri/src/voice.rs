use base64;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

// XXX: 環境変数から読めるようにする
static BASEURL: &str = "http://localhost:50021";

#[derive(Deserialize, Serialize)]
struct Mora {
    text: String,
    consonant: Option<String>,
    consonant_length: Option<f64>,
    vowel: String,
    vowel_length: f64,
    pitch: f64,
}

#[derive(Deserialize, Serialize)]
struct AccentPhrase {
    moras: Vec<Mora>,
    accent: f64,
    pause_mora: Option<Vec<Mora>>,
    is_interrogative: Option<bool>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize)]
pub struct VoiceQuery {
    accent_phrases: Vec<AccentPhrase>,
    speedScale: f64,
    pitchScale: f64,
    intonationScale: f64,
    volumeScale: f64,
    prePhonemeLength: f64,
    postPhonemeLength: f64,
    outputSamplingRate: f64,
    outputStereo: bool,
    kana: Option<String>,
}

async fn _generate_query(speaker: i64, text: String) -> Result<String, Box<dyn Error>> {
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
    let ret: Result<String, Box<dyn Error>> = _generate_query(speaker, text).await;
    match ret {
        Ok(query) => Ok(query.to_string()),
        Err(msg) => Err(msg.to_string()),
    }
}

async fn _generate_voice(speaker: i64, query_text: String) -> Result<String, Box<dyn Error>> {
    let query: VoiceQuery = serde_json::from_str(&query_text).unwrap();
    let mut params = HashMap::new();
    params.insert("speaker", speaker.to_string());
    let url = reqwest::Url::parse_with_params(&format!("{}/synthesis", BASEURL), params).unwrap();
    let client = reqwest::Client::new();
    let b_voice = client.post(url).json(&query).send().await?.bytes().await?;
    let base64_voice = base64::encode(b_voice);
    Ok(base64_voice)
}

#[tauri::command]
pub async fn generate_voice(speaker: i64, query: String) -> Result<String, String> {
    let ret: Result<String, Box<dyn Error>> = _generate_voice(speaker, query).await;
    match ret {
        Ok(query) => Ok(query.to_string()),
        Err(msg) => Err(msg.to_string()),
    }
}
