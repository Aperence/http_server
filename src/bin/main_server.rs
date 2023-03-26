use tokio;
use http_server::server::Server;

#[tokio::main]
async fn main(){
    let _server = Server::server("[::1]:8080").await;
}