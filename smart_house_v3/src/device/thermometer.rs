use std::fmt::Debug;
use crate::device::{Device, DeviceState};

#[derive(Debug)]
pub struct Thermometer {
    pub state: DeviceState,
    pub temperature: f32,
}

impl Thermometer {
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

impl Device for Thermometer {
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
        String::from("Thermometer. Can turn on and turn off. Collect temperature.")
    }
}

