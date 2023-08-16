use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tauri::{Window};


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Json {
    response: Option<String>,
    done: bool,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
    done: bool,
}

#[tauri::command]
pub async fn greet(window: Window, prompt_message: &str) -> Result<(), String> {
    let mut map = HashMap::new();
    map.insert("model", "llama2");
    map.insert("prompt", prompt_message);

    let client = reqwest::Client::new();

    let mut res = client.post("http://127.0.0.1:11434/api/generate")
        .json(&map)
        .send()
        .await
        .unwrap();

    while let Some(chunk) = res.chunk().await.expect("Failed on chunks") {
        let json: Json = serde_json::from_slice(&chunk).expect("Failed on parsing");

        window
            .emit("generate-answer-listener", Payload {
                message: json.response.unwrap_or("".into()),
                done: json.done,
            })
            .expect("Failed on sending event back to front");

        if json.done {
            break;
        }
    }

    Ok(())
}