use super::responses::*;
use super::types::*;
use crate::{NoResponse, TrueFalseResponse};
use atat_derive::AtatCmd;
use serde_at::serde::Deserialize;

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MODBUS_UART", NoResponse)]
pub struct ModbusUartSetup {
    pub baud_rate: u32,
    pub data_bits: UartDataBits,
    pub parity: UartParity,
    pub stop_bits: UartStopBits,
    pub device_id: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MODBUS_UART=?", UartSetupResponse)]
pub struct ModbusUartSetupGet {}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MODBUS_ENABLED=?", TrueFalseResponse)]
pub struct ModbusEnabledGet {}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MODBUS_ENABLED", NoResponse)]
pub struct ModbusEnabledSet {
    pub enabled: bool,
}
