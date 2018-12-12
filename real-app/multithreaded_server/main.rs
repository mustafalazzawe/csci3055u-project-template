extern crate hello;

use hello::ThreadPool;

use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::thread;
use std::time::Duration;


fn main() {
    //listen for TCP coneections
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        //terminate if stream has any errors
        let stream = stream.unwrap();

        //println!("Connection established!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    
    println!("Shutting down.");
}

//read data from the TCP stream and print it so we can see the data being sent from the browser
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    //read bytes from the TCPStream and put them in the buffer
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    //holds the success message's data
    let response = format!("{}{}", status_line, contents);

    //as_bytes() - convert the string data to bytes
    //write - takes a &[u8] and sends those bytes directly down the connection.
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
