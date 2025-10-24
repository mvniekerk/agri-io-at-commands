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

#[derive(Clone, Debug, AtatEnum, PartialEq, Copy)]
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

#[derive(Clone, Debug, AtatEnum, PartialEq, Copy)]
pub enum Operation {
    #[at_arg(value = 0)]
    SUBTRACT,
    #[at_arg(value = 1)]
    ADD,
    #[at_arg(value = 2)]
    MULTIPLY,
    #[at_arg(value = 3)]
    DIVIDE,
}

impl From<&Operation> for u8 {
    fn from(operation: &Operation) -> Self {
        match operation {
            Operation::SUBTRACT => 0,
            Operation::ADD => 1,
            Operation::MULTIPLY => 2,
            Operation::DIVIDE => 3,
        }
    }
}

#[derive(Clone, Debug, AtatEnum, PartialEq, Copy)]
pub enum GenericDeviceType {
    #[at_arg(value = 0)]
    Generic,
    #[at_arg(value = 1)]
    MacSensorLd300
}

impl From<&GenericDeviceType> for u8 {
    fn from(device_type: &GenericDeviceType) -> Self {
        match device_type {
            GenericDeviceType::Generic => 0,
            GenericDeviceType::MacSensorLd300 => 1
        }
    }
}
