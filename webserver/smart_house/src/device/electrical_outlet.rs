use crate::device::{Device, DeviceState};
use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_variables)]
    fn test_create() {
        let electrical_outlet = ElectricalOutlet {
            state: DeviceState::Off,
            power: 0.0,
        };
    }

    #[test]
    fn test_get_power() {
        let electrical_outlet = ElectricalOutlet {
            state: DeviceState::Off,
            power: 10.0,
        };

        assert_eq!(electrical_outlet.get_power(), 10.0)
    }

    #[test]
    fn test_get_state() {
        let electrical_outlet = ElectricalOutlet {
            state: DeviceState::Off,
            power: 0.0,
        };

        assert_eq!(electrical_outlet.get_state(), DeviceState::Off)
    }

    #[test]
    fn test_get_description() {
        let electrical_outlet = ElectricalOutlet {
            state: DeviceState::Off,
            power: 0.0,
        };

        assert_eq!(
            electrical_outlet.get_descripion(),
            "Electrical outlet. Can turn on and turn off".to_string()
        )
    }

    #[test]
    fn test_turn_on() {
        let mut electrical_outlet = ElectricalOutlet {
            state: DeviceState::Off,
            power: 10.0,
        };

        electrical_outlet.turn_on();

        assert_eq!(electrical_outlet.get_state(), DeviceState::On)
    }

    #[test]
    fn test_turn_off() {
        let mut electrical_outlet = ElectricalOutlet {
            state: DeviceState::On,
            power: 10.0,
        };

        electrical_outlet.turn_off();

        assert_eq!(electrical_outlet.get_state(), DeviceState::Off)
    }

    #[test]
    fn test_turn_double_toggle_check() {
        let mut electrical_outlet = ElectricalOutlet {
            state: DeviceState::Off,
            power: 10.0,
        };

        electrical_outlet.turn_on();
        electrical_outlet.turn_off();

        assert_eq!(electrical_outlet.get_state(), DeviceState::Off)
    }
}
