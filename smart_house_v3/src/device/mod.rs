use std::fmt::Debug;
use crate::report;

#[derive(Debug, Clone)]
pub enum DeviceState {
    On,
    Off,
}

pub trait Device: Debug + report::Report {
    fn get_state(&self) -> DeviceState;

    fn get_descripion(&self) -> String;

    fn turn_on(&mut self);

    fn turn_off(&mut self);
}

#[derive(Debug)]
pub struct StoredDevice<'a> {
    pub name: String,
    pub device: &'a dyn Device,
}

pub mod electrical_outlet;
pub mod thermometer;