use atat_derive::AtatEnum;

#[derive(Debug, Eq, PartialEq, Clone, AtatEnum, Hash, Copy)]
#[repr(u8)]
pub enum PinOnOff {
    #[at_arg(value = 0)]
    Off,
    #[at_arg(value = 1)]
    On,
    #[at_arg(value = 2)]
    ToOn,
    #[at_arg(value = 3)]
    ToOff,
}

impl From<&PinOnOff> for u8 {
    fn from(value: &PinOnOff) -> Self {
        match value {
            PinOnOff::Off => 0,
            PinOnOff::On => 1,
            PinOnOff::ToOn => 2,
            PinOnOff::ToOff => 3,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, AtatEnum, Hash, Copy)]
#[repr(u8)]
pub enum PinSequenceListeners {
    #[at_arg(value = 0)]
    BootMode = 0,
    #[at_arg(value = 1)]
    SequenceNumber = 1,
    #[at_arg(value = 2)]
    StartStopPin = 2,
}

impl From<&PinSequenceListeners> for u8 {
    fn from(value: &PinSequenceListeners) -> Self {
        match value {
            PinSequenceListeners::BootMode => 0,
            PinSequenceListeners::SequenceNumber => 1,
            PinSequenceListeners::StartStopPin => 2,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, AtatEnum, Hash, Copy)]
#[repr(u8)]
pub enum RebootMode {
    #[at_arg(value = 0)]
    NotSet = 0,
    #[at_arg(value = 1)]
    Normal = 1,
    #[at_arg(value = 2)]
    Config = 2,
    #[at_arg(value = 3)]
    Debug = 3,
    #[at_arg(value = 4)]
    FactoryReset = 4
}