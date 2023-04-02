use atat_derive::AtatCmd;
use serde_at::serde::{Deserialize};
use super::types::*;
use super::responses::*;
use crate::{YesNoResponse, YesNo, NoResponse};

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MODBUS_UART", NoResponse)]
pub struct ModbusUartSetup {
    pub baud_rate: u32,
    pub data_bits: UartDataBits,
    pub stop_bits: UartStopBits,
    pub parity: UartParity,
    pub device_id: u8
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MODBUS_UART?", UartSetupResponse)]
pub struct ModbusUartSetupGet {}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MODBUS_ENABLED?", YesNoResponse)]
pub struct ModbusEnabledGet {}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MODBUS_ENABLED", NoResponse)]
pub struct ModbusEnabledSet {
    pub enabled: YesNo,
}