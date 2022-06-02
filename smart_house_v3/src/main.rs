use smart_house_v3::{edifice, device, print_report};

fn main() {
    let mut my_house = edifice::house::House {
        name: "My home".to_string(),
        rooms: Vec::new(),
    };

    let mut kitchen = edifice::room::Room {
        name: "Kitchen".to_string(),
        devices: Vec::new(),
    };

    let electrical_outlet_1 = device::electrical_outlet::ElectricalOutlet {
        state: device::DeviceState::Off,
        power: 0.0,
    };

    let electrical_outlet_2 = device::electrical_outlet::ElectricalOutlet {
        state: device::DeviceState::Off,
        power: 0.0,
    };

    kitchen.add_device("Near microwave".to_string(), &electrical_outlet_1);
    kitchen.add_device("Near teapot".to_string(), &electrical_outlet_2);

    my_house.add_room(kitchen);

    let mut bathroom = edifice::room::Room {
        name: "Bathroom".to_string(),
        devices: Vec::new(),
    };

    let bathroom_thermometer = device::thermometer::Thermometer {
        state: device::DeviceState::Off,
        temperature: 0.0,
    };

    bathroom.add_device("Thermometer".to_string(), &bathroom_thermometer);

    my_house.add_room(bathroom);

    print_report(my_house);
}