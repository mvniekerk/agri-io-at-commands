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

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct ModbusGenericValueOperationResponse {
    pub id: u8,
    pub value_id: u8,
    pub register: u16,
    pub left_shift: u8,
    pub right_shift: u8,
    pub divided_by: u16,
    pub multiplied_by: u16,
    pub mask: u16,
}

impl NumberResponse for ModbusGenericValueOperationResponse {}