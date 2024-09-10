use frclib_core::units::angle::Rotation;

use crate::{
    devices::{
        config_native::{deserialize_double, deserialize_int, serialize_double, serialize_int},
        ConfigProtocol,
    },
    error::StatusCode,
    seal,
    spn::SPN,
    Status,
};

pub trait CanCoderConfigType: ConfigProtocol {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CanCoderConfig {
    /// True if we should factory default newer unsupported configs,
    /// false to leave newer unsupported configs alone.
    ///
    /// This flag addresses a corner case where the device may have
    /// firmware with newer configs that didn't exist when this
    /// version of the API was built. If this occurs and this
    /// flag is true, unsupported new configs will be factory
    /// defaulted to avoid unexpected behavior.
    ///
    /// This is also the behavior in Phoenix 5, so this flag
    /// is defaulted to true to match.
    ///
    pub future_proof_configs: bool,

    /// Configs that affect the magnet sensor and how to interpret it.
    ///
    /// Includes sensor range and other configs related to sensor.
    pub magnet_sensor: MagnetSensorConfigs,
}
impl Default for CanCoderConfig {
    fn default() -> Self {
        Self {
            future_proof_configs: true,
            magnet_sensor: MagnetSensorConfigs::default(),
        }
    }
}
impl std::fmt::Display for CanCoderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CanCoderConfig {{ future_proof_configs: {}, magnet_sensor: {} }}",
            self.future_proof_configs, self.magnet_sensor
        )
    }
}
seal! {CanCoderConfig}
impl ConfigProtocol for CanCoderConfig {
    fn serialize(&self) -> Status<String> {
        let mut ss = String::new();
        ss.push_str(self.magnet_sensor.serialize()?.as_ref());
        Ok(ss)
    }

    fn deserialize(to_deserialize: &str) -> Status<Self> {
        Ok(Self {
            future_proof_configs: Default::default(),
            magnet_sensor: MagnetSensorConfigs::deserialize(to_deserialize)?,
        })
    }

    fn future_proof_configs(&self) -> bool {
        self.future_proof_configs
    }
}
impl CanCoderConfigType for CanCoderConfig {}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(i32)]
pub enum SensorDirectionValue {
    #[default]
    CounterClockwisePositive = 0,
    ClockwisePositive = 1,
}
impl std::fmt::Display for SensorDirectionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorDirectionValue::CounterClockwisePositive => write!(f, "CounterClockwisePositive"),
            SensorDirectionValue::ClockwisePositive => write!(f, "ClockwisePositive"),
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(i32)]
pub enum AbsoluteSensorRangeValue {
    #[default]
    SignedPlusMinusHalf = 1,
    Unsigned0To1 = 0,
}
impl std::fmt::Display for AbsoluteSensorRangeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbsoluteSensorRangeValue::SignedPlusMinusHalf => write!(f, "SignedPlusMinusHalf"),
            AbsoluteSensorRangeValue::Unsigned0To1 => write!(f, "Unsigned0To1"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MagnetSensorConfigs {
    /// Direction of the sensor to determine positive facing the
    /// LED side of the CANcoder.
    pub sensor_direction: SensorDirectionValue,

    /// This offset is added to the reported position, allowing the
    /// application to trim the zero position.  When set to the default
    /// value of zero, position reports zero when magnet north pole aligns
    /// with the LED.
    ///
    /// Range: -1 full rotation to 1 full rotation
    pub magnet_offset: Rotation,

    /// The range of the absolute sensor, either [-0.5, 0.5) or [0, 1).
    pub absolute_sensor_range: AbsoluteSensorRangeValue,
}
impl Default for MagnetSensorConfigs {
    fn default() -> Self {
        Self {
            sensor_direction: SensorDirectionValue::default(),
            magnet_offset: Rotation::new(0.0),
            absolute_sensor_range: AbsoluteSensorRangeValue::default(),
        }
    }
}
impl std::fmt::Display for MagnetSensorConfigs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "MagnetSensorConfigs {{ sensor_direction: {}, magnet_offset: {}, absolute_sensor_range: {} }}",
            self.sensor_direction,
            self.magnet_offset,
            self.absolute_sensor_range
        )
    }
}
seal! {MagnetSensorConfigs}
impl ConfigProtocol for MagnetSensorConfigs {
    fn serialize(&self) -> Status<String> {
        let mut ss = String::new();
        ss.push_str(
            serialize_int(SPN::CANCODER_SENSOR_DIRECTION, self.sensor_direction.into())?.as_ref(),
        );
        ss.push_str(
            serialize_double(SPN::CANCODER_MAGNET_OFFSET, self.magnet_offset.value())?.as_ref(),
        );
        ss.push_str(
            serialize_int(
                SPN::CANCODER_ABSOLUTE_SENSOR_RANGE,
                self.absolute_sensor_range.into(),
            )?
            .as_ref(),
        );
        Ok(ss)
    }

    fn deserialize(to_deserialize: &str) -> Status<Self> {
        Ok(Self {
            sensor_direction: deserialize_int(SPN::CANCODER_SENSOR_DIRECTION, to_deserialize)?
                .try_into()
                .map_err(|_| StatusCode::CouldNotDeserializeString)?,
            magnet_offset: Rotation::new(deserialize_double(
                SPN::CANCODER_MAGNET_OFFSET,
                to_deserialize,
            )?),
            absolute_sensor_range: deserialize_int(
                SPN::CANCODER_ABSOLUTE_SENSOR_RANGE,
                to_deserialize,
            )?
            .try_into()
            .map_err(|_| StatusCode::CouldNotDeserializeString)?,
        })
    }
}
impl CanCoderConfigType for MagnetSensorConfigs {}
