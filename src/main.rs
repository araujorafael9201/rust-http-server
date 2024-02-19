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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);

    let response_status = "HTTP/1.1 200 OK";
    let response_content = fs::read_to_string("src/pages/index.html").unwrap();
    let response_content_len = response_content.len();

    let response = format!(
        "{response_status}\r\nContent-Length: {response_content_len}\r\n\r\n{response_content}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
