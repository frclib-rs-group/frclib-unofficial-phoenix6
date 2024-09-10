use std::sync::{Arc, Weak as Aweak};

use frclib_core::units::{angle::Rotation, angular_velocity::RotationPerSec, energy::Volt};
use parking_lot::RwLock;

use crate::{
    cold_signal,
    devices::DeviceIdentifier,
    error::StatusCode,
    signal, signal_setup,
    signals::{
        native, types::MagnetHealthValue, BaseSignal, RefreshableStatusSignal, SPNValue,
        SignalValue, SignalValueRaw,
    },
    spn::SPN,
    Status,
};

use super::CanCoder;

#[derive(Debug, Default)]
pub(super) struct CanCoderCache {
    velocity: SignalValueRaw,
    raw_velocity: SignalValueRaw,
    position: SignalValueRaw,
    abs_position: SignalValueRaw,
    raw_position: SignalValueRaw,
    supply_voltage: SignalValueRaw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CanCoderSignalField {
    Velocity = SPN::CANCODER_VELOCITY as i32,
    RawVelocity = SPN::CANCODER_RAW_VEL as i32,
    Position = SPN::CANCODER_POSITION as i32,
    AbsolutePosition = SPN::CANCODER_ABS_POSITION as i32,
    RawPosition = SPN::CANCODER_RAW_POS as i32,
    SupplyVoltage = SPN::CANCODER_SUPPLY_VOLTAGE as i32,
    MagnetHealth = SPN::CANCODER_MAG_HEALTH as i32,
    IsPro = SPN::LICENSING_IS_PRO_LICENSED as i32,
    //(sticky)faults
    StickyFaultHardware = SPN::STICKY_FAULT_HARDWARE as i32,
    FaultHardware = SPN::FAULT_HARDWARE as i32,
    StickyFaultUnderVoltage = SPN::STICKY_FAULT_UNDERVOLTAGE as i32,
    FaultUnderVoltage = SPN::FAULT_UNDERVOLTAGE as i32,
    StickyFaultBootDuringEnable = SPN::STICKY_FAULT_BOOT_DURING_ENABLE as i32,
    FaultBootDuringEnable = SPN::FAULT_BOOT_DURING_ENABLE as i32,
    StickyFaultUnliscensedFeatureInUse = SPN::STICKY_FAULT_UNLICENSED_FEATURE_IN_USE as i32,
    FaultUnliscensedFeatureInUse = SPN::FAULT_UNLICENSED_FEATURE_IN_USE as i32,
}

pub struct CanCoderSignal<T: SPNValue> {
    identifier: DeviceIdentifier,
    field: CanCoderSignalField,
    cache: Option<Aweak<RwLock<CanCoderCache>>>,
    phantom: std::marker::PhantomData<T>,
}
impl<T: SPNValue> CanCoderSignal<T> {
    fn new(
        identifier: DeviceIdentifier,
        field: CanCoderSignalField,
        cache: Aweak<RwLock<CanCoderCache>>,
    ) -> Self {
        Self {
            identifier,
            field,
            cache: Some(cache),
            phantom: std::marker::PhantomData,
        }
    }

    fn new_cold(identifier: DeviceIdentifier, field: CanCoderSignalField) -> Self {
        Self {
            identifier,
            field,
            cache: None,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: SPNValue> BaseSignal<T> for CanCoderSignal<T> {
    fn value(&self) -> Status<SignalValue<T>> {
        if let Some(cache) = &self.cache {
            let cache = cache.upgrade().ok_or(StatusCode::CouldNotValidate)?;
            let cache = cache.read();
            let value = match self.field {
                CanCoderSignalField::Velocity => cache.velocity,
                CanCoderSignalField::RawVelocity => cache.raw_velocity,
                CanCoderSignalField::Position => cache.position,
                CanCoderSignalField::AbsolutePosition => cache.abs_position,
                CanCoderSignalField::RawPosition => cache.raw_position,
                CanCoderSignalField::SupplyVoltage => cache.supply_voltage,
                _ => unreachable!("This should not happen, this is a cold signal."),
            };
            Ok(SignalValue::<T>::from(value))
        } else {
            let ret = native::request_signal_value_single(
                native::SignalMeta {
                    can_bus: self.identifier.canbus.clone(),
                    timeout: crate::DEFAULT_TIMEOUT,
                },
                native::SignalSpecifier {
                    hash: self.identifier.hash.0,
                    spn: self.get_spn(),
                },
            )?;
            Ok(SignalValue::<T>::from(ret))
        }
    }

    fn get_spn(&self) -> SPN {
        (self.field as i32).try_into().expect("Invalid SPN")
    }

    fn get_device_hash(&self) -> u32 {
        self.identifier.hash.0
    }

    fn set_update_freq(&self, freq_hz: f64) -> Status<()> {
        native::set_update_freq(
            native::SignalMeta {
                can_bus: self.identifier.canbus.clone(),
                timeout: crate::DEFAULT_TIMEOUT,
            },
            native::SignalSpecifier {
                hash: self.identifier.hash.0,
                spn: self.get_spn(),
            },
            freq_hz,
        )
    }
}

impl<T: SPNValue> RefreshableStatusSignal<T> for CanCoderSignal<T> {
    fn refresh(&self) -> Status<()> {
        if self.cache.is_none() {
            return Ok(());
        }
        let ret = native::request_signal_value_single(
            native::SignalMeta {
                can_bus: self.identifier.canbus.clone(),
                timeout: crate::DEFAULT_TIMEOUT,
            },
            native::SignalSpecifier {
                hash: self.identifier.hash.0,
                spn: self.get_spn(),
            },
        )?;
        let cache = self
            .cache
            .as_ref()
            .expect("Cache was None, this should not happen.")
            .upgrade()
            .ok_or(StatusCode::InvalidDeviceDescriptor)?;
        let mut cache = cache.write();
        match self.field {
            CanCoderSignalField::Velocity => cache.velocity = ret,
            CanCoderSignalField::RawVelocity => cache.raw_velocity = ret,
            CanCoderSignalField::Position => cache.position = ret,
            CanCoderSignalField::AbsolutePosition => cache.abs_position = ret,
            CanCoderSignalField::RawPosition => cache.raw_position = ret,
            CanCoderSignalField::SupplyVoltage => cache.supply_voltage = ret,
            _ => unreachable!("This should not happen, this is a cold signal."),
        };
        Ok(())
    }
}

signal_setup! {
    device: CanCoder,
    signal: CanCoderSignal,
    fields: CanCoderSignalField
}

signal! {velocity -> Velocity<RotationPerSec>}
signal! {raw_velocity -> RawVelocity<RotationPerSec>}
signal! {position -> Position<Rotation>}
signal! {abs_position -> AbsolutePosition<Rotation>}
signal! {raw_position -> RawPosition<Rotation>}
signal! {supply_voltage -> SupplyVoltage<Volt>}

cold_signal! {magnet_health -> MagnetHealth<MagnetHealthValue>}
cold_signal! {is_pro -> IsPro<bool>}

cold_signal! (sticky_fault_hardware -> StickyFaultHardware<bool>);
cold_signal! (fault_hardware -> FaultHardware<bool>);
cold_signal! (sticky_fault_under_voltage -> StickyFaultUnderVoltage<bool>);
cold_signal! (fault_under_voltage -> FaultUnderVoltage<bool>);
cold_signal! (sticky_fault_boot_during_enable -> StickyFaultBootDuringEnable<bool>);
cold_signal! (fault_boot_during_enable -> FaultBootDuringEnable<bool>);
cold_signal! (sticky_fault_unliscensed_feature_in_use -> StickyFaultUnliscensedFeatureInUse<bool>);
cold_signal! (fault_unliscensed_feature_in_use -> FaultUnliscensedFeatureInUse<bool>);
