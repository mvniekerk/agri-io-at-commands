use crate::general::types::PinOnOff;
use atat_derive::{AtatEnum, AtatLen};
use heapless::Vec;
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};

pub const MAX_STATE_MACHINES_FOR_PIN: usize = 2;
pub const MAX_AMOUNT_OF_PIN_STATES_PER_TRANSITION: usize = 4;

#[derive(Clone, Debug, AtatEnum, PartialEq, MaxSize)]
#[repr(u8)]
pub enum MeasurementConfigType {
    #[at_arg(value = 0)]
    WarnLow(f32),
    #[at_arg(value = 1)]
    WarnHigh(f32),
    #[at_arg(value = 2)]
    WaitFor(bool),
    #[at_arg(value = 3)]
    DontStart(f32),
    #[at_arg(value = 4)]
    HighGuard(bool),
    #[at_arg(value = 5)]
    LowGuard(bool),
    #[at_arg(value = 6)]
    OffHigh(f32),
    #[at_arg(value = 7)]
    OffLow(f32),
    #[at_arg(value = 8)]
    Scale(f32),
}

#[derive(Clone, Debug, AtatEnum, PartialEq, MaxSize)]
#[repr(u8)]
pub enum MeasurementConfigTypeRequest {
    #[at_arg(value = 0)]
    WarnLow,
    #[at_arg(value = 1)]
    WarnHigh,
    #[at_arg(value = 2)]
    WaitFor,
    #[at_arg(value = 3)]
    DontStart,
    #[at_arg(value = 4)]
    HighGuard,
    #[at_arg(value = 5)]
    LowGuard,
    #[at_arg(value = 6)]
    OffHigh,
    #[at_arg(value = 7)]
    OffLow,
    #[at_arg(value = 8)]
    Scale,
}

#[derive(Clone, Debug, AtatEnum, PartialEq, MaxSize)]
#[repr(u8)]
pub enum MeasurementSensorType {
    #[at_arg(value = 0)]
    Current,
    #[at_arg(value = 1)]
    FlowRate,
    #[at_arg(value = 2)]
    Kw,
    #[at_arg(value = 3)]
    Level,
    #[at_arg(value = 4)]
    Pressure,
    #[at_arg(value = 5)]
    Speed,
    #[at_arg(value = 6)]
    Volts,
}

#[derive(Clone, Debug, AtatEnum, PartialEq, MaxSize)]
#[repr(u8)]
pub enum SensorDeviceType {
    #[at_arg(value = 0)]
    BoardPin,
    #[at_arg(value = 1)]
    MotorelliAd1000,
    #[at_arg(value = 2)]
    AbbAcs,
}

#[derive(Debug, Eq, PartialEq, Clone, AtatEnum, Hash)]
pub enum GpioPinType {
    #[at_arg(value = 0)]
    BoardPin,
    #[at_arg(value = 1)]
    MotorelliAd1000,
    #[at_arg(value = 2)]
    AbbAcs,
    #[at_arg(value = 3)]
    Mcp23S17,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, MaxSize, AtatLen, Hash)]
pub struct GpioPinAtState {
    pub milliseconds: u8,
    pub value: bool,
    pub start_value: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, AtatLen, Hash)]
pub struct PinStateContainer {
    pub state_index: u8,
    pub state: PinOnOff,
    pub sequence: Vec<GpioPinAtState, MAX_AMOUNT_OF_PIN_STATES_PER_TRANSITION>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, AtatLen, Hash)]
pub struct GpioPinConfig {
    pub pin_index: u8,
    pub state: PinOnOff,
    pub start_high: bool,
    pub internal_pull_up: bool,
    pub pin_type: GpioPinType,
    pub at_state: Vec<PinStateContainer, MAX_STATE_MACHINES_FOR_PIN>,
}
