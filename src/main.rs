use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (response_status, file_path) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src/pages/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/pages/404.html")
    };

    let response_content = fs::read_to_string(file_path).unwrap();
    let response_content_len = response_content.len();

    let response = format!(
        "{response_status}\r\nContent-Length: {response_content_len}\r\n\r\n{response_content}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
