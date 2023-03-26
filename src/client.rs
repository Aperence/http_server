use bytes::Bytes;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::{Sender, channel};
use tokio::task::JoinHandle;
use tokio::net::TcpSocket;

#[derive(Debug)]
pub enum Command{
    Get(String),
    Set(String, Bytes)
}

#[derive(Debug)]
struct Msg(Command, Sender<()>);

pub struct Client{
    handler : Sender<Msg>,
    join : JoinHandle<()>
}

impl Client{
    pub async fn connect(addr : &str) -> Client{
        let (tx, mut rx) = channel(32);

        let addr = addr.parse().unwrap();
        let socket = TcpSocket::new_v6().unwrap();
        let mut stream = socket.connect(addr).await.unwrap();
    
        let join = tokio::spawn(async move{
            println!("Starting server...");
            while let Some(cmd) = rx.recv().await{
                match cmd{
                    Msg(Command::Get(key), sender) => {
                        // handle a get request
                        println!("Received command get with key : {}", key);
                        stream.write(key.as_bytes()).await.unwrap();
                        sender.send(());
                    },
                    Msg(Command::Set(key, value), sender) =>{
                        // handle a set request
                        println!("Received command set with (key, value) : ({}, {:?})", key, value.to_ascii_lowercase());
                        sender.send(());
                    }
                }
            }
            println!("Shutting down...");
        });
        Client{handler : tx, join : join}
    }

    pub async fn request(&self, c : Command) -> JoinHandle<()>{
        println!("Initiating request...");
        let handler = self.handler.clone();
        tokio::spawn(async move{
            println!("Sending request {:?}", c);
            let (tx, mut rx) = channel(1);
            handler.send(Msg(c, tx)).await.unwrap();
            rx.recv().await.unwrap();
        })
    }
}