use image::{DynamicImage, GenericImageView, Rgba};
use rand::{seq::SliceRandom, thread_rng};
use std::{io, io::prelude::*, net::TcpStream, thread, time::Instant};

const IP: &str = "127.0.0.1:1234";
const OFFSET: &str = "OFFSET 0 0";

fn main() {
    let image = match load_image() {
        Some(img) => img,
        None => return,
    };

    for _ in 0..9 {
        let data = build_commands(&image).into_bytes();
        thread::spawn(move || try_flut(&data));
    }

    let data = build_commands(&image).into_bytes();
    try_flut(&data)
}

fn load_image() -> Option<DynamicImage> {
    let image_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "image.png".to_string());

    match image::open(&image_path) {
        Ok(img) => Some(img),
        Err(e) => {
            println!("could not load {}: {:?}", image_path, e);
            None
        }
    }
}

fn build_commands(image: &DynamicImage) -> String {
    let (width, height) = image.dimensions();
    let mut pixels = (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .collect::<Vec<_>>();
    pixels.shuffle(&mut thread_rng());
    pixels
        .iter()
        .filter_map(|&(x, y)| {
            let Rgba([r, g, b, a]) = image.get_pixel(x, y);
            if a > 64 {
                Some(format!("PX {} {} {:02X}{:02X}{:02X}\n", x, y, r, g, b))
            } else {
                None
            }
        })
        .collect()
}

fn try_flut(data: &[u8]) {
    if let Err(e) = flut(data) {
        println!("could not flut: {:?}", e);
    }
}

fn flut(data: &[u8]) -> io::Result<()> {
    let mut tcp = TcpStream::connect(IP)?;
    tcp.write_all(OFFSET.as_bytes()).unwrap();

    println!("fluting...");
    loop {
        let start = Instant::now();
        if let Err(e) = tcp.write_all(data) {
            println!("{:?}, reconnecting", e);
            tcp = TcpStream::connect(IP)?;
            tcp.write_all(OFFSET.as_bytes()).unwrap();
        } else {
            println!("write took {:?}", start.elapsed());
        }
    }
}
