use frclib_core::units::angle::{Angle, Degree};

use crate::{
    devices::{
        config_native::{deserialize_bool, deserialize_double, serialize_bool, serialize_double},
        ConfigProtocol,
    },
    seal,
    spn::SPN,
    Status,
};

pub trait PigeonConfigType: ConfigProtocol {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pigeon2Configuration {
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
    pub future_proof_configs: bool,
    /// Configs for Pigeon 2's Mount Pose configuration.
    ///
    /// These configs allow the Pigeon2 to be mounted in whatever orientation
    /// that's desired and ensure the reported Yaw/Pitch/Roll is from the
    /// robot's reference.
    pub mount_pose: MountPoseConfigs,
    /// Configs to trim the Pigeon2's gyroscope.
    ///
    /// Pigeon2 allows the user to trim the gyroscope's sensitivity. While
    /// this isn't necessary for the Pigeon2, as it comes calibrated
    /// out-of-the-box, users can make use of this to make the Pigeon2 even
    /// more accurate for their application.
    pub gyro_trim: GyroTrimConfigs,
    /// Configs to enable/disable various features of the Pigeon2.
    ///
    /// These configs allow the user to enable or disable various aspects of
    /// the Pigeon2.
    pub features: Pigeon2FeaturesConfigs,
}

impl Default for Pigeon2Configuration {
    fn default() -> Self {
        Self {
            future_proof_configs: true,
            mount_pose: MountPoseConfigs::default(),
            gyro_trim: GyroTrimConfigs::default(),
            features: Pigeon2FeaturesConfigs::default(),
        }
    }
}

impl std::fmt::Display for Pigeon2Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pigeon2Configuration {{ future_proof_configs: {}, mount_pose: {}, gyro_trim: {}, features: {} }}",
            self.future_proof_configs,
            self.mount_pose,
            self.gyro_trim,
            self.features
        )
    }
}

seal! {Pigeon2Configuration}
impl ConfigProtocol for Pigeon2Configuration {
    fn serialize(&self) -> Status<String> {
        let mut ss = String::new();
        ss.push_str(self.mount_pose.serialize()?.as_ref());
        ss.push_str(self.gyro_trim.serialize()?.as_ref());
        ss.push_str(self.features.serialize()?.as_ref());
        Ok(ss)
    }

    fn deserialize(to_deserialize: &str) -> Status<Self> {
        Ok(Self {
            mount_pose: MountPoseConfigs::deserialize(to_deserialize)?,
            gyro_trim: GyroTrimConfigs::deserialize(to_deserialize)?,
            features: Pigeon2FeaturesConfigs::deserialize(to_deserialize)?,
            ..Default::default()
        })
    }

    fn future_proof_configs(&self) -> bool {
        self.future_proof_configs
    }
}
impl PigeonConfigType for Pigeon2Configuration {}

/// Configs for Pigeon 2's Mount Pose configuration.
///
/// These configs allow the Pigeon2 to be mounted in whatever orientation
/// that's desired and ensure the reported Yaw/Pitch/Roll is from the
/// robot's reference.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MountPoseConfigs {
    /// The mounting calibration yaw-component
    ///
    /// Use [MountPoseConfigs::with_yaw] to set the yaw with validation
    pub mount_pose_yaw: Degree,
    /// The mounting calibration pitch-component
    ///
    /// Use [MountPoseConfigs::with_pitch] to set the pitch with validation
    pub mount_pose_pitch: Degree,
    /// The mounting calibration roll-component
    ///
    /// Use [MountPoseConfigs::with_roll] to set the roll with validation
    pub mount_pose_roll: Degree,
}

impl MountPoseConfigs {
    /// Constructs a new `MountPoseConfigs` with the given yaw, pitch, and roll.
    pub fn new(yaw: impl Angle, pitch: impl Angle, roll: impl Angle) -> Self {
        Self {
            mount_pose_yaw: yaw.standard().into(),
            mount_pose_pitch: pitch.standard().into(),
            mount_pose_roll: roll.standard().into(),
        }
    }

    /// Returns degree in 360 to -360 range
    fn modulo_degrees(angle: Degree) -> Degree {
        let mut angle = angle.value();
        while angle > 360.0 {
            angle -= 360.0;
        }
        while angle < -360.0 {
            angle += 360.0;
        }
        Degree(angle)
    }

    /// Returns the configuration with the given yaw set.
    ///
    /// The yaw will be coerced into the 360 to -360 range when stored in degrees.
    pub fn with_yaw(mut self, yaw: impl Angle) -> Self {
        self.mount_pose_yaw = Self::modulo_degrees(yaw.standard().into());
        self
    }

    /// Returns the configuration with the given pitch set.
    ///
    /// The pitch will be coerced into the 360 to -360 range when stored in degrees.
    pub fn with_pitch(mut self, pitch: impl Angle) -> Self {
        self.mount_pose_pitch = Self::modulo_degrees(pitch.standard().into());
        self
    }

    /// Returns the configuration with the given roll set.
    ///
    /// The roll will be coerced into the 360 to -360 range when stored in degrees.
    pub fn with_roll(mut self, roll: impl Angle) -> Self {
        self.mount_pose_roll = Self::modulo_degrees(roll.standard().into());
        self
    }

    /// Returns the yaw of the configuration.
    pub fn yaw(&self) -> Degree {
        self.mount_pose_yaw
    }

    /// Returns the pitch of the configuration.
    pub fn pitch(&self) -> Degree {
        self.mount_pose_pitch
    }

    /// Returns the roll of the configuration.
    pub fn roll(&self) -> Degree {
        self.mount_pose_roll
    }
}

impl std::fmt::Display for MountPoseConfigs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MountPoseConfigs {{ yaw: {}, pitch: {}, roll: {} }}",
            self.mount_pose_yaw, self.mount_pose_pitch, self.mount_pose_roll
        )
    }
}

seal! {MountPoseConfigs}
impl ConfigProtocol for MountPoseConfigs {
    fn serialize(&self) -> Status<String> {
        let mut ss = String::new();
        ss.push_str(&serialize_double(
            SPN::PIGEON2_MOUNT_POSE_YAW,
            self.mount_pose_yaw.value(),
        )?);
        ss.push_str(&serialize_double(
            SPN::PIGEON2_MOUNT_POSE_PITCH,
            self.mount_pose_pitch.value(),
        )?);
        ss.push_str(&serialize_double(
            SPN::PIGEON2_MOUNT_POSE_ROLL,
            self.mount_pose_roll.value(),
        )?);
        Ok(ss)
    }

    fn deserialize(to_deserialize: &str) -> Status<Self> {
        Ok(Self {
            mount_pose_yaw: Degree(deserialize_double(
                SPN::PIGEON2_MOUNT_POSE_YAW,
                to_deserialize,
            )?),
            mount_pose_pitch: Degree(deserialize_double(
                SPN::PIGEON2_MOUNT_POSE_PITCH,
                to_deserialize,
            )?),
            mount_pose_roll: Degree(deserialize_double(
                SPN::PIGEON2_MOUNT_POSE_ROLL,
                to_deserialize,
            )?),
        })
    }
}
impl PigeonConfigType for MountPoseConfigs {}

/// Configs to trim the Pigeon2's gyroscope.
///
/// Pigeon2 allows the user to trim the gyroscope's sensitivity. While
/// this isn't necessary for the Pigeon2, as it comes calibrated
/// out-of-the-box, users can make use of this to make the Pigeon2 even
/// more accurate for their application.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct GyroTrimConfigs {
    /// The scalar x of the gyroscope.
    ///
    /// The scalar is in `degrees per rotation` and has a range of -180 to 180.
    ///
    /// Use [GyroTrimConfigs::with_scalar_x] to set the scalar with validation
    pub gyro_scalar_x: f64,
    /// The scalar y of the gyroscope.
    ///
    /// The scalar is in `degrees per rotation` and has a range of -180 to 180.
    ///
    /// Use [GyroTrimConfigs::with_scalar_y] to set the scalar with validation
    pub gyro_scalar_y: f64,
    /// The scalar z of the gyroscope.
    ///
    /// The scalar is in `degrees per rotation` and has a range of -180 to 180.
    ///
    /// Use [GyroTrimConfigs::with_scalar_z] to set the scalar with validation
    pub gyro_scalar_z: f64,
}

impl GyroTrimConfigs {
    /// Constructs a new `GyroTrimConfigs` with the given scalar values.
    pub fn new(scalar_x: f64, scalar_y: f64, scalar_z: f64) -> Self {
        Self {
            gyro_scalar_x: scalar_x,
            gyro_scalar_y: scalar_y,
            gyro_scalar_z: scalar_z,
        }
    }

    /// Returns the configuration with the given scalar x set.
    ///
    /// The scalar is in `degrees per rotation` and is clamped from -180 to 180.
    pub fn with_scalar_x(mut self, scalar_x: f64) -> Self {
        self.gyro_scalar_x = scalar_x.clamp(-180.0, 180.0);
        self
    }

    /// Returns the configuration with the given scalar y set.
    ///
    /// The scalar is in `degrees per rotation` and is clamped from -180 to 180.
    pub fn with_scalar_y(mut self, scalar_y: f64) -> Self {
        self.gyro_scalar_y = scalar_y.clamp(-180.0, 180.0);
        self
    }

    /// Returns the configuration with the given scalar z set.
    ///
    /// The scalar is in `degrees per rotation` and is clamped from -180 to 180.
    pub fn with_scalar_z(mut self, scalar_z: f64) -> Self {
        self.gyro_scalar_z = scalar_z.clamp(-180.0, 180.0);
        self
    }

    /// Returns the scalar x of the configuration.
    ///
    /// The scalar is in `degrees per rotation`.
    pub fn scalar_x(&self) -> f64 {
        self.gyro_scalar_x
    }

    /// Returns the scalar y of the configuration.
    ///
    /// The scalar is in `degrees per rotation`.
    pub fn scalar_y(&self) -> f64 {
        self.gyro_scalar_y
    }

    /// Returns the scalar z of the configuration.
    ///
    /// The scalar is in `degrees per rotation`.
    pub fn scalar_z(&self) -> f64 {
        self.gyro_scalar_z
    }
}

impl std::fmt::Display for GyroTrimConfigs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GyroTrimConfigs {{ scalar_x: {}, scalar_y: {}, scalar_z: {} }}",
            self.gyro_scalar_x, self.gyro_scalar_y, self.gyro_scalar_z
        )
    }
}

seal! {GyroTrimConfigs}
impl ConfigProtocol for GyroTrimConfigs {
    fn serialize(&self) -> Status<String> {
        let mut ss = String::new();
        ss.push_str(&serialize_double(
            SPN::PIGEON2_GYRO_SCALARX,
            self.gyro_scalar_x,
        )?);
        ss.push_str(&serialize_double(
            SPN::PIGEON2_GYRO_SCALARY,
            self.gyro_scalar_y,
        )?);
        ss.push_str(&serialize_double(
            SPN::PIGEON2_GYRO_SCALARZ,
            self.gyro_scalar_z,
        )?);
        Ok(ss)
    }

    fn deserialize(to_deserialize: &str) -> Status<Self> {
        Ok(Self {
            gyro_scalar_x: deserialize_double(SPN::PIGEON2_GYRO_SCALARX, to_deserialize)?,
            gyro_scalar_y: deserialize_double(SPN::PIGEON2_GYRO_SCALARY, to_deserialize)?,
            gyro_scalar_z: deserialize_double(SPN::PIGEON2_GYRO_SCALARZ, to_deserialize)?,
        })
    }
}
impl PigeonConfigType for GyroTrimConfigs {}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pigeon2FeaturesConfigs {
    /// Turns on or off the magnetometer fusing for 9-axis. FRC users are not
    /// recommended to turn this on, as the magnetic influence of the robot
    /// will likely negatively affect the performance of the Pigeon2.
    pub enable_compass: bool,
    /// Disables using the temperature compensation feature
    pub disable_temperature_compensation: bool,
    /// Disables using the no-motion calibration feature
    pub disable_no_motion_calibration: bool,
}

impl Pigeon2FeaturesConfigs {
    /// Constructs a new `Pigeon2FeaturesConfigs` with the given features enabled/disabled.
    pub fn new(
        enable_compass: bool,
        disable_temperature_compensation: bool,
        disable_no_motion_calibration: bool,
    ) -> Self {
        Self {
            enable_compass,
            disable_temperature_compensation,
            disable_no_motion_calibration,
        }
    }

    /// Returns the configuration with the compass enabled.
    pub fn with_compass_enabled(mut self) -> Self {
        self.enable_compass = true;
        self
    }

    /// Returns the configuration with the compass disabled.
    pub fn with_compass_disabled(mut self) -> Self {
        self.enable_compass = false;
        self
    }

    /// Returns the configuration with the temperature compensation disabled.
    pub fn with_temperature_compensation_disabled(mut self) -> Self {
        self.disable_temperature_compensation = true;
        self
    }

    /// Returns the configuration with the temperature compensation enabled.
    pub fn with_temperature_compensation_enabled(mut self) -> Self {
        self.disable_temperature_compensation = false;
        self
    }

    /// Returns the configuration with the no motion calibration disabled.
    pub fn with_no_motion_calibration_disabled(mut self) -> Self {
        self.disable_no_motion_calibration = true;
        self
    }

    /// Returns the configuration with the no motion calibration enabled.
    pub fn with_no_motion_calibration_enabled(mut self) -> Self {
        self.disable_no_motion_calibration = false;
        self
    }

    /// Returns whether the compass is enabled.
    pub fn is_compass_enabled(&self) -> bool {
        self.enable_compass
    }

    /// Returns whether the temperature compensation is enabled.
    pub fn is_temperature_compensation_enabled(&self) -> bool {
        !self.disable_temperature_compensation
    }

    /// Returns whether the no motion calibration is enabled.
    pub fn is_no_motion_calibration_enabled(&self) -> bool {
        !self.disable_no_motion_calibration
    }
}

impl std::fmt::Display for Pigeon2FeaturesConfigs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pigeon2FeaturesConfigs {{ compass_enabled: {}, temperature_compensation_enabled: {}, no_motion_calibration_enabled: {} }}",
            self.is_compass_enabled(), self.is_temperature_compensation_enabled(), self.is_no_motion_calibration_enabled()
        )
    }
}

seal! {Pigeon2FeaturesConfigs}
impl ConfigProtocol for Pigeon2FeaturesConfigs {
    fn serialize(&self) -> Status<String> {
        let mut ss = String::new();
        ss.push_str(&serialize_bool(
            SPN::PIGEON2_USE_COMPASS,
            self.enable_compass,
        )?);
        ss.push_str(&serialize_bool(
            SPN::PIGEON2_DISABLE_TEMPERATURE_COMPENSATION,
            self.disable_temperature_compensation,
        )?);
        ss.push_str(&serialize_bool(
            SPN::PIGEON2_DISABLE_NO_MOTION_CALIBRATION,
            self.disable_no_motion_calibration,
        )?);
        Ok(ss)
    }

    fn deserialize(to_deserialize: &str) -> Status<Self> {
        Ok(Self {
            enable_compass: deserialize_bool(SPN::PIGEON2_USE_COMPASS, to_deserialize)?,
            disable_temperature_compensation: deserialize_bool(
                SPN::PIGEON2_DISABLE_TEMPERATURE_COMPENSATION,
                to_deserialize,
            )?,
            disable_no_motion_calibration: deserialize_bool(
                SPN::PIGEON2_DISABLE_NO_MOTION_CALIBRATION,
                to_deserialize,
            )?,
        })
    }
}
impl PigeonConfigType for Pigeon2FeaturesConfigs {}
