use atat::{AtatUrc, nom};
use atat::nom::{bytes, combinator, sequence};
use atat_derive::AtatResp;
use serde_at::serde::{Serialize};

#[derive(Debug, Clone, PartialEq, Eq, AtatResp, Serialize)]
pub struct MotorelliMeasurement {
    pub running_frequency: u16,
    pub set_frequency: u16,
    pub bus_voltage: u16,
    pub output_voltage: u16,
    pub output_current: u16,
    pub rotating_speed: u16,
    pub output_power: i16,
    pub output_torque: i16,
    pub closed_loop_setting: i16,
    pub closed_loop_feedback: i16,
    pub input_state: u16,
    pub output_state: u16,
    pub analog_input_1: u16,
    pub analog_input_2: u16,
    pub analog_input_3: i16,
    pub error: u16,
}

impl AtatUrc for MotorelliMeasurement {
    type Response = Self;
    fn parse(resp: &[u8]) -> Option<Self::Response> {
        let mut get_next_val = sequence::tuple(
            (
                bytes::streaming::take_until::<_, _, nom::error::Error<_>>(","),
                bytes::streaming::tag(","),
                combinator::success(&b""[..]),
            )
        );
        let (_, (running_frequency, _, resp)) = get_next_val(resp).ok()?;
        let (_, (set_frequency, _, resp)) = get_next_val(resp).ok()?;
        let (_, (bus_voltage, _, resp)) = get_next_val(resp).ok()?;
        let (_, (output_voltage, _, resp)) = get_next_val(resp).ok()?;
        let (_, (output_current, _, resp)) = get_next_val(resp).ok()?;
        let (_, (rotating_speed, _, resp)) = get_next_val(resp).ok()?;
        let (_, (output_power, _, resp)) = get_next_val(resp).ok()?;
        let (_, (output_torque, _, resp)) = get_next_val(resp).ok()?;
        let (_, (closed_loop_setting, _, resp)) = get_next_val(resp).ok()?;
        let (_, (closed_loop_feedback, _, resp)) = get_next_val(resp).ok()?;
        let (_, (input_state, _, resp)) = get_next_val(resp).ok()?;
        let (_, (output_state, _, resp)) = get_next_val(resp).ok()?;
        let (_, (analog_input_1, _, resp)) = get_next_val(resp).ok()?;
        let (_, (analog_input_2, _, resp)) = get_next_val(resp).ok()?;
        let (_, (analog_input_3, _, resp)) = get_next_val(resp).ok()?;
        let error = resp;

        let running_frequency = core::str::from_utf8(running_frequency).ok()?;
        let set_frequency = core::str::from_utf8(set_frequency).ok()?;
        let bus_voltage = core::str::from_utf8(bus_voltage).ok()?;
        let output_voltage = core::str::from_utf8(output_voltage).ok()?;
        let output_current = core::str::from_utf8(output_current).ok()?;
        let rotating_speed = core::str::from_utf8(rotating_speed).ok()?;
        let output_power = core::str::from_utf8(output_power).ok()?;
        let output_torque = core::str::from_utf8(output_torque).ok()?;
        let closed_loop_setting = core::str::from_utf8(closed_loop_setting).ok()?;
        let closed_loop_feedback = core::str::from_utf8(closed_loop_feedback).ok()?;
        let input_state = core::str::from_utf8(input_state).ok()?;
        let output_state = core::str::from_utf8(output_state).ok()?;
        let analog_input_1 = core::str::from_utf8(analog_input_1).ok()?;
        let analog_input_2 = core::str::from_utf8(analog_input_2).ok()?;
        let analog_input_3 = core::str::from_utf8(analog_input_3).ok()?;
        let error = core::str::from_utf8(error).ok()?;

        Some(
            MotorelliMeasurement {
                running_frequency: running_frequency.parse().ok()?,
                set_frequency: set_frequency.parse().ok()?,
                bus_voltage: bus_voltage.parse().ok()?,
                output_voltage: output_voltage.parse().ok()?,
                output_current: output_current.parse().ok()?,
                rotating_speed: rotating_speed.parse().ok()?,
                output_power: output_power.parse().ok()?,
                output_torque: output_torque.parse().ok()?,
                closed_loop_setting: closed_loop_setting.parse().ok()?,
                closed_loop_feedback: closed_loop_feedback.parse().ok()?,
                input_state: input_state.parse().ok()?,
                output_state: output_state.parse().ok()?,
                analog_input_1: analog_input_1.parse().ok()?,
                analog_input_2: analog_input_2.parse().ok()?,
                analog_input_3: analog_input_3.parse().ok()?,
                error: error.parse().ok()?,
            }
        )
    }

}