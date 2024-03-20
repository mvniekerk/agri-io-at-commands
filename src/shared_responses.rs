use crate::NumberResponse;
use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U8Response {
    pub value: u8,
}
impl NumberResponse for U8Response {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U16Response {
    pub value: u16,
}
impl NumberResponse for U16Response {}
