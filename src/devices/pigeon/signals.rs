use std::sync::{Arc, Weak as Aweak};

use frclib_core::units::{
    angle::{Degree, Rotation},
    angular_velocity::DegreePerSec,
    energy::Volt,
};
use parking_lot::RwLock;

use crate::{
    cold_signal,
    devices::DeviceIdentifier,
    error::StatusCode,
    signal, signal_setup,
    signals::{native, BaseSignal, RefreshableStatusSignal, SPNValue, SignalValue, SignalValueRaw},
    spn::SPN,
    Status,
};

use super::Pigeon2;

#[derive(Debug, Default)]
pub(super) struct PigeonCache {
    yaw: SignalValueRaw,
    pitch: SignalValueRaw,
    roll: SignalValueRaw,
    quat_w: SignalValueRaw,
    quat_x: SignalValueRaw,
    quat_y: SignalValueRaw,
    quat_z: SignalValueRaw,
    gravity_x: SignalValueRaw,
    gravity_y: SignalValueRaw,
    gravity_z: SignalValueRaw,
    temp: SignalValueRaw,
    accum_gyro_x: SignalValueRaw,
    accum_gyro_y: SignalValueRaw,
    accum_gyro_z: SignalValueRaw,
    angular_velocity_x: SignalValueRaw,
    angular_velocity_y: SignalValueRaw,
    angular_velocity_z: SignalValueRaw,
    angular_velocity_x_world: SignalValueRaw,
    angular_velocity_y_world: SignalValueRaw,
    angular_velocity_z_world: SignalValueRaw,
    accel_x: SignalValueRaw,
    accel_y: SignalValueRaw,
    accel_z: SignalValueRaw,
    supply_voltage: SignalValueRaw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PigeonSignalField {
    Yaw = SPN::PIGEON2_YAW as i32,
    Pitch = SPN::PIGEON2_PITCH as i32,
    Roll = SPN::PIGEON2_ROLL as i32,
    QuatW = SPN::PIGEON2_QUATW as i32,
    QuatX = SPN::PIGEON2_QUATX as i32,
    QuatY = SPN::PIGEON2_QUATY as i32,
    QuatZ = SPN::PIGEON2_QUATZ as i32,
    GravityX = SPN::PIGEON2_GRAVITY_VECTORX as i32,
    GravityY = SPN::PIGEON2_GRAVITY_VECTORY as i32,
    GravityZ = SPN::PIGEON2_GRAVITY_VECTORZ as i32,
    Temp = SPN::PIGEON2_TEMPERATURE as i32,
    AccumGyroX = SPN::PIGEON2_ACCUM_GYROX as i32,
    AccumGyroY = SPN::PIGEON2_ACCUM_GYROY as i32,
    AccumGyroZ = SPN::PIGEON2_ACCUM_GYROZ as i32,
    AngularVelocityX = SPN::PIGEON2_ANGULAR_VELOCITYX as i32,
    AngularVelocityY = SPN::PIGEON2_ANGULAR_VELOCITYY as i32,
    AngularVelocityZ = SPN::PIGEON2_ANGULAR_VELOCITYZ as i32,
    AngularVelocityXWorld = SPN::PIGEON2_ANGULAR_VELOCITY_XWORLD as i32,
    AngularVelocityYWorld = SPN::PIGEON2_ANGULAR_VELOCITY_YWORLD as i32,
    AngularVelocityZWorld = SPN::PIGEON2_ANGULAR_VELOCITY_ZWORLD as i32,
    AccelX = SPN::PIGEON2_ACCELERATIONX as i32,
    AccelY = SPN::PIGEON2_ACCELERATIONY as i32,
    AccelZ = SPN::PIGEON2_ACCELERATIONZ as i32,
    SupplyVoltage = SPN::PIGEON2_SUPPLY_VOLTAGE as i32,
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
    StickyBootupAccelerometer = SPN::STICKY_FAULT_PIGEON2_BOOTUP_ACCEL as i32,
    BootupAccelerometer = SPN::FAULT_PIGEON2_BOOTUP_ACCEL as i32,
    StickyBootupGyroscope = SPN::STICKY_FAULT_PIGEON2_BOOTUP_GYROS as i32,
    BootupGyroscope = SPN::FAULT_PIGEON2_BOOTUP_GYROS as i32,
    StickyBootupMagnetometer = SPN::STICKY_FAULT_PIGEON2_BOOTUP_MAGNE as i32,
    BootupMagnetometer = SPN::FAULT_PIGEON2_BOOTUP_MAGNE as i32,
    StickyBootIntoMotion = SPN::STICKY_FAULT_PIGEON2_BOOT_INTO_MOTION as i32,
    BootIntoMotion = SPN::FAULT_PIGEON2_BOOT_INTO_MOTION as i32,
    StickyDataAcquiredLate = SPN::STICKY_FAULT_PIGEON2_DATA_ACQUIRED_LATE as i32,
    DataAcquiredLate = SPN::FAULT_PIGEON2_DATA_ACQUIRED_LATE as i32,
    StickyLoopTimeSlow = SPN::STICKY_FAULT_PIGEON2_LOOP_TIME_SLOW as i32,
    LoopTimeSlow = SPN::FAULT_PIGEON2_LOOP_TIME_SLOW as i32,
    StickySaturatedAccelerometer = SPN::STICKY_FAULT_PIGEON2_SATURATED_ACCEL as i32,
    SaturatedAccelerometer = SPN::FAULT_PIGEON2_SATURATED_ACCEL as i32,
    StickySaturatedGyroscope = SPN::STICKY_FAULT_PIGEON2_SATURATED_GYROS as i32,
    SaturatedGyroscope = SPN::FAULT_PIGEON2_SATURATED_GYROS as i32,
    StickySaturatedMagnetometer = SPN::STICKY_FAULT_PIGEON2_SATURATED_MAGNE as i32,
    SaturatedMagnetometer = SPN::FAULT_PIGEON2_SATURATED_MAGNE as i32,
}

pub struct PigeonSignal<T: SPNValue> {
    identifier: DeviceIdentifier,
    field: PigeonSignalField,
    cache: Option<Aweak<RwLock<PigeonCache>>>,
    phantom: std::marker::PhantomData<T>,
}
impl<T: SPNValue> PigeonSignal<T> {
    fn new(
        identifier: DeviceIdentifier,
        field: PigeonSignalField,
        cache: Aweak<RwLock<PigeonCache>>,
    ) -> Self {
        Self {
            identifier,
            field,
            cache: Some(cache),
            phantom: std::marker::PhantomData,
        }
    }

    fn new_cold(identifier: DeviceIdentifier, field: PigeonSignalField) -> Self {
        Self {
            identifier,
            field,
            cache: None,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: SPNValue> BaseSignal<T> for PigeonSignal<T> {
    fn value(&self) -> Status<SignalValue<T>> {
        if let Some(cache) = &self.cache {
            let cache = cache.upgrade().ok_or(StatusCode::CouldNotValidate)?;
            let cache = cache.read();
            let value = match self.field {
                PigeonSignalField::Yaw => cache.yaw,
                PigeonSignalField::Pitch => cache.pitch,
                PigeonSignalField::Roll => cache.roll,
                PigeonSignalField::QuatW => cache.quat_w,
                PigeonSignalField::QuatX => cache.quat_x,
                PigeonSignalField::QuatY => cache.quat_y,
                PigeonSignalField::QuatZ => cache.quat_z,
                PigeonSignalField::GravityX => cache.gravity_x,
                PigeonSignalField::GravityY => cache.gravity_y,
                PigeonSignalField::GravityZ => cache.gravity_z,
                PigeonSignalField::Temp => cache.temp,
                PigeonSignalField::AccumGyroX => cache.accum_gyro_x,
                PigeonSignalField::AccumGyroY => cache.accum_gyro_y,
                PigeonSignalField::AccumGyroZ => cache.accum_gyro_z,
                PigeonSignalField::AngularVelocityX => cache.angular_velocity_x,
                PigeonSignalField::AngularVelocityY => cache.angular_velocity_y,
                PigeonSignalField::AngularVelocityZ => cache.angular_velocity_z,
                PigeonSignalField::AngularVelocityXWorld => cache.angular_velocity_x_world,
                PigeonSignalField::AngularVelocityYWorld => cache.angular_velocity_y_world,
                PigeonSignalField::AngularVelocityZWorld => cache.angular_velocity_z_world,
                PigeonSignalField::AccelX => cache.accel_x,
                PigeonSignalField::AccelY => cache.accel_y,
                PigeonSignalField::AccelZ => cache.accel_z,
                PigeonSignalField::SupplyVoltage => cache.supply_voltage,
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

impl<T: SPNValue> RefreshableStatusSignal<T> for PigeonSignal<T> {
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
            .and_then(|c| c.upgrade())
            .ok_or(StatusCode::CouldNotValidate)?;
        let mut cache = cache.write();
        match self.field {
            PigeonSignalField::Yaw => cache.yaw = ret,
            PigeonSignalField::Pitch => cache.pitch = ret,
            PigeonSignalField::Roll => cache.roll = ret,
            PigeonSignalField::QuatW => cache.quat_w = ret,
            PigeonSignalField::QuatX => cache.quat_x = ret,
            PigeonSignalField::QuatY => cache.quat_y = ret,
            PigeonSignalField::QuatZ => cache.quat_z = ret,
            PigeonSignalField::GravityX => cache.gravity_x = ret,
            PigeonSignalField::GravityY => cache.gravity_y = ret,
            PigeonSignalField::GravityZ => cache.gravity_z = ret,
            PigeonSignalField::Temp => cache.temp = ret,
            PigeonSignalField::AccumGyroX => cache.accum_gyro_x = ret,
            PigeonSignalField::AccumGyroY => cache.accum_gyro_y = ret,
            PigeonSignalField::AccumGyroZ => cache.accum_gyro_z = ret,
            PigeonSignalField::AngularVelocityX => cache.angular_velocity_x = ret,
            PigeonSignalField::AngularVelocityY => cache.angular_velocity_y = ret,
            PigeonSignalField::AngularVelocityZ => cache.angular_velocity_z = ret,
            PigeonSignalField::AngularVelocityXWorld => cache.angular_velocity_x_world = ret,
            PigeonSignalField::AngularVelocityYWorld => cache.angular_velocity_y_world = ret,
            PigeonSignalField::AngularVelocityZWorld => cache.angular_velocity_z_world = ret,
            PigeonSignalField::AccelX => cache.accel_x = ret,
            PigeonSignalField::AccelY => cache.accel_y = ret,
            PigeonSignalField::AccelZ => cache.accel_z = ret,
            PigeonSignalField::SupplyVoltage => cache.supply_voltage = ret,
            _ => unreachable!("This should not happen, this is a cold signal."),
        }
        Ok(())
    }
}

signal_setup! {
    device: Pigeon2,
    signal: PigeonSignal,
    fields: PigeonSignalField
}

signal! {yaw -> Yaw<Rotation>}
signal! {pitch -> Pitch<Rotation>}
signal! {roll -> Roll<Rotation>}
signal! {quat_w -> QuatW<f64>}
signal! {quat_x -> QuatX<f64>}
signal! {quat_y -> QuatY<f64>}
signal! {quat_z -> QuatZ<f64>}
signal! {gravity_x -> GravityX<f64>}
signal! {gravity_y -> GravityY<f64>}
signal! {gravity_z -> GravityZ<f64>}
signal! {temp -> Temp<f64>}
signal! {accum_gyro_x -> AccumGyroX<Degree>}
signal! {accum_gyro_y -> AccumGyroY<Degree>}
signal! {accum_gyro_z -> AccumGyroZ<Degree>}
signal! {angular_velocity_x -> AngularVelocityX<DegreePerSec>}
signal! {angular_velocity_y -> AngularVelocityY<DegreePerSec>}
signal! {angular_velocity_z -> AngularVelocityZ<DegreePerSec>}
signal! {angular_velocity_x_world -> AngularVelocityXWorld<DegreePerSec>}
signal! {angular_velocity_y_world -> AngularVelocityYWorld<DegreePerSec>}
signal! {angular_velocity_z_world -> AngularVelocityZWorld<DegreePerSec>}
signal! {accel_x -> AccelX<f64>}
signal! {accel_y -> AccelY<f64>}
signal! {accel_z -> AccelZ<f64>}
signal! {supply_voltage -> SupplyVoltage<Volt>}

cold_signal! {is_pro -> IsPro<bool>}

cold_signal! {sticky_fault_hardware -> StickyFaultHardware<bool>}
cold_signal! {fault_hardware -> FaultHardware<bool>}
cold_signal! {sticky_fault_under_voltage -> StickyFaultUnderVoltage<bool>}
cold_signal! {fault_under_voltage -> FaultUnderVoltage<bool>}
cold_signal! {sticky_fault_boot_during_enable -> StickyFaultBootDuringEnable<bool>}
cold_signal! {fault_boot_during_enable -> FaultBootDuringEnable<bool>}
cold_signal! {sticky_fault_unliscensed_feature_in_use -> StickyFaultUnliscensedFeatureInUse<bool>}
cold_signal! {fault_unliscensed_feature_in_use -> FaultUnliscensedFeatureInUse<bool>}
cold_signal! {sticky_bootup_accelerometer -> StickyBootupAccelerometer<bool>}
cold_signal! {bootup_accelerometer -> BootupAccelerometer<bool>}
cold_signal! {sticky_bootup_gyroscope -> StickyBootupGyroscope<bool>}
cold_signal! {bootup_gyroscope -> BootupGyroscope<bool>}
cold_signal! {sticky_bootup_magnetometer -> StickyBootupMagnetometer<bool>}
cold_signal! {bootup_magnetometer -> BootupMagnetometer<bool>}
cold_signal! {sticky_boot_into_motion -> StickyBootIntoMotion<bool>}
cold_signal! {boot_into_motion -> BootIntoMotion<bool>}
cold_signal! {sticky_data_acquired_late -> StickyDataAcquiredLate<bool>}
cold_signal! {data_acquired_late -> DataAcquiredLate<bool>}
cold_signal! {sticky_loop_time_slow -> StickyLoopTimeSlow<bool>}
cold_signal! {loop_time_slow -> LoopTimeSlow<bool>}
cold_signal! {sticky_saturated_accelerometer -> StickySaturatedAccelerometer<bool>}
cold_signal! {saturated_accelerometer -> SaturatedAccelerometer<bool>}
cold_signal! {sticky_saturated_gyroscope -> StickySaturatedGyroscope<bool>}
cold_signal! {saturated_gyroscope -> SaturatedGyroscope<bool>}
cold_signal! {sticky_saturated_magnetometer -> StickySaturatedMagnetometer<bool>}
cold_signal! {saturated_magnetometer -> SaturatedMagnetometer<bool>}
