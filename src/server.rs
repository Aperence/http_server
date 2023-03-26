use tokio::net::{TcpSocket};

pub struct Server{
    _i : u32
}

impl Server{
    pub async fn server(addr : &str){
        let addr = addr.parse().unwrap();
        let socket = TcpSocket::new_v6().unwrap();
        socket.bind(addr).unwrap();
        let listener = socket.listen(1024).unwrap();
        loop{
            match listener.accept().await{
                Result::Ok((_stream, addr)) => {
                    println!("Connection received from {}", addr.to_string());
                }
                Result::Err(e) =>{
                    println!("Error : {}", e)
                }
            }
        }
        //Server{_i : 3}
    }
}