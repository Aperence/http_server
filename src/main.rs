use crate::client::{Client, Command};
use crate::server::Server;
use std::env::args;
use tokio;

mod client;
mod server;
    

#[tokio::main]
async fn main(){
    let arg : Vec<String> = args().collect::<Vec<String>>();
    if arg[1] == "client"{
        let client = Client::connect("[::1]:8080").await;
        client.request(Command::Get("hello".to_string())).await;
    }else{
        let _server = Server::server("[::1]:8080").await;
    }
}