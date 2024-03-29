pub mod device;
pub mod edifice;
pub mod report;

pub fn print_report<T: report::Report>(report_item: T) {
    println!("{}", report_item.get_report());
}

pub fn format_report<T: report::Report>(report_item: T) {
    format!("{}", report_item.get_report());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_report_for_house() {
        let house = edifice::house::House {
            name: "".to_string(),
            rooms: Vec::new(),
        };

        print_report(house);
    }

    #[test]
    fn test_print_report_for_room() {
        let room = edifice::room::Room {
            name: "".to_string(),
            devices: Vec::new(),
        };

        print_report(room);
    }

    #[test]
    fn test_print_report_for_stored_device() {
        let device = device::electrical_outlet::ElectricalOutlet {
            state: device::DeviceState::Off,
            power: 0.0,
        };

        let stored_device = device::StoredDevice {
            name: "Stored device".to_string(),
            device: device::DeviceKind::ElectricalOutlet(device),
        };

        print_report(stored_device);
    }

    #[test]
    fn test_text_from_print_report_for_electrical_outlet() {
        use crate::report::Report;

        let device = device::electrical_outlet::ElectricalOutlet {
            state: device::DeviceState::On,
            power: 10.0,
        };

        assert_eq!(device.get_report(), "On, power: 10.0".to_string())
    }

    #[test]
    fn test_text_from_print_report_for_thermometer() {
        use crate::report::Report;

        let device = device::thermometer::Thermometer {
            state: device::DeviceState::On,
            temperature: 232.78,
        };

        assert_eq!(device.get_report(), "On, temperature: 232.78".to_string())
    }

    #[test]
    fn test_remove_room_with_devices_will_fail() {
        let mut house = edifice::house::House {
            name: "Test house".to_string(),
            rooms: Vec::new(),
        };

        let mut room = edifice::room::Room {
            name: "Test room".to_string(),
            devices: Vec::new(),
        };

        let device1 = device::thermometer::Thermometer {
            state: device::DeviceState::Off,
            temperature: 0.0,
        };

        room.add_device("Thermometer".to_string(), device::DeviceKind::Thermometer(device1))
            .unwrap();

        assert!(house.remove_room("Test room".to_string()).is_err());
    }
}
