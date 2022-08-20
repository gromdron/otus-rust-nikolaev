use crate::device::{Device, StoredDevice};
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Room<'a> {
    pub name: String,
    pub devices: Vec<StoredDevice<'a>>,
}

impl<'a> Room<'a> {
    pub fn add_device(
        &mut self,
        deivce_name: String,
        device: &'a dyn Device,
    ) -> Result<(), AddDeviceError> {
        if let Some(_d) = self.devices.iter().find(|d| d.name == deivce_name) {
            Err(AddDeviceError::AlreadyExist(deivce_name)))
        } else {
            self.devices.push(StoredDevice {
                name: deivce_name,
                device,
            });
            Ok(())
        }
    }
    pub fn remove_device(&mut self, device_name: String) -> Result<(), RemoveDeviceError> {
        let storable_device_info_position = self
            .devices
            .iter()
            .position(|storable_device| *storable_device.name == device_name);

        if let Some(position) = storable_device_info_position {
            self.devices.remove(position);
            Ok(())
        } else {
            Err(RemoveDeviceError::DeviceNotFound)
        }
    }
    #[allow(dead_code)]
    pub fn get_devices(&self) -> Vec<String> {
        self.devices.iter().map(|x| x.name.clone()).collect()
    }
}

#[derive(Error, Debug)]
pub enum AddDeviceError {
    #[error("Device with name `{0}` already exist")]
    AlreadyExist(String)
}

#[derive(Error, Debug)]
pub enum RemoveDeviceError {
    #[error("Room not found!")]
    DeviceNotFound,
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

        assert!(room.add_device("Thermometer".to_string(), &device).is_ok());
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

        room.add_device("Thermometer".to_string(), &device1)
            .unwrap();

        assert!(room
            .add_device("Thermometer".to_string(), &device2)
            .is_err());
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

        room.add_device("Thermometer".to_string(), &device1)
            .unwrap();

        assert_eq!(room.get_devices(), vec!["Thermometer"]);
    }

    #[test]
    fn test_remove_device() {
        let mut room = Room {
            name: "Test room".to_string(),
            devices: Vec::new(),
        };

        let device1 = device::thermometer::Thermometer {
            state: device::DeviceState::Off,
            temperature: 0.0,
        };

        room.add_device("Thermometer".to_string(), &device1)
            .unwrap();

        assert_eq!(room.get_devices(), vec!["Thermometer"]);

        assert!(room.remove_device("Thermometer".to_string()).is_ok());

        assert_eq!(room.get_devices(), Vec::<String>::new());
    }
}
