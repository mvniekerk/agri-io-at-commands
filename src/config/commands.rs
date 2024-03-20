use super::responses::*;
use super::types::*;
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
#[at_cmd("+GPIO_GET", GpioPinConfigGetResponse)]
pub struct GpioPinConfigGet {
    pub pin_index: u16,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, MaxSize, AtatCmd)]
#[at_cmd("+GPIO_DEL", GpioPinAddOrSetResponse, timeout_ms = 4000)]
pub struct GpioPinConfigDelete {
    pub index: u16,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, MaxSize, AtatCmd)]
#[at_cmd("+GPIO_CLEAR_STATES", GpioPinAddOrSetResponse, timeout_ms = 4000)]
pub struct GpioPinConfigClearStates {
    pub index: u16,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, MaxSize, AtatCmd)]
#[at_cmd("+GPIO_ADD", GpioPinAddOrSetResponse, timeout_ms = 4000)]
pub struct GpioPinConfigAdd {
    pub gpio_pin_config: GpioPinConfig,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, MaxSize, AtatCmd)]
#[at_cmd("+GPIO_SET", GpioPinAddOrSetResponse, timeout_ms = 4000)]
pub struct GpioPinConfigSet {
    pub index: u16,
    pub gpio_pin_config: GpioPinConfig,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, MaxSize, AtatCmd)]
#[at_cmd("+GPIO_STATE_ADD", GpioPinAddOrSetStateResponse, timeout_ms = 4000)]
pub struct GpioPinStateAdd {
    pub pin_index: u16,
    pub pin_state: PinStateContainer,
}
#[derive(Deserialize, Debug, Eq, PartialEq, Clone, MaxSize, AtatCmd)]
#[at_cmd("+GPIO_PAS_ADD", GpioPinAddOrSetPinAtStateResponse, timeout_ms = 4000)]
pub struct GpioPinAtStateAdd {
    pub pin_index: u16,
    pub state_index: u8,
    pub pin_at_state: GpioPinAtState,
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
