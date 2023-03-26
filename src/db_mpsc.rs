use tokio::task::JoinHandle;
use core::time;
use std::{sync::mpsc::{channel, Sender, Receiver, SendError}, collections::HashMap, thread::sleep};

enum Operation{
    Set(String, String),
    Get(String, Sender<String>),
    Shutdown
}

#[derive(Clone)]
struct DB{
    sx : Sender<Operation>,
}

impl DB{
    pub fn new()-> DB{
        let (sx, rx) = channel();

        tokio::spawn(async move{
            let receiver : Receiver<Operation> = rx;
            let mut map = HashMap::new();
            loop{
                let request = receiver.recv();
                match request.unwrap(){
                    Operation::Shutdown =>  {
                        println!("Shutting down db");
                        break
                    },
                    Operation::Set(key, value) =>{
                        println!("Set {} as {}", key, value);
                        map.insert(key, value);
                    },
                    Operation::Get(key, sender) =>{
                        println!("Get {}", key);
                        let res = map.get(&key).unwrap();
                        let sending : String = res.clone().to_string();
                        if let Err(e) = sender.send(sending){
                            println!("Error sending response {}", e.to_string());
                        }
                    }
                }
            }
        });
    
        DB{sx}
    }

    fn shutdown(self : &DB) -> Result<(), SendError<Operation>>{
        self.sx.send(Operation::Shutdown)
    }

    fn send(self : &DB, op : Operation) -> Result<(), SendError<Operation>>{
        self.sx.send(op)
    }

}


fn client(db : DB, key : String, value : String) -> JoinHandle<Result<(),SendError<Operation>>>{
    let res = tokio::spawn(async move{
        db.send(Operation::Set(key.clone(), value.clone()))?;
        let (sx, rx) = channel();
        sleep(time::Duration::from_secs(1));
        db.send(Operation::Get(key, sx))?;
        let res = rx.recv().unwrap();
        println!("Received : {} when sent {}", res, value);
        Ok(())
    });
    res
}

pub async fn lauch_test_db(){

    let db = DB::new();

    let mut v = Vec::new();
    for i in 0..10{
        let future = client(db.clone(), i.to_string(), (i+1).to_string());
        v.push(future);
    }

    for i in v{
        match i.await.unwrap(){
            Ok(()) => continue,
            Err(e) => {
                println!("Error sending {}", e.to_string());

            }
        }
    }

    if let Err(e) = db.shutdown(){
        println!("Error during db shutdown {}", e.to_string());
    }
}