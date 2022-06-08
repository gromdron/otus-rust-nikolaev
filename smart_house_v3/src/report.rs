pub trait Report {
    fn get_report(&self) -> String;
}

impl<'a> Report for crate::edifice::house::House<'a> {
    fn get_report(&self) -> String {
        return format!(
            "{:?} report: \r\nExist {} room(s), \r\n{}",
            self.name,
            self.rooms.len(),
            self.rooms
                .iter()
                .map(|x| x.get_report() + "\r\n")
                .collect::<String>()
        );
    }
}

impl<'a> Report for crate::edifice::room::Room<'a> {
    fn get_report(&self) -> String {
        return format!(
            "Room {:?} has devices: \r\n{}",
            self.name,
            self.devices
                .iter()
                .map(|x| x.get_report() + "\r\n")
                .collect::<String>()
        );
    }
}

impl<'a> Report for crate::device::StoredDevice<'a> {
    fn get_report(&self) -> String {
        return format!(
            "Device {:?} state is: {}",
            self.name,
            self.device.get_report()
        );
    }
}

impl Report for crate::device::electrical_outlet::ElectricalOutlet {
    fn get_report(&self) -> String {
        return format!("{:?}, power: {:?}", self.state, self.get_power());
    }
}

impl Report for crate::device::thermometer::Thermometer {
    fn get_report(&self) -> String {
        return format!(
            "{:?}, temperature: {:?}",
            self.state,
            self.get_temperature()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_for_house() {
        let house = crate::edifice::house::House {
            name: "Without room".to_string(),
            rooms: Vec::new(),
        };

        assert_eq!(
            house.get_report(),
            "\"Without room\" report: \r\nExist 0 room(s), \r\n".to_string()
        )
    }

    #[test]
    fn test_text_for_house_with_room() {
        let mut house = crate::edifice::house::House {
            name: "With room".to_string(),
            rooms: Vec::new(),
        };

        let room = crate::edifice::room::Room {
            name: "Kitchen".to_string(),
            devices: Vec::new(),
        };

        house.add_room(room);

        assert_eq!(
            house.get_report(),
            "\"With room\" report: \r\nExist 1 room(s), \r\nRoom \"Kitchen\" has devices: \r\n\r\n"
                .to_string()
        )
    }

    #[test]
    fn test_text_for_room() {
        let room = crate::edifice::room::Room {
            name: "Kitchen".to_string(),
            devices: Vec::new(),
        };

        assert_eq!(
            room.get_report(),
            "Room \"Kitchen\" has devices: \r\n".to_string()
        )
    }

    #[test]
    fn test_text_for_stored_device() {
        let mut device = crate::device::electrical_outlet::ElectricalOutlet {
            state: crate::device::DeviceState::Off,
            power: 0.0,
        };

        let stored_device = crate::device::StoredDevice {
            name: "Stored device".to_string(),
            device: &mut device,
        };

        assert_eq!(
            stored_device.get_report(),
            "Device \"Stored device\" state is: Off, power: 0.0".to_string()
        )
    }

    #[test]
    fn test_text_for_electrical_outlet() {
        let device = crate::device::electrical_outlet::ElectricalOutlet {
            state: crate::device::DeviceState::On,
            power: 10.0,
        };

        assert_eq!(device.get_report(), "On, power: 10.0".to_string())
    }

    #[test]
    fn test_text_for_thermometer() {
        let device = crate::device::thermometer::Thermometer {
            state: crate::device::DeviceState::On,
            temperature: 232.78,
        };

        assert_eq!(device.get_report(), "On, temperature: 232.78".to_string())
    }
}
