use crate::config::types::MeasurementSensorType;
use crate::modbus::commands::ModbusGenericValueOperationAdd;
use crate::modbus::types::{GenericDeviceType, Operation, UartDataBits, UartParity, UartStopBits};
use crate::NumberResponse;
use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct ModbusGenericDeviceResponse {
    pub id: u8,
    pub unit_id: u8,
    pub device_type: GenericDeviceType,
}

impl NumberResponse for ModbusGenericDeviceResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct UartSetupResponse {
    pub baudrate: u32,
    pub databits: UartDataBits,
    pub stopbits: UartStopBits,
    pub parity: UartParity,
    pub device_id: u8,
    pub active_high: bool,
}

impl NumberResponse for UartSetupResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct ModbusGenericValueGetResponse {
    pub id: u8,
    pub value_id: u8,
    pub sensor_type: MeasurementSensorType,
}

impl NumberResponse for ModbusGenericValueGetResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct ModbusGenericValueOperationResponse {
    pub id: u8,
    pub value_id: u8,
    pub register_or_value: u16,
    pub is_register: bool,
    pub left_shift: u8,
    pub right_shift: u8,
    pub divided_by: u16,
    pub multiplied_by: u16,
    pub mask: u16,
    pub operation: Operation,
    pub is_coil: bool,
}

impl NumberResponse for ModbusGenericValueOperationResponse {}

impl From<&ModbusGenericValueOperationResponse> for ModbusGenericValueOperationAdd {
    fn from(value: &ModbusGenericValueOperationResponse) -> Self {
        let ModbusGenericValueOperationResponse {
            id,
            value_id,
            register_or_value,
            is_register,
            left_shift,
            right_shift,
            divided_by,
            multiplied_by,
            mask,
            operation,
            is_coil,
        } = value;

        ModbusGenericValueOperationAdd {
            id: *id,
            value_id: *value_id,
            register_or_value: *register_or_value,
            is_register: *is_register,
            left_shift: *left_shift,
            right_shift: *right_shift,
            divided_by: *divided_by,
            multiplied_by: *multiplied_by,
            mask: *mask,
            operation: *operation,
            is_coil: *is_coil,
        }
    }
}
