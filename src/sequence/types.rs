use atat_derive::AtatEnum;

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum ActionType {
    #[at_arg(value = 0)]
    GpioOn,
    #[at_arg(value = 1)]
    GpioOff,
    #[at_arg(value = 2)]
    Throttle,
    #[at_arg(value = 3)]
    Wait,
    #[at_arg(value = 4)]
    LedPulse,
    #[at_arg(value = 5)]
    LedColour,
}
