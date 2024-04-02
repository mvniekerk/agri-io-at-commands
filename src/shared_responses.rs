use crate::NumberResponse;
use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;
use serde_at::HexStr;

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U8Response {
    pub value: u8,
}
impl NumberResponse for U8Response {}

impl From<u8> for U8Response {
    fn from(value: u8) -> Self {
        Self {
            value
        }
    }
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U16Response {
    pub value: u16,
}
impl NumberResponse for U16Response {}

impl From<u16> for U16Response {
    fn from(value: u16) -> Self {
        U16Response {
            value
        }
    }
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U16HexResponse {
    pub value: HexStr<u16>,
}

impl From<u16> for U16HexResponse {
    fn from(val: u16) -> Self {
        let mut value = HexStr::default();
        value.val = val;
        Self {
            value
        }
    }
}

impl NumberResponse for U16HexResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct BoolResponse {
    pub value: bool,
}
impl NumberResponse for BoolResponse {}
