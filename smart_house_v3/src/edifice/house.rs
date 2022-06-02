use std::fmt::Debug;
use crate::edifice::room::Room;

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
    pub fn get_room(&self, room_name: String) -> Result<&Room, String> {
        if let Some(d) = self.rooms.iter().find(|d| d.name == room_name) {
            Ok(d)
        } else {
            Err("Room not found".to_string())
        }
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
            devices: Vec::new()
        };

        assert_eq!(house.add_room( room ), true);
    }

    #[test]
    fn test_add_existed_room() {
        let mut house = House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        let room1 = Room {
            name: "Kitchen".to_string(),
            devices: Vec::new()
        };

        let room2 = Room {
            name: "Kitchen".to_string(),
            devices: Vec::new()
        };

        house.add_room( room1 );

        assert_eq!(house.add_room( room2 ), false);
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
            rooms: vec![
                Room {
                    name: "Kitchen".to_string(),
                    devices: Vec::new()
                },
            ],
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
            rooms: vec![
                Room {
                    name: "Kitchen".to_string(),
                    devices: Vec::new()
                },
            ],
        };

        assert_eq!(house.get_room("Kitchen".to_string()).is_ok(), true);
    }
}