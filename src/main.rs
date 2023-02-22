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
                // let mut buf = [0; 512];
                // stream.read(&mut buf).unwrap();
                // stream.write("+PONG\r\n".as_bytes()).unwrap();
            Ok(mut stream) => {
                loop {
                    println!("\n accepted new connection");
                    let pong:String  = String::from("+PONG\r\n");
                    let mut buf = [0; 512];
                    let bytes = stream.read(&mut buf).unwrap();
                    if bytes == 0 {
                        println!("Finish sending");
                        break;
                    }
                    print!("return {:?}",String::from_utf8(buf.to_vec()));
                    stream.write(pong.as_bytes()).expect("Failed to write to server");
                }

            }
            Err(e) => {
                println!("error: {}", e);
            }
            
        }
    }
}
