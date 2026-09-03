use std::path::Path;
use crate::file_info::FileInfo;

const SERVER: &str = "http://localhost:3000";

// The http layer should not take over the control flow
// Display and filesystem work should be extracted
// Implement streaming

pub async fn upload(path: &str) {
    let data = std::fs::read(path)
        .expect("failed to read file");

    let filename = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .expect("failed to read filename");

    let response = reqwest::Client::new()
        .post(format!("{SERVER}/files"))
        .header("x-file-name", filename)
        .body(data)
        .send()
        .await
        .expect("failed to send request");

    if !response.status().is_success() {
        panic!("server returned {}", response.status());
    }

    let id = response
        .text()
        .await
        .expect("failed to read response");
    
    println!("{id}");
}

pub async fn list() {
    let response = reqwest::get(format!("{SERVER}/files"))
        .await
        .expect("failed to get a response");

    if !response.status().is_success() {
        panic!("server returned {}", response.status());
    }

    response.json::<Vec<FileInfo>>()
        .await
        .expect("failed to read response")
        .iter()
        .for_each(|file_info| println!("{:?}", file_info))
}

pub async fn get(id: &str) {
    let response = reqwest::get(format!("{SERVER}/files/{id}"))
        .await
        .expect("failed to get a response");

    if !response.status().is_success() {
        panic!("server returned {}", response.status());
    }

    let filename = response
        .headers()
        .get("x-file-name")
        .expect("missing filename")
        .to_str()
        .expect("invalid filename header")
        .to_owned();

    let data = response
        .bytes()
        .await
        .expect("failed to read response");

    std::fs::write(&filename, data)
        .expect("failed to write file");

    println!("{filename}");
}

pub async fn remove(id: &str) {
    let response = reqwest::Client::new()
        .delete(format!("{SERVER}/files/{id}"))
        .send()
        .await
        .expect("failed to send request");

    if !response.status().is_success() {
        panic!("server returned {}", response.status());
    }

    println!("{id}");
}
