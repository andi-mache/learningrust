use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    let (tx, _rx) = broadcast::channel::<(SocketAddr, String)>(100);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New client connected: {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);


            loop {
                let mut buffer = String::new();
                //let msg = "he connecter";
                //tx.send((addr, msg.to_string())).unwrap();
                tokio::select! {
                    msg = rx.recv() => {
                        if let Ok((other_addr, msg)) = msg {
                            if other_addr != addr {

                                writer.write_all(format!("{}: {}", other_addr, msg).as_bytes()).await;
                            }
                        }
                    }
                    result = reader.read_line(&mut buffer) => {
                        if result.is_err() || buffer.trim() == "exit" || buffer.is_empty() {
                            println!("Client disconnected: {}", addr);

                            break;
                        }

                        // Only broadcast if the message isn't empty
                        if !buffer.trim().is_empty() {
                            tx.send((addr, buffer.clone())).unwrap();
                        }
                    }
                }
            }
        });
    }
}