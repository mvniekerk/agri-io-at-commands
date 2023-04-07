#![no_std]
#![feature(generic_const_exprs)]

use atat::AtatLen;
use atat_derive::{AtatEnum, AtatLen, AtatResp};
use defmt::error;
use heapless::Vec;
use serde::Serialize;
use serde_at::SerializeOptions;

pub mod config;
pub mod general;
pub mod led;
pub mod lora;
pub mod modbus;
pub mod motorelli;
pub mod urc;

pub trait NumberResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct NoResponse {}

impl NumberResponse for NoResponse {}

#[derive(Debug, Clone, AtatEnum, PartialEq)]
pub enum YesNo {
    #[at_arg(value = 0)]
    No,
    #[at_arg(value = 1)]
    Yes,
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct YesNoResponse {
    pub yes_no: YesNo,
}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct TrueFalseResponse {
    pub true_false: bool,
}

impl NumberResponse for TrueFalseResponse {}

#[derive(Debug, Clone, AtatResp, PartialEq, Serialize, AtatLen)]
pub struct ErrorResponse {
    pub error_group: u8,
    pub error_code: u8,
}

impl NumberResponse for ErrorResponse {}


impl<T> ToVecBytesResponse for T
where
    T: AtatLen + Serialize + NumberResponse,
    [(); T::LEN]:,
{
    fn to_vec_bytes_response(&self, cmd: &str) -> Result<Vec<u8, 1600>, ()> {
        let b =
            atat::serde_at::to_string::<_, { T::LEN }>(self, "", VALUES_SERIALIZE_OPTIONS).map_err(|_| {
                error!("Error serializing response");
                ()
            })?;
        <Self as ToVecBytesResponse>::wrap_response(cmd, b.as_bytes())
    }
}

impl ToVecBytesResponse for YesNoResponse {
    fn to_vec_bytes_response(&self, cmd: &str) -> Result<Vec<u8, 1600>, ()> {
        let b = atat::serde_at::to_string::<_, { <Self as AtatLen>::LEN }>(
            self,
            "",
            VALUES_SERIALIZE_OPTIONS,
        ).map_err(|_| {
            error!("Error serializing response");
            ()
        })?;
        <Self as ToVecBytesResponse>::wrap_response(cmd, b.as_bytes())
    }
}

pub trait ToVecBytesResponse {
    fn to_vec_bytes_response(&self, cmd: &str) -> Result<Vec<u8, 1600>, ()>;

    fn wrap_response(cmd: &str, vals: &[u8]) -> Result<Vec<u8, 1600>, ()> {
        let mut vec = Vec::new();
        vec.extend_from_slice(cmd.as_bytes())?;
        vec.extend_from_slice(": ".as_bytes())?;
        vec.extend_from_slice(vals)?;
        vec.extend_from_slice(b"OK\r\n")?;
        Ok(vec)
    }
}

impl ErrorResponse {
    pub fn error(error_group: u8, error_code: u8) -> Result<Vec<u8, 1600>, ()> {
        let err = Self {
            error_group,
            error_code,
        };
        err.to_vec_bytes_response("+ERR")
    }

    pub fn error_urc(error_group: u8, error_code: u8) -> Result<Vec<u8, 1600>, ()> {
        let err = Self {
            error_group,
            error_code,
        };

        err.to_vec_bytes_response("+ERR_URC")
    }
}

pub fn urc_on_string(cmd: &str) -> Result<Vec<u8, 1600>, ()> {
    let err = NoResponse {};
    err.to_vec_bytes_response(cmd)
}

pub const VALUES_SERIALIZE_OPTIONS: SerializeOptions = SerializeOptions {
    cmd_prefix: "",
    value_sep: false,
    termination: "\r\n",
    quote_escape_strings: false,
};
