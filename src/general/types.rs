use atat_derive::AtatEnum;

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum PinStateType {
    #[at_arg(value = 0)]
    Off,
    #[at_arg(value = 1)]
    On,
    #[at_arg(value = 2)]
    ToOn,
    #[at_arg(value = 3)]
    ToOff,
}
