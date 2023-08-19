use crate::NumberResponse;
use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;
use serde_at::HexStr;

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct AppKeyGetResponse {
    pub app_key: HexStr<u128>,
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct DevEuiGetResponse {
    pub dev_eui: HexStr<u64>,
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct AdrGetResponse {
    pub adr: bool,
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct DataRateGetResponse {
    pub data_rate: u8,
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct DurationBetweenSendsGetResponse {
    pub duration_between_sends: u8,
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct LoraPacketReceived {
    pub port: u8,
    pub data: HexStr<[u8; 243]>,
    pub length: u16,
    pub rssi: i32,
    pub snr: f32,
}

impl NumberResponse for AppKeyGetResponse {}
impl NumberResponse for DevEuiGetResponse {}
impl NumberResponse for AdrGetResponse {}
impl NumberResponse for DataRateGetResponse {}
impl NumberResponse for DurationBetweenSendsGetResponse {}
impl NumberResponse for LoraPacketReceived {}