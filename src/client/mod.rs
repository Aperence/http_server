use bytes::Bytes;
use tokio::io::{AsyncWriteExt, BufReader};
use tokio::sync::mpsc::{Sender, channel};
use tokio::task::JoinHandle;
use tokio::net::TcpSocket;
use tokio::sync::oneshot;

#[derive(Debug)]
pub enum Command{
    Get(String),
    Set(String, Bytes)
}

#[derive(Debug)]
struct Msg(Command, oneshot::Sender<()>);

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
                        let mut buffer = vec![0; 1024];
                        stream.readable().await;
                        let res = stream.try_read(&mut buffer);
                        stream.shutdown().await.unwrap();
                        if let Ok(n) = res{
                            println!("Received from server : {}", String::from_utf8(buffer[..n].to_ascii_lowercase()).unwrap());
                        }else{
                            println!("{:?}", res);
                        }
                        sender.send(()).unwrap();
                    },
                    Msg(Command::Set(key, value), sender) =>{
                        // handle a set request
                        println!("Received command set with (key, value) : ({}, {:?})", key, value.to_ascii_lowercase());
                        sender.send(()).unwrap();
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
            let (tx, rx) = oneshot::channel();
            handler.send(Msg(c, tx)).await.unwrap();
            // wait for the response
            rx.await.unwrap();
        })
    }

    pub async fn wait_finished(self){
        drop(self.handler);
        self.join.await.unwrap();
    }
}