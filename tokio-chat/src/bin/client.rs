use std::io::stdin;

use tokio::{io::{ AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080";
    let socket = TcpStream::connect(addr).await?;
    let (mut reader, mut writer) = socket.into_split();

    tokio::spawn(async move {
        let mut reader = BufReader::new(&mut reader);
        loop {
            let mut buffer = String::new();
            reader.read_line(&mut buffer).await.unwrap();
            println!("{}", buffer);
        }
    });

    loop {
        let mut buffer = String::new();
        
        stdin().read_line(&mut buffer)?;
        writer.write_all(buffer.as_bytes()).await?;
    }
}
