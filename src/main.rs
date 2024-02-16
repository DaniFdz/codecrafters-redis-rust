use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};

fn handle_request(stream: &mut TcpStream, input: &str) -> Result<()> {
    let response = match input.to_uppercase().as_str() {
        "PING" => "+PONG\r\n",
        _ => return Ok(())
    };
    println!("[i] Sending response: {}", response.trim());
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> Result<()> {
    println!("[i] Connection established with {}", stream.peer_addr()?);
    let mut buffer = [0; 1024];
    let mut input = String::new();
    loop {
        let n = stream
            .read(&mut buffer)
            .expect("Failed to read data from stream");
        if n == 0 {
            break;
        }
        println!("[i] Received {} bytes", n);
        input.push_str(&String::from_utf8_lossy(&buffer[..n]));

        input.split("\r\n").collect::<Vec<&str>>().iter().for_each(|x| {
            println!("[i] Handling request: {}", x);
            handle_request(stream, x).expect("Failed to handle request");
        });
    }

    Ok(())
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(&mut _stream)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
