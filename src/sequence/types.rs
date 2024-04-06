use atat_derive::AtatEnum;

#[derive(Clone, Debug, AtatEnum, PartialEq, Copy)]
pub enum ActionType {
    #[at_arg(value = 0)]
    GpioOn,
    #[at_arg(value = 1)]
    GpioOff,
    #[at_arg(value = 2)]
    Throttle,
    #[at_arg(value = 3)]
    LedPulse,
    #[at_arg(value = 4)]
    LedColour,
}
