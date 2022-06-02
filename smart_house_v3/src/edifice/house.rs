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
