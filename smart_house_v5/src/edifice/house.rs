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
    pub fn add_room(&mut self, room: Room<'a>) -> Result<(), String> {
        if let Some(d) = self.rooms.iter().find(|d| d.name == room.name) {
            Err(format!("Room with name {:?} already exist", d.name.clone()))
        } else {
            self.rooms.push(room);
            Ok(())
        }
    }
    pub fn remove_room(&mut self, room_name: String) -> Result<(), String> {
        let room_index = self.rooms.iter().position(|room| *room.name == room_name);

        if let Some(position) = room_index {
            let room = self.rooms.get(position).unwrap();

            if !room.devices.is_empty() {
                Err("Room has devices. Remove devices before remove room".to_string())
            } else {
                self.rooms.remove(position);
                Ok(())
            }
        } else {
            Err("Room not found!".to_string())
        }
    }

    #[allow(dead_code)]
    pub fn get_room(&self, room_name: String) -> Option<&Room> {
        self.rooms.iter().find(|d| d.name == room_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(house.add_room(room).is_ok());
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

        house.add_room(room1).unwrap();

        assert!(house.add_room(room2).is_err());
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

        assert!(house.get_room("Kitchen".to_string()).is_none());
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

        assert!(house.get_room("Kitchen".to_string()).is_some());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_remove_room() {
        let mut house = House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        house.add_room(Room {
            name: "Kitchen".to_string(),
            devices: Vec::new(),
        });

        house.add_room(Room {
            name: "Bathroom".to_string(),
            devices: Vec::new(),
        });

        assert_eq!(house.rooms.len(), 2);

        assert!(house.remove_room("Bathroom".to_string()).is_ok());

        assert_eq!(house.rooms.len(), 1);
    }

    #[test]
    fn test_remove_unexisted_room() {
        let mut house = House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        assert!(house.remove_room("Bathroom".to_string()).is_err());
    }
}
