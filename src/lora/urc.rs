use atat::AtatUrc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LoraUrcMessages {
    Joining,
    Joined,
    JoinFailed,
    Sending,
    Sent,
    SendFailed,
    Receiving,
    Received,
    ReceiveFailed,
}

impl AtatUrc for LoraUrcMessages {
    type Response = Self;

    fn parse(resp: &[u8]) -> Option<Self::Response> {
        if resp == b"JOINING" {
            return Some(LoraUrcMessages::Joining);
        }
        if resp == b"JOINED" {
            return Some(LoraUrcMessages::Joined);
        }
        if resp == b"JOIN_FAILED" {
            return Some(LoraUrcMessages::JoinFailed);
        }
        if resp.starts_with(b"SENDING,") {
            return Some(LoraUrcMessages::Sending);
        }
        if resp == b"SENT" {
            return Some(LoraUrcMessages::Sent);
        }
        if resp == b"SEND_FAILED" {
            return Some(LoraUrcMessages::SendFailed);
        }
        if resp == b"RECEIVING" {
            return Some(LoraUrcMessages::Receiving);
        }
        if resp.starts_with(b"RECEIVED,") {
            return Some(LoraUrcMessages::Received);
        }
        if resp == b"RECEIVE_FAILED" {
            return Some(LoraUrcMessages::ReceiveFailed);
        }
        None
    }
}