use atat_derive::AtatCmd;
use super::types::*;
use super::responses::*;
use crate::{YesNoResponse, YesNo, NoResponse};

#[derive(Debug, Clone, AtatCmd)]
#[at_cmd("+MODBUS_UART", NoResponse)]
pub struct ModbusUartSetup {
    pub baudrate: u32,
    pub databits: UartDataBits,
    pub stopbits: UartStopBits,
    pub parity: UartParity,
}

#[derive(Debug, Clone, AtatCmd)]
#[at_cmd("+MODBUS_UART?", UartSetupResponse)]
pub struct ModbusUartSetupGet {}

#[derive(Debug, Clone, AtatCmd)]
#[at_cmd("+MODBUS_ENABLED?", YesNoResponse)]
pub struct ModbusEnabledGet {}

#[derive(Debug, Clone, AtatCmd)]
#[at_cmd("+MODBUS_ENABLED", NoResponse)]
pub struct ModbusEnabledSet {
    pub enabled: YesNo,
}