use async_trait::async_trait;
use smart_socket_client::SocketClient;
use std::io;

#[async_trait]
pub trait State {
    async fn update(&mut self, client: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error>;

    fn exit(&self) -> bool {
        false
    }
}

pub struct MainState;

#[async_trait]
impl State for MainState {
    async fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        println!(
            "Select option:
    1) Get report
    2) Turn on
    3) Turn off
    4) Get state
    Other) Exit"
        );
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;

        let selected = buf.trim();
        println!("Selected: {}", selected);

        match selected {
            "1" => Ok(Box::new(GetReportState)),
            "2" => Ok(Box::new(TurnOnState)),
            "3" => Ok(Box::new(TurnOffState)),
            "4" => Ok(Box::new(GetStateState)),
            _ => Ok(Box::new(Exit)),
        }
    }
}

struct GetReportState;

#[async_trait]
impl State for GetReportState {
    async fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.get_report().await?;
        println!("{}", socket_operation_result);

        Ok(Box::new(MainState))
    }
}

struct TurnOnState;

#[async_trait]
impl State for TurnOnState {
    async fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.turn_on().await?;
        println!("{}", socket_operation_result);

        Ok(Box::new(MainState))
    }
}

struct TurnOffState;

#[async_trait]
impl State for TurnOffState {
    async fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.turn_off().await?;
        println!("{}", socket_operation_result);

        Ok(Box::new(MainState))
    }
}

struct GetStateState;

#[async_trait]
impl State for GetStateState {
    async fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.get_state().await?;
        println!("{}", socket_operation_result);

        Ok(Box::new(MainState))
    }
}

struct Exit;

#[async_trait]
impl State for Exit {
    async fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        unreachable!()
    }

    fn exit(&self) -> bool {
        true
    }
}
