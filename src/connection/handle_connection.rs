use crate::database::redis_database::RedisDatabase;
use crate::connection::parse_request::parse_request;
use std::sync::{Arc, Mutex};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use tokio::time::Duration;
use std::thread;

fn handle_request(request: String, db: Arc<Mutex<RedisDatabase>>) -> String {
    match request.split_whitespace().collect::<Vec<&str>>().as_slice() {
        ["PING"] => "+PONG\r\n".to_string(),
        ["ECHO", message] => format!("+{}\r\n", message),
        ["SET", key, value] => {
            let mut db = db.lock().unwrap();
            db.set_key(key.to_string(), value.to_string())
        }
        ["SET", key, value, "PX", n] => {
            let mut db_clone = db.lock().unwrap();
            db_clone.set_key(key.to_string(), value.to_string());

            let db = Arc::clone(&db);
            let timeout = n.parse::<u64>().unwrap();
            let k = key.to_string();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(timeout));
                let mut db_clone = db.lock().unwrap();
                db_clone.remove_key(k);
            });
            
            "+OK\r\n".to_string()
        }
        ["GET", key] => {
            let db = db.lock().unwrap();
            db.get_key(key.to_string())
        }
        _ => "".to_string(),
    }
}

async fn read_input(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer).await {
        Ok(0) => String::new(),
        Ok(n) => String::from_utf8_lossy(&buffer[..n]).to_string(),
        Err(e) => {
            eprintln!("[!] Failed to read from socket: {}", e);
            String::new()
        }
    }
}

pub async fn handle_connection(mut stream: TcpStream, db: Arc<Mutex<RedisDatabase>>) {
    loop {
        let request = read_input(&mut stream).await;
        let db_clone = Arc::clone(&db);
        let response = handle_request(parse_request(&request), db_clone);
        if !response.is_empty() {
            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("[!] Failed to write to socket: {}", e);
            }
        }
    }
}
