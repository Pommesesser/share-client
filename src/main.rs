mod http;
mod file_info;

use std::env;

// Pls merge project with share-server if possible

#[tokio::main]
async fn main() {
    let mut args = env::args();

    let _program = args.next();

    let command = match args.next() {
        Some(command) => command,
        None => {
            println!("usage: share-client <command>");
            return;
        }
    };

    match command.as_str() {
        "upload" => {
            let path = match args.next() {
                Some(path) => path,
                None => {
                    println!("usage: share-client upload <path>");
                    return;
                }
            };

            http::upload(&path).await;
        }

        "ls" => {
            http::list().await;
        }

        "get" => {
            let id = match args.next() {
                Some(id) => id,
                None => {
                    println!("usage: share-client get <id>");
                    return;
                }
            };

            http::get(&id).await;
        }

        "rm" => {
            let id = match args.next() {
                Some(id) => id,
                None => {
                    println!("usage: share-client rm <id>");
                    return;
                }
            };

            http::remove(&id).await;
        }

        _ => {
            println!("unknown command: {command}");
        }
    }
}
