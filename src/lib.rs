#![no_std]
#![feature(generic_const_exprs)]
extern crate alloc;

use atat::AtatLen;
use atat_derive::{AtatEnum, AtatLen, AtatResp};
#[cfg(feature = "debug")]
use defmt::error;
use heapless::{String, Vec};
use serde::Serialize;
use serde_at::SerializeOptions;

pub mod config;
pub mod general;
pub mod led;
pub mod lora;
pub mod modbus;
pub mod motorelli;
pub mod sequence;
pub mod shared_responses;
pub mod speed;
pub mod urc;

pub use shared_responses::{F32Response, U16HexResponse, U16Response, U32Response, U8Response};

pub trait NumberResponse {}

pub const BUFFER_SIZE_IN_BYTES: usize = 400;

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
    pub true_false: String<5>,
}

impl From<bool> for TrueFalseResponse {
    fn from(b: bool) -> Self {
        let mut s = String::new();
        if b {
            s.push_str("true").unwrap();
        } else {
            s.push_str("false").unwrap();
        }
        Self { true_false: s }
    }
}

impl From<TrueFalseResponse> for bool {
    fn from(value: TrueFalseResponse) -> Self {
        value.true_false == "true"
    }
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
    fn to_vec_bytes_response(&self, cmd: &str) -> Result<Vec<u8, BUFFER_SIZE_IN_BYTES>, ()> {
        let b = atat::serde_at::to_string::<_, { T::LEN }>(self, "", VALUES_SERIALIZE_OPTIONS)
            .map_err(|_| {
                #[cfg(feature = "debug")]
                error!("Error serializing response");
                ()
            })?;
        <Self as ToVecBytesResponse>::wrap_response(cmd, b.as_bytes())
    }
}

impl ToVecBytesResponse for YesNoResponse {
    fn to_vec_bytes_response(&self, cmd: &str) -> Result<Vec<u8, BUFFER_SIZE_IN_BYTES>, ()> {
        let b = atat::serde_at::to_string::<_, { <Self as AtatLen>::LEN }>(
            self,
            "",
            VALUES_SERIALIZE_OPTIONS,
        )
        .map_err(|_| {
            #[cfg(feature = "debug")]
            error!("Error serializing response");
            ()
        })?;
        <Self as ToVecBytesResponse>::wrap_response(cmd, b.as_bytes())
    }
}

pub trait ToVecBytesResponse {
    fn to_vec_bytes_response(&self, cmd: &str) -> Result<Vec<u8, BUFFER_SIZE_IN_BYTES>, ()>;

    fn wrap_response(cmd: &str, vals: &[u8]) -> Result<Vec<u8, BUFFER_SIZE_IN_BYTES>, ()> {
        let mut vec = Vec::new();
        vec.extend_from_slice(cmd.as_bytes())?;
        vec.extend_from_slice(": ".as_bytes())?;
        vec.extend_from_slice(vals)?;
        vec.extend_from_slice(b"OK\r\n")?;
        Ok(vec)
    }
}

impl ErrorResponse {
    pub fn error(error_group: u8, error_code: u8) -> Result<Vec<u8, BUFFER_SIZE_IN_BYTES>, ()> {
        let err = Self {
            error_group,
            error_code,
        };
        err.to_vec_bytes_response("+ERR")
    }

    pub fn error_urc(error_group: u8, error_code: u8) -> Result<Vec<u8, BUFFER_SIZE_IN_BYTES>, ()> {
        let err = Self {
            error_group,
            error_code,
        };

        err.to_vec_bytes_response("+ERR_URC")
    }
}

pub fn urc_on_string(cmd: &str) -> Result<Vec<u8, BUFFER_SIZE_IN_BYTES>, ()> {
    let err = NoResponse {};
    err.to_vec_bytes_response(cmd)
}

pub const VALUES_SERIALIZE_OPTIONS: SerializeOptions = SerializeOptions {
    cmd_prefix: "",
    value_sep: false,
    termination: "\r\n",
    quote_escape_strings: false,
};
