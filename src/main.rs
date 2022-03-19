use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    let server_variable = "SERVER_ADDRESS";
    let server_port = "SERVER_PORT";
    let server_address = match env::var(server_variable) {
        Ok(v) => v,
        Err(_) => "127.0.0.1".to_string(),
    };
    let server_port = match env::var(server_port) {
        Ok(v) => v,
        Err(_) => "4040".to_string(),
    };

    println!("Attempting connection: {server_address}:{server_port}");

    if let Ok(mut stream) = TcpStream::connect(format!(
        "{address}:{port}",
        address = &server_address,
        port = &server_port
    ))
    .await
    {
        println!(
            "connected to server: {}:{}",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        let mut counter: i64 = 0;

        loop {
            let message = "Hello World!\n";
            let _ = stream.write_all(message.as_bytes()).await;
            // println!("sent: {}", message);

            // read the result
            let mut buffer = [0; 1024];
            let len = stream.read(&mut buffer).await.unwrap();
            counter += 1;
            if counter % 10000 == 0 {
                let message = String::from_utf8_lossy(&buffer[..len]);
                println!("received: {}", &message);
            }
        }
    } else {
        println!(
            "couldn't connect to echo server: {}:{}",
            &server_address, &server_port
        );
    }
}
