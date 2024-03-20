use super::responses::*;
use crate::NoResponse;
use atat_derive::AtatCmd;
use serde_at::serde::Deserialize;

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+ROTATING_SPEED_GET", RotatingSpeedGetResponse)]
pub struct RotatingSpeedGet {
    pub sensor_index: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+ROTATING_SPEED_SET", NoResponse)]
pub struct RotatingSpeedSet {
    pub sensor_index: u8,
    pub rpm: i32,
    pub scale: i8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+CURRENT_ROTATING_SPEED_GET", RotatingSpeedGetResponse)]
pub struct CurrentRotatingSpeedGet {
    pub sensor_index: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+ROTATING_SPEED_CLEAR", NoResponse)]
pub struct RotatingSpeedClear {
    pub sensor_index: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+HERTZ_GET", HertzGetResponse)]
pub struct HertzGet {
    pub sensor_index: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+HERTZ_SET", NoResponse)]
pub struct HertzSet {
    pub sensor_index: u8,
    pub hertz: u32,
    pub scale: i8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+CURRENT_HERTZ_GET", HertzGetResponse)]
pub struct CurrentHertzGet {
    pub sensor_index: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+HERTZ_CLEAR", NoResponse)]
pub struct HertzClear {
    pub sensor_index: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+THROTTLE_GET", ThrottleGetResponse)]
pub struct ThrottleGet {
    pub sensor_index: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+THROTTLE_SET", NoResponse)]
pub struct ThrottleSet {
    pub sensor_index: u8,
    pub percentage: i32,
    pub scale: i8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+THROTTLE_CLEAR", NoResponse)]
pub struct ThrottleClear {
    pub sensor_index: u8,
}
