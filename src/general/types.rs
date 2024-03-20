use atat_derive::AtatEnum;
use postcard::experimental::max_size::MaxSize;

#[derive(Debug, Eq, PartialEq, Clone, MaxSize, AtatEnum)]
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
