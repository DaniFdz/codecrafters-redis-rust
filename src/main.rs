use tokio::{
    io::{ AsyncReadExt, AsyncWriteExt },
    net::{ TcpListener, TcpStream }
};

fn handle_request(request: &str) -> String {
    let response: String;
    match request.to_uppercase().split_whitespace().collect::<Vec<&str>>().as_slice() {
        ["PING"] => {
            response = "+PONG\r\n".to_string();
        },
        _ => {
            response = "".to_string();
        }
    }
    response
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop{
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
        for line in request.lines() {
            let response = handle_request(line);
            if !response.is_empty() {
                if let Err(e) = stream.write_all(response.as_bytes()).await {
                    eprintln!("[!] Failed to write to socket: {}", e);
                    return;
                }
            }
        }
    }
}

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("Port already in use");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_connection(stream).await;
                });
            },
            Err(e) => {
                eprintln!("[!] Failed to accept connection: {}", e);
            }
        }
    }
}
