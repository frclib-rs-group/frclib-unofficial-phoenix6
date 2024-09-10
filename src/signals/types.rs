use paste::paste;

macro_rules! from_py {
    ($(
        class $name:ident ( Enum ): //class
            $_a:literal $doc:literal $_b:literal //docs
            $($variant:ident = $value:expr)* $(,)? //variants
    )*) =>
    {
        $(
            paste! {
                #[doc = $doc]
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, num_enum::TryFromPrimitive)]
                #[repr(u32)]
                pub enum [< $name:camel> ] {
                    $([< $variant:camel >] = $value),*
                }
                impl super::__sealed::Sealed for [< $name:camel> ] {}
                impl super::SPNValue for [< $name:camel> ] {
                    fn try_from_f64(value: f64) -> crate::Status<Self> {
                        (value as u32).try_into().map_err(|_| crate::error::StatusCode::CouldNotCast)
                    }
                }
                impl Default for [< $name:camel> ] {
                    fn default() -> Self {
                        [< $name:camel> ]::try_from(0).unwrap()
                    }
                }
            }
        )*
    };
}

from_py! {
class System_StateValue(Enum):
    """
    System state of the device
    """
    BOOTUP_0 = 0
    BOOTUP_1 = 1
    BOOTUP_2 = 2
    BOOTUP_3 = 3
    BOOTUP_4 = 4
    BOOTUP_5 = 5
    BOOTUP_6 = 6
    BOOTUP_7 = 7
    BOOT_BEEP = 8
    CONTROL_DISABLED = 9
    CONTROL_ENABLED = 10
    CONTROL_ENABLED_11 = 11
    FAULT = 12
    RECOVER = 13
    NOT_LICENSED = 14
    PRODUCTION = 15
}

from_py! {
class IsPROLicensedValue(Enum):
    """
    Whether the device is Pro licensed
    """
    NOT_LICENSED = 0
    LICENSED = 1
}
from_py! {
class Licensing_IsSeasonPassedValue(Enum):
    """
    Whether the device is Season Pass licensed
    """
    NOT_LICENSED = 0
    LICENSED = 1
}
from_py! {
class SensorDirectionValue(Enum):
    """
    Direction of the sensor to determine positive facing the LED side of the
    CANcoder.
    """
    COUNTER_CLOCKWISE_POSITIVE = 0
    CLOCKWISE_POSITIVE = 1
}
from_py! {
class FrcLockValue(Enum):
    """
    True if device is locked by FRC.
    """
    FRC_LOCKED = 1
    FRC_UNLOCKED = 0
}
from_py! {
class RobotEnableValue(Enum):
    """
    True if the robot is enabled.
    """
    ENABLED = 1
    DISABLED = 0
}
from_py! {
class Led1OnColorValue(Enum):
    """
    The Color of LED1 when it's `On`.
    """
    OFF = 0
    RED = 1
    GREEN = 2
    ORANGE = 3
    BLUE = 4
    PINK = 5
    CYAN = 6
    WHITE = 7
}
from_py! {
class Led1OffColorValue(Enum):
    """
    The Color of LED1 when it's `Off`.
    """
    OFF = 0
    RED = 1
    GREEN = 2
    ORANGE = 3
    BLUE = 4
    PINK = 5
    CYAN = 6
    WHITE = 7
}
from_py! {
class Led2OnColorValue(Enum):
    """
    The Color of LED2 when it's `On`.
    """
    OFF = 0
    RED = 1
    GREEN = 2
    ORANGE = 3
    BLUE = 4
    PINK = 5
    CYAN = 6
    WHITE = 7
}
from_py! {
class Led2OffColorValue(Enum):
    """
    The Color of LED2 when it's `Off`.
    """
    OFF = 0
    RED = 1
    GREEN = 2
    ORANGE = 3
    BLUE = 4
    PINK = 5
    CYAN = 6
    WHITE = 7
}
from_py! {
class AbsoluteSensorRangeValue(Enum):
    """
    The range of the absolute sensor, either [-0.5, 0.5) or [0, 1).
    """
    UNSIGNED_0_TO1 = 0
    SIGNED_PLUS_MINUS_HALF = 1
}
from_py! {
class DeviceEnableValue(Enum):
    """
    True if the device is enabled.
    """
    ENABLED = 1
    DISABLED = 0
}
from_py! {
class ForwardLimitValue(Enum):
    """
    Forward Limit Pin.
    """
    CLOSED_TO_GROUND = 0
    OPEN = 1
}
from_py! {
class ReverseLimitValue(Enum):
    """
    Reverse Limit Pin.
    """
    CLOSED_TO_GROUND = 0
    OPEN = 1
}
from_py! {
class AppliedRotorPolarityValue(Enum):
    """
    The applied rotor polarity.  This typically is determined by the Inverted
    config, but can be overridden if using Follower features.
    """
    POSITIVE_IS_COUNTER_CLOCKWISE = 0
    POSITIVE_IS_CLOCKWISE = 1
}
from_py! {
class ControlModeValue(Enum):
    """
    The active control mode of the motor controller
    """
    DISABLED_OUTPUT = 0
    NEUTRAL_OUT = 1
    STATIC_BRAKE = 2
    DUTY_CYCLE_OUT = 3
    POSITION_DUTY_CYCLE = 4
    VELOCITY_DUTY_CYCLE = 5
    MOTION_MAGIC_DUTY_CYCLE = 6
    DUTY_CYCLE_FOC = 7
    POSITION_DUTY_CYCLE_FOC = 8
    VELOCITY_DUTY_CYCLE_FOC = 9
    MOTION_MAGIC_DUTY_CYCLE_FOC = 10
    VOLTAGE_OUT = 11
    POSITION_VOLTAGE = 12
    VELOCITY_VOLTAGE = 13
    MOTION_MAGIC_VOLTAGE = 14
    VOLTAGE_FOC = 15
    POSITION_VOLTAGE_FOC = 16
    VELOCITY_VOLTAGE_FOC = 17
    MOTION_MAGIC_VOLTAGE_FOC = 18
    TORQUE_CURRENT_FOC = 19
    POSITION_TORQUE_CURRENT_FOC = 20
    VELOCITY_TORQUE_CURRENT_FOC = 21
    MOTION_MAGIC_TORQUE_CURRENT_FOC = 22
    FOLLOWER = 23
    RESERVED = 24
    COAST_OUT = 25
    UNAUTHORIZED_DEVICE = 26
    MUSIC_TONE = 27
    MOTION_MAGIC_VELOCITY_DUTY_CYCLE = 28
    MOTION_MAGIC_VELOCITY_DUTY_CYCLE_FOC = 29
    MOTION_MAGIC_VELOCITY_VOLTAGE = 30
    MOTION_MAGIC_VELOCITY_VOLTAGE_FOC = 31
    MOTION_MAGIC_VELOCITY_TORQUE_CURRENT_FOC = 32
    MOTION_MAGIC_EXPO_DUTY_CYCLE = 33
    MOTION_MAGIC_EXPO_DUTY_CYCLE_FOC = 34
    MOTION_MAGIC_EXPO_VOLTAGE = 35
    MOTION_MAGIC_EXPO_VOLTAGE_FOC = 36
    MOTION_MAGIC_EXPO_TORQUE_CURRENT_FOC = 37
}
from_py! {
class MotionMagicIsRunningValue(Enum):
    """
    Check if Motion Magic® is running.  This is equivalent to checking that the
    reported control mode is a Motion Magic® based mode.
    """
    ENABLED = 1
    DISABLED = 0
}
from_py! {
class DifferentialControlModeValue(Enum):
    """
    The active control mode of the differential controller
    """
    DISABLED_OUTPUT = 0
    NEUTRAL_OUT = 1
    STATIC_BRAKE = 2
    DUTY_CYCLE_OUT = 3
    POSITION_DUTY_CYCLE = 4
    VELOCITY_DUTY_CYCLE = 5
    MOTION_MAGIC_DUTY_CYCLE = 6
    DUTY_CYCLE_FOC = 7
    POSITION_DUTY_CYCLE_FOC = 8
    VELOCITY_DUTY_CYCLE_FOC = 9
    MOTION_MAGIC_DUTY_CYCLE_FOC = 10
    VOLTAGE_OUT = 11
    POSITION_VOLTAGE = 12
    VELOCITY_VOLTAGE = 13
    MOTION_MAGIC_VOLTAGE = 14
    VOLTAGE_FOC = 15
    POSITION_VOLTAGE_FOC = 16
    VELOCITY_VOLTAGE_FOC = 17
    MOTION_MAGIC_VOLTAGE_FOC = 18
    TORQUE_CURRENT_FOC = 19
    POSITION_TORQUE_CURRENT_FOC = 20
    VELOCITY_TORQUE_CURRENT_FOC = 21
    MOTION_MAGIC_TORQUE_CURRENT_FOC = 22
    FOLLOWER = 23
    RESERVED = 24
    COAST_OUT = 25
}
from_py! {
class GravityTypeValue(Enum):
    """
    Gravity Feedforward Type
    
    This determines the type of the gravity feedforward. Choose Elevator_Static for
    systems where the gravity feedforward is constant, such as an elevator. The
    gravity feedforward output will always have the same sign. Choose Arm_Cosine for
    systems where the gravity feedforward is dependent on the angular position of
    the mechanism, such as an arm. The gravity feedforward output will vary
    depending on the mechanism angular position. Note that the sensor offset and
    ratios must be configured so that the sensor reports a position of 0 when the
    mechanism is horizonal (parallel to the ground), and the reported sensor
    position is 1:1 with the mechanism.
    """
    ELEVATOR_STATIC = 0
    ARM_COSINE = 1
}
from_py! {
class InvertedValue(Enum):
    """
    Invert state of the device
    """
    COUNTER_CLOCKWISE_POSITIVE = 0
    CLOCKWISE_POSITIVE = 1
}
from_py! {
class NeutralModeValue(Enum):
    """
    The state of the motor controller bridge when output is neutral or disabled.
    """
    COAST = 0
    BRAKE = 1
}
from_py! {
class FeedbackSensorSourceValue(Enum):
    """
    Choose what sensor source is reported via API and used by closed-loop and limit
    features.  The default is RotorSensor, which uses the internal rotor sensor in
    the Talon FX.  Choose RemoteCANcoder to use another CANcoder on the same CAN bus
    (this also requires setting FeedbackRemoteSensorID).  Talon FX will update its
    position and velocity whenever CANcoder publishes its information on CAN bus. 
    Choose FusedCANcoder (requires Phoenix Pro) and Talon FX will fuse another
    CANcoder's information with the internal rotor, which provides the best possible
    position and velocity for accuracy and bandwidth (note this requires setting
    FeedbackRemoteSensorID).  FusedCANcoder was developed for applications such as
    swerve-azimuth.  Choose SyncCANcoder (requires Phoenix Pro) and Talon FX will
    synchronize its internal rotor position against another CANcoder, then continue
    to use the rotor sensor for closed loop control (note this requires setting
    FeedbackRemoteSensorID).  The TalonFX will report if its internal position
    differs significantly from the reported CANcoder position.  SyncCANcoder was
    developed for mechanisms where there is a risk of the CANcoder failing in such a
    way that it reports a position that does not match the mechanism, such as the
    sensor mounting assembly breaking off.  Choose RemotePigeon2_Yaw,
    RemotePigeon2_Pitch, and RemotePigeon2_Roll to use another Pigeon2 on the same
    CAN bus (this also requires setting FeedbackRemoteSensorID).  Talon FX will
    update its position to match the selected value whenever Pigeon2 publishes its
    information on CAN bus. Note that the Talon FX position will be in rotations and
    not degrees.

    Note: When the Talon Source is changed to FusedCANcoder, the Talon needs a
    period of time to fuse before sensor-based (soft-limit, closed loop, etc.)
    features are used. This period of time is determined by the update frequency of
    the CANcoder's Position signal.
    """
    ROTOR_SENSOR = 0
    REMOTE_CANCODER = 1
    REMOTE_PIGEON2_YAW = 2
    REMOTE_PIGEON2_PITCH = 3
    REMOTE_PIGEON2_ROLL = 4
    FUSED_CANCODER = 5
    SYNC_CANCODER = 6
}
from_py! {
class ForwardLimitTypeValue(Enum):
    """
    Determines if limit is normally-open (default) or normally-closed.
    """
    NORMALLY_OPEN = 0
    NORMALLY_CLOSED = 1
}
from_py! {
class ForwardLimitSourceValue(Enum):
    """
    Determines where to poll the forward limit switch.  This defaults to the limit
    switch pin on the limit switch connector.
    """
    LIMIT_SWITCH_PIN = 0
    REMOTE_TALON_FX = 1
    REMOTE_CANIFIER = 2
    REMOTE_CANCODER = 4
    DISABLED = 3
}
from_py! {
class ReverseLimitTypeValue(Enum):
    """
    Determines if limit is normally-open (default) or normally-closed.
    """
    NORMALLY_OPEN = 0
    NORMALLY_CLOSED = 1
}
from_py! {
class ReverseLimitSourceValue(Enum):
    """
    Determines where to poll the reverse limit switch.  This defaults to the limit
    switch pin on the limit switch connector.
    """
    LIMIT_SWITCH_PIN = 0
    REMOTE_TALON_FX = 1
    REMOTE_CANIFIER = 2
    REMOTE_CANCODER = 4
    DISABLED = 3
}
from_py! {
class MagnetHealthValue(Enum):
    """
    Magnet health as measured by CANcoder.

    Magnet health as measured by CANcoder. Red indicates too close or too far,
    Orange is adequate but with reduced accuracy, green is ideal. Invalid means the
    accuracy cannot be determined.
    """
    MAGNET_RED = 1
    MAGNET_ORANGE = 2
    MAGNET_GREEN = 3
    MAGNET_INVALID = 0
}
from_py! {
class BridgeOutputValue(Enum):
    """
    The applied output of the bridge.
    """
    BRIDGE_REQ_COAST = 0
    BRIDGE_REQ_BRAKE = 1
    BRIDGE_REQ_TRAPEZ = 6
    BRIDGE_REQ_FOCTORQUE = 7
    BRIDGE_REQ_MUSIC_TONE = 8
    BRIDGE_REQ_FOCEASY = 9
    BRIDGE_REQ_FAULT_BRAKE = 12
    BRIDGE_REQ_FAULT_COAST = 13
}
from_py! {
class DifferentialSensorSourceValue(Enum):
    """
    Choose what sensor source is used for differential control of a mechanism.  The
    default is Disabled.  All other options require setting the
    DifferentialTalonFXSensorID, as the average of this Talon FX's sensor and the
    remote TalonFX's sensor is used for the differential controller's primary
    targets.  Choose RemoteTalonFX_Diff to use another TalonFX on the same CAN bus. 
    Talon FX will update its differential position and velocity whenever the remote
    TalonFX publishes its information on CAN bus.  The differential controller will
    use the difference between this TalonFX's sensor and the remote Talon FX's
    sensor for the differential component of the output.  Choose RemotePigeon2_Yaw,
    RemotePigeon2_Pitch, and RemotePigeon2_Roll to use another Pigeon2 on the same
    CAN bus (this also requires setting DifferentialRemoteSensorID).  Talon FX will
    update its differential position to match the selected value whenever Pigeon2
    publishes its information on CAN bus. Note that the Talon FX differential
    position will be in rotations and not degrees. Choose RemoteCANcoder to use
    another CANcoder on the same CAN bus (this also requires setting
    DifferentialRemoteSensorID).  Talon FX will update its differential position and
    velocity to match the CANcoder whenever CANcoder publishes its information on
    CAN bus.
    """
    DISABLED = 0
    REMOTE_TALON_FX_DIFF = 1
    REMOTE_PIGEON2_YAW = 2
    REMOTE_PIGEON2_PITCH = 3
    REMOTE_PIGEON2_ROLL = 4
    REMOTE_CANCODER = 5
}
