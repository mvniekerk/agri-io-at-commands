use super::responses::*;
use super::types::*;
use crate::{NoResponse, TrueFalseResponse, U16Response, U8Response};
use atat_derive::AtatCmd;
use serde_at::serde::Deserialize;

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+MODBUS_UART", UartSetupResponse)]
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

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_DEBUG_ENABLED", TrueFalseResponse, timeout_ms = 4000)]
pub struct ModbusDebugEnabledSet {
    pub enabled: bool,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_DEBUG_ENABLED=?", TrueFalseResponse, timeout_ms = 4000)]
pub struct ModbusDebugEnabledGet {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_COIL_SET", NoResponse, timeout_ms = 4000)]
pub struct ModbusCoilSet {
    pub unit_id: u8,
    pub coil: u16,
    pub value: bool,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_COIL_GET", U8Response, timeout_ms = 4000)]
pub struct ModbusCoilGet {
    pub unit_id: u8,
    pub coil: u16,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_REGISTER_SET", NoResponse, timeout_ms = 4000)]
pub struct ModbusRegisterSet {
    pub unit_id: u8,
    pub register: u16,
    pub value: u16,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_REGISTER_GET", U16Response, timeout_ms = 4000)]
pub struct ModbusRegisterGet {
    pub unit_id: u8,
    pub register: u16,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_GENERIC_DEVICE_ADD", U8Response, timeout_ms = 4000)]
pub struct ModbusGenericAdd {
    pub unit_id: u8,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_GENERIC_DEVICE_REMOVE", U8Response, timeout_ms = 4000)]
pub struct ModbusGenericRemove {
    pub id: u8,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_GENERIC_VALUE_ADD", U8Response, timeout_ms = 4000)]
pub struct ModbusGenericValueAdd {
    pub id: u8,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_GENERIC_VALUE_REMOVE", NoResponse, timeout_ms = 4000)]
pub struct ModbusGenericValueRemove {
    pub id: u8,
    pub value_id: u8,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_GENERIC_VALUE_OPERATION_ADD", U8Response, timeout_ms = 4000)]
pub struct ModbusGenericValueOperationAdd {
    pub id: u8,
    pub value_id: u8,
    pub register: u16,
    pub left_shift: u8,
    pub right_shift: u8,
    pub divided_by: u16,
    pub multiplied_by: u16,
    pub mask: u16,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_GENERIC_VALUE_OPERATION_REMOVE", NoResponse, timeout_ms = 4000)]
pub struct ModbusGenericValueOperationRemove {
    pub id: u8,
    pub value_id: u8,
    pub operation_id: u8,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_GENERIC_VALUE_OPERATION_GET", ModbusGenericValueOperationResponse, timeout_ms = 4000)]
pub struct ModbusGenericValueOperationGet {
    pub id: u8,
    pub value_id: u8,
    pub operation_id: u8,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+MODBUS_GENERIC_VALUE_OPERATION_COUNT", U8Response, timeout_ms = 4000)]
pub struct ModbusGenericValueOperationCount {
    pub id: u8,
    pub value_id: u8
}
