// Tokio basic HTTP server
// Tokio is opinionated async framework for IO application (Rayon is useful for CPU-bound applications)
// At certain point a simple application needs scaling, depending on app
// it can be either computation bound or IO bound.
// CPU-bound: Expensive operations that could be parallized and handled by multiple CPU cores
// IO-bound: Waiting on IO of other tasks such as database operations, file operations, network
// There are multiple ways to scale the application: threads, multi-processing, async
// There are few problems when data is exchanged or required to be communicated.
// One has to careful about deadlocks, race-conditions. This is where mutex, locking and semaphores or message passing could be helpful.
//
// Here we, will continue the series on creating a simple http server on port 7878

// Run project in separater terminal: cargo run
// In other terminal run benchmarking: oha --disable-keepalive -n 500 -z 1min http://localhost:7878
// oha assumes it can reuse the socket so we disable with keep-alive flag to avoid reusing connections.

use std::io::Result;

use tokio::{
    fs,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    time::{Duration, sleep},
};

async fn handle_client(mut stream: TcpStream) -> Result<()> {
    let request_line = {
        let buffer_reader = BufReader::new(&mut stream);

        // Handle None properly if not it throws following
        //  thread 'tokio-rt-worker' (492373) panicked at ..
        // called `Option::unwrap()` on a `None` value if we use the following
        // buffer_reader.lines().next_line().await.unwrap().unwrap()
        match buffer_reader.lines().next_line().await? {
            Some(line) => line,
            None => return Ok(()),
        }
    };
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5)).await;
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).await.unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Bind listener to the address
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    loop {
        // Handle
        let (stream, _) = listener.accept().await.unwrap();
        // A new task is spawned for each inbound socket. The socket is
        // moved to the new task and processed there.
        tokio::spawn(async move {
            if let Err(err) = handle_client(stream).await {
                eprintln!("connection error: {err}");
            };
        });
    }
}
