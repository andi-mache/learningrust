use std::net::{TcpListener, TcpStream};
use std::thread;
use  std::net::IpAddr::V4;

use std::io::{Error, Read, Write};

// function to handle client
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("[INFO] INCOMING CONNECTION FROM : {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {
            return Ok(());
        }
        stream.write(&buf[..bytes_read])?;
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8888").expect("could not bind");

    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("failed : {}", e)
            }
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}