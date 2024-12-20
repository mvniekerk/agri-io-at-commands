use crate::{NoResponse, U8Response};
use atat_derive::AtatCmd;
use serde_at::serde::Deserialize;

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LED_SET_COLOR", NoResponse)]
pub struct SetColor {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LED_PULSE", NoResponse)]
pub struct Pulse {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub count: usize,
    pub on_time_ms: u64,
    pub off_time_ms: u64,
}


#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LED_INVERT=?", U8Response)]
pub struct LedInvertGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LED_INVERT", U8Response)]
pub struct LedInvertSet {
    pub value: u8,
}