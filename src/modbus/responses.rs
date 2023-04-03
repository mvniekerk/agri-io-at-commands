use crate::modbus::types::{UartDataBits, UartParity, UartStopBits};
use crate::NumberResponse;
use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct UartSetupResponse {
    pub baudrate: u32,
    pub databits: UartDataBits,
    pub stopbits: UartStopBits,
    pub parity: UartParity,
    pub device_id: u8,
}

impl NumberResponse for UartSetupResponse {}
