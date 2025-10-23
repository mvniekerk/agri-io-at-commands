use crate::{NumberResponse, VALUES_SERIALIZE_OPTIONS};
use atat_derive::{AtatLen, AtatResp};
#[cfg(feature = "debug")]
use defmt::error;
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
    pub positive: bool,
    pub unit: u16,
    pub decimal: u8,
    pub decimal_len: u8,
    pub exponent: i8,
}

impl From<f32> for F32Response {
    fn from(value: f32) -> Self {
        let positive = value > 0.0;
        match serde_at::to_string::<_, 42>(&value, "", VALUES_SERIALIZE_OPTIONS) {
            Ok(s) => {
                let mut s = s.split('e');
                let val = s.next().unwrap_or("0.0");
                let exponent = s.next().unwrap_or("0");
                let exponent = exponent.parse::<i8>().unwrap_or(0);
                let mut val = val.split('.');
                let unit = val.next().unwrap_or("0");
                let unit = if positive { unit } else { &unit[1..] };
                let unit = unit.parse::<u16>().unwrap_or(0);
                let decimal = val.next().unwrap_or("0");
                let decimal_len = decimal.len() as u8;
                let decimal = decimal.parse::<u8>().unwrap_or(0);
                F32Response {
                    positive,
                    unit,
                    decimal,
                    decimal_len,
                    exponent,
                }
            }
            Err(_) => F32Response {
                positive: false,
                unit: 0,
                decimal: 0,
                decimal_len: 0,
                exponent: 0,
            },
        }
    }
}

impl From<&F32Response> for f32 {
    fn from(value: &F32Response) -> Self {
        let mut decimal = value.decimal as f32;
        decimal /= 10u32.pow(value.decimal_len as u32) as f32;

        let mut ret = value.unit as f32 + decimal;
        if value.exponent < 0 {
            ret /= 10u32.pow(value.exponent.abs() as u32) as f32;
        } else {
            ret *= 10u32.pow(value.exponent as i32 as u32) as f32;
        }
        if !value.positive {
            ret *= -1.0;
        }
        ret
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
        assert_eq!("+abc: true,1,1,2,0\r\nOK\r\n", s.as_str());
        let s = "+abc: true,1,1,2,0";
        let val: F32Response = serde_at::from_str(s).unwrap();
        let val: f32 = (&val).into();
        assert_eq!(1.01, val);

        let b: F32Response = 3.102e4f32.into();
        let b = b.to_vec_bytes_response("+abc").unwrap();
        let s = String::from_utf8(b).unwrap();
        assert_eq!("+abc: true,3,102,3,4\r\nOK\r\n", s.as_str());
        let s = "+abc: true,3,102,3,4";
        let val: F32Response = serde_at::from_str(s).unwrap();
        let val: f32 = (&val).into();
        assert_eq!(31.02e3, val);

        let b = 3.102e4f32 / 100_000_000.00 * -1.0;
        let b: F32Response = b.into();
        let b = b.to_vec_bytes_response("+abc").unwrap();
        let s = String::from_utf8(b).unwrap();
        assert_eq!("+abc: false,3,102,3,-4\r\nOK\r\n", s.as_str());
        let s = "+abc: false,3,102,3,-4";
        let val: F32Response = serde_at::from_str(s).unwrap();
        let val: f32 = (&val).into();
        assert_eq!(3.102e4f32 / 100_000_000.00 * -1.0, val);
    }
}
