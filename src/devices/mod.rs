pub mod cancoder;
mod config_native;
pub mod pigeon;

use std::collections::HashSet;

use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::{
    __sealed::Sealed,
    error::{StatusCode, StatusCodeType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Model {
    inner: &'static str,
}
impl Model {
    pub fn custom(model: &'static str) -> Self {
        Self { inner: model }
    }
    pub fn talon_fx() -> Self {
        Self { inner: "talon fx" }
    }
    pub fn pigeon2() -> Self {
        Self { inner: "pigeon 2" }
    }
    pub fn cancoder() -> Self {
        Self { inner: "cancoder" }
    }
}
impl ToString for Model {
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}
impl From<Model> for *const ::std::os::raw::c_char {
    fn from(model: Model) -> Self {
        model.inner.as_ptr() as *const ::std::os::raw::c_char
    }
}

const MIN_DEVICE_ID: i8 = 1;
const MAX_DEVICE_ID: i8 = 62;

#[derive(Debug)]
pub struct DeviceIdOutOfRangeError(i8);
impl std::fmt::Display for DeviceIdOutOfRangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Device ID must be between {MIN_DEVICE_ID} and {MAX_DEVICE_ID} but was {0}",
            self.0
        )
    }
}
impl std::error::Error for DeviceIdOutOfRangeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceCanID {
    inner: i8,
}
impl DeviceCanID {
    pub fn new(index: i8) -> Result<Self, DeviceIdOutOfRangeError> {
        if !(MIN_DEVICE_ID..=MAX_DEVICE_ID).contains(&index) {
            Err(DeviceIdOutOfRangeError(index))
        } else {
            Ok(Self { inner: index })
        }
    }
    pub fn index(&self) -> i8 {
        self.inner
    }
}
impl From<i32> for DeviceCanID {
    fn from(index: i32) -> Self {
        Self { inner: index as i8 }
    }
}
impl From<i8> for DeviceCanID {
    fn from(index: i8) -> Self {
        Self { inner: index }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceHash(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceIdentifier {
    pub index: DeviceCanID,
    pub model: Model,
    pub canbus: String,
    pub(crate) hash: DeviceHash,
}
impl DeviceIdentifier {
    pub fn try_new(can_id: DeviceCanID, model: Model, can_bus: String) -> Result<Self, StatusCode> {
        let hash_ret: *mut u32 = &mut 0;
        unsafe {
            ctre_phoenix6_sys::c_ctre_phoenix6_encode_device(
                can_id.index() as i32,
                model.into(),
                can_bus.as_ptr() as *const ::std::os::raw::c_char,
                hash_ret,
            )
            .to_result()?;
        }
        Ok(Self {
            index: can_id,
            model,
            canbus: can_bus,
            hash: DeviceHash(unsafe { *hash_ret }),
        })
    }

    pub(crate) fn from_hash(hash: u32) -> Option<Self> {
        for dev in ACTIVE_DEVICES.read().iter() {
            if dev.hash.0 == hash {
                return Some(dev.clone());
            }
        }
        None
    }
}
impl std::fmt::Display for DeviceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{model} on {canbus} with id {index}",
            model = self.model.to_string(),
            canbus = self.canbus,
            index = self.index.index()
        )
    }
}

pub trait ConfigProtocol: Sealed + Sized + Default + std::fmt::Display {
    fn serialize(&self) -> crate::Status<String>;

    fn deserialize(to_deserialize: &str) -> crate::Status<Self>;

    fn future_proof_configs(&self) -> bool {
        true
    }
}

static ACTIVE_DEVICES: RwLock<Lazy<HashSet<DeviceIdentifier>>> =
    RwLock::new(Lazy::new(HashSet::new));

/// Evaluates the uniqueness of the device and inserts it into the active devices list
/// if it is unique.
fn propose_device(dev_id: DeviceIdentifier) -> Result<(), StatusCode> {
    if ACTIVE_DEVICES.read().contains(&dev_id) {
        Err(StatusCode::AccessDenied)
    } else {
        ACTIVE_DEVICES.write().insert(dev_id);
        Ok(())
    }
}

/// Removes the device from the active devices list
fn close_device(dev_id: &DeviceIdentifier) {
    ACTIVE_DEVICES.write().remove(dev_id);
}

#[doc(hidden)]
#[macro_export]
macro_rules! signal_setup {
    (device: $device:ident, signal: $signal:ident, fields: $fields:ident) => {
        #[doc(hidden)]
        type ThisSyncDevice = $device;
        #[doc(hidden)]
        type ThisSyncSignal<T> = $signal<T>;
        #[doc(hidden)]
        type ThisSyncFields = $fields;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! signal {
    ($fn_name:ident -> $field_name:ident < $type:ident >) => {
        paste::paste! {
            impl ThisSyncDevice {
                #[doc = "This function returns a signal that can be used to read a value from a CTRE CAN device."]
                #[doc = "To update the underlying value you must call refresh on the signal."]
                #[doc = "If multiple signals for the same value and device they will share data and refresh at the same time."]
                #[doc = "The value is also packaged with the timestamp of the value."]
                pub fn [< $fn_name _signal >](&self) -> ThisSyncSignal<$type> {
                    ThisSyncSignal::new(
                        self.identifier.clone(),
                        ThisSyncFields::$field_name,
                        Arc::downgrade(&self.cache)
                    )
                }

                #[doc = "Refreshes the value of the signal and returns the new value,"]
                #[doc = "this can be cheaper than calling `Self." $fn_name "_signal().value()`"]
                pub fn [< get_ $fn_name >] (&self) -> Status<SignalValue<$type>> {
                    let ret = native::request_signal_value_single(
                        native::SignalMeta {
                            can_bus: self.identifier.canbus.clone(),
                            timeout: $crate::DEFAULT_TIMEOUT
                        },
                        native::SignalSpecifier {
                            hash: self.identifier.hash.0,
                            spn: (ThisSyncFields::$field_name as i32).try_into().expect("Invalid SPN")
                        }
                    )?;
                    self.cache.write().[< $fn_name >] = ret;
                    Ok(SignalValue::<$type>::from(ret))
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! cold_signal {
    ($fn_name:ident -> $field_name:ident < $type:ident >) => {
        paste::paste! {
            impl ThisSyncDevice {
                #[doc = "This function returns a signal that can be used to read a value from a CTRE CAN device."]
                #[doc = "To update the underlying value you must call refresh on the signal."]
                #[doc = "If multiple signals for the same value and device they will share data and refresh at the same time."]
                #[doc = "The value is also packaged with the timestamp of the value."]
                #[doc = "# COLD:"]
                #[doc = "This signals value is not cached in the device instance, every time you get the value an implicit refresh will happen."]
                pub fn [< $fn_name _signal >](&self) -> ThisSyncSignal<$type> {
                    ThisSyncSignal::new_cold(
                        self.identifier.clone(),
                        ThisSyncFields::$field_name
                    )
                }

                #[doc = "Returns the value of the signal"]
                #[doc = "# COLD:"]
                #[doc = "This signals value is not cached in the device instance, every time you get the value an implicit refresh will happen."]
                pub fn [< get_ $fn_name >] (&self) -> Status<SignalValue<$type>> {
                    let ret = native::request_signal_value_single(
                        native::SignalMeta {
                            can_bus: self.identifier.canbus.clone(),
                            timeout: $crate::DEFAULT_TIMEOUT
                        },
                        native::SignalSpecifier {
                            hash: self.identifier.hash.0,
                            spn: (ThisSyncFields::$field_name as i32).try_into().expect("Invalid SPN")
                        }
                    )?;
                    Ok(SignalValue::<$type>::from(ret))
                }
            }
        }
    };
}
