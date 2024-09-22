use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use webserver::ThreadPool;

// use crate::ThreadPool;
fn main() {
    let server = TcpListener::bind("127.0.0.1:9090")
                            .expect("Ensure port 9090 is not in use");

    let mut pool: ThreadPool = ThreadPool::new(6);

    for stream in server.incoming().take(2) {
        let mut stream = stream.unwrap();

        println!("Started server on 127.0.0.1:9090");

        pool.execute(|| handle(&mut stream));

        println!("Server Exiting!!!");
    }

}

fn handle(mut stream: &mut TcpStream) {
    let client = stream.peer_addr().unwrap();
    println!("Connection accepted from : {:?}", client);

    // let mut request = String::new();
    // stream.read_to_string(&mut request).expect("Could not read request");

    let request: String = BufReader::new(&mut stream)
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>()
        .join("\n");

    println!("[{client}]: {request}");

    let header = "HTTP/1.1 200 OK";
    let content = "Hi guys";

    let response = format!("{header}\r\nContent-Length: {}\r\n\r\n{content}", content.len());
    stream.write_all(response.as_bytes()).expect("Ensure client can accept connection back");
}
