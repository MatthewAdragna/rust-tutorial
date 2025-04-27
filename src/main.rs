use std::env::args;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
fn main() {
    let args_in = parse_args();
    let formatted_ip = format!("{}:{}", args_in.ip, args_in.port);
    println!("Attempting to start server @ {formatted_ip}");
    let listener_tcp = TcpListener::bind(&formatted_ip).expect("Could not set up a Tcp Listener");

    println!("Web server started @{formatted_ip}");
    println!("Path of executable is {}", args_in.path);

    for stream in listener_tcp.incoming() {
        match stream {
            Ok(stream) => {
                handle_tcp_connection(stream);
                println!("Connection established");
            }
            _ => println!("A connection was attempted, but failed"),
        }
    }

    println!("Server closing down, goodbye!");
}
struct ArgsParsed {
    path: String,
    ip: String,
    port: i32,
}

fn handle_tcp_connection(mut tcp_stream: TcpStream) {
    let buf_reader = BufReader::new(&tcp_stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|monad| monad.unwrap_or_else(|_| String::from("")))
        .take_while(|line| !line.is_empty())
        .collect();
    let temp_response = "HTTP:/1.1 200 OK\r\n\r\n";
    // println!("Request: {http_req:#?}");
    tcp_stream.write_all(temp_response.as_bytes()).unwrap_or(());
}

fn parse_args() -> ArgsParsed {
    let arg_collect: Vec<String> = args().collect();
    let finished_struct = ArgsParsed {
        path: String::from(arg_collect.first().expect("Program path not found")),
        ip: match arg_collect.get(1) {
            None => "127.0.0.1".to_string(),
            Some(a) => a.clone(),
        },
        port: arg_collect
            .get(2)
            .map(|monad| monad.parse::<i32>().expect("Not a valid Port Number"))
            .unwrap_or_else(|| 7878),
    };
    finished_struct
}
