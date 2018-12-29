use std::io::prelude::*;
use std::net::*;

fn main() -> std::io::Result<()> {
    let mut tcp = TcpStream::connect("151.217.40.82:1234")?;

    tcp.write("SIZE\n".as_bytes())?;
    let mut buf = [0; 512];
    tcp.read(&mut buf)?;
    let response = String::from_utf8_lossy(&buf);
    println!("res: {}", response);

    for x in 0..1920 {
        for y in 0..1080 {
            std::thread::spawn(move || {
                if let Ok(mut tcp) = TcpStream::connect("151.217.40.82:1234") {
                    set_pixel(&mut tcp, x, y, 0, 255, 0);
                }
            });
        }
    }

    Ok(())
}

fn set_pixel(tcp: &mut TcpStream, x: u16, y: u16, r: u8, g: u8, b: u8) -> std::io::Result<()> {
    let msg = format!("PX {} {} {:02X}{:02X}{:02X}", x, y, r, g, b);
    println!("{}", msg);
    tcp.write(msg.as_bytes())?;
    tcp.flush()?;
    Ok(())
}
