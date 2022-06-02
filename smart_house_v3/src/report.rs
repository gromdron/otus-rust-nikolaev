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