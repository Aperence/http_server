use crate::db_mpsc::lauch_test_db;
use crate::thread_pool::{run};

mod db_mpsc;
mod thread_pool;

#[tokio::main]
async fn main(){
    lauch_test_db().await;
    run();
}