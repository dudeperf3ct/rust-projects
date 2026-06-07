use std::{
    fs::{self},
    io::{BufRead, BufReader, Result, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use basic_server_threadpool::ThreadPool;

fn handle_client(mut stream: TcpStream) {
    let buffer_reader = BufReader::new(&stream);

    let request_line: String = buffer_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let num_threads = 4;
    let mut pool = ThreadPool::new(num_threads);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_client(stream);
        })
    }

    Ok(())
}
