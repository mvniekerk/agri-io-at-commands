#[cfg(feature = "std")]
use crate::config::types::MeasurementSensorType;
#[cfg(feature = "std")]
use crate::modbus::commands::ModbusGenericValueOperationAdd;
use atat_derive::AtatEnum;
use postcard::experimental::max_size::MaxSize;

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

#[derive(Clone, Debug, AtatEnum, PartialEq, Copy, Hash)]
pub enum Operation {
    #[at_arg(value = 0)]
    SUBTRACT,
    #[at_arg(value = 1)]
    ADD,
    #[at_arg(value = 2)]
    MULTIPLY,
    #[at_arg(value = 3)]
    DIVIDE,
    #[at_arg(value = 4)]
    VALIDATE,
}

impl From<&Operation> for u8 {
    fn from(operation: &Operation) -> Self {
        match operation {
            Operation::SUBTRACT => 0,
            Operation::ADD => 1,
            Operation::MULTIPLY => 2,
            Operation::DIVIDE => 3,
            Operation::VALIDATE => 4,
        }
    }
}

#[derive(Clone, Debug, AtatEnum, PartialEq, Copy, Eq, MaxSize)]
pub enum GenericDeviceType {
    #[at_arg(value = 0)]
    Generic,
    #[at_arg(value = 1)]
    MacSensorLd300,
}

impl From<&GenericDeviceType> for u8 {
    fn from(device_type: &GenericDeviceType) -> Self {
        match device_type {
            GenericDeviceType::Generic => 0,
            GenericDeviceType::MacSensorLd300 => 1,
        }
    }
}

#[cfg(feature = "std")]
#[derive(Clone)]
pub struct ModbusGenericDevice {
    pub operations: alloc::vec::Vec<(
        MeasurementSensorType,
        alloc::vec::Vec<ModbusGenericValueOperationAdd>,
    )>,
}

#[cfg(feature = "std")]
impl GenericDeviceType {
    pub fn new_from_type(&self, index: u8) -> ModbusGenericDevice {
        match self {
            GenericDeviceType::Generic => ModbusGenericDevice {
                operations: alloc::vec![],
            },
            GenericDeviceType::MacSensorLd300 => {
                let operations = alloc::vec![
                    // Instantaneous flow rate
                    (
                        MeasurementSensorType::FlowRate,
                        alloc::vec![
                            ModbusGenericValueOperationAdd {
                                id: index,
                                value_id: 0,
                                register_or_value: 0x10,
                                is_register: true,
                                left_shift: 0xFF,
                                right_shift: 0xFF,
                                divided_by: 0x0001,
                                multiplied_by: 0xFFFF,
                                mask: 0xFFFF,
                                operation: Operation::VALIDATE,
                                is_coil: false,
                                retry_count: 12,
                                ms_per_retry: 1000,
                                lower_limit: 0,
                                upper_limit: 0,
                            },
                            ModbusGenericValueOperationAdd {
                                id: index,
                                value_id: 0,
                                register_or_value: 0,
                                is_register: true,
                                left_shift: 0x16,
                                right_shift: 0xFF,
                                divided_by: 1000,
                                multiplied_by: 0xFFFF,
                                mask: 0xFFFF,
                                operation: Operation::ADD,
                                is_coil: false,
                                retry_count: 12,
                                ms_per_retry: 50,
                                lower_limit: 0,
                                upper_limit: 0
                            },
                            ModbusGenericValueOperationAdd {
                                id: index,
                                value_id: 0,
                                register_or_value: 1,
                                is_register: true,
                                left_shift: 0xFF,
                                right_shift: 0xFF,
                                divided_by: 1000,
                                multiplied_by: 0xFFFF,
                                mask: 0xFFFF,
                                operation: Operation::ADD,
                                is_coil: false,
                                retry_count: 12,
                                ms_per_retry: 50,
                                lower_limit: 0,
                                upper_limit: 0
                            },
                        ]
                    ),
                    // Cumulative flow rate
                    (
                        MeasurementSensorType::FlowRate,
                        alloc::vec![
                            ModbusGenericValueOperationAdd {
                                id: index,
                                value_id: 0,
                                register_or_value: 0x10,
                                is_register: true,
                                left_shift: 0xFF,
                                right_shift: 0xFF,
                                divided_by: 0x0001,
                                multiplied_by: 0xFFFF,
                                mask: 0xFFFF,
                                operation: Operation::VALIDATE,
                                is_coil: false,
                                retry_count: 12,
                                ms_per_retry: 50,
                                lower_limit: 0,
                                upper_limit: 0,
                            },
                            ModbusGenericValueOperationAdd {
                                id: index,
                                value_id: 0,
                                register_or_value: 5,
                                is_register: true,
                                left_shift: 0x16,
                                right_shift: 0xFF,
                                divided_by: 0xFFFF,
                                multiplied_by: 0xFFFF,
                                mask: 0xFFFF,
                                operation: Operation::ADD,
                                is_coil: false,
                                retry_count: 12,
                                ms_per_retry: 50,
                                lower_limit: 0,
                                upper_limit: 0,
                            },
                            ModbusGenericValueOperationAdd {
                                id: index,
                                value_id: 0,
                                register_or_value: 6,
                                is_register: true,
                                left_shift: 0xFF,
                                right_shift: 0xFF,
                                divided_by: 0xFFFF,
                                multiplied_by: 0xFFFF,
                                mask: 0xFFFF,
                                operation: Operation::ADD,
                                is_coil: false,
                                retry_count: 12,
                                ms_per_retry: 50,
                                lower_limit: 0,
                                upper_limit: 0
                            },
                            ModbusGenericValueOperationAdd {
                                id: index,
                                value_id: 0,
                                register_or_value: 7,
                                is_register: true,
                                left_shift: 0xFF,
                                right_shift: 0xFF,
                                divided_by: 1000,
                                multiplied_by: 0xFFFF,
                                mask: 0xFFFF,
                                operation: Operation::ADD,
                                is_coil: false,
                                retry_count: 12,
                                ms_per_retry: 50,
                                lower_limit: 0,
                                upper_limit: 0
                            },
                        ]
                    )
                ];

                ModbusGenericDevice { operations }
            }
        }
    }
}
