// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::{Read,Write};


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            // Ok(mut stream) => {
                // println!("accepted new connection");
                // let mut buf = [0; 512];
                // stream.read(&mut buf).unwrap();
                // stream.write("+PONG\r\n".as_bytes()).unwrap();
            Ok(mut stream) => {
                let pong:String  = String::from("PONG");
                let mut buf = [0; 512];
                stream.read(&mut buf);
                // print!("return {}",String::from(buf));
                stream.write(pong.as_bytes()).expect("Failed to write to server");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
