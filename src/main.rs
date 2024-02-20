use redis_starter_rust::database::redis_database::RedisDatabase;
use redis_starter_rust::connection::handle_connection::handle_connection;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Port already in use");

    let db = Arc::new(Mutex::new(RedisDatabase::new()));
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                println!(
                    "[*] Accepted connection from: {}",
                    stream.peer_addr().unwrap()
                );
                let db_clone = Arc::clone(&db);
                tokio::spawn(async move {
                    handle_connection(stream, db_clone).await;
                });
            }
            Err(e) => {
                eprintln!("[!] Failed to accept connection: {}", e);
            }
        }
    }
}
