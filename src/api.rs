use reqwest::header::AUTHORIZATION;
use std::fs::File;
use std::io::Write;
use std::path::{self, PathBuf};

use crate::config::{BASE_URL, SEARCH_PATH};
use crate::responses::SearchResult;
use urlencoding::{decode, encode};

pub async fn genius_search(keyword: String, bearer_token: String) -> SearchResult {
    let client = reqwest::Client::new();
    let joined_path = format!(
        "{0}?q={1}",
        path::PathBuf::new()
            .join(BASE_URL)
            .join(SEARCH_PATH)
            .to_str()
            .unwrap(),
        encode(&keyword).into_owned()
    );

    let resp = client
        .get(joined_path)
        .header(AUTHORIZATION, format!("Bearer {}", bearer_token))
        .send()
        .await
        .unwrap();

    let text = resp.text().await.unwrap();

    let sr: SearchResult = serde_json::from_str(&text).unwrap();

    sr
}

pub async fn get_and_save(keyword: String, sr: SearchResult, bearer_token: String) -> Vec<PathBuf> {
    let dir_path = PathBuf::new().join("assets").join(keyword.clone());
    let _ = std::fs::create_dir_all(dir_path.clone());
    let client = reqwest::Client::new();
    let mut files: Vec<PathBuf> = Vec::new();

    for (idx, r) in sr.response.hits.iter().enumerate() {
        println!("{}", r.result.header_image_url);
        let file_name = format!("{}-{}", keyword, idx);
        let mut dest = File::create(dir_path.join(file_name.clone())).unwrap();
        let resp = client
            .get(r.result.header_image_url.clone())
            .header(AUTHORIZATION, format!("Bearer {}", bearer_token))
            .send()
            .await
            .unwrap();

        if let Ok(_) = dest.write_all(&resp.bytes().await.unwrap()) {
            files.push(dir_path.join(file_name));
        }
    }

    files
}
