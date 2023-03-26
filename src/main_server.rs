use tokio;
use crate::server::Server;

mod server;

#[tokio::main]
async fn main(){
    let server = Server::server("[::1]:8080").await;
}