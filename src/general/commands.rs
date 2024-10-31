use super::responses::*;
use super::types::*;
use crate::{NoResponse, U8Response};
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
    pub state: PinOnOff,
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

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+RESET_FAULT_STATUS", NoResponse, timeout_ms = 4000)]
pub struct ResetFaultStatus {}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+SEQUENCE_PIN_LISTENING_GET", U8Response, timeout_ms = 4000)]
pub struct SequencePinListeningGet {
    pub pin: PinSequenceListeners
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+SEQUENCE_PIN_LISTENING_SET", U8Response, timeout_ms = 4000)]
pub struct SequencePinListeningSet {
    pub pin: PinSequenceListeners,
    pub value: bool
}

#[derive(Clone, Debug, AtatCmd, PartialEq, Deserialize)]
#[at_cmd("+SEQUENCE_PIN_LISTENING_START", U8Response, timeout_ms = 4000)]
pub struct SequencePinListeningStart {
    pub pin: PinSequenceListeners
}