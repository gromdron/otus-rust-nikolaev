use std::fmt::Debug;
use crate::device::{Device, StoredDevice};

#[derive(Debug)]
pub struct Room<'a> {
    pub name: String,
    pub devices: Vec<StoredDevice<'a>>,
}

impl<'a> Room<'a> {
    pub fn add_device(&mut self, deivce_name: String, device: &'a dyn Device) -> bool {
        if let Some(_d) = self.devices.iter().find(|d| d.name == deivce_name) {
            false
        } else {
            self.devices.push(StoredDevice {
                name: deivce_name,
                device,
            });
            true
        }
    }
    #[allow(dead_code)]
    pub fn get_devices(&self) -> Vec<String> {
        self.devices.iter().map(|x| x.name.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    #[allow(unused_variables)]
    fn test_create_room() {
        let room = Room {
            name: "Test room".to_string(),
            devices: Vec::new(),
        };
    }

    #[test]
    fn test_add_device() {
        let mut room = Room {
            name: "Test room".to_string(),
            devices: Vec::new(),
        };

        let device = device::thermometer::Thermometer {
            state: device::DeviceState::Off,
            temperature: 0.0,
        };

        assert_eq!(room.add_device( "Thermometer".to_string(), &device ), true);
    }

    #[test]
    fn test_add_existed_device() {
        let mut room = Room {
            name: "Test room".to_string(),
            devices: Vec::new(),
        };

        let device1 = device::thermometer::Thermometer {
            state: device::DeviceState::Off,
            temperature: 0.0,
        };

        let device2 = device::thermometer::Thermometer {
            state: device::DeviceState::Off,
            temperature: 0.0,
        };

        room.add_device( "Thermometer".to_string(), &device1 );

        assert_eq!(room.add_device( "Thermometer".to_string(), &device2 ), false);
    }

    #[test]
    fn test_get_devices_for_empty_room() {

        let room = Room {
            name: "Test room".to_string(),
            devices: Vec::new(),
        };

        assert_eq!(room.get_devices(), Vec::<String>::new());
    }

    #[test]
    fn test_get_devices_for_non_empty_room() {
        let mut room = Room {
            name: "Test room".to_string(),
            devices: Vec::new(),
        };

        let device1 = device::thermometer::Thermometer {
            state: device::DeviceState::Off,
            temperature: 0.0,
        };

        room.add_device( "Thermometer".to_string(), &device1 );

        assert_eq!(room.get_devices(), vec!["Thermometer"]);
    }
}