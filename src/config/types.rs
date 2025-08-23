use crate::general::types::PinOnOff;
use atat_derive::{AtatEnum, AtatLen};
use heapless::Vec;
use postcard::experimental::max_size::MaxSize;
use serde::{Deserialize, Serialize};
use serde_at::HexStr;

pub const MAX_STATE_MACHINES_FOR_PIN: usize = 2;
pub const MAX_AMOUNT_OF_PIN_STATES_PER_TRANSITION: usize = 4;
pub const MAX_AMOUNT_OF_PINS_PER_STATE_MACHINE: usize = 4;

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
    #[at_arg(value = 9)]
    AdcConfig(u8, u16, f32),
}

#[derive(Clone, Debug, AtatEnum, PartialEq, MaxSize, Copy)]
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

#[derive(Clone, Debug, AtatEnum, PartialEq, MaxSize, Copy)]
#[repr(u8)]
pub enum SensorDeviceType {
    #[at_arg(value = 0)]
    BoardPin,
    #[at_arg(value = 1)]
    MotorelliAd1000,
    #[at_arg(value = 2)]
    AbbAcs,
    #[at_arg(value = 3)]
    GenericModbus,
}

#[derive(Debug, Eq, PartialEq, Clone, AtatEnum, Hash, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Clone, AtatLen)]
pub struct AdcConfig {
    pub pin: u8,
    pub ref_0: u16,
    pub per_volt: HexStr<[u8; 4]>,
}

impl PartialEq for AdcConfig {
    fn eq(&self, other: &Self) -> bool {
        if self.pin == other.pin && self.ref_0 == other.ref_0 {
            let mut l =
                f32::from_le_bytes(self.per_volt.val) - f32::from_le_bytes(other.per_volt.val);
            if l < 0.0 {
                l *= -1.0;
            }
            l < f32::EPSILON
        } else {
            false
        }
    }
}
