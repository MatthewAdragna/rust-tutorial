use std::env::args;
use std::net::TcpListener;
fn main() {
    let args_in = parse_args();
    let formatted_ip = format!("{}:{}", args_in.ip, args_in.port);
    let listener_tcp = TcpListener::bind(&args_in.ip).expect("Could not set up a Tcp Listener");

    println!("Web server started @{formatted_ip}");
    println!("Path of executable is {}", args_in.path);

    for stream in listener_tcp.incoming() {
        let stream = stream.unwrap();
    }
    println!("Finished recieving packets, goodbye!");
}
struct ArgsParsed {
    path: String,
    ip: String,
    port: i32,
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
