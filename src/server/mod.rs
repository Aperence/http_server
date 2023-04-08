use tokio::io::{BufWriter, AsyncWriteExt};
use tokio::net::{TcpSocket, TcpStream};
use tokio::sync::Semaphore;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::sync::Arc;
use bytes::Bytes;



type DB = Arc<Mutex<HashMap<String, Bytes>>>;

pub async fn process(mut stream : TcpStream, addr : SocketAddr, db : DB) -> Result<(), Box<dyn Error>>{

    let (rd, mut wr) = stream.split();
    println!("Processing request...");

    rd.readable().await?;

    let mut buffer = BufWriter::new(wr);

    buffer.write(b"test message").await;

    buffer.shutdown().await.unwrap();

    Ok(())

}

pub async fn server(addr : &str){

    let connections = Arc::new(Semaphore::new(3));

    let addr = addr.parse().unwrap();
    let socket = TcpSocket::new_v6().unwrap();
    socket.bind(addr).unwrap();
    let listener = socket.listen(1024).unwrap();
    let db : DB = Arc::new(Mutex::new(HashMap::new()));
    loop{
        let sem = connections.clone();
        match listener.accept().await{
            Result::Ok((stream, addr)) => {
                println!("Connection received from {}", addr.to_string());
                let db =  db.clone();
                tokio::spawn(async move {
                    let permit = sem.acquire().await.unwrap();
                    process(stream, addr, db).await;
                    drop(permit);
                });
            }
            Result::Err(e) =>{
                println!("Error : {}", e)
            }
        }
    }
}
