use rand::Rng;
use std::{net::UdpSocket, thread, time::Duration};

fn main() {
    let mut rng = rand::thread_rng();

    let bind_addr = &"127.0.0.1:34254";

    let dest_addr = &"127.0.0.1:34253";

    let socket = UdpSocket::bind(bind_addr).expect("couldn't bind to address");

    println!("Starting send temperature from {bind_addr} to {dest_addr}");

    loop {
        let temperature: f32 = rng.gen_range(18.0..=22.2);
        
        let bytes = ["t".as_bytes(), &temperature.to_be_bytes()].concat();

        let send_result = socket.send_to(&bytes, dest_addr);
        if let Err(err) = send_result {
            println!("can't send temperature: {err}")
        }
        println!("Done, {:?}", temperature);
        thread::sleep(Duration::from_secs(1))
    }
}
