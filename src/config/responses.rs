use crate::config::types::{AdcConfig, MeasurementSensorType, SensorDeviceType};
use crate::general::types::PinOnOff;
use crate::NumberResponse;
use atat::heapless::String;
use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;
use serde_at::HexStr;

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct MeasurementConfigGetResponse {
    pub index: u8,
    pub id: u8,
    pub sensor_type: SensorDeviceType,
    pub measurement_sensor_type: MeasurementSensorType,
    pub warn_low: HexStr<[u8; 4]>,
    pub warn_high: HexStr<[u8; 4]>,
    pub wait_for: bool,
    pub dont_start: HexStr<[u8; 4]>,
    pub high_guard: bool,
    pub low_guard: bool,
    pub off_high: HexStr<[u8; 4]>,
    pub off_low: HexStr<[u8; 4]>,
    pub scale: HexStr<[u8; 4]>,
    pub adc_config: Option<AdcConfig>,
}

impl NumberResponse for MeasurementConfigGetResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct GpioPinConfigGetResponse {
    pub pin_index: u8,
    pub state: PinOnOff,
    pub start_high: bool,
    pub internal_pull_up: bool,
    pub pin_type: SensorDeviceType,
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct GpioPinAddOrSetResponse {
    pub index: u16,
}

impl NumberResponse for GpioPinAddOrSetResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct GpioPinAddOrSetStateResponse {
    pub pin_index: u16,
    pub state_index: u8,
}

impl NumberResponse for GpioPinAddOrSetStateResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct GpioPinAddOrSetPinAtStateResponse {
    pub pin_index: u16,
    pub state_index: u8,
    pub state: PinOnOff,
    pub at_state_index: u8,
}

impl NumberResponse for GpioPinAddOrSetPinAtStateResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct NameGetResponse {
    pub name: String<32>,
}

impl NumberResponse for NameGetResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct ConfigGetResponse {
    pub config: String<8192>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use atat::nom::AsBytes;
    use core::str::FromStr;
    use heapless::String;

    #[test]
    fn test_name_set() {
        let name = "TestName";
        let name = String::from_str(name).unwrap();
        let name_get_response = NameGetResponse { name: name };
        let bytes = name_get_response.to_vec_bytes_response("+NAME").unwrap();
        let expected = "+NAME: \"TestName\"\r\nOK\r\n";
        let utf8 = core::str::from_utf8(bytes.as_bytes()).unwrap();
        assert_eq!(utf8, expected);
    }

    #[test]
    fn test_json_string() {
        let config = r##"{"version":1,"app_key_1":15445943918857047555,"app_key_2":9976673127415750557,"dev_eui":7512461167133412109,"adr":false,"data_rate":5,"duration_between_sends":15,"pin_states":[{"pin_index":14,"state":"Off","start_high":false,"internal_pull_up":false,"at_state":[{"state_index":0,"state":"ToOff","sequence":[{"milliseconds":0,"value":false,"start_value":false}]},{"state_index":0,"state":"ToOn","sequence":[{"milliseconds":0,"value":true,"start_value":true}]}],"pin_type":"BoardPin"}],"modbus_uart_config":{"baud_rate":19200,"data_bits":8,"parity":0,"stop_bits":1,"device_id":1},"access_point_name":"GMETSILLAH","access_point_password":"K0si3Kw@Kker","wifi_enabled":false,"readings":{"current":{"sensors":[{"warn_low":0.0,"warn_high":0.0,"wait_for":false,"dont_start":0.0,"high_guard":false,"low_guard":false,"off_high":0.0,"off_low":0.0,"scale":1.0,"sensor_type":"MotorelliAd1000","sensor_id":0}]},"flow_rate":{"sensors":[]},"kw":{"sensors":[]},"pressure":{"sensors":[]},"speed":{"sensors":[]},"volts":{"sensors":[]}},"model":257,"send_state_every_seconds":30,"name":"TEST2"}"##;
        let config = String::from_str(config).unwrap();
        let response = ConfigGetResponse { config };
        let bytes = response.to_vec_bytes_response("_CONFIG_JSON").unwrap();
        let expected = "+CONFIG_JSON: {\"version\":1,\"app_key_1\":15445943918857047555,\"app_key_2\":9976673127415750557,\"dev_eui\":7512461167133412109,\"adr\":false,\"data_rate\":5,\"duration_between_sends\":15,\"pin_states\":[{\"pin_index\":14,\"state\":\"Off\",\"start_high\":false,\"internal_pull_up\":false,\"at_state\":[{\"state_index\":0,\"state\":\"ToOff\",\"sequence\":[{\"milliseconds\":0,\"value\":false,\"start_value\":false}]},{\"state_index\":0,\"state\":\"ToOn\",\"sequence\":[{\"milliseconds\":0,\"value\":true,\"start_value\":true}]}],\"pin_type\":\"BoardPin\"}],\"modbus_uart_config\":{\"baud_rate\":19200,\"data_bits\":8,\"parity\":0,\"stop_bits\":1,\"device_id\":1},\"access_point_name\":\"GMETSILLAH\",\"access_point_password\":\"K0si3Kw@Kker\",\"wifi_enabled\":false,\"readings\":{\"current\":{\"sensors\":[{\"warn_low\":0.0,\"warn_high\":0.0,\"wait_for\":false,\"dont_start\":0.0,\"high_guard\":false,\"low_guard\":false,\"off_high\":0.0,\"off_low\":0.0,\"scale\":1.0,\"sensor_type\":\"MotorelliAd1000\",\"sensor_id\":0}]},\"flow_rate\":{\"sensors\":[]},\"kw\":{\"sensors\":[]},\"pressure\":{\"sensors\":[]},\"speed\":{\"sensors\":[]},\"volts\":{\"sensors\":[]}},\"model\":257,\"send_state_every_seconds\":30,\"name\":\"TEST2\"}\r\n";
        let utf8 = core::str::from_utf8(bytes.as_bytes()).unwrap();
        assert_eq!(utf8, expected);
    }
}
