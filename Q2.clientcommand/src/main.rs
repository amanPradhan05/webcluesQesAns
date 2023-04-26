use std::env;
use std::io::Write;
use std::net::TcpStream;
use std::io::Read;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./simple --mode=<cache|read>");
        std::process::exit(1);
    }

    let mode = args[1].as_str();
    match mode {
        "--mode=cache" => cache(),
        "--mode=read" => read(),
        _ => {
            println!("Invalid mode");
            std::process::exit(1);
        }
    }
}

fn cache() {
    let start_time = Instant::now();
    let mut data_points: Vec<f64> = Vec::new();
    while start_time.elapsed() < Duration::from_secs(10) {
     let mut stream = TcpStream::connect("127.0.0.1:8000").unwrap();
      let _ = stream.write(b"USD\n");
      let mut buf = [0; 1024];
      let size = stream.read(&mut buf).unwrap();
        let price = String::from_utf8_lossy(&buf[..size])
            .trim()
            .parse::<f64>()
            .unwrap();
        data_points.push(price);
    }

    let aggregate = data_points.iter().sum::<f64>() / data_points.len() as f64;
    let data_points_str = data_points.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    let result = format!("Aggregate: {}\nData points: {}", aggregate, data_points_str);
    println!("{}", result);
    std::fs::write("data.txt", result).unwrap();
}

fn read() {
    let result = std::fs::read_to_string("data.txt").unwrap();
    println!("{}", result);
}
