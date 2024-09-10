#![allow(clippy::useless_conversion, dead_code)]
//! This module contains all of the control requests that can be sent to a Phoenix 6 motor controller.
use crate::{devices::DeviceIdentifier, error::StatusCodeType, Status};
/// Request coast neutral output of actuator.
/// The bridge is disabled and the rotor is allowed to coast.
pub struct CoastOut {
    pub update_freq_hz: f64,
}
impl CoastOut {
    pub fn new() -> Self {
        Self {
            update_freq_hz: 100.0,
        }
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlCoastOut(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
        )
        .to_result()
    }
}
impl Default for CoastOut {
    fn default() -> Self {
        Self::new()
    }
}

/// Request a specified motor duty cycle with a differential position closed-loop.
/// This control mode will output a proportion of the supplied voltage which is supplied by the user.
/// It will also set the motor's differential position setpoint to the specified position.
pub struct DifferentialDutyCycle {
    pub target_output: f64,
    pub differential_position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Select which gains are applied to the differential controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub differential_slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DifferentialDutyCycle {
    pub fn new() -> Self {
        Self {
            target_output: f64::default(),
            differential_position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            differential_slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's target_output parameter
    /// and returns itself for method chaining.
    pub fn with_target_output(mut self, new_target_output: f64) -> Self {
        self.target_output = new_target_output;
        self
    }
    /// Modifies this Control Request's differential_position parameter
    /// and returns itself for method chaining.
    pub fn with_differential_position(
        mut self,
        new_differential_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.differential_position = new_differential_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's differential_slot parameter
    /// and returns itself for method chaining.
    pub fn with_differential_slot(mut self, new_differential_slot: i32) -> Self {
        self.differential_slot = new_differential_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialDutyCycle(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.target_output.into(),
            self.differential_position.into(),
            self.enable_foc.into(),
            self.differential_slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialDutyCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Follow the differential motor output of another Talon.
/// If Talon is in torque control, the torque is copied - which will increase the total torque applied.
/// If Talon is in percent supply output control, the duty cycle is matched.
/// Motor direction either matches master's configured direction or opposes it based on OpposeMasterDirection.
pub struct DifferentialFollower {
    /// Device ID of the differential master to follow.
    pub master_id: i32,
    /// Set to false for motor invert to match the master's configured Invert - which is typical when master and follower are mechanically linked and spin in the same direction.
    /// Set to true for motor invert to oppose the master's configured Invert - this is typical where the the master and follower mechanically spin in opposite directions.
    pub oppose_master_direction: bool,
    pub update_freq_hz: f64,
}
impl DifferentialFollower {
    pub fn new() -> Self {
        Self {
            master_id: i32::default(),
            oppose_master_direction: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's master_id parameter
    /// and returns itself for method chaining.
    pub fn with_master_id(mut self, new_master_id: i32) -> Self {
        self.master_id = new_master_id;
        self
    }
    /// Modifies this Control Request's oppose_master_direction parameter
    /// and returns itself for method chaining.
    pub fn with_oppose_master_direction(mut self, new_oppose_master_direction: bool) -> Self {
        self.oppose_master_direction = new_oppose_master_direction;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialFollower(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.master_id.into(),
            self.oppose_master_direction.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialFollower {
    fn default() -> Self {
        Self::new()
    }
}

/// Requests Motion Magic® to target a final position using a motion profile, and PID to a differential position setpoint.
/// Motion Magic® produces a motion profile in real-time while attempting to honor the Cruise Velocity, Acceleration, and Jerk value specified via the Motion Magic® configuration values.
/// This control mode does not use the Expo_kV or Expo_kA configs.
/// Target position can be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is duty cycle based, so relevant closed-loop gains will use fractional duty cycle for the numerator: +1.
/// 0 represents full forward output.
pub struct DifferentialMotionMagicDutyCycle {
    /// Average position to drive toward in rotations.
    pub target_position: frclib_core::units::angle::Rotation,
    /// Differential position to drive toward in rotations.
    pub differential_position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Select which gains are applied to the primary controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub target_slot: i32,
    /// Select which gains are applied to the differential controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub differential_slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DifferentialMotionMagicDutyCycle {
    pub fn new() -> Self {
        Self {
            target_position: frclib_core::units::angle::Rotation::default(),
            differential_position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            target_slot: i32::default(),
            differential_slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's target_position parameter
    /// and returns itself for method chaining.
    pub fn with_target_position(
        mut self,
        new_target_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.target_position = new_target_position;
        self
    }
    /// Modifies this Control Request's differential_position parameter
    /// and returns itself for method chaining.
    pub fn with_differential_position(
        mut self,
        new_differential_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.differential_position = new_differential_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's target_slot parameter
    /// and returns itself for method chaining.
    pub fn with_target_slot(mut self, new_target_slot: i32) -> Self {
        self.target_slot = new_target_slot;
        self
    }
    /// Modifies this Control Request's differential_slot parameter
    /// and returns itself for method chaining.
    pub fn with_differential_slot(mut self, new_differential_slot: i32) -> Self {
        self.differential_slot = new_differential_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialMotionMagicDutyCycle(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.target_position.into(),
            self.differential_position.into(),
            self.enable_foc.into(),
            self.target_slot.into(),
            self.differential_slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialMotionMagicDutyCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Requests Motion Magic® to target a final position using a motion profile, and PID to a differential position setpoint.
/// Motion Magic® produces a motion profile in real-time while attempting to honor the Cruise Velocity, Acceleration, and Jerk value specified via the Motion Magic® configuration values.
/// This control mode does not use the Expo_kV or Expo_kA configs.
/// Target position can be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is voltage-based, so relevant closed-loop gains will use Volts for the numerator.
pub struct DifferentialMotionMagicVoltage {
    /// Average position to drive toward in rotations.
    pub target_position: frclib_core::units::angle::Rotation,
    /// Differential position to drive toward in rotations.
    pub differential_position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Select which gains are applied to the primary controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub target_slot: i32,
    /// Select which gains are applied to the differential controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub differential_slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DifferentialMotionMagicVoltage {
    pub fn new() -> Self {
        Self {
            target_position: frclib_core::units::angle::Rotation::default(),
            differential_position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            target_slot: i32::default(),
            differential_slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's target_position parameter
    /// and returns itself for method chaining.
    pub fn with_target_position(
        mut self,
        new_target_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.target_position = new_target_position;
        self
    }
    /// Modifies this Control Request's differential_position parameter
    /// and returns itself for method chaining.
    pub fn with_differential_position(
        mut self,
        new_differential_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.differential_position = new_differential_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's target_slot parameter
    /// and returns itself for method chaining.
    pub fn with_target_slot(mut self, new_target_slot: i32) -> Self {
        self.target_slot = new_target_slot;
        self
    }
    /// Modifies this Control Request's differential_slot parameter
    /// and returns itself for method chaining.
    pub fn with_differential_slot(mut self, new_differential_slot: i32) -> Self {
        self.differential_slot = new_differential_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialMotionMagicVoltage(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.target_position.into(),
            self.differential_position.into(),
            self.enable_foc.into(),
            self.target_slot.into(),
            self.differential_slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialMotionMagicVoltage {
    fn default() -> Self {
        Self::new()
    }
}

/// Request PID to target position with a differential position setpoint.
/// This control mode will set the motor's position setpoint to the position specified by the user.
/// It will also set the motor's differential position setpoint to the specified position.
pub struct DifferentialPositionDutyCycle {
    /// Average position to drive toward in rotations.
    pub target_position: frclib_core::units::angle::Rotation,
    /// Differential position to drive toward in rotations.
    pub differential_position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Select which gains are applied to the primary controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub target_slot: i32,
    /// Select which gains are applied to the differential controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub differential_slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DifferentialPositionDutyCycle {
    pub fn new() -> Self {
        Self {
            target_position: frclib_core::units::angle::Rotation::default(),
            differential_position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            target_slot: i32::default(),
            differential_slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's target_position parameter
    /// and returns itself for method chaining.
    pub fn with_target_position(
        mut self,
        new_target_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.target_position = new_target_position;
        self
    }
    /// Modifies this Control Request's differential_position parameter
    /// and returns itself for method chaining.
    pub fn with_differential_position(
        mut self,
        new_differential_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.differential_position = new_differential_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's target_slot parameter
    /// and returns itself for method chaining.
    pub fn with_target_slot(mut self, new_target_slot: i32) -> Self {
        self.target_slot = new_target_slot;
        self
    }
    /// Modifies this Control Request's differential_slot parameter
    /// and returns itself for method chaining.
    pub fn with_differential_slot(mut self, new_differential_slot: i32) -> Self {
        self.differential_slot = new_differential_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialPositionDutyCycle(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.target_position.into(),
            self.differential_position.into(),
            self.enable_foc.into(),
            self.target_slot.into(),
            self.differential_slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialPositionDutyCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Request PID to target position with a differential position setpoint This control mode will set the motor's position setpoint to the position specified by the user.
/// It will also set the motor's differential position setpoint to the specified position.
pub struct DifferentialPositionVoltage {
    /// Average position to drive toward in rotations.
    pub target_position: frclib_core::units::angle::Rotation,
    /// Differential position to drive toward in rotations.
    pub differential_position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Select which gains are applied to the primary controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub target_slot: i32,
    /// Select which gains are applied to the differential controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub differential_slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DifferentialPositionVoltage {
    pub fn new() -> Self {
        Self {
            target_position: frclib_core::units::angle::Rotation::default(),
            differential_position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            target_slot: i32::default(),
            differential_slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's target_position parameter
    /// and returns itself for method chaining.
    pub fn with_target_position(
        mut self,
        new_target_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.target_position = new_target_position;
        self
    }
    /// Modifies this Control Request's differential_position parameter
    /// and returns itself for method chaining.
    pub fn with_differential_position(
        mut self,
        new_differential_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.differential_position = new_differential_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's target_slot parameter
    /// and returns itself for method chaining.
    pub fn with_target_slot(mut self, new_target_slot: i32) -> Self {
        self.target_slot = new_target_slot;
        self
    }
    /// Modifies this Control Request's differential_slot parameter
    /// and returns itself for method chaining.
    pub fn with_differential_slot(mut self, new_differential_slot: i32) -> Self {
        self.differential_slot = new_differential_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialPositionVoltage(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.target_position.into(),
            self.differential_position.into(),
            self.enable_foc.into(),
            self.target_slot.into(),
            self.differential_slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialPositionVoltage {
    fn default() -> Self {
        Self::new()
    }
}

/// Follow the differential motor output of another Talon while ignoring the master's invert setting.
/// If Talon is in torque control, the torque is copied - which will increase the total torque applied.
/// If Talon is in percent supply output control, the duty cycle is matched.
/// Motor direction is strictly determined by the configured invert and not the master.
/// If you want motor direction to match or oppose the master, use FollowerRequest instead.
pub struct DifferentialStrictFollower {
    /// Device ID of the differential master to follow.
    pub master_id: i32,
    pub update_freq_hz: f64,
}
impl DifferentialStrictFollower {
    pub fn new() -> Self {
        Self {
            master_id: i32::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's master_id parameter
    /// and returns itself for method chaining.
    pub fn with_master_id(mut self, new_master_id: i32) -> Self {
        self.master_id = new_master_id;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialStrictFollower(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.master_id.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialStrictFollower {
    fn default() -> Self {
        Self::new()
    }
}

/// Request PID to target velocity with a differential position setpoint.
/// This control mode will set the motor's velocity setpoint to the velocity specified by the user.
/// It will also set the motor's differential position setpoint to the specified position.
pub struct DifferentialVelocityDutyCycle {
    /// Average velocity to drive toward in rotations per second.
    pub target_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Differential position to drive toward in rotations.
    pub differential_position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Select which gains are applied to the primary controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub target_slot: i32,
    /// Select which gains are applied to the differential controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub differential_slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DifferentialVelocityDutyCycle {
    pub fn new() -> Self {
        Self {
            target_velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            differential_position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            target_slot: i32::default(),
            differential_slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's target_velocity parameter
    /// and returns itself for method chaining.
    pub fn with_target_velocity(
        mut self,
        new_target_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.target_velocity = new_target_velocity;
        self
    }
    /// Modifies this Control Request's differential_position parameter
    /// and returns itself for method chaining.
    pub fn with_differential_position(
        mut self,
        new_differential_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.differential_position = new_differential_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's target_slot parameter
    /// and returns itself for method chaining.
    pub fn with_target_slot(mut self, new_target_slot: i32) -> Self {
        self.target_slot = new_target_slot;
        self
    }
    /// Modifies this Control Request's differential_slot parameter
    /// and returns itself for method chaining.
    pub fn with_differential_slot(mut self, new_differential_slot: i32) -> Self {
        self.differential_slot = new_differential_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialVelocityDutyCycle(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.target_velocity.into(),
            self.differential_position.into(),
            self.enable_foc.into(),
            self.target_slot.into(),
            self.differential_slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialVelocityDutyCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Request PID to target velocity with a differential position setpoint.
/// This control mode will set the motor's velocity setpoint to the velocity specified by the user.
/// It will also set the motor's differential position setpoint to the specified position.
pub struct DifferentialVelocityVoltage {
    /// Average velocity to drive toward in rotations per second.
    pub target_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Differential position to drive toward in rotations.
    pub differential_position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Select which gains are applied to the primary controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub target_slot: i32,
    /// Select which gains are applied to the differential controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub differential_slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DifferentialVelocityVoltage {
    pub fn new() -> Self {
        Self {
            target_velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            differential_position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            target_slot: i32::default(),
            differential_slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's target_velocity parameter
    /// and returns itself for method chaining.
    pub fn with_target_velocity(
        mut self,
        new_target_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.target_velocity = new_target_velocity;
        self
    }
    /// Modifies this Control Request's differential_position parameter
    /// and returns itself for method chaining.
    pub fn with_differential_position(
        mut self,
        new_differential_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.differential_position = new_differential_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's target_slot parameter
    /// and returns itself for method chaining.
    pub fn with_target_slot(mut self, new_target_slot: i32) -> Self {
        self.target_slot = new_target_slot;
        self
    }
    /// Modifies this Control Request's differential_slot parameter
    /// and returns itself for method chaining.
    pub fn with_differential_slot(mut self, new_differential_slot: i32) -> Self {
        self.differential_slot = new_differential_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialVelocityVoltage(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.target_velocity.into(),
            self.differential_position.into(),
            self.enable_foc.into(),
            self.target_slot.into(),
            self.differential_slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialVelocityVoltage {
    fn default() -> Self {
        Self::new()
    }
}

/// Request a specified voltage with a differential position closed-loop.
/// This control mode will attempt to apply the specified voltage to the motor.
/// If the supply voltage is below the requested voltage, the motor controller will output the supply voltage.
/// It will also set the motor's differential position setpoint to the specified position.
pub struct DifferentialVoltage {
    pub target_output: frclib_core::units::energy::Volt,
    pub differential_position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Select which gains are applied to the differential controller by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub differential_slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DifferentialVoltage {
    pub fn new() -> Self {
        Self {
            target_output: frclib_core::units::energy::Volt::default(),
            differential_position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            differential_slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's target_output parameter
    /// and returns itself for method chaining.
    pub fn with_target_output(
        mut self,
        new_target_output: frclib_core::units::energy::Volt,
    ) -> Self {
        self.target_output = new_target_output;
        self
    }
    /// Modifies this Control Request's differential_position parameter
    /// and returns itself for method chaining.
    pub fn with_differential_position(
        mut self,
        new_differential_position: frclib_core::units::angle::Rotation,
    ) -> Self {
        self.differential_position = new_differential_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's differential_slot parameter
    /// and returns itself for method chaining.
    pub fn with_differential_slot(mut self, new_differential_slot: i32) -> Self {
        self.differential_slot = new_differential_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDifferentialVoltage(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.target_output.into(),
            self.differential_position.into(),
            self.enable_foc.into(),
            self.differential_slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DifferentialVoltage {
    fn default() -> Self {
        Self::new()
    }
}

/// Request a specified motor duty cycle.
/// This control mode will output a proportion of the supplied voltage which is supplied by the user.
pub struct DutyCycleOut {
    pub output: f64,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DutyCycleOut {
    pub fn new() -> Self {
        Self {
            output: f64::default(),
            enable_foc: bool::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's output parameter
    /// and returns itself for method chaining.
    pub fn with_output(mut self, new_output: f64) -> Self {
        self.output = new_output;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDutyCycleOut(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.output.into(),
            self.enable_foc.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DutyCycleOut {
    fn default() -> Self {
        Self::new()
    }
}

/// Requires Phoenix Pro and CANivore; Requests Motion Magic® to target a final position using a motion profile.
/// This dynamic request allows runtime changes to Cruise Velocity, Acceleration, and Jerk.
/// Users can optionally provide a duty cycle feedforward.
/// This control requires use of a CANivore.
/// Motion Magic® produces a motion profile in real-time while attempting to honor the specified Cruise Velocity, Acceleration, and Jerk value.
/// This control mode does not use the Expo_kV or Expo_kA configs.
/// Target position can be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is duty cycle based, so relevant closed-loop gains will use fractional duty cycle for the numerator: +1.
/// 0 represents full forward output.
pub struct DynamicMotionMagicDutyCycle {
    /// Position to drive toward in rotations.
    pub position: frclib_core::units::angle::Rotation,
    /// Cruise velocity for profiling.
    /// The signage does not matter as the device will use the absolute value for profile generation.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Acceleration for profiling.
    pub acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    /// Jerk for profiling.
    pub jerk: f64,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Feedforward to apply in fractional units between -1 and +1.
    pub feed_forward: f64,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DynamicMotionMagicDutyCycle {
    pub fn new() -> Self {
        Self {
            position: frclib_core::units::angle::Rotation::default(),
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr::default(),
            jerk: f64::default(),
            enable_foc: bool::default(),
            feed_forward: f64::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's position parameter
    /// and returns itself for method chaining.
    pub fn with_position(mut self, new_position: frclib_core::units::angle::Rotation) -> Self {
        self.position = new_position;
        self
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's acceleration parameter
    /// and returns itself for method chaining.
    pub fn with_acceleration(
        mut self,
        new_acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    ) -> Self {
        self.acceleration = new_acceleration;
        self
    }
    /// Modifies this Control Request's jerk parameter
    /// and returns itself for method chaining.
    pub fn with_jerk(mut self, new_jerk: f64) -> Self {
        self.jerk = new_jerk;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: f64) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDynamicMotionMagicDutyCycle(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.position.into(),
            self.velocity.into(),
            self.acceleration.into(),
            self.jerk.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DynamicMotionMagicDutyCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Requires Phoenix Pro and CANivore; Requests Motion Magic® to target a final position using a motion profile.
/// This dynamic request allows runtime changes to Cruise Velocity, Acceleration, and Jerk.
/// Users can optionally provide a torque current feedforward.
/// This control requires use of a CANivore.
/// Motion Magic® produces a motion profile in real-time while attempting to honor the specified Cruise Velocity, Acceleration, and Jerk value.
/// This control mode does not use the Expo_kV or Expo_kA configs.
/// Target position can be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is based on torque current, so relevant closed-loop gains will use Amperes for the numerator.
pub struct DynamicMotionMagicTorqueCurrentFOC {
    /// Position to drive toward in rotations.
    pub position: frclib_core::units::angle::Rotation,
    /// Cruise velocity for profiling.
    /// The signage does not matter as the device will use the absolute value for profile generation.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Acceleration for profiling.
    /// The signage does not matter as the device will use the absolute value for profile generation.
    pub acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    /// Jerk for profiling.
    /// The signage does not matter as the device will use the absolute value for profile generation.
    pub jerk: f64,
    /// Feedforward to apply in torque current in Amperes.
    /// User can use motor's kT to scale Newton-meter to Amperes.
    pub feed_forward: frclib_core::units::energy::Amp,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to coast the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0A (zero torque).
    pub override_coast_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DynamicMotionMagicTorqueCurrentFOC {
    pub fn new() -> Self {
        Self {
            position: frclib_core::units::angle::Rotation::default(),
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr::default(),
            jerk: f64::default(),
            feed_forward: frclib_core::units::energy::Amp::default(),
            slot: i32::default(),
            override_coast_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's position parameter
    /// and returns itself for method chaining.
    pub fn with_position(mut self, new_position: frclib_core::units::angle::Rotation) -> Self {
        self.position = new_position;
        self
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's acceleration parameter
    /// and returns itself for method chaining.
    pub fn with_acceleration(
        mut self,
        new_acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    ) -> Self {
        self.acceleration = new_acceleration;
        self
    }
    /// Modifies this Control Request's jerk parameter
    /// and returns itself for method chaining.
    pub fn with_jerk(mut self, new_jerk: f64) -> Self {
        self.jerk = new_jerk;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Amp) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_coast_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_coast_dur_neutral(mut self, new_override_coast_dur_neutral: bool) -> Self {
        self.override_coast_dur_neutral = new_override_coast_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDynamicMotionMagicTorqueCurrentFOC(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.position.into(),
            self.velocity.into(),
            self.acceleration.into(),
            self.jerk.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_coast_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DynamicMotionMagicTorqueCurrentFOC {
    fn default() -> Self {
        Self::new()
    }
}

/// Requires Phoenix Pro and CANivore; Requests Motion Magic® to target a final position using a motion profile.
/// This dynamic request allows runtime changes to Cruise Velocity, Acceleration, and Jerk.
/// Users can optionally provide a voltage feedforward.
/// This control requires use of a CANivore.
/// Motion Magic® produces a motion profile in real-time while attempting to honor the specified Cruise Velocity, Acceleration, and Jerk value.
/// This control mode does not use the Expo_kV or Expo_kA configs.
/// Target position can be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is voltage-based, so relevant closed-loop gains will use Volts for the numerator.
pub struct DynamicMotionMagicVoltage {
    /// Position to drive toward in rotations.
    pub position: frclib_core::units::angle::Rotation,
    /// Cruise velocity for profiling.
    /// The signage does not matter as the device will use the absolute value for profile generation.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Acceleration for profiling.
    /// The signage does not matter as the device will use the absolute value for profile generation.
    pub acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    /// Jerk for profiling.
    /// The signage does not matter as the device will use the absolute value for profile generation.
    pub jerk: f64,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    pub feed_forward: frclib_core::units::energy::Volt,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl DynamicMotionMagicVoltage {
    pub fn new() -> Self {
        Self {
            position: frclib_core::units::angle::Rotation::default(),
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr::default(),
            jerk: f64::default(),
            enable_foc: bool::default(),
            feed_forward: frclib_core::units::energy::Volt::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's position parameter
    /// and returns itself for method chaining.
    pub fn with_position(mut self, new_position: frclib_core::units::angle::Rotation) -> Self {
        self.position = new_position;
        self
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's acceleration parameter
    /// and returns itself for method chaining.
    pub fn with_acceleration(
        mut self,
        new_acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    ) -> Self {
        self.acceleration = new_acceleration;
        self
    }
    /// Modifies this Control Request's jerk parameter
    /// and returns itself for method chaining.
    pub fn with_jerk(mut self, new_jerk: f64) -> Self {
        self.jerk = new_jerk;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Volt) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlDynamicMotionMagicVoltage(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.position.into(),
            self.velocity.into(),
            self.acceleration.into(),
            self.jerk.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for DynamicMotionMagicVoltage {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic Empty Control class used to do nothing.
pub struct EmptyControl {
    pub update_freq_hz: f64,
}
impl EmptyControl {
    pub fn new() -> Self {
        Self {
            update_freq_hz: 100.0,
        }
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlEmpty(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
        )
        .to_result()
    }
}
impl Default for EmptyControl {
    fn default() -> Self {
        Self::new()
    }
}

/// Follow the motor output of another Talon.
/// If Talon is in torque control, the torque is copied - which will increase the total torque applied.
/// If Talon is in percent supply output control, the duty cycle is matched.
/// Motor direction either matches master's configured direction or opposes it based on OpposeMasterDirection.
pub struct Follower {
    /// Device ID of the master to follow.
    pub master_id: i32,
    /// Set to false for motor invert to match the master's configured Invert - which is typical when master and follower are mechanically linked and spin in the same direction.
    /// Set to true for motor invert to oppose the master's configured Invert - this is typical where the the master and follower mechanically spin in opposite directions.
    pub oppose_master_direction: bool,
    pub update_freq_hz: f64,
}
impl Follower {
    pub fn new() -> Self {
        Self {
            master_id: i32::default(),
            oppose_master_direction: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's master_id parameter
    /// and returns itself for method chaining.
    pub fn with_master_id(mut self, new_master_id: i32) -> Self {
        self.master_id = new_master_id;
        self
    }
    /// Modifies this Control Request's oppose_master_direction parameter
    /// and returns itself for method chaining.
    pub fn with_oppose_master_direction(mut self, new_oppose_master_direction: bool) -> Self {
        self.oppose_master_direction = new_oppose_master_direction;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlFollower(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.master_id.into(),
            self.oppose_master_direction.into(),
        )
        .to_result()
    }
}
impl Default for Follower {
    fn default() -> Self {
        Self::new()
    }
}

/// Requests Motion Magic® to target a final position using a motion profile.
/// Users can optionally provide a duty cycle feedforward.
/// Motion Magic® produces a motion profile in real-time while attempting to honor the Cruise Velocity, Acceleration, and Jerk value specified via the Motion Magic® configuration values.
/// This control mode does not use the Expo_kV or Expo_kA configs.
/// Target position can be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is duty cycle based, so relevant closed-loop gains will use fractional duty cycle for the numerator: +1.
/// 0 represents full forward output.
pub struct MotionMagicDutyCycle {
    /// Position to drive toward in rotations.
    pub position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Feedforward to apply in fractional units between -1 and +1.
    pub feed_forward: f64,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl MotionMagicDutyCycle {
    pub fn new() -> Self {
        Self {
            position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            feed_forward: f64::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's position parameter
    /// and returns itself for method chaining.
    pub fn with_position(mut self, new_position: frclib_core::units::angle::Rotation) -> Self {
        self.position = new_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: f64) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlMotionMagicDutyCycle(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.position.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for MotionMagicDutyCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Requires Phoenix Pro; Requests Motion Magic® to target a final position using a motion profile.
/// Users can optionally provide a torque current feedforward.
/// Motion Magic® produces a motion profile in real-time while attempting to honor the Cruise Velocity, Acceleration, and Jerk value specified via the Motion Magic® configuration values.
/// This control mode does not use the Expo_kV or Expo_kA configs.
/// Target position can be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is based on torque current, so relevant closed-loop gains will use Amperes for the numerator.
pub struct MotionMagicTorqueCurrentFOC {
    /// Position to drive toward in rotations.
    pub position: frclib_core::units::angle::Rotation,
    /// Feedforward to apply in torque current in Amperes.
    /// User can use motor's kT to scale Newton-meter to Amperes.
    pub feed_forward: frclib_core::units::energy::Amp,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to coast the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0A (zero torque).
    pub override_coast_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl MotionMagicTorqueCurrentFOC {
    pub fn new() -> Self {
        Self {
            position: frclib_core::units::angle::Rotation::default(),
            feed_forward: frclib_core::units::energy::Amp::default(),
            slot: i32::default(),
            override_coast_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's position parameter
    /// and returns itself for method chaining.
    pub fn with_position(mut self, new_position: frclib_core::units::angle::Rotation) -> Self {
        self.position = new_position;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Amp) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_coast_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_coast_dur_neutral(mut self, new_override_coast_dur_neutral: bool) -> Self {
        self.override_coast_dur_neutral = new_override_coast_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlMotionMagicTorqueCurrentFOC(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.position.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_coast_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for MotionMagicTorqueCurrentFOC {
    fn default() -> Self {
        Self::new()
    }
}

/// Requests Motion Magic® to target a final velocity using a motion profile.
/// This allows smooth transitions between velocity set points.
/// Users can optionally provide a duty cycle feedforward.
/// Motion Magic® Velocity produces a motion profile in real-time while attempting to honor the specified Acceleration and Jerk value.
/// This control mode does not use the CruiseVelocity, Expo_kV, or Expo_kA configs.
/// If the specified acceleration is zero, the Acceleration under Motion Magic® configuration parameter is used instead.
/// This allows for runtime adjustment of acceleration for advanced users.
/// Jerk is also specified in the Motion Magic® persistent configuration values.
/// If Jerk is set to zero, Motion Magic® will produce a trapezoidal acceleration profile.
/// Target velocity can also be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is duty cycle based, so relevant closed-loop gains will use fractional duty cycle for the numerator: +1.
/// 0 represents full forward output.
pub struct MotionMagicVelocityDutyCycle {
    /// Target velocity to drive toward in rotations per second.
    /// This can be changed on-the fly.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// This is the absolute Acceleration to use generating the profile.
    /// If this parameter is zero, the Acceleration persistent configuration parameter is used instead.
    /// Acceleration is in rotations per second squared.
    /// If nonzero, the signage does not matter as the absolute value is used.
    pub acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Feedforward to apply in fractional units between -1 and +1.
    pub feed_forward: f64,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl MotionMagicVelocityDutyCycle {
    pub fn new() -> Self {
        Self {
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr::default(),
            enable_foc: bool::default(),
            feed_forward: f64::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's acceleration parameter
    /// and returns itself for method chaining.
    pub fn with_acceleration(
        mut self,
        new_acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    ) -> Self {
        self.acceleration = new_acceleration;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: f64) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlMotionMagicVelocityDutyCycle(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.velocity.into(),
            self.acceleration.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for MotionMagicVelocityDutyCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Requests Motion Magic® to target a final velocity using a motion profile.
/// This allows smooth transitions between velocity set points.
/// Users can optionally provide a torque feedforward.
/// Motion Magic® Velocity produces a motion profile in real-time while attempting to honor the specified Acceleration and Jerk value.
/// This control mode does not use the CruiseVelocity, Expo_kV, or Expo_kA configs.
/// If the specified acceleration is zero, the Acceleration under Motion Magic® configuration parameter is used instead.
/// This allows for runtime adjustment of acceleration for advanced users.
/// Jerk is also specified in the Motion Magic® persistent configuration values.
/// If Jerk is set to zero, Motion Magic® will produce a trapezoidal acceleration profile.
/// Target velocity can also be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is based on torque current, so relevant closed-loop gains will use Amperes for the numerator.
pub struct MotionMagicVelocityTorqueCurrentFOC {
    /// Target velocity to drive toward in rotations per second.
    /// This can be changed on-the fly.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// This is the absolute Acceleration to use generating the profile.
    /// If this parameter is zero, the Acceleration persistent configuration parameter is used instead.
    /// Acceleration is in rotations per second squared.
    /// If nonzero, the signage does not matter as the absolute value is used.
    pub acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Feedforward to apply in torque current in Amperes.
    /// User can use motor's kT to scale Newton-meter to Amperes.
    pub feed_forward: frclib_core::units::energy::Amp,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to coast the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0A (zero torque).
    pub override_coast_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl MotionMagicVelocityTorqueCurrentFOC {
    pub fn new() -> Self {
        Self {
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr::default(),
            enable_foc: bool::default(),
            feed_forward: frclib_core::units::energy::Amp::default(),
            slot: i32::default(),
            override_coast_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's acceleration parameter
    /// and returns itself for method chaining.
    pub fn with_acceleration(
        mut self,
        new_acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    ) -> Self {
        self.acceleration = new_acceleration;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Amp) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_coast_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_coast_dur_neutral(mut self, new_override_coast_dur_neutral: bool) -> Self {
        self.override_coast_dur_neutral = new_override_coast_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlMotionMagicVelocityTorqueCurrentFOC(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.velocity.into(),
            self.acceleration.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_coast_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for MotionMagicVelocityTorqueCurrentFOC {
    fn default() -> Self {
        Self::new()
    }
}

/// Requests Motion Magic® to target a final velocity using a motion profile.
/// This allows smooth transitions between velocity set points.
/// Users can optionally provide a voltage feedforward.
/// Motion Magic® Velocity produces a motion profile in real-time while attempting to honor the specified Acceleration and Jerk value.
/// This control mode does not use the CruiseVelocity, Expo_kV, or Expo_kA configs.
/// If the specified acceleration is zero, the Acceleration under Motion Magic® configuration parameter is used instead.
/// This allows for runtime adjustment of acceleration for advanced users.
/// Jerk is also specified in the Motion Magic® persistent configuration values.
/// If Jerk is set to zero, Motion Magic® will produce a trapezoidal acceleration profile.
/// Target velocity can also be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is voltage-based, so relevant closed-loop gains will use Volts for the numerator.
pub struct MotionMagicVelocityVoltage {
    /// Target velocity to drive toward in rotations per second.
    /// This can be changed on-the fly.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// This is the absolute Acceleration to use generating the profile.
    /// If this parameter is zero, the Acceleration persistent configuration parameter is used instead.
    /// Acceleration is in rotations per second squared.
    /// If nonzero, the signage does not matter as the absolute value is used.
    pub acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    pub feed_forward: frclib_core::units::energy::Volt,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl MotionMagicVelocityVoltage {
    pub fn new() -> Self {
        Self {
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr::default(),
            enable_foc: bool::default(),
            feed_forward: frclib_core::units::energy::Volt::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's acceleration parameter
    /// and returns itself for method chaining.
    pub fn with_acceleration(
        mut self,
        new_acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    ) -> Self {
        self.acceleration = new_acceleration;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Volt) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlMotionMagicVelocityVoltage(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.velocity.into(),
            self.acceleration.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for MotionMagicVelocityVoltage {
    fn default() -> Self {
        Self::new()
    }
}

/// Requests Motion Magic® to target a final position using a motion profile.
/// Users can optionally provide a voltage feedforward.
/// Motion Magic® produces a motion profile in real-time while attempting to honor the Cruise Velocity, Acceleration, and Jerk value specified via the Motion Magic® configuration values.
/// This control mode does not use the Expo_kV or Expo_kA configs.
/// Target position can be changed on-the-fly and Motion Magic® will do its best to adjust the profile.
/// This control mode is voltage-based, so relevant closed-loop gains will use Volts for the numerator.
pub struct MotionMagicVoltage {
    /// Position to drive toward in rotations.
    pub position: frclib_core::units::angle::Rotation,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    pub feed_forward: frclib_core::units::energy::Volt,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl MotionMagicVoltage {
    pub fn new() -> Self {
        Self {
            position: frclib_core::units::angle::Rotation::default(),
            enable_foc: bool::default(),
            feed_forward: frclib_core::units::energy::Volt::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's position parameter
    /// and returns itself for method chaining.
    pub fn with_position(mut self, new_position: frclib_core::units::angle::Rotation) -> Self {
        self.position = new_position;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Volt) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlMotionMagicVoltage(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.position.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for MotionMagicVoltage {
    fn default() -> Self {
        Self::new()
    }
}

/// Plays a single tone at the user specified frequency.
pub struct MusicTone {
    /// Sound frequency to play.
    /// A value of zero will silence the device.
    /// The effective frequency range is 10-20000Hz.
    /// Any nonzero frequency less than 10 Hz will be capped to 10Hz.
    /// Any frequency above 20Khz will be capped to 20KHz.
    pub audio_frequency: f64,
    pub update_freq_hz: f64,
}
impl MusicTone {
    pub fn new() -> Self {
        Self {
            audio_frequency: f64::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's audio_frequency parameter
    /// and returns itself for method chaining.
    pub fn with_audio_frequency(mut self, new_audio_frequency: f64) -> Self {
        self.audio_frequency = new_audio_frequency;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlMusicTone(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.audio_frequency.into(),
        )
        .to_result()
    }
}
impl Default for MusicTone {
    fn default() -> Self {
        Self::new()
    }
}

/// Request neutral output of actuator.
/// The applied brake type is determined by the NeutralMode configuration.
pub struct NeutralOut {
    pub update_freq_hz: f64,
}
impl NeutralOut {
    pub fn new() -> Self {
        Self {
            update_freq_hz: 100.0,
        }
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlNeutralOut(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
        )
        .to_result()
    }
}
impl Default for NeutralOut {
    fn default() -> Self {
        Self::new()
    }
}

/// Request PID to target position with duty cycle feedforward.
/// This control mode will set the motor's position setpoint to the position specified by the user.
/// In addition, it will apply an additional duty cycle as an arbitrary feedforward value.
pub struct PositionDutyCycle {
    /// Position to drive toward in rotations.
    pub position: frclib_core::units::angle::Rotation,
    /// Velocity to drive toward in rotations per second.
    /// This is typically used for motion profiles generated by the robot program.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Feedforward to apply in fractional units between -1 and +1.
    pub feed_forward: f64,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl PositionDutyCycle {
    pub fn new() -> Self {
        Self {
            position: frclib_core::units::angle::Rotation::default(),
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            enable_foc: bool::default(),
            feed_forward: f64::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's position parameter
    /// and returns itself for method chaining.
    pub fn with_position(mut self, new_position: frclib_core::units::angle::Rotation) -> Self {
        self.position = new_position;
        self
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: f64) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlPositionDutyCycle(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.position.into(),
            self.velocity.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for PositionDutyCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Requires Phoenix Pro; Request PID to target position with torque current feedforward.
/// This control mode will set the motor's position setpoint to the position specified by the user.
/// In addition, it will apply an additional torque current as an arbitrary feedforward value.
pub struct PositionTorqueCurrentFOC {
    /// Position to drive toward in rotations.
    pub position: frclib_core::units::angle::Rotation,
    /// Velocity to drive toward in rotations per second.
    /// This is typically used for motion profiles generated by the robot program.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Feedforward to apply in torque current in Amperes.
    /// User can use motor's kT to scale Newton-meter to Amperes.
    pub feed_forward: frclib_core::units::energy::Amp,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to coast the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0A (zero torque).
    pub override_coast_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl PositionTorqueCurrentFOC {
    pub fn new() -> Self {
        Self {
            position: frclib_core::units::angle::Rotation::default(),
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            feed_forward: frclib_core::units::energy::Amp::default(),
            slot: i32::default(),
            override_coast_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's position parameter
    /// and returns itself for method chaining.
    pub fn with_position(mut self, new_position: frclib_core::units::angle::Rotation) -> Self {
        self.position = new_position;
        self
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Amp) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_coast_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_coast_dur_neutral(mut self, new_override_coast_dur_neutral: bool) -> Self {
        self.override_coast_dur_neutral = new_override_coast_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlPositionTorqueCurrentFOC(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.position.into(),
            self.velocity.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_coast_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for PositionTorqueCurrentFOC {
    fn default() -> Self {
        Self::new()
    }
}

/// Request PID to target position with voltage feedforward This control mode will set the motor's position setpoint to the position specified by the user.
/// In addition, it will apply an additional voltage as an arbitrary feedforward value.
pub struct PositionVoltage {
    /// Position to drive toward in rotations.
    pub position: frclib_core::units::angle::Rotation,
    /// Velocity to drive toward in rotations per second.
    /// This is typically used for motion profiles generated by the robot program.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    pub feed_forward: frclib_core::units::energy::Volt,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl PositionVoltage {
    pub fn new() -> Self {
        Self {
            position: frclib_core::units::angle::Rotation::default(),
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            enable_foc: bool::default(),
            feed_forward: frclib_core::units::energy::Volt::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's position parameter
    /// and returns itself for method chaining.
    pub fn with_position(mut self, new_position: frclib_core::units::angle::Rotation) -> Self {
        self.position = new_position;
        self
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Volt) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlPositionVoltage(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.position.into(),
            self.velocity.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for PositionVoltage {
    fn default() -> Self {
        Self::new()
    }
}

/// Applies full neutral-brake by shorting motor leads together.
pub struct StaticBrake {
    pub update_freq_hz: f64,
}
impl StaticBrake {
    pub fn new() -> Self {
        Self {
            update_freq_hz: 100.0,
        }
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlStaticBrake(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
        )
        .to_result()
    }
}
impl Default for StaticBrake {
    fn default() -> Self {
        Self::new()
    }
}

/// Follow the motor output of another Talon while ignoring the master's invert setting.
/// If Talon is in torque control, the torque is copied - which will increase the total torque applied.
/// If Talon is in percent supply output control, the duty cycle is matched.
/// Motor direction is strictly determined by the configured invert and not the master.
/// If you want motor direction to match or oppose the master, use FollowerRequest instead.
pub struct StrictFollower {
    /// Device ID of the master to follow.
    pub master_id: i32,
    pub update_freq_hz: f64,
}
impl StrictFollower {
    pub fn new() -> Self {
        Self {
            master_id: i32::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's master_id parameter
    /// and returns itself for method chaining.
    pub fn with_master_id(mut self, new_master_id: i32) -> Self {
        self.master_id = new_master_id;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlStrictFollower(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.master_id.into(),
        )
        .to_result()
    }
}
impl Default for StrictFollower {
    fn default() -> Self {
        Self::new()
    }
}

/// Requires Phoenix Pro; Request a specified motor current (field oriented control).
/// This control request will drive the motor to the requested motor (stator) current value.
/// This leverages field oriented control (FOC), which means greater peak power than what is documented.
/// This scales to torque based on Motor's kT constant.
pub struct TorqueCurrentFOC {
    pub output: frclib_core::units::energy::Amp,
    /// The maximum absolute motor output that can be applied, which effectively limits the velocity.
    /// For example, 0.
    /// 50 means no more than 50% output in either direction.
    /// This is useful for preventing the motor from spinning to its terminal velocity when there is no external torque applied unto the rotor.
    /// Note this is absolute maximum, so the value should be between zero and one.
    pub max_abs_duty_cycle: f64,
    /// Deadband in Amperes.
    /// If torque request is within deadband, the bridge output is neutral.
    /// If deadband is set to zero then there is effectively no deadband.
    /// Note if deadband is zero, a free spinning motor will spin for quite a while as the firmware attempts to hold the motor's bemf.
    /// If user expects motor to cease spinning quickly with a demand of zero, we recommend a deadband of one Ampere.
    /// This value will be converted to an integral value of amps.
    pub deadband: frclib_core::units::energy::Amp,
    /// Set to true to coast the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0A (zero torque).
    pub override_coast_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl TorqueCurrentFOC {
    pub fn new() -> Self {
        Self {
            output: frclib_core::units::energy::Amp::default(),
            max_abs_duty_cycle: f64::default(),
            deadband: frclib_core::units::energy::Amp::default(),
            override_coast_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's output parameter
    /// and returns itself for method chaining.
    pub fn with_output(mut self, new_output: frclib_core::units::energy::Amp) -> Self {
        self.output = new_output;
        self
    }
    /// Modifies this Control Request's max_abs_duty_cycle parameter
    /// and returns itself for method chaining.
    pub fn with_max_abs_duty_cycle(mut self, new_max_abs_duty_cycle: f64) -> Self {
        self.max_abs_duty_cycle = new_max_abs_duty_cycle;
        self
    }
    /// Modifies this Control Request's deadband parameter
    /// and returns itself for method chaining.
    pub fn with_deadband(mut self, new_deadband: frclib_core::units::energy::Amp) -> Self {
        self.deadband = new_deadband;
        self
    }
    /// Modifies this Control Request's override_coast_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_coast_dur_neutral(mut self, new_override_coast_dur_neutral: bool) -> Self {
        self.override_coast_dur_neutral = new_override_coast_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlTorqueCurrentFOC(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.output.into(),
            self.max_abs_duty_cycle.into(),
            self.deadband.into(),
            self.override_coast_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for TorqueCurrentFOC {
    fn default() -> Self {
        Self::new()
    }
}

/// Request PID to target velocity with duty cycle feedforward.
/// This control mode will set the motor's velocity setpoint to the velocity specified by the user.
/// In addition, it will apply an additional voltage as an arbitrary feedforward value.
pub struct VelocityDutyCycle {
    /// Velocity to drive toward in rotations per second.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Acceleration to drive toward in rotations per second squared.
    /// This is typically used for motion profiles generated by the robot program.
    pub acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Feedforward to apply in fractional units between -1 and +1.
    pub feed_forward: f64,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl VelocityDutyCycle {
    pub fn new() -> Self {
        Self {
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr::default(),
            enable_foc: bool::default(),
            feed_forward: f64::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's acceleration parameter
    /// and returns itself for method chaining.
    pub fn with_acceleration(
        mut self,
        new_acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    ) -> Self {
        self.acceleration = new_acceleration;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: f64) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlVelocityDutyCycle(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.velocity.into(),
            self.acceleration.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for VelocityDutyCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Requires Phoenix Pro; Request PID to target velocity with torque current feedforward.
/// This control mode will set the motor's velocity setpoint to the velocity specified by the user.
/// In addition, it will apply an additional torque current as an arbitrary feedforward value.
pub struct VelocityTorqueCurrentFOC {
    /// Velocity to drive toward in rotations per second.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Acceleration to drive toward in rotations per second squared.
    /// This is typically used for motion profiles generated by the robot program.
    pub acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    /// Feedforward to apply in torque current in Amperes.
    /// User can use motor's kT to scale Newton-meter to Amperes.
    pub feed_forward: frclib_core::units::energy::Amp,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to coast the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0A (zero torque).
    pub override_coast_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl VelocityTorqueCurrentFOC {
    pub fn new() -> Self {
        Self {
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr::default(),
            feed_forward: frclib_core::units::energy::Amp::default(),
            slot: i32::default(),
            override_coast_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's acceleration parameter
    /// and returns itself for method chaining.
    pub fn with_acceleration(
        mut self,
        new_acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    ) -> Self {
        self.acceleration = new_acceleration;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Amp) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_coast_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_coast_dur_neutral(mut self, new_override_coast_dur_neutral: bool) -> Self {
        self.override_coast_dur_neutral = new_override_coast_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlVelocityTorqueCurrentFOC(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.velocity.into(),
            self.acceleration.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_coast_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for VelocityTorqueCurrentFOC {
    fn default() -> Self {
        Self::new()
    }
}

/// Request PID to target velocity with voltage feedforward.
/// This control mode will set the motor's velocity setpoint to the velocity specified by the user.
/// In addition, it will apply an additional voltage as an arbitrary feedforward value.
pub struct VelocityVoltage {
    /// Velocity to drive toward in rotations per second.
    pub velocity: frclib_core::units::angular_velocity::RotationPerSec,
    /// Acceleration to drive toward in rotations per second squared.
    /// This is typically used for motion profiles generated by the robot program.
    pub acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    pub feed_forward: frclib_core::units::energy::Volt,
    /// Select which gains are applied by selecting the slot.
    /// Use the configuration api to set the gain values for the selected slot before enabling this feature.
    /// Slot must be within [0,2].
    pub slot: i32,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl VelocityVoltage {
    pub fn new() -> Self {
        Self {
            velocity: frclib_core::units::angular_velocity::RotationPerSec::default(),
            acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr::default(),
            enable_foc: bool::default(),
            feed_forward: frclib_core::units::energy::Volt::default(),
            slot: i32::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's velocity parameter
    /// and returns itself for method chaining.
    pub fn with_velocity(
        mut self,
        new_velocity: frclib_core::units::angular_velocity::RotationPerSec,
    ) -> Self {
        self.velocity = new_velocity;
        self
    }
    /// Modifies this Control Request's acceleration parameter
    /// and returns itself for method chaining.
    pub fn with_acceleration(
        mut self,
        new_acceleration: frclib_core::units::angular_acceleration::RotationPerSecSqr,
    ) -> Self {
        self.acceleration = new_acceleration;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's feed_forward parameter
    /// and returns itself for method chaining.
    pub fn with_feed_forward(mut self, new_feed_forward: frclib_core::units::energy::Volt) -> Self {
        self.feed_forward = new_feed_forward;
        self
    }
    /// Modifies this Control Request's slot parameter
    /// and returns itself for method chaining.
    pub fn with_slot(mut self, new_slot: i32) -> Self {
        self.slot = new_slot;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlVelocityVoltage(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.velocity.into(),
            self.acceleration.into(),
            self.enable_foc.into(),
            self.feed_forward.into(),
            self.slot.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for VelocityVoltage {
    fn default() -> Self {
        Self::new()
    }
}

/// Request a specified voltage.
/// This control mode will attempt to apply the specified voltage to the motor.
/// If the supply voltage is below the requested voltage, the motor controller will output the supply voltage.
pub struct VoltageOut {
    pub output: frclib_core::units::energy::Volt,
    /// Set to true to use FOC commutation (requires Phoenix Pro), which increases peak power by ~15%.
    /// Set to false to use trapezoidal commutation.
    /// FOC improves motor performance by leveraging torque (current) control.
    /// However, this may be inconvenient for applications that require specifying duty cycle or voltage.
    /// CTR-Electronics has developed a hybrid method that combines the performances gains of FOC while still allowing applications to provide duty cycle or voltage demand.
    /// This not to be confused with simple sinusoidal control or phase voltage control which lacks the performance gains.
    pub enable_foc: bool,
    /// Set to true to static-brake the rotor when output is zero (or within deadband).
    /// Set to false to use the NeutralMode configuration setting (default).
    /// This flag exists to provide the fundamental behavior of this control when output is zero, which is to provide 0V to the motor.
    pub override_brake_dur_neutral: bool,
    /// Set to true to force forward limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_forward_motion: bool,
    /// Set to true to force reverse limiting.
    /// This allows users to use other limit switch sensors connected to robot controller.
    /// This also allows use of active sensors that require external power.
    pub limit_reverse_motion: bool,
    pub update_freq_hz: f64,
}
impl VoltageOut {
    pub fn new() -> Self {
        Self {
            output: frclib_core::units::energy::Volt::default(),
            enable_foc: bool::default(),
            override_brake_dur_neutral: bool::default(),
            limit_forward_motion: bool::default(),
            limit_reverse_motion: bool::default(),
            update_freq_hz: 100.0,
        }
    }
    /// Modifies this Control Request's output parameter
    /// and returns itself for method chaining.
    pub fn with_output(mut self, new_output: frclib_core::units::energy::Volt) -> Self {
        self.output = new_output;
        self
    }
    /// Modifies this Control Request's enable_foc parameter
    /// and returns itself for method chaining.
    pub fn with_enable_foc(mut self, new_enable_foc: bool) -> Self {
        self.enable_foc = new_enable_foc;
        self
    }
    /// Modifies this Control Request's override_brake_dur_neutral parameter
    /// and returns itself for method chaining.
    pub fn with_override_brake_dur_neutral(mut self, new_override_brake_dur_neutral: bool) -> Self {
        self.override_brake_dur_neutral = new_override_brake_dur_neutral;
        self
    }
    /// Modifies this Control Request's limit_forward_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_forward_motion(mut self, new_limit_forward_motion: bool) -> Self {
        self.limit_forward_motion = new_limit_forward_motion;
        self
    }
    /// Modifies this Control Request's limit_reverse_motion parameter
    /// and returns itself for method chaining.
    pub fn with_limit_reverse_motion(mut self, new_limit_reverse_motion: bool) -> Self {
        self.limit_reverse_motion = new_limit_reverse_motion;
        self
    }
    /// Sets the period at which this control will update at.
    /// This is designated in Hertz, with a minimum of 20 Hz
    /// (every 50 ms) and a maximum of 1000 Hz (every 1 ms).
    ///
    /// If this field is set to 0 Hz, the control request will
    /// be sent immediately as a one-shot frame.
    /// This may be useful for advanced applications that require outputs
    /// to be synchronized with data acquisition.
    /// In this case, we recommend not exceeding 50 ms between control calls.
    pub fn with_update_freq_hz(mut self, new_update_freq_hz: f64) -> Self {
        self.update_freq_hz = new_update_freq_hz;
        self
    }
    /// Sends this request out over CAN bus to the device for
    /// the device to apply.
    pub(crate) unsafe fn send(
        self,
        device: DeviceIdentifier,
        cancel_other_requests: bool,
    ) -> Status<()> {
        ctre_phoenix6_sys::c_ctre_phoenix6_RequestControlVoltageOut(
            device.canbus.as_ptr() as *const i8,
            device.hash.0,
            self.update_freq_hz,
            cancel_other_requests,
            self.output.into(),
            self.enable_foc.into(),
            self.override_brake_dur_neutral.into(),
            self.limit_forward_motion.into(),
            self.limit_reverse_motion.into(),
        )
        .to_result()
    }
}
impl Default for VoltageOut {
    fn default() -> Self {
        Self::new()
    }
}

pub enum ControlRequest {
    CoastOut(CoastOut),
    DifferentialDutyCycle(DifferentialDutyCycle),
    DifferentialFollower(DifferentialFollower),
    DifferentialMotionMagicDutyCycle(DifferentialMotionMagicDutyCycle),
    DifferentialMotionMagicVoltage(DifferentialMotionMagicVoltage),
    DifferentialPositionDutyCycle(DifferentialPositionDutyCycle),
    DifferentialPositionVoltage(DifferentialPositionVoltage),
    DifferentialStrictFollower(DifferentialStrictFollower),
    DifferentialVelocityDutyCycle(DifferentialVelocityDutyCycle),
    DifferentialVelocityVoltage(DifferentialVelocityVoltage),
    DifferentialVoltage(DifferentialVoltage),
    DutyCycleOut(DutyCycleOut),
    DynamicMotionMagicDutyCycle(DynamicMotionMagicDutyCycle),
    DynamicMotionMagicTorqueCurrentFOC(DynamicMotionMagicTorqueCurrentFOC),
    DynamicMotionMagicVoltage(DynamicMotionMagicVoltage),
    EmptyControl(EmptyControl),
    Follower(Follower),
    MotionMagicDutyCycle(MotionMagicDutyCycle),
    MotionMagicTorqueCurrentFOC(MotionMagicTorqueCurrentFOC),
    MotionMagicVelocityDutyCycle(MotionMagicVelocityDutyCycle),
    MotionMagicVelocityTorqueCurrentFOC(MotionMagicVelocityTorqueCurrentFOC),
    MotionMagicVelocityVoltage(MotionMagicVelocityVoltage),
    MotionMagicVoltage(MotionMagicVoltage),
    MusicTone(MusicTone),
    NeutralOut(NeutralOut),
    PositionDutyCycle(PositionDutyCycle),
    PositionTorqueCurrentFOC(PositionTorqueCurrentFOC),
    PositionVoltage(PositionVoltage),
    StaticBrake(StaticBrake),
    StrictFollower(StrictFollower),
    TorqueCurrentFOC(TorqueCurrentFOC),
    VelocityDutyCycle(VelocityDutyCycle),
    VelocityTorqueCurrentFOC(VelocityTorqueCurrentFOC),
    VelocityVoltage(VelocityVoltage),
    VoltageOut(VoltageOut),
}
impl ControlRequest {
    pub(crate) fn send(self, device: DeviceIdentifier, cancel_other_requests: bool) -> Status<()> {
        unsafe {
            match self {
                ControlRequest::CoastOut(req) => req.send(device, cancel_other_requests),
                ControlRequest::DifferentialDutyCycle(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DifferentialFollower(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DifferentialMotionMagicDutyCycle(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DifferentialMotionMagicVoltage(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DifferentialPositionDutyCycle(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DifferentialPositionVoltage(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DifferentialStrictFollower(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DifferentialVelocityDutyCycle(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DifferentialVelocityVoltage(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DifferentialVoltage(req) => req.send(device, cancel_other_requests),
                ControlRequest::DutyCycleOut(req) => req.send(device, cancel_other_requests),
                ControlRequest::DynamicMotionMagicDutyCycle(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DynamicMotionMagicTorqueCurrentFOC(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::DynamicMotionMagicVoltage(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::EmptyControl(req) => req.send(device, cancel_other_requests),
                ControlRequest::Follower(req) => req.send(device, cancel_other_requests),
                ControlRequest::MotionMagicDutyCycle(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::MotionMagicTorqueCurrentFOC(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::MotionMagicVelocityDutyCycle(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::MotionMagicVelocityTorqueCurrentFOC(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::MotionMagicVelocityVoltage(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::MotionMagicVoltage(req) => req.send(device, cancel_other_requests),
                ControlRequest::MusicTone(req) => req.send(device, cancel_other_requests),
                ControlRequest::NeutralOut(req) => req.send(device, cancel_other_requests),
                ControlRequest::PositionDutyCycle(req) => req.send(device, cancel_other_requests),
                ControlRequest::PositionTorqueCurrentFOC(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::PositionVoltage(req) => req.send(device, cancel_other_requests),
                ControlRequest::StaticBrake(req) => req.send(device, cancel_other_requests),
                ControlRequest::StrictFollower(req) => req.send(device, cancel_other_requests),
                ControlRequest::TorqueCurrentFOC(req) => req.send(device, cancel_other_requests),
                ControlRequest::VelocityDutyCycle(req) => req.send(device, cancel_other_requests),
                ControlRequest::VelocityTorqueCurrentFOC(req) => {
                    req.send(device, cancel_other_requests)
                }
                ControlRequest::VelocityVoltage(req) => req.send(device, cancel_other_requests),
                ControlRequest::VoltageOut(req) => req.send(device, cancel_other_requests),
            }
        }
    }
}
impl From<CoastOut> for ControlRequest {
    fn from(req: CoastOut) -> Self {
        ControlRequest::CoastOut(req)
    }
}
impl crate::__sealed::Sealed for CoastOut {}
impl From<DifferentialDutyCycle> for ControlRequest {
    fn from(req: DifferentialDutyCycle) -> Self {
        ControlRequest::DifferentialDutyCycle(req)
    }
}
impl crate::__sealed::Sealed for DifferentialDutyCycle {}
impl From<DifferentialFollower> for ControlRequest {
    fn from(req: DifferentialFollower) -> Self {
        ControlRequest::DifferentialFollower(req)
    }
}
impl crate::__sealed::Sealed for DifferentialFollower {}
impl From<DifferentialMotionMagicDutyCycle> for ControlRequest {
    fn from(req: DifferentialMotionMagicDutyCycle) -> Self {
        ControlRequest::DifferentialMotionMagicDutyCycle(req)
    }
}
impl crate::__sealed::Sealed for DifferentialMotionMagicDutyCycle {}
impl From<DifferentialMotionMagicVoltage> for ControlRequest {
    fn from(req: DifferentialMotionMagicVoltage) -> Self {
        ControlRequest::DifferentialMotionMagicVoltage(req)
    }
}
impl crate::__sealed::Sealed for DifferentialMotionMagicVoltage {}
impl From<DifferentialPositionDutyCycle> for ControlRequest {
    fn from(req: DifferentialPositionDutyCycle) -> Self {
        ControlRequest::DifferentialPositionDutyCycle(req)
    }
}
impl crate::__sealed::Sealed for DifferentialPositionDutyCycle {}
impl From<DifferentialPositionVoltage> for ControlRequest {
    fn from(req: DifferentialPositionVoltage) -> Self {
        ControlRequest::DifferentialPositionVoltage(req)
    }
}
impl crate::__sealed::Sealed for DifferentialPositionVoltage {}
impl From<DifferentialStrictFollower> for ControlRequest {
    fn from(req: DifferentialStrictFollower) -> Self {
        ControlRequest::DifferentialStrictFollower(req)
    }
}
impl crate::__sealed::Sealed for DifferentialStrictFollower {}
impl From<DifferentialVelocityDutyCycle> for ControlRequest {
    fn from(req: DifferentialVelocityDutyCycle) -> Self {
        ControlRequest::DifferentialVelocityDutyCycle(req)
    }
}
impl crate::__sealed::Sealed for DifferentialVelocityDutyCycle {}
impl From<DifferentialVelocityVoltage> for ControlRequest {
    fn from(req: DifferentialVelocityVoltage) -> Self {
        ControlRequest::DifferentialVelocityVoltage(req)
    }
}
impl crate::__sealed::Sealed for DifferentialVelocityVoltage {}
impl From<DifferentialVoltage> for ControlRequest {
    fn from(req: DifferentialVoltage) -> Self {
        ControlRequest::DifferentialVoltage(req)
    }
}
impl crate::__sealed::Sealed for DifferentialVoltage {}
impl From<DutyCycleOut> for ControlRequest {
    fn from(req: DutyCycleOut) -> Self {
        ControlRequest::DutyCycleOut(req)
    }
}
impl crate::__sealed::Sealed for DutyCycleOut {}
impl From<DynamicMotionMagicDutyCycle> for ControlRequest {
    fn from(req: DynamicMotionMagicDutyCycle) -> Self {
        ControlRequest::DynamicMotionMagicDutyCycle(req)
    }
}
impl crate::__sealed::Sealed for DynamicMotionMagicDutyCycle {}
impl From<DynamicMotionMagicTorqueCurrentFOC> for ControlRequest {
    fn from(req: DynamicMotionMagicTorqueCurrentFOC) -> Self {
        ControlRequest::DynamicMotionMagicTorqueCurrentFOC(req)
    }
}
impl crate::__sealed::Sealed for DynamicMotionMagicTorqueCurrentFOC {}
impl From<DynamicMotionMagicVoltage> for ControlRequest {
    fn from(req: DynamicMotionMagicVoltage) -> Self {
        ControlRequest::DynamicMotionMagicVoltage(req)
    }
}
impl crate::__sealed::Sealed for DynamicMotionMagicVoltage {}
impl From<EmptyControl> for ControlRequest {
    fn from(req: EmptyControl) -> Self {
        ControlRequest::EmptyControl(req)
    }
}
impl crate::__sealed::Sealed for EmptyControl {}
impl From<Follower> for ControlRequest {
    fn from(req: Follower) -> Self {
        ControlRequest::Follower(req)
    }
}
impl crate::__sealed::Sealed for Follower {}
impl From<MotionMagicDutyCycle> for ControlRequest {
    fn from(req: MotionMagicDutyCycle) -> Self {
        ControlRequest::MotionMagicDutyCycle(req)
    }
}
impl crate::__sealed::Sealed for MotionMagicDutyCycle {}
impl From<MotionMagicTorqueCurrentFOC> for ControlRequest {
    fn from(req: MotionMagicTorqueCurrentFOC) -> Self {
        ControlRequest::MotionMagicTorqueCurrentFOC(req)
    }
}
impl crate::__sealed::Sealed for MotionMagicTorqueCurrentFOC {}
impl From<MotionMagicVelocityDutyCycle> for ControlRequest {
    fn from(req: MotionMagicVelocityDutyCycle) -> Self {
        ControlRequest::MotionMagicVelocityDutyCycle(req)
    }
}
impl crate::__sealed::Sealed for MotionMagicVelocityDutyCycle {}
impl From<MotionMagicVelocityTorqueCurrentFOC> for ControlRequest {
    fn from(req: MotionMagicVelocityTorqueCurrentFOC) -> Self {
        ControlRequest::MotionMagicVelocityTorqueCurrentFOC(req)
    }
}
impl crate::__sealed::Sealed for MotionMagicVelocityTorqueCurrentFOC {}
impl From<MotionMagicVelocityVoltage> for ControlRequest {
    fn from(req: MotionMagicVelocityVoltage) -> Self {
        ControlRequest::MotionMagicVelocityVoltage(req)
    }
}
impl crate::__sealed::Sealed for MotionMagicVelocityVoltage {}
impl From<MotionMagicVoltage> for ControlRequest {
    fn from(req: MotionMagicVoltage) -> Self {
        ControlRequest::MotionMagicVoltage(req)
    }
}
impl crate::__sealed::Sealed for MotionMagicVoltage {}
impl From<MusicTone> for ControlRequest {
    fn from(req: MusicTone) -> Self {
        ControlRequest::MusicTone(req)
    }
}
impl crate::__sealed::Sealed for MusicTone {}
impl From<NeutralOut> for ControlRequest {
    fn from(req: NeutralOut) -> Self {
        ControlRequest::NeutralOut(req)
    }
}
impl crate::__sealed::Sealed for NeutralOut {}
impl From<PositionDutyCycle> for ControlRequest {
    fn from(req: PositionDutyCycle) -> Self {
        ControlRequest::PositionDutyCycle(req)
    }
}
impl crate::__sealed::Sealed for PositionDutyCycle {}
impl From<PositionTorqueCurrentFOC> for ControlRequest {
    fn from(req: PositionTorqueCurrentFOC) -> Self {
        ControlRequest::PositionTorqueCurrentFOC(req)
    }
}
impl crate::__sealed::Sealed for PositionTorqueCurrentFOC {}
impl From<PositionVoltage> for ControlRequest {
    fn from(req: PositionVoltage) -> Self {
        ControlRequest::PositionVoltage(req)
    }
}
impl crate::__sealed::Sealed for PositionVoltage {}
impl From<StaticBrake> for ControlRequest {
    fn from(req: StaticBrake) -> Self {
        ControlRequest::StaticBrake(req)
    }
}
impl crate::__sealed::Sealed for StaticBrake {}
impl From<StrictFollower> for ControlRequest {
    fn from(req: StrictFollower) -> Self {
        ControlRequest::StrictFollower(req)
    }
}
impl crate::__sealed::Sealed for StrictFollower {}
impl From<TorqueCurrentFOC> for ControlRequest {
    fn from(req: TorqueCurrentFOC) -> Self {
        ControlRequest::TorqueCurrentFOC(req)
    }
}
impl crate::__sealed::Sealed for TorqueCurrentFOC {}
impl From<VelocityDutyCycle> for ControlRequest {
    fn from(req: VelocityDutyCycle) -> Self {
        ControlRequest::VelocityDutyCycle(req)
    }
}
impl crate::__sealed::Sealed for VelocityDutyCycle {}
impl From<VelocityTorqueCurrentFOC> for ControlRequest {
    fn from(req: VelocityTorqueCurrentFOC) -> Self {
        ControlRequest::VelocityTorqueCurrentFOC(req)
    }
}
impl crate::__sealed::Sealed for VelocityTorqueCurrentFOC {}
impl From<VelocityVoltage> for ControlRequest {
    fn from(req: VelocityVoltage) -> Self {
        ControlRequest::VelocityVoltage(req)
    }
}
impl crate::__sealed::Sealed for VelocityVoltage {}
impl From<VoltageOut> for ControlRequest {
    fn from(req: VoltageOut) -> Self {
        ControlRequest::VoltageOut(req)
    }
}
impl crate::__sealed::Sealed for VoltageOut {}
trait ControlRequestType: crate::__sealed::Sealed {
    fn send(self, device: DeviceIdentifier, cancel_other_requests: bool) -> Status<()>;
}
impl<T: crate::__sealed::Sealed + Into<ControlRequest>> ControlRequestType for T {
    fn send(self, device: DeviceIdentifier, cancel_other_requests: bool) -> Status<()> {
        self.into().send(device, cancel_other_requests)
    }
}
