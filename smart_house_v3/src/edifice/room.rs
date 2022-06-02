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