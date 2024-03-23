use crate::sequence::types::ActionType;
use crate::NoResponse;
use atat_derive::AtatCmd;
use serde::Deserialize;

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_START", NoResponse)]
pub struct SequenceStart {
    pub sequence: u8,
    pub from: Option<u16>,
    pub to: Option<u16>,
    pub repeat: Option<bool>,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_STOP", NoResponse)]
pub struct SequenceStop {
    pub sequence: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_ADD", NoResponse)]
pub struct SequenceAdd {}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_DEL", NoResponse)]
pub struct SequenceDelete {
    pub sequence: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_ADD_ACTION", NoResponse)]
pub struct SequenceAddAction {
    pub sequence_index: u8,
    pub action_type: ActionType,
    pub action_data: u32,
    pub action_data_1: Option<u32>,
    pub action_data_2: Option<u32>,
    pub action_data_3: Option<u32>,
    pub action_data_4: Option<u32>,
    pub action_data_5: Option<u32>,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_CLEAR", NoResponse)]
pub struct SequenceClearAll {}