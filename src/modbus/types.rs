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

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum UartStopBits {
    #[at_arg(value = 0)]
    One,
    #[at_arg(value = 1)]
    Two,
}

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum UartDataBits {
    #[at_arg(value = 0)]
    Five,
    #[at_arg(value = 1)]
    Six,
    #[at_arg(value = 2)]
    Seven,
    #[at_arg(value = 3)]
    Eight,
}