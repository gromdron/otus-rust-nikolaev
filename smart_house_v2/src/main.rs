use std::fmt::Debug;

#[derive(Debug, Clone)]
enum DeviceState {
    On,
    Off,
}

#[derive(Debug)]
struct House<'a> {
    name: String,
    rooms: Vec<Room<'a>>,
}

impl<'a> House<'a> {
    #[allow(dead_code)]
    fn get_rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|x| x.name.clone()).collect()
    }
    fn add_room(&mut self, room: Room<'a>) -> bool {
        if let Some(_d) = self.rooms.iter().find(|d| d.name == room.name) {
            false
        } else {
            self.rooms.push(room);
            true
        }
    }
    #[allow(dead_code)]
    fn get_room(&self, room_name: String) -> Result<&Room, String> {
        if let Some(d) = self.rooms.iter().find(|d| d.name == room_name) {
            Ok(d)
        } else {
            Err("Device not found".to_string())
        }
    }
}

#[derive(Debug)]
struct Room<'a> {
    name: String,
    devices: Vec<StoredDevice<'a>>,
}

impl<'a> Room<'a> {
    fn add_device(&mut self, deivce_name: String, device: &'a dyn Device) -> bool {
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
    fn get_devices(&self) -> Vec<String> {
        self.devices.iter().map(|x| x.name.clone()).collect()
    }
}

#[derive(Debug)]
struct StoredDevice<'a> {
    name: String,
    device: &'a dyn Device,
}

trait Device: Debug + Report {
    fn get_state(&self) -> DeviceState;

    fn get_descripion(&self) -> String;

    fn turn_on(&mut self);

    fn turn_off(&mut self);
}

#[derive(Debug)]
struct ElectricalOutlet {
    state: DeviceState,
    power: f32,
}

impl ElectricalOutlet {
    fn get_power(&self) -> f32 {
        self.power
    }
}

impl Device for ElectricalOutlet {
    fn get_state(&self) -> DeviceState {
        self.state.clone()
    }

    fn get_descripion(&self) -> String {
        String::from("Electrical outlet. Can turn on and turn off")
    }

    fn turn_on(&mut self) {
        self.state = DeviceState::On;
    }

    fn turn_off(&mut self) {
        self.state = DeviceState::Off;
    }
}

#[derive(Debug)]
struct Thermometer {
    state: DeviceState,
    temperature: f32,
}

impl Device for Thermometer {
    fn get_state(&self) -> DeviceState {
        self.state.clone()
    }

    fn turn_on(&mut self) {
        self.state = DeviceState::On;
    }

    fn turn_off(&mut self) {
        self.state = DeviceState::Off;
    }

    fn get_descripion(&self) -> String {
        String::from("Thermometer. Can turn on and turn off. Collect temperature.")
    }
}

impl Thermometer {
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

trait Report {
    fn get_report(&self) -> String;
}

impl<'a> Report for House<'a> {
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

impl<'a> Report for Room<'a> {
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

impl<'a> Report for StoredDevice<'a> {
    fn get_report(&self) -> String {
        return format!(
            "Device {:?} state is: {}",
            self.name,
            self.device.get_report()
        );
    }
}

impl Report for ElectricalOutlet {
    fn get_report(&self) -> String {
        return format!("{:?}, power: {:?}", self.state, self.get_power());
    }
}

impl Report for Thermometer {
    fn get_report(&self) -> String {
        return format!(
            "{:?}, temperature: {:?}",
            self.state,
            self.get_temperature()
        );
    }
}

fn print_report<T: Report>(report_item: T) {
    println!("{}", report_item.get_report());
}

fn main() {
    let mut my_house = House {
        name: "My home".to_string(),
        rooms: Vec::new(),
    };

    let mut kitchen = Room {
        name: "Kitchen".to_string(),
        devices: Vec::new(),
    };

    let electrical_outlet_1 = ElectricalOutlet {
        state: DeviceState::Off,
        power: 0.0,
    };

    let electrical_outlet_2 = ElectricalOutlet {
        state: DeviceState::Off,
        power: 0.0,
    };

    kitchen.add_device("Near microwave".to_string(), &electrical_outlet_1);
    kitchen.add_device("Near teapot".to_string(), &electrical_outlet_2);

    my_house.add_room(kitchen);

    let mut bathroom = Room {
        name: "Bathroom".to_string(),
        devices: Vec::new(),
    };

    let bathroom_thermometer = Thermometer {
        state: DeviceState::Off,
        temperature: 0.0,
    };

    bathroom.add_device("Thermometer".to_string(), &bathroom_thermometer);

    my_house.add_room(bathroom);

    print_report(my_house);
}
