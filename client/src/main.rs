use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use common::Message;
use crossterm::event::{EventStream, KeyCode, Event};

use futures::{future::FutureExt, StreamExt};
use tokio::{net::{TcpStream}, io::{AsyncWriteExt, AsyncReadExt}};



#[tokio::main]
async fn main() {
    let adresse = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let socket = TcpStream::connect(&adresse).await;

    match socket {
        Ok(mut socket) => {
            send_message("je suis connecter".to_owned(), &mut socket).await;
            process_socket(socket).await;
        },
        Err(err) => panic!("can't connect to server : {}", err),
    }
}

async fn process_socket(mut socket : TcpStream){
    let mut buf = [0; 1024];
    let mut reader = EventStream::new();

    loop {
        let event = reader.next().fuse();

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

            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        if event == Event::Key(KeyCode::Char('c').into()) {
                            send_message("Message here".to_owned(), &mut socket).await;
                        }
                    },
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
        }
    }
}

async fn send_message(message : String, socket :  &mut TcpStream){
    let message_to_serialize = Message{message : message};
    let message_to_send = bincode::serialize(&message_to_serialize).unwrap();
    let result = socket.write_all(&message_to_send).await;
    if result.is_err(){
        eprintln!("error, can't send message");
    }
}