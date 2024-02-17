use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const CONTROL_CHARACTERS: [char; 5] = ['+', '-', ':', '$', '*'];

fn parse_request(request: &str) -> String {
    let mut parsed_request = String::new();
    request.to_uppercase().split("\r\n").for_each(|line| {
        let mut pass = false;
        for c in CONTROL_CHARACTERS.iter() {
            if line.starts_with(*c) {
                pass = true;
                break;
            }
        }
        if !pass {
            parsed_request = format!("{} {}", parsed_request, line);
        }
    });
    return parsed_request;
}

fn handle_request(request: String) -> String {
    let response: String;
    match request
        .to_uppercase()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .as_slice()
    {
        ["PING"] => response = "+PONG\r\n".to_string(),
        ["ECHO", message] => response = format!("+{}\r\n", message),
        _ => response = "-ERR unknown command\r\n".to_string(),
    }
    response
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        let n = match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("[!] Failed to read from socket: {}", e);
                return;
            }
        };
        let request = String::from_utf8_lossy(&buffer[..n]);
        println!("[*] Received: {}", request);
        let response = handle_request(parse_request(&request));
        if !response.is_empty() {
            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("[!] Failed to write to socket: {}", e);
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379")
        .await
        .expect("Port already in use");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_connection(stream).await;
                });
            }
            Err(e) => {
                eprintln!("[!] Failed to accept connection: {}", e);
            }
        }
    }
}
