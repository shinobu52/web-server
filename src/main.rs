use std::{net::{TcpListener, TcpStream}, io::prelude::*, fs::File};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // TCP Streamを返却する
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "contents/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "contents/404.html")
    };

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
