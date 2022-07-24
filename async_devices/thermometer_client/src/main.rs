use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    }
};
use tokio::net::UdpSocket;
use std::io;
use tokio::time::{sleep, Duration};

#[derive(Default)]
struct Temperature(Mutex<f32>);

impl Temperature {
    pub fn get(&self) -> f32 {
        *self.0.lock().unwrap()
    }

    pub fn set(&self, val: f32) {
        *self.0.lock().unwrap() = val
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let bind_addr = &"127.0.0.1:34253";

    let socket = UdpSocket::bind(bind_addr).await.expect("couldn't bind to address");

    let finished = Arc::new(AtomicBool::new(false));
    let temperature = Arc::new(Temperature::default());

    let temperature_clone = temperature.clone();
    let finished_clone = finished.clone();

    tokio::spawn(async move { loop {
        if finished_clone.load(Ordering::SeqCst) {
            return;
        }

        let mut buf = [0; 5];

        socket.readable().await.unwrap();

        let mut received_len = 0;

        let buf_len = buf.len();
        while received_len < buf_len {
            match socket.recv_from(&mut buf[received_len..buf_len]).await {
                Ok((len, _)) => received_len += len,
                Err(error) => {
                    println!("Can't receive datagram: {:?}", error);
                    break;
                },
            }
        }

        let amt = buf.len();

        if amt != 5 {
            println!("incorrect datagram received");
            continue;
        }

        let char_symbol = buf[0].clone();

        match char::from(char_symbol) {
            't' => {
                let temp_array = [ buf[1], buf[2], buf[3], buf[4] ];
                let val = f32::from_be_bytes(temp_array);
                temperature_clone.set(val);
            },

            _ => {
                println!("incorrect datagram received")
            },
        }
    }});

    for _ in 0..120 {
        println!("Received: {:?}", temperature.get());
        sleep(Duration::from_secs(1)).await;
    }

    finished.store(true, Ordering::SeqCst);

    Ok(())
}
