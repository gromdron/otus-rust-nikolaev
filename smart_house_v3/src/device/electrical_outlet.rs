use std::fmt::Debug;
use crate::device::{Device, DeviceState};

#[derive(Debug)]
pub struct ElectricalOutlet {
    pub state: DeviceState,
    pub power: f32,
}

impl ElectricalOutlet {
    pub fn get_power(&self) -> f32 {
        self.power
    }
}

impl Device for ElectricalOutlet {
    fn get_state(&self) -> DeviceState {
        self.state.clone()
    }

    fn turn_on(&mut self) {
        self.state = DeviceState::On;
    }

    fn turn_off(&mut self) {
        self.state = DeviceState::Off;
    }

    fn get_descripion(&self) -> String {
        String::from("Electrical outlet. Can turn on and turn off")
    }
}