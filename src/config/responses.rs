use crate::config::types::{MeasurementConfigType, SensorDeviceType};
use crate::general::types::PinStateType;
use crate::ToVecBytesResponse;
use atat::heapless::String;
use atat::AtatLen;
use atat_derive::{AtatLen, AtatResp};
use heapless::Vec;
use serde::Serialize;
use serde_at::SerializeOptions;

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

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct NameGetResponse {
    pub name: String<32>,
}

impl ToVecBytesResponse for NameGetResponse {
    fn to_vec_bytes_response(&self, cmd: &str) -> Result<Vec<u8, 1600>, ()> {
        let options = SerializeOptions {
            cmd_prefix: "",
            value_sep: false,
            ..SerializeOptions::default()
        };
        let b =
            atat::serde_at::to_string::<_, { <Self as AtatLen>::LEN }>(self, "", options).unwrap();
        <Self as ToVecBytesResponse>::wrap_response(cmd, b.as_bytes())
    }
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
}
