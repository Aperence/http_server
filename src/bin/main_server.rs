use tokio;
use http_server::server::server;

#[tokio::main]
async fn main(){
    server("[::1]:8080").await;
}