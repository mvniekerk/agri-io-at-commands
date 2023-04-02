use atat_derive::AtatResp;
use crate::general::types::{PinStateType};

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct PinStateGetResponse {
    pub pin: u8,
    pub state: PinStateType,
}