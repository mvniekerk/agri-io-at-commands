use atat_derive::AtatCmd;
use heapless::{String, Vec};
use serde::Deserialize;
use crate::sequence::types::ActionType;
use crate::NoResponse;

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_START", NoResponse)]
pub struct SequenceStart {
    pub sequence: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_STOP", NoResponse)]
pub struct SequenceStop {
    pub sequence: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_ADD", NoResponse)]
pub struct SequenceAdd {
    pub name: String<32>,
    pub actions: Vec<SequenceAddAction, 54>,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_ADD_ACTION", NoResponse)]
pub struct SequenceAddAction {
    pub action_type: ActionType,
    pub action_data: u32,
}