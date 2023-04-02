use atat_derive::AtatEnum;

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum MeasurementConfigType {
    #[at_arg(value = 0)]
    WarnLow(f32),
    #[at_arg(value = 1)]
    WarnHigh(f32),
    #[at_arg(value = 2)]
    WaitFor(bool),
    #[at_arg(value = 3)]
    DontStart(f32),
    #[at_arg(value = 4)]
    HighGuard(bool),
    #[at_arg(value = 5)]
    LowGuard(bool),
    #[at_arg(value = 6)]
    OffHigh(f32),
    #[at_arg(value = 7)]
    OffLow(f32),
    #[at_arg(value = 8)]
    Scale(f32),
}

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum MeasurementSensorType {
    #[at_arg(value = 0)]
    Current,
    #[at_arg(value = 1)]
    FlowRate,
    #[at_arg(value = 2)]
    Kw,
    #[at_arg(value = 3)]
    Pressure,
    #[at_arg(value = 4)]
    Speed,
    #[at_arg(value = 5)]
    Volts,
}

#[derive(Clone, Debug, AtatEnum, PartialEq)]
pub enum SensorDeviceType {
    #[at_arg(value = 0)]
    BoardPin,
    #[at_arg(value = 1)]
    MotorelliAd1000,
}