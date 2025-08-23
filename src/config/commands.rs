use super::responses::*;
use super::types::*;
use crate::general::types::PinOnOff;
use crate::{
    F32Response, NoResponse, NumberResponse, TrueFalseResponse, U16HexResponse, U16Response,
    U8Response,
};
use atat::heapless::String;
use atat_derive::AtatCmd;
use serde_at::serde::Deserialize;
use serde_at::HexStr;

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_CONF_GET", MeasurementConfigGetResponse)]
pub struct MeasurementConfigGet {
    pub index: u8,
    pub sensor_type: MeasurementSensorType,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_CONF", MeasurementConfigGetResponse, timeout_ms = 4000)]
pub struct MeasurementConfigSet {
    pub index: u8,
    pub sensor_id: u8,
    pub sensor_type: MeasurementSensorType,
    pub sensor_device_type: SensorDeviceType,
    pub warn_low: HexStr<[u8; 4]>,
    pub warn_high: HexStr<[u8; 4]>,
    pub wait_for: bool,
    pub dont_start: HexStr<[u8; 4]>,
    pub high_guard: bool,
    pub low_guard: bool,
    pub off_high: HexStr<[u8; 4]>,
    pub off_low: HexStr<[u8; 4]>,
    pub scale: HexStr<[u8; 4]>,
    pub value_index: u8,
    pub adc_config: Option<AdcConfig>,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_CONF_COUNT", U8Response, timeout_ms = 4000)]
pub struct SensorConfigCountGet {
    pub sensor_type: MeasurementSensorType,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+ADD_SENSOR", U8Response, timeout_ms = 4000)]
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

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, AtatCmd)]
#[at_cmd("+GPIO_DEL", GpioPinAddOrSetResponse, timeout_ms = 4000)]
pub struct GpioPinConfigDelete {
    pub index: u16,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, AtatCmd)]
#[at_cmd("+GPIO_CLEAR_STATES", GpioPinAddOrSetResponse, timeout_ms = 4000)]
pub struct GpioPinConfigClearStates {
    pub index: u16,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, AtatCmd)]
#[at_cmd("+GPIO_COUNT", GpioPinAddOrSetResponse, timeout_ms = 4000)]
pub struct GpioCount {}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, AtatCmd)]
#[at_cmd("+GPIO_ADD", GpioPinAddOrSetResponse, timeout_ms = 4000)]
pub struct GpioPinConfigAdd {
    pub pin_index: u8,
    pub state: PinOnOff,
    pub start_high: bool,
    pub internal_pull_up: bool,
    pub pin_type: GpioPinType,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, AtatCmd)]
#[at_cmd("+GPIO_SET", GpioPinAddOrSetResponse, timeout_ms = 4000)]
pub struct GpioPinConfigSet {
    pub index: u16,
    pub gpio_pin_config: GpioPinConfig,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, AtatCmd)]
#[at_cmd("+GPIO_STATE_ADD", GpioPinAddOrSetStateResponse, timeout_ms = 4000)]
pub struct GpioPinStateAdd {
    pub pin_index: u16,
    pub state_index: u8,
    pub state: PinOnOff,
}
impl NumberResponse for GpioPinStateAdd {}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone, AtatCmd)]
#[at_cmd("+GPIO_PAS_ADD", GpioPinAddOrSetPinAtStateResponse, timeout_ms = 4000)]
pub struct GpioPinAtStateAdd {
    pub pin_index: u16,
    pub state_index: u8,
    pub state: PinOnOff,
    pub pin_at_state: GpioPinAtState,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+GPIO_CLEAR", NoResponse, timeout_ms = 4000)]
pub struct GpioClearConfig {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+PIN_STATES_COUNT", U16Response, timeout_ms = 4000)]
pub struct PinStatesCount {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MCP23S17_DEVICES_COUNT", U16Response, timeout_ms = 4000)]
pub struct McpDevicesCount {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+GPIO_DELAY_INIT_MS", U16Response, timeout_ms = 4000)]
pub struct GpioDelayInitMs {
    pub ms: u16,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+GPIO_DELAY_INIT_MS=?", U16Response, timeout_ms = 4000)]
pub struct GpioDelayInitMsGet {}

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

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+REINIT_PINS_STATE", NoResponse, timeout_ms = 4000)]
pub struct ReInitPinsState {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MCP23S17_DEBUG", TrueFalseResponse, timeout_ms = 4000)]
pub struct Mcp23S17Debug {
    pub enabled: bool,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MCP23S17_DEBUG=?", TrueFalseResponse, timeout_ms = 4000)]
pub struct Mcp23S17DebugGet {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MCP23S17_INIT_VALUE", U16HexResponse, timeout_ms = 4000)]
pub struct Mcp23S17InitValueSet {
    pub value: HexStr<u16>,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MCP23S17_INIT_VALUE=?", U16HexResponse, timeout_ms = 4000)]
pub struct Mcp23S17InitValueGet {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MCP23S17_LATCH_VALUE_GET", U16HexResponse, timeout_ms = 4000)]
pub struct Mcp23S17LatchValueGet {
    pub device_index: u8,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MCP23S17_LATCH_VALUE", U16HexResponse, timeout_ms = 4000)]
pub struct Mcp23S17LatchValueSet {
    pub device_index: u8,
    pub value: HexStr<u16>,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MCP23S17_REG_GET", U8Response, timeout_ms = 4000)]
pub struct Mcp23S17RegisterGet {
    pub device_index: u8,
    pub register: u8,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MCP23S17_REG_SET", U8Response, timeout_ms = 4000)]
pub struct Mcp23S17RegisterSet {
    pub device_index: u8,
    pub register: u8,
    pub value: u8,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_ADC_GET", U16Response, timeout_ms = 4000)]
pub struct MeasurementAdcGet {
    pub index: u8,
    pub sensor_type: MeasurementSensorType,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MEASUREMENT_VALUE_GET", F32Response, timeout_ms = 4000)]
pub struct MeasurementValueGet {
    pub index: u8,
    pub sensor_type: MeasurementSensorType,
}
