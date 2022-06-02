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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_variables)]
    fn test_create() {
        let thermometer = Thermometer {
            state: DeviceState::Off,
            temperature: 0.0,
        };
    }

    #[test]
    fn test_get_power() {
        let thermometer = Thermometer {
            state: DeviceState::Off,
            temperature: 10.0,
        };

        assert_eq!(thermometer.get_temperature(), 10.0)
    }

    #[test]
    fn test_get_state() {
        let thermometer = Thermometer {
            state: DeviceState::Off,
            temperature: 0.0,
        };

        assert_eq!(thermometer.get_state(), DeviceState::Off)
    }

    #[test]
    fn test_get_description() {
        let thermometer = Thermometer {
            state: DeviceState::Off,
            temperature: 0.0,
        };

        assert_eq!(thermometer.get_descripion(), "Thermometer. Can turn on and turn off. Collect temperature.".to_string())
    }

    #[test]
    fn test_turn_on() {
        let mut thermometer = Thermometer {
            state: DeviceState::Off,
            temperature: 10.0,
        };

        thermometer.turn_on();

        assert_eq!(thermometer.get_state(), DeviceState::On)
    }

    #[test]
    fn test_turn_off() {
        let mut thermometer = Thermometer {
            state: DeviceState::On,
            temperature: 10.0,
        };

        thermometer.turn_off();

        assert_eq!(thermometer.get_state(), DeviceState::Off)
    }

    #[test]
    fn test_turn_double_toggle_check() {
        let mut thermometer = Thermometer {
            state: DeviceState::Off,
            temperature: 10.0,
        };

        thermometer.turn_on();
        thermometer.turn_off();

        assert_eq!(thermometer.get_state(), DeviceState::Off)
    }

}