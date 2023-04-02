use atat_derive::AtatResp;
use serde_at::HexStr;

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct AppKeyGetResponse {
    pub app_key: HexStr<u128>,
}

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct DevEuiGetResponse {
    pub dev_eui: HexStr<u64>,
}

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct AdrGetResponse {
    pub adr: bool,
}

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct DataRateGetResponse {
    pub data_rate: u8,
}

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct DurationBetweenSendsGetResponse {
    pub duration_between_sends: u8,
}