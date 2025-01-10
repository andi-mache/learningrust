use std::net::SocketAddr; 

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    
    let listener = TcpListener::bind("0.0.0.0:8978").await?;

    let (tx, _rx) = broadcast::channel::<(SocketAddr, String)>(100);

    loop {
        let (mut socket, addr) = listener.accept().await?;

        print!("listening to {}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);

            loop {
                let mut buffer = String::new();

                tokio::select! {
                    msg = rx.recv() => {
                        if let Ok((other_addr , msg)) = msg {
                            if other_addr != addr {
                                writer.write_all(format!("{}: {}", other_addr, msg).as_bytes()).await.unwrap();
                            }
                        }
                    }

                    result = reader.read_line(&mut buffer) => {
                        if result.is_err() || buffer.trim() == "exit" || buffer.is_empty() {
                            println!("Disconetced, {}", addr);
                            break;
                        }
                        tx.send((addr, buffer)).unwrap();
                    }
                }

            }
        });
    }
}
