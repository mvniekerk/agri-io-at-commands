use super::responses::*;
use super::types::*;
use crate::NoResponse;
use atat_derive::AtatCmd;
use serde::Deserialize;

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+RESET", NoResponse)]
pub struct Reset {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+PIN_STATE_GET", PinStateGetResponse, timeout_ms = 4000)]
pub struct PinStateGet {
    pub pin: u8,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+PIN_STATE", PinStateGetResponse, timeout_ms = 4000)]
pub struct PinStateSet {
    #[at_arg(position = 0)]
    pub pin: u8,
    #[at_arg(position = 1)]
    pub state: PinStateType,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+SEND_STATE", NoResponse, timeout_ms = 4000)]
pub struct SendState {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+FIRMWARE_VERSION=?", FirmwareVersionGetResponse, timeout_ms = 4000)]
pub struct FirmwareVersionGet {}


#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+FACTORY_RESET", NoResponse, timeout_ms = 4000)]
pub struct FactoryReset {}

