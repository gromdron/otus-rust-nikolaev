#[derive(Debug, Clone, PartialEq)]
pub enum DeviceState {
    On,
    Off,
}

#[derive(Debug, Clone)]
pub struct SmartSocket {
    pub state: DeviceState,
    pub power: f32,
}

impl Default for SmartSocket {
    fn default() -> Self {
        SmartSocket {
            state: DeviceState::Off,
            power: 0.0,
        }
    }
}

impl SmartSocket {
    pub fn get_power(&self) -> f32 {
        self.power
    }

    pub fn get_state(&self) -> DeviceState {
        self.state.clone()
    }

    pub fn turn_on(&mut self) {
        self.state = DeviceState::On;
    }

    pub fn turn_off(&mut self) {
        self.state = DeviceState::Off;
    }

    pub fn get_report(&self) -> String {
        format!(
            "SmartSocket state: {:?}, current power: {:?}",
            self.get_state(),
            self.get_power()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_variables)]
    fn test_create() {
        let smrt_socket = SmartSocket {
            state: DeviceState::Off,
            power: 0.0,
        };
    }

    #[test]
    fn test_get_power() {
        let smrt_socket = SmartSocket {
            state: DeviceState::Off,
            power: 10.0,
        };

        assert_eq!(smrt_socket.get_power(), 10.0)
    }

    #[test]
    fn test_get_state() {
        let smrt_socket = SmartSocket {
            state: DeviceState::Off,
            power: 0.0,
        };

        assert_eq!(smrt_socket.get_state(), DeviceState::Off)
    }

    #[test]
    fn test_get_description() {
        let smrt_socket = SmartSocket {
            state: DeviceState::Off,
            power: 0.0,
        };

        assert_eq!(
            smrt_socket.get_report(),
            "SmartSocket state: Off, current power: 0.0".to_string()
        )
    }

    #[test]
    fn test_turn_on() {
        let mut smrt_socket = SmartSocket {
            state: DeviceState::Off,
            power: 10.0,
        };

        smrt_socket.turn_on();

        assert_eq!(smrt_socket.get_state(), DeviceState::On)
    }

    #[test]
    fn test_turn_off() {
        let mut smrt_socket = SmartSocket {
            state: DeviceState::On,
            power: 10.0,
        };

        smrt_socket.turn_off();

        assert_eq!(smrt_socket.get_state(), DeviceState::Off)
    }

    #[test]
    fn test_turn_double_toggle_check() {
        let mut smrt_socket = SmartSocket {
            state: DeviceState::Off,
            power: 10.0,
        };

        smrt_socket.turn_on();
        smrt_socket.turn_off();

        assert_eq!(smrt_socket.get_state(), DeviceState::Off)
    }
}
