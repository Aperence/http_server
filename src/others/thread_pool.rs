use tokio;
use std::{sync::{Arc, Mutex, mpsc::SendError}, collections::VecDeque, time::Duration};
use std::sync::mpsc::{channel, Sender};
use std::thread::sleep;

#[derive(Clone, Debug)]
pub enum Communication{
    Shutdown,
    NewTask
}

pub struct ThreadPool{
    tasks : Arc<Mutex<VecDeque<Box<dyn Fn() + Send + Sync>>>>,
    channels : Vec<Sender<Communication>>
}

impl ThreadPool{
    pub fn new(n : u32) -> ThreadPool{
        let mut channels = Vec::new();
        let tasks : Arc<Mutex<VecDeque<Box<dyn Fn() + Send + Sync>>>>  = Arc::new(Mutex::new(VecDeque::new()));
        for i in 0..n{
            let t = tasks.clone();
            let (sx, rx) = channel();
            channels.push(sx);
            tokio::spawn(async move{
                loop{
                    let r = rx.recv_timeout(Duration::from_millis(10));

                    // stop running
                    if let Ok(Communication::Shutdown) = r{
                        break;
                    }
                    let mut func : Option<Box<dyn Fn() + Send + Sync>> =Option::None;
                    match t.lock().unwrap().pop_front(){
                        Some(f) => {
                            println!("Thread {} received a task", i);
                            func = Option::Some(f);
                        },
                        None => {}
                    }
                    match func{
                        Some(f) => f(),
                        None => {
                            let message = rx.recv();
                            println!("Received message : {:?}", message);
                            match message{
                                Ok(Communication::NewTask) => continue,
                                Ok(Communication::Shutdown) => break,
                                Err(e) => {
                                    println!("Error occured {}", e.to_string());
                                    break;
                                }
                            }
                        }
                    }
                }
            });
        }

        ThreadPool{
            tasks : tasks,
            channels : channels
        }
    }

    fn broadcast(&self, message : Communication) -> Result<(), SendError<Communication>>{
        for channel in &self.channels{
            channel.send(message.clone())?;
        }
        Ok(())
    }

    pub fn add_task(&self, task : Box<dyn Fn() + Send + Sync>) -> Result<(), SendError<Communication>>{
        self.tasks.lock().unwrap().push_back(task);
        println!("Adding task");
        self.broadcast(Communication::NewTask)
    }

    pub fn shutdown(&self) -> Result<(), SendError<Communication>>{
        self.broadcast(Communication::Shutdown)
    }
}

pub fn run(){
    let t = ThreadPool::new(2);
    t.add_task(Box::new(|| {
        sleep(Duration::from_secs(1)); 
        println!("Done");
    })).unwrap();

    t.add_task(Box::new(|| {
        sleep(Duration::from_secs(1)); 
        println!("Done");
    })).unwrap();

    t.add_task(Box::new(|| {
        sleep(Duration::from_secs(1)); 
        println!("Done");
    })).unwrap();

    println!("Going to sleep");
    sleep(Duration::from_secs(5));
    println!("Just woke up");
    t.add_task(Box::new(|| {
        sleep(Duration::from_secs(1)); 
        println!("Done");
    })).unwrap();

    sleep(Duration::from_secs(3));
    t.shutdown().unwrap();
}