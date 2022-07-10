use smart_socket_client::SocketClient;
use std::io;

pub trait State {
    fn update(&mut self, client: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error>;

    fn exit(&self) -> bool {
        false
    }
}

pub struct MainState;

impl State for MainState {
    fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
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

impl State for GetReportState {
    fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.get_report()?;
        println!("{}", socket_operation_result);

        Ok(Box::new(MainState))
    }
}

struct TurnOnState;

impl State for TurnOnState {
    fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.turn_on()?;
        println!("{}", socket_operation_result);

        Ok(Box::new(MainState))
    }
}

struct TurnOffState;

impl State for TurnOffState {
    fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.turn_off()?;
        println!("{}", socket_operation_result);

        Ok(Box::new(MainState))
    }
}

struct GetStateState;

impl State for GetStateState {
    fn update(&mut self, socket: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        let socket_operation_result = socket.get_state()?;
        println!("{}", socket_operation_result);

        Ok(Box::new(MainState))
    }
}

struct Exit;

impl State for Exit {
    fn update(&mut self, _: &mut SocketClient) -> Result<Box<dyn State>, anyhow::Error> {
        unreachable!()
    }

    fn exit(&self) -> bool {
        true
    }
}
