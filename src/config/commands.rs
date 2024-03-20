use super::responses::*;
use super::types::*;
use crate::general::types::PinOnOff;
use crate::NoResponse;
use atat::heapless::String;
use atat_derive::AtatCmd;
use postcard::experimental::max_size::MaxSize;
use serde_at::serde::Deserialize;

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_CONF_GET", MeasurementConfigGetResponse)]
pub struct MeasurementConfigGet {
    pub index: u8,
    pub sensor_type: MeasurementSensorType,
    pub config: MeasurementConfigTypeRequest,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_CONF", MeasurementConfigGetResponse, timeout_ms = 4000)]
pub struct MeasurementConfigSet {
    pub index: u8,
    pub sensor_type: MeasurementSensorType,
    pub config: MeasurementConfigType,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_CONF_COUNT", MeasurementConfigGetResponse)]
pub struct SensorConfigCountGet {
    pub sensor_type: MeasurementSensorType,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+ADD_SENSOR", NoResponse, timeout_ms = 4000)]
pub struct AddSensor {
    pub sensor_type: MeasurementSensorType,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+REMOVE_SENSOR", NoResponse, timeout_ms = 4000)]
pub struct RemoveSensor {
    pub index: u8,
    pub sensor_type: MeasurementSensorType,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_CONF_CLEAR", NoResponse, timeout_ms = 4000)]
pub struct MeasurementConfigClear {
    pub sensor_type: MeasurementSensorType,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_CONF_CLEAR_ALL", NoResponse, timeout_ms = 4000)]
pub struct MeasurementConfigClearAll {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+GPIO_CONF_GET", GpioPinConfigGetResponse)]
pub struct GpioPinConfigGet {
    pub pin_index: u8,
    pub pin_type: GpioPinType,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, MaxSize, AtatCmd)]
#[at_cmd("+GPIO_CONF", GpioPinConfigGetResponse, timeout_ms = 4000)]
pub struct GpioPinConfigSet {
    pub pin_index: u8,
    pub state: PinOnOff,
    pub start_high: bool,
    pub internal_pull_up: bool,
    pub pin_type: GpioPinType,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+NAME_GET=?", NameGetResponse, timeout_ms = 4000)]
pub struct NameGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+NAME", NameGetResponse, timeout_ms = 4000)]
pub struct NameSet {
    pub name: String<32>,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+CONFIG?", ConfigGetResponse, timeout_ms = 4000)]
pub struct ConfigGet {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+CONFIG", ConfigGetResponse, timeout_ms = 4000)]
pub struct ConfigSet {
    config: String<8096>,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+ADC_DEBUG", NoResponse, timeout_ms = 4000)]
pub struct AdcDebugEnable {}
