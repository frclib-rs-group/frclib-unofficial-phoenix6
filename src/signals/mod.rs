use crate::{spn::SPN, Status};

use self::{__sealed::Sealed, native::SignalSpecifier};

pub mod logger;
pub(crate) mod native;
pub(crate) mod queue_thread;
pub mod types;

pub(crate) type SignalSource = SignalSpecifier;

pub trait BaseSignal<T: SPNValue> {
    fn get_spn(&self) -> SPN;

    fn get_device_hash(&self) -> u32;

    fn set_update_freq(&self, freq_hz: f64) -> Status<()>;

    fn value(&self) -> Status<SignalValue<T>>;

    fn same_source_as(&self, other: &dyn BaseSignal<T>) -> bool {
        self.get_device_hash() == other.get_device_hash() && self.get_spn() == other.get_spn()
    }
}

pub trait QueuedStatusSignal<T: SPNValue>: BaseSignal<T> {
    fn is_empty(&self) -> Status<bool>;
}

pub trait RefreshableStatusSignal<T: SPNValue>: BaseSignal<T> {
    fn refresh(&self) -> Status<()>;
}

use crate::__sealed;

pub trait SPNValue: Sealed + Sized + Default + Copy {
    fn try_from_f64(value: f64) -> Status<Self>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SignalValue<T: SPNValue> {
    pub value: T,
    pub can_timestamp: f64,
    pub software_timestamp: f64,
    pub device_timestamp: f64,
}
pub type SignalValueRaw = native::SignalValueResponse;
impl<T: SPNValue> From<SignalValueRaw> for SignalValue<T> {
    fn from(raw: SignalValueRaw) -> Self {
        Self {
            value: T::try_from_f64(raw.value).unwrap(),
            can_timestamp: raw.can_timestamp,
            software_timestamp: raw.software_timestamp,
            device_timestamp: raw.device_timestamp,
        }
    }
}

impl __sealed::Sealed for f64 {}
impl SPNValue for f64 {
    fn try_from_f64(value: f64) -> Status<Self> {
        Ok(value)
    }
}

impl __sealed::Sealed for i32 {}
impl SPNValue for i32 {
    fn try_from_f64(value: f64) -> Status<Self> {
        Ok(value as i32)
    }
}

impl __sealed::Sealed for u32 {}
impl SPNValue for u32 {
    fn try_from_f64(value: f64) -> Status<Self> {
        Ok(value as u32)
    }
}

impl __sealed::Sealed for bool {}
impl SPNValue for bool {
    fn try_from_f64(value: f64) -> Status<Self> {
        Ok(value > 0.0)
    }
}

macro_rules! spn_for_unit {
    ($quan:ident :: $unit:ident) => {
        impl __sealed::Sealed for frclib_core::units::$quan::$unit {}
        impl SPNValue for frclib_core::units::$quan::$unit {
            fn try_from_f64(value: f64) -> Status<Self> {
                Ok(frclib_core::units::$quan::$unit::from(value))
            }
        }
    };
}

spn_for_unit!(angle::Rotation);
spn_for_unit!(angle::Degree);
spn_for_unit!(angular_velocity::RotationPerSec);
spn_for_unit!(angular_velocity::DegreePerSec);
spn_for_unit!(angular_acceleration::RotationPerSecSqr);
spn_for_unit!(temperature::Celsius);
spn_for_unit!(energy::Amp);
spn_for_unit!(energy::Volt);
