use crate::device::Device;
use crate::edifice::room::Room;
use std::fmt::Debug;

#[derive(Debug)]
pub struct House<'a> {
    pub name: String,
    pub rooms: Vec<Room<'a>>,
}

impl<'a> House<'a> {
    #[allow(dead_code)]
    pub fn get_rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|x| x.name.clone()).collect()
    }
    pub fn add_room(&mut self, room: Room<'a>) -> bool {
        if let Some(_d) = self.rooms.iter().find(|d| d.name == room.name) {
            false
        } else {
            self.rooms.push(room);
            true
        }
    }
    #[allow(dead_code)]
    pub fn get_room(&self, room_name: String) -> Result<&Room<'a>, String> {
        if let Some(d) = self.rooms.iter().find(|d| d.name == room_name) {
            Ok(d)
        } else {
            Err("Room not found".to_string())
        }
    }
    #[allow(dead_code)]
    pub fn get_device_by_names(
        &mut self,
        room_name: String,
        device_name: String,
    ) -> Result<&mut dyn Device, String> {
        if let Some(room) = self.rooms.iter_mut().find(|r| r.name == room_name) {
            if let Some(d) = room.devices.iter_mut().find(|d| d.name == device_name) {
                Ok(d.device)
            } else {
                Err("Device not found".to_string())
            }
        } else {
            Err("Room not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    #[allow(unused_variables)]
    fn test_create_house() {
        let house = House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };
    }

    #[test]
    fn test_add_room() {
        let mut house = House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        let room = Room {
            name: "Kitchen".to_string(),
            devices: Vec::new(),
        };

        assert_eq!(house.add_room(room), true);
    }

    #[test]
    fn test_add_existed_room() {
        let mut house = House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        let room1 = Room {
            name: "Kitchen".to_string(),
            devices: Vec::new(),
        };

        let room2 = Room {
            name: "Kitchen".to_string(),
            devices: Vec::new(),
        };

        house.add_room(room1);

        assert_eq!(house.add_room(room2), false);
    }

    #[test]
    fn test_get_rooms_for_empty_house() {
        let house = House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        assert_eq!(house.get_rooms(), Vec::<String>::new());
    }

    #[test]
    fn test_get_rooms_for_non_empty_house() {
        let house = House {
            name: "Test house".to_string(),
            rooms: vec![Room {
                name: "Kitchen".to_string(),
                devices: Vec::new(),
            }],
        };

        assert_eq!(house.get_rooms(), vec!["Kitchen"]);
    }

    #[test]
    fn test_error_when_try_to_get_room_from_empty_house() {
        let house = House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        assert_eq!(house.get_room("Kitchen".to_string()).is_err(), true);
    }

    #[test]
    fn test_get_existed_room_from_non_empty_house() {
        let house = House {
            name: "Test house".to_string(),
            rooms: vec![Room {
                name: "Kitchen".to_string(),
                devices: Vec::new(),
            }],
        };

        assert_eq!(house.get_room("Kitchen".to_string()).is_ok(), true);
    }

    #[test]
    fn test_get_device_by_room_and_device_names() {
        let mut house = edifice::house::House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        let mut room = edifice::room::Room {
            name: "Kitchen".to_string(),
            devices: Vec::new(),
        };

        let mut thermometer = device::thermometer::Thermometer {
            state: device::DeviceState::On,
            temperature: 10.0,
        };

        room.add_device("Thermometer".to_string(), &mut thermometer);
        house.add_room(room);

        let device = house.get_device_by_names("Kitchen".to_string(), "Thermometer".to_string());
        assert_eq!(device.is_ok(), true);
    }

    #[test]
    fn test_turn_off_device_getted_by_name_and_print_report() {
        let mut house = edifice::house::House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        let mut room = edifice::room::Room {
            name: "Kitchen".to_string(),
            devices: Vec::new(),
        };

        let mut thermometer = device::thermometer::Thermometer {
            state: device::DeviceState::On,
            temperature: 10.0,
        };

        room.add_device("Thermometer".to_string(), &mut thermometer);
        house.add_room(room);

        let device = house.get_device_by_names("Kitchen".to_string(), "Thermometer".to_string());

        assert_eq!(device.is_ok(), true);

        let device = device.unwrap();

        assert_eq!(device.get_state(), device::DeviceState::On);

        device.turn_off();

        assert_eq!(device.get_state(), device::DeviceState::Off);
    }
}
