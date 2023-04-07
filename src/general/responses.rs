use crate::general::types::PinStateType;
use crate::NumberResponse;
use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct PinStateGetResponse {
    pub pin: u8,
    pub state: PinStateType,
}

impl NumberResponse for PinStateGetResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct FirmwareVersionGetResponse {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub date: u64,
}

impl NumberResponse for FirmwareVersionGetResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use atat::nom::AsBytes;
    use core::str::FromStr;
    use heapless::String;
    use crate::ToVecBytesResponse;

    #[test]
    fn test_name_set() {
        let pin_state_get = PinStateGetResponse {
            pin: 1,
            state: PinStateType::ToOn,
        };
        let bytes = pin_state_get
            .to_vec_bytes_response("+PIN_STATE_GET")
            .unwrap();
        let expected = "+PIN_STATE_GET: 1,2\r\nOK\r\n";
        let utf8 = core::str::from_utf8(bytes.as_bytes()).unwrap();
        assert_eq!(utf8, expected);
    }
}
