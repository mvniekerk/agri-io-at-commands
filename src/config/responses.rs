use atat_derive::AtatResp;
use crate::config::types::{MeasurementConfigType, SensorDeviceType};
use crate::general::types::PinStateType;

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct MeasurementConfigGetResponse {
    pub config: MeasurementConfigType,
}

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct MeasurementConfigCountGetResponse {
    pub count: u8,
}

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct GpioPinConfigGetResponse {
    pub pin_index: u8,
    pub state: PinStateType,
    pub start_high: bool,
    pub internal_pull_up: bool,
    pub pin_type: SensorDeviceType,
}