use atat_derive::AtatEnum;

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum UartParity {
    #[at_arg(value = 0)]
    None,
    #[at_arg(value = 1)]
    Odd,
    #[at_arg(value = 2)]
    Even,
}

impl From<&UartParity> for u8 {
    fn from(parity: &UartParity) -> Self {
        match parity {
            UartParity::None => 0,
            UartParity::Odd => 1,
            UartParity::Even => 2,
        }
    }
}

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum UartStopBits {
    #[at_arg(value = 1)]
    One,
    #[at_arg(value = 2)]
    Two,
}

impl From<&UartStopBits> for u8 {
    fn from(stop_bits: &UartStopBits) -> Self {
        match stop_bits {
            UartStopBits::One => 1,
            UartStopBits::Two => 2,
        }
    }
}

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum UartDataBits {
    #[at_arg(value = 5)]
    Five,
    #[at_arg(value = 6)]
    Six,
    #[at_arg(value = 7)]
    Seven,
    #[at_arg(value = 8)]
    Eight,
}

impl From<&UartDataBits> for u8 {
    fn from(data_bits: &UartDataBits) -> Self {
        match data_bits {
            UartDataBits::Five => 5,
            UartDataBits::Six => 6,
            UartDataBits::Seven => 7,
            UartDataBits::Eight => 8,
        }
    }
}
