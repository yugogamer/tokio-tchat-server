use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};



#[tokio::main]
async fn main() {
    let adresse = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let socket = TcpStream::connect(&adresse).await;

    match socket {
        Ok(mut socket) => {
            socket.write_all(b"je suis connecter").await.expect("message not send");
            process_socket(socket).await;
        },
        Err(err) => panic!("can't connect to server : {}", err),
    }
}

async fn process_socket(mut socket : TcpStream){
    let mut buf = [0; 1024];

    loop {
        tokio::select! {
            Ok(message_lenght) = socket.read(&mut buf) => {
                if message_lenght == 0 {
                    return
                }
                
                let message_bin = buf.to_vec();
                
                let message_text = std::str::from_utf8(&message_bin);
                
                match message_text{
                    Ok(message_text) => {
                        println!("{}", message_text);
                    },
                    Err(err) => {
                        eprintln!("{}", err)
                    }
                }
            }
        }
        
    }
}