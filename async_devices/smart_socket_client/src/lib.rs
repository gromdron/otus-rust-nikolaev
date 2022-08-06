use stp::client::{RequestResult, StpClient};
use stp::error::ConnectResult;
use tokio::net::ToSocketAddrs;

pub struct SocketClient {
    stp: StpClient,
}

impl SocketClient {
    pub async fn new<Addr: ToSocketAddrs>(addr: Addr) -> ConnectResult<Self> {
        let stp = StpClient::connect(addr).await?;
        Ok(Self { stp })
    }

    pub async fn get_state(&mut self) -> RequestResult {
        let request = "get_state|||".to_string();
        self.stp.send_request(request).await
    }

    pub async fn turn_on(&mut self) -> RequestResult {
        let request = "turn_on|||".to_string();
        self.stp.send_request(request).await
    }

    pub async fn turn_off(&mut self) -> RequestResult {
        let request = "turn_off|||".to_string();
        self.stp.send_request(request).await
    }

    pub async fn get_report(&mut self) -> RequestResult {
        let request = "get_report|||".to_string();
        self.stp.send_request(request).await
    }
}
