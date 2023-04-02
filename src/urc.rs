use atat::AtatUrc;
use crate::lora::urc::LoraUrcMessages;
use crate::motorelli::urc::MotorelliMeasurement;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UrcMessages {
    Echo,
    Lora(LoraUrcMessages),
    Motorelli(MotorelliMeasurement),
    Error,
    Resetting,
    Starting,
    Started,
}

impl AtatUrc for UrcMessages {
    type Response = Self;

    fn parse(resp: &[u8]) -> Option<Self::Response> {
        if &resp[..3] == b"AT+" {
            return Some(UrcMessages::Echo);
        }
        if resp.starts_with(b"+LORA,") {
            if let Some(lora_urc) = LoraUrcMessages::parse(&resp[6..]) {
                return Some(UrcMessages::Lora(lora_urc));
            }
            return None
        }
        if resp.starts_with(b"+MOTORELLI,") {
            if let Some(motorelli_urc) = MotorelliMeasurement::parse(&resp[11..]) {
                return Some(UrcMessages::Motorelli(motorelli_urc));
            }
            return None;
        }
        if resp == b"+ERROR" {
            return Some(UrcMessages::Error);
        }
        if resp == b"RESETTING" {
            return Some(UrcMessages::Resetting);
        }
        if resp == b"STARTING" {
            return Some(UrcMessages::Starting);
        }
        if resp ==b"STARTED" {
            return Some(UrcMessages::Started);
        }
        None
    }
}