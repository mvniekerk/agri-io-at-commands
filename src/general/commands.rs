use atat_derive::AtatCmd;
use serde_at::serde::{Deserialize};
use super::responses::*;
use super::types::*;
use crate::NoResponse;

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
    pub pin: u8,
    pub state: PinStateType,
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+SEND_STATE", NoResponse, timeout_ms = 4000)]
pub struct SendState {}
