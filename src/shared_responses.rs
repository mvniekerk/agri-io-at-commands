use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;
use crate::NumberResponse;

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U8Response {
    pub value: u8,
}
impl NumberResponse for U8Response {}