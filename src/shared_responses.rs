use crate::NumberResponse;
use atat_derive::{AtatLen, AtatResp};
use core::fmt::Write;
use core::str::FromStr;
#[cfg(feature = "debug")]
use defmt::error;
use heapless::String;
use serde::Serialize;
use serde_at::HexStr;

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U8Response {
    pub value: u8,
}
impl NumberResponse for U8Response {}

impl From<u8> for U8Response {
    fn from(value: u8) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U16Response {
    pub value: u16,
}
impl NumberResponse for U16Response {}

impl From<u16> for U16Response {
    fn from(value: u16) -> Self {
        U16Response { value }
    }
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U16HexResponse {
    pub value: HexStr<u16>,
}

impl From<u16> for U16HexResponse {
    fn from(val: u16) -> Self {
        let mut value = HexStr::default();
        value.val = val;
        Self { value }
    }
}

impl NumberResponse for U16HexResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct BoolResponse {
    pub value: bool,
}
impl NumberResponse for BoolResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct U32Response {
    pub value: u32,
}
impl NumberResponse for U32Response {}

impl From<u32> for U32Response {
    fn from(value: u32) -> Self {
        U32Response { value }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, AtatResp, AtatLen)]
pub struct F32Response {
    pub value: String<50>,
}

pub fn f32_to_string(f: f32) -> String<50> {
    let mut s: String<50> = String::new();
    if write!(&mut s, "\"{}\"", f).is_err() {
        return String::from_str("\"0.0\"").unwrap_or_default();
    }
    s
}

impl From<f32> for F32Response {
    fn from(value: f32) -> Self {
        let value = f32_to_string(value);
        Self { value }
    }
}

impl From<&F32Response> for f32 {
    fn from(value: &F32Response) -> Self {
        let value: f32 = value.value.parse().unwrap_or(0.0);
        value
    }
}

impl NumberResponse for F32Response {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ToVecBytesResponse;
    use heapless::String;

    #[test]
    fn f32_response_parsing() {
        let b: F32Response = 1.01f32.into();
        let b = b.to_vec_bytes_response("+abc").unwrap();
        let s = String::from_utf8(b).unwrap();
        assert_eq!("+abc: \"1.01\"\r\nOK\r\n", s.as_str());
        let s = "+abc: \"1.01\"";
        let val: F32Response = serde_at::from_str(s).unwrap();
        let val: f32 = (&val).into();
        assert_eq!(1.01, val);

        let b: F32Response = 3.102e4f32.into();
        let b = b.to_vec_bytes_response("+abc").unwrap();
        let s = String::from_utf8(b).unwrap();
        assert_eq!("+abc: \"31020\"\r\nOK\r\n", s.as_str());
        let s = "+abc: \"31020\"";
        let val: F32Response = serde_at::from_str(s).unwrap();
        let val: f32 = (&val).into();
        assert_eq!(31.02e3, val);

        let b = 3.102e4f32 / 100_000_000.00 * -1.0;
        let b: F32Response = b.into();
        let b = b.to_vec_bytes_response("+abc").unwrap();
        let s = String::from_utf8(b).unwrap();
        assert_eq!("+abc: \"-0.0003102\"\r\nOK\r\n", s.as_str());
        let s = "+abc: \"-0.0003102\"";
        let val: F32Response = serde_at::from_str(s).unwrap();
        let val: f32 = (&val).into();
        assert_eq!(3.102e4f32 / 100_000_000.00 * -1.0, val);
    }
}
