use super::responses::*;
use crate::{NoResponse, TrueFalseResponse, U16Response};
use atat_derive::AtatCmd;
use heapless::String;
use serde_at::serde::Deserialize;
use serde_at::HexStr;

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+APP_KEY=?", AppKeyGetResponse)]
pub struct AppKeyGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd(
    "+APP_KEY",
    AppKeyGetResponse,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
pub struct AppKeySet {
    pub app_key: HexStr<u128>,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+DEV_EUI=?", DevEuiGetResponse)]
pub struct DevEuiGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd(
    "+DEV_EUI",
    DevEuiGetResponse,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
pub struct DevEuiSet {
    pub dev_eui: HexStr<u64>,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+ADR=?", AdrGetResponse)]
pub struct AdrGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd(
    "+ADR",
    AdrGetResponse,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
pub struct AdrSet {
    pub adr: bool,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+DATA_RATE=?", DataRateGetResponse)]
pub struct DataRateGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd(
    "+DATA_RATE",
    DataRateGetResponse,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
pub struct DataRateSet {
    pub data_rate: u8,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+DURATION_BETWEEN_SENDS=?", DurationBetweenSendsGetResponse)]
pub struct DurationBetweenSendsGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd(
    "+DURATION_BETWEEN_SENDS",
    DurationBetweenSendsGetResponse,
    quote_escape_strings = false,
    timeout_ms = 4000
)]
pub struct DurationBetweenSendsSet {
    pub duration_between_sends: u8,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_DEBUG", NoResponse)]
pub struct LoraDebugSet {
    pub on: bool,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_FACTORY_RESET", NoResponse)]
pub struct LoraFactoryReset {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_FORCE_TX_POWER", NoResponse)]
pub struct LoraForcePowerSet {
    pub db_m: u8,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_FORCE_TX_POWER=?", NoResponse)]
pub struct LoraForcePowerGet {
    pub db_m: u8,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_POWER_TABLE", LoraPowerTable)]
pub struct LoraPowerTableGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_DISABLE=?", TrueFalseResponse)]
pub struct LoraDisableGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_DISABLE", TrueFalseResponse)]
pub struct LoraDisableSet {
    pub disable: bool,
}
#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_REBOOT_AFTER_TRIES=?", U16Response)]
pub struct LoraRebootAfterTriesGet {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_REBOOT_AFTER_TRIES", U16Response)]
pub struct LoraRebootAfterTriesSet {
    pub tries: u16,
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_START", NoResponse)]
pub struct LoraStart {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_FIRMWARE_VERSION", LoraFirmwareVersionResponse)]
pub struct LoraFirmwareVersion {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_DEBUG_CMDS", NoResponse)]
pub struct LoraDebugCmds {
    pub enabled: bool
}
