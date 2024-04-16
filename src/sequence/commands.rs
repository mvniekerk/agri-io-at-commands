use crate::sequence::types::ActionType;
use crate::{NoResponse, U16Response, TrueFalseResponse};
use atat_derive::AtatCmd;
use serde::Deserialize;

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_START", NoResponse)]
pub struct SequenceStart {
    pub sequence: u8,
    pub from: Option<u32>,
    pub to: Option<u32>,
    pub repeat: Option<bool>,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_STOP", NoResponse)]
pub struct SequenceStop {
    pub sequence: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_ADD", NoResponse)]
pub struct SequenceAdd {
    pub stopping_it_causes_shutdown_sequence: bool,
    pub can_run_with_others: bool
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_DEL", NoResponse)]
pub struct SequenceDelete {
    pub sequence: u8,
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_AC", NoResponse)]
pub struct SequenceAddAction {
    pub sequence_index: u8,
    pub action_type: ActionType,
    pub second_start: u32,
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

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_INIT", NoResponse)]
pub struct SequenceInit {}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_DEBUG=?", TrueFalseResponse)]
pub struct SequenceDebugGet {}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_DEBUG", TrueFalseResponse)]
pub struct SequenceDebugSet {
    pub debug_on: bool
}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_RUNNING_COUNT", U16Response)]
pub struct SequenceRunningCount {}

#[derive(Debug, Clone, AtatCmd, Deserialize, PartialEq)]
#[at_cmd("+SEQ_IS_RUNNING", TrueFalseResponse)]
pub struct SequenceIsRunning {
    pub sequence_index: u8
}