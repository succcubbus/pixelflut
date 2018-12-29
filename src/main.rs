use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

const ip: &str = "151.217.40.82:1234";
const offset: &str = "OFFSET 800 950";

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let data = build_data();
    let bytes = data.into_bytes();
    println!("data size: {}KiB", bytes.len() as f64 / 1024.0);
    println!("took {:?}", start.elapsed());

    for _ in 0..16 {
        flut(&bytes);
    }

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

fn build_data() -> String {
    let r = 255;
    let g = 0;
    let b = 192;

    (0..100)
        .flat_map(|x| (0..100).map(move |y| (x, y)))
        .map(|(x, y)| format!("PX {} {} {:02X}{:02X}{:02X}\n", x, y, r, g, b))
        .collect::<Vec<_>>()
        .join("")
}

fn flut(data: &Vec<u8>) -> JoinHandle<()> {
    let mine = data.clone();

    thread::spawn(move || {
        let mut tcp = TcpStream::connect(ip).unwrap();
        tcp.write(offset.as_bytes());

        loop {
            if let Err(e) = tcp.write(&mine) {
                println!("reconnect");
                tcp = TcpStream::connect(ip).unwrap();
                tcp.write(offset.as_bytes());
            }
        }
    })
}
