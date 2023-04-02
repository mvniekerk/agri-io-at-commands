use atat_derive::AtatResp;
use crate::modbus::types::{UartDataBits, UartParity, UartStopBits};

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct UartSetupResponse {
    pub baudrate: u32,
    pub databits: UartDataBits,
    pub stopbits: UartStopBits,
    pub parity: UartParity,
} 