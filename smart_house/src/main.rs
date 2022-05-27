#[derive(Debug, Clone)]
enum DeviceState {
    On,
    Off,
}

trait Device {
    fn get_state(&self) -> DeviceState;

    fn turn_on(&mut self);

    fn turn_off(&mut self);

    fn get_descripion(&self) -> String;
}

struct ElectricalOutlet {
    state: DeviceState,
    power: f32,
}

impl Device for ElectricalOutlet {
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
        String::from("Electrical outlet. Can turn on and turn off")
    }
}

impl Default for ElectricalOutlet {
    fn default() -> Self {
        Self {
            state: DeviceState::Off,
            power: 0.0,
        }
    }
}

impl ElectricalOutlet {
    fn get_power(&self) -> f32 {
        self.power
    }
}

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

impl Default for Thermometer {
    fn default() -> Self {
        Self {
            state: DeviceState::Off,
            temperature: 36.6,
        }
    }
}

impl Thermometer {
    fn get_temperature(&self) -> f32 {
        self.temperature
    }
}

fn main() {
    let mut electrical_outlet = ElectricalOutlet::default();

    println!("{}", electrical_outlet.get_descripion());
    println!("Initial power: {}", electrical_outlet.get_power());
    println!("Iitial state: {:#?}", electrical_outlet.get_state());

    electrical_outlet.turn_on();

    println!("After turn on state: {:#?}", electrical_outlet.get_state());

    let mut thermometer = Thermometer::default();

    println!("{}", thermometer.get_descripion());
    println!("Iitial state: {:#?}", thermometer.get_state());

    thermometer.turn_on();

    println!("After turn on state: {:#?}", thermometer.get_state());
    println!("Current temperature: {:#?}", thermometer.get_temperature());
}
