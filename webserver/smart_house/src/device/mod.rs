use crate::report;
use std::fmt::Debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeviceState {
    On,
    Off,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DeviceKind {
    ElectricalOutlet(electrical_outlet::ElectricalOutlet),
    Thermometer(thermometer::Thermometer)
}

pub trait Device: Debug + report::Report {
    fn get_state(&self) -> DeviceState;

    fn get_descripion(&self) -> String;

    fn turn_on(&mut self);

    fn turn_off(&mut self);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredDevice {
    pub name: String,
    pub device: DeviceKind,
}

pub mod electrical_outlet;
pub mod thermometer;
