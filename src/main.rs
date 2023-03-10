use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener,TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming().take(20) {
        let stream: TcpStream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream)
        });
    }
}

fn  handle_connection(mut stream : TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

   let (status_line,filename) = match &request_line[..] {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 Ok" , "fe/index.html"),
    "GET /sleep HTTP/1.1" => {
        thread::sleep(Duration::from_secs(20));
        ("HTTP/1.1 200 OK","fe/index.html")
    },
    _ => ("HTTP/1.1 404 NOT FOUND","fe/404.html"),   
    };      

    let contents: String = fs::read_to_string(filename).unwrap();
    let length: usize = contents.len();
    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}