use crate::NumberResponse;
use atat_derive::{AtatLen, AtatResp};
use serde::Serialize;

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize)]
pub struct SequencesState {
    pub count: u8,
    pub running_count: u8,
}

impl NumberResponse for SequencesState {}

#[derive(Debug, Clone, AtatResp, PartialEq, AtatLen, Serialize, Default, Hash, Eq)]
pub struct SequenceState {
    pub sequence_index: u8,
    pub action_count: u16,
    pub max_seconds: u32,
    pub stopping_it_causes_shutdown_sequence: bool,
    pub can_run_with_others: bool,
    pub running: bool,
    pub from_sec: u32,
    pub to_sec: u32,
    pub repeat: bool,
    pub started_at: u64,
    pub next_step_at: u64,
}

impl NumberResponse for SequenceState {}
