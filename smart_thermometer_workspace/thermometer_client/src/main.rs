use std::{
    error::Error,
    net::UdpSocket,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

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

fn main() -> Result<(), Box<dyn Error>> {
    let bind_addr = &"127.0.0.1:34253";

    let socket = UdpSocket::bind(bind_addr).expect("couldn't bind to address");
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;

    let finished = Arc::new(AtomicBool::new(false));
    let temperature = Arc::new(Temperature::default());

    let temperature_clone = temperature.clone();
    let finished_clone = finished.clone();

    thread::spawn(move || loop {
        if finished_clone.load(Ordering::SeqCst) {
            return;
        }

        let mut buf = [0; 4];
        if let Err(err) = socket.recv_from(&mut buf) {
            println!("can't receive datagram: {err}");
            continue;
        }

        let val = f32::from_be_bytes(buf);
        temperature_clone.set(val);
    });

    for _ in 0..120 {
        println!("Received: {:?}", temperature.get());
        thread::sleep(Duration::from_secs(1));
    }

    finished.store(true, Ordering::SeqCst);

    Ok(())
}
