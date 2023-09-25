use super::responses::*;
use atat_derive::AtatCmd;
use serde_at::HexStr;
use serde_at::serde::Deserialize;
use crate::NoResponse;

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
    pub on: bool
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_FACTORY_RESET", NoResponse)]
pub struct LoraFactoryReset {}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_FORCE_TX_POWER", NoResponse)]
pub struct LoraForcePowerSet {
    pub db_m: u8
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_FORCE_TX_POWER=?", NoResponse)]
pub struct LoraForcePowerGet {
    pub db_m: u8
}

#[derive(Clone, Debug, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+LORA_POWER_TABLE", LoraPowerTable)]
pub struct LoraPowerTableGet {}


