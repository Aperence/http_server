use std::{io::Read, fs::read};

use bytes::{BufMut, Buf, BytesMut};

fn main(){
    println!("Here");

    let mut buf = BytesMut::with_capacity(1024);

    buf.put(&b"hello"[..]);

    //assert_eq!(buf, b"hello");
    
    let hello = String::from_utf8(buf.split().to_vec()).unwrap();

    print!("{}", hello);

    buf.put(&b" world"[..]);

    
    let world = String::from_utf8(buf.split().to_vec()).unwrap();

    println!("{}", world);
}