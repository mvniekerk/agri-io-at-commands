#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use atat_derive::{AtatEnum, AtatResp};

pub mod general;
pub mod led;
pub mod lora;
pub mod modbus;
pub mod urc;
pub mod motorelli;
pub mod config;

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct NoResponse {}

#[derive(Debug, Clone, AtatEnum, PartialEq)]
pub enum YesNo {
    #[at_arg(value = 0)]
    No,
    #[at_arg(value = 1)]
    Yes,
}

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct YesNoResponse {
    pub yes_no: YesNo,
}