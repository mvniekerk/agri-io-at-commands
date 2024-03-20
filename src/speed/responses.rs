use crate::NumberResponse;
use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct RotatingSpeedGetResponse {
    pub rpm: i32,
    pub scale: i8,
}

impl NumberResponse for RotatingSpeedGetResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct HertzGetResponse {
    pub hertz: u32,
    pub scale: i8,
}

impl NumberResponse for HertzGetResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct ThrottleGetResponse {
    pub percentage: i32,
    pub scale: i8,
}

impl NumberResponse for ThrottleGetResponse {}
