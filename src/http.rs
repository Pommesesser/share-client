use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use std::path::Path;
use tokio_util::io::ReaderStream;
use crate::file_info::FileInfo;

const SERVER: &str = "http://185.216.178.220:3000";

pub async fn upload(path: &str) {
    let path = Path::new(path);

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("failed to read filename");

    let file = tokio::fs::File::open(path)
        .await
        .expect("failed to open file");
    let stream = ReaderStream::new(file);

    let response = reqwest::Client::new()
        .post(format!("{SERVER}/files"))
        .header("x-file-name", filename)
        .body(reqwest::Body::wrap_stream(stream))
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

    println!("{SERVER}/files/{id}");
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

    let mut file = tokio::fs::File::create(&filename)
        .await
        .expect("failed to create file");

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk
            .expect("failed to read response chunk");

        file.write_all(&chunk)
            .await
            .expect("failed to write file");
    }

    println!("downloaded {filename}");
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
        .for_each(|file_info| println!("{} | {}", file_info.id, file_info.name))
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

    println!("removed {id}");
}
