use common::Message;
use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}, sync::broadcast::{self, Sender}};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Can't open server");
    let (main_tx, mut _main_rx) = broadcast::channel::<Message>(16);
    
    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                let sender = main_tx.clone();
                tokio::spawn(async move {
                    println!("new connection from : {:?}", addr);
                    process_socket(socket, sender).await;
                });
            },
            Err(err) => eprintln!("Err connection refused : {}", err),
        }
    }
}


async fn process_socket(mut socket : TcpStream, sender : Sender<Message>){
    let mut buf = [0; 1024];
    
    loop {
        let sender = sender.clone();
        let mut receiver = sender.subscribe();
        tokio::select! {
            
            Ok(message_lenght) = socket.read(&mut buf) => {
                if message_lenght == 0 {
                    return
                }
                
                let message_bin = buf.to_vec();
                
                let decoded = bincode::deserialize(&message_bin);

                match decoded{
                    Ok(message) => {
                        println!("message received from : {:?}, message is : {:?}", socket.peer_addr(), message);
                        let result = sender.send(message);
                        if result.is_err(){
                            eprintln!("err, can't write in channel")
                        }
                    },
                    Err(err) => eprintln!("unexpected message type : {}", err),
                }
            }
            
            Ok(message_receive) = receiver.recv() => {
                let to_send = bincode::serialize(&message_receive).unwrap();
                let result = socket.write_all(&to_send).await;
                if result.is_err(){
                    eprintln!("err, can't write in socket")
                }
            }
        }
    }
}