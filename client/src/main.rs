use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use common::Message;
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};



#[tokio::main]
async fn main() {
    let adresse = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let socket = TcpStream::connect(&adresse).await;

    match socket {
        Ok(mut socket) => {
            let to_serialze = Message::Message("Je suis connecter".to_owned());
            let to_send = bincode::serialize(&to_serialze).unwrap();
            socket.write_all(&to_send).await.expect("message not send");
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
                
                let decoded = bincode::deserialize::<Message>(&message_bin);

                match decoded{
                    Ok(message) => {
                        println!("message received from : {:?}, message is : {:?}", socket.peer_addr(), message);
                    },
                    Err(err) => eprintln!("unexpected message type : {}", err),
                }
            }
        }
        
    }
}