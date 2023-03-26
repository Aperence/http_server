use http_server::client::{Client, Command};
use tokio;
    

#[tokio::main]
async fn main(){
    let client = Client::connect("[::1]:8080").await;
    client.request(Command::Get("hello".to_string())).await;
    client.wait_finished().await;
}