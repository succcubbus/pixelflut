use std::{io::prelude::*, net::TcpStream, thread, time::Instant};

const IP: &str = "127.0.0.1:1234";
const OFFSET: &str = "OFFSET 800 950";

fn main() {
    let start = Instant::now();
    let data = build_data();
    let bytes = data.into_bytes();
    println!("data size: {}KiB", bytes.len() as f64 / 1024.0);
    println!("took {:?}", start.elapsed());

    let handles = (0..16).map(|_| flut(&bytes)).collect::<Vec<_>>();
    handles.into_iter().for_each(|h| h.join().unwrap());
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

fn open_connection() -> TcpStream {
    let mut tcp = TcpStream::connect(IP).expect("could not connect to flut server");
    tcp.write_all(OFFSET.as_bytes()).unwrap();
    tcp
}

fn flut(data: &[u8]) -> thread::JoinHandle<()> {
    let mine = data.to_owned();

    thread::spawn(move || {
        let mut tcp = open_connection();

        loop {
            if tcp.write_all(&mine).is_err() {
                println!("reconnect");
                tcp = open_connection();
            }
        }
    })
}
