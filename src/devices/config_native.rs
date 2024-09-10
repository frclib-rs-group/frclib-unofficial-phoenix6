#![allow(dead_code)]
use std::{ffi::CStr, os::raw::c_char, ptr};

use crate::{
    error::{StatusCode, StatusCodeType},
    spn::SPN,
    Status,
};

use super::{ConfigProtocol, DeviceIdentifier};

pub fn serialize_int(spn: SPN, int: i32) -> Status<String> {
    unsafe {
        let mut str_: *mut std::os::raw::c_char = std::ptr::null_mut();
        ctre_phoenix6_sys::c_ctre_phoenix6_serialize_int(i32::from(spn), int, &mut str_)
            .to_result()?;
        let string = std::ffi::CStr::from_ptr(str_)
            .to_str()
            .map_err(|_| StatusCode::CouldNotSerialize)?
            .to_owned();
        ctre_phoenix6_sys::c_ctre_phoenix6_free_memory(&mut str_);
        Ok(string)
    }
}

pub fn serialize_double(spn: SPN, double: f64) -> Status<String> {
    unsafe {
        let mut str_: *mut ::std::os::raw::c_char = ptr::null_mut();
        ctre_phoenix6_sys::c_ctre_phoenix6_serialize_double(i32::from(spn), double, &mut str_)
            .to_result()?;
        let string = CStr::from_ptr(str_)
            .to_str()
            .map_err(|_| StatusCode::CouldNotSerialize)?
            .to_owned();
        ctre_phoenix6_sys::c_ctre_phoenix6_free_memory(&mut str_);
        Ok(string)
    }
}

pub fn serialize_bool(spn: SPN, bool: bool) -> Status<String> {
    unsafe {
        let mut str_: *mut ::std::os::raw::c_char = ptr::null_mut();
        ctre_phoenix6_sys::c_ctre_phoenix6_serialize_bool(i32::from(spn), bool, &mut str_)
            .to_result()?;
        let string = CStr::from_ptr(str_)
            .to_str()
            .map_err(|_| StatusCode::CouldNotSerialize)?
            .to_owned();
        ctre_phoenix6_sys::c_ctre_phoenix6_free_memory(&mut str_);
        Ok(string)
    }
}

pub fn deserialize_int(spn: SPN, string: &str) -> Status<i32> {
    unsafe {
        let mut int: i32 = 0;
        ctre_phoenix6_sys::c_ctre_phoenix6_deserialize_int(
            i32::from(spn),
            string.as_ptr() as *const c_char,
            string.len() as u32,
            &mut int,
        )
        .to_result()?;
        Ok(int)
    }
}

pub fn deserialize_double(spn: SPN, string: &str) -> Status<f64> {
    unsafe {
        let mut double: f64 = 0.0;
        ctre_phoenix6_sys::c_ctre_phoenix6_deserialize_double(
            i32::from(spn),
            string.as_ptr() as *const c_char,
            string.len() as u32,
            &mut double,
        )
        .to_result()?;
        Ok(double)
    }
}

pub fn deserialize_bool(spn: SPN, string: &str) -> Status<bool> {
    unsafe {
        let mut bool: bool = false;
        ctre_phoenix6_sys::c_ctre_phoenix6_deserialize_bool(
            i32::from(spn),
            string.as_ptr() as *const c_char,
            string.len() as u32,
            &mut bool,
        )
        .to_result()?;
        Ok(bool)
    }
}

pub fn set_config(
    device: DeviceIdentifier,
    config: impl ConfigProtocol,
    timeout: f64,
    future_proof_configs: bool,
    override_if_duplicate: bool,
) -> Status<()> {
    let config_string = config.serialize()?;
    unsafe {
        ctre_phoenix6_sys::c_ctre_phoenix6_set_configs(
            0,
            device.canbus.as_ptr() as *const c_char,
            device.hash.0 as i32,
            timeout,
            config_string.as_ptr() as *const c_char,
            config_string.len() as u32,
            future_proof_configs,
            override_if_duplicate,
            false,
        )
        .to_result()
    }
}

pub fn get_config<T: ConfigProtocol>(device: DeviceIdentifier, timeout: f64) -> Status<T> {
    unsafe {
        let mut config: *mut ::std::os::raw::c_char = ptr::null_mut();
        ctre_phoenix6_sys::c_ctre_phoenix6_get_configs(
            0,
            device.canbus.as_ptr() as *const c_char,
            device.hash.0 as i32,
            timeout,
            &mut config,
            false,
        )
        .to_result()?;
        let str_buffer = CStr::from_ptr(config)
            .to_str()
            .map_err(|_| StatusCode::CouldNotSerialize)?;
        let ret = T::deserialize(str_buffer);
        ctre_phoenix6_sys::c_ctre_phoenix6_free_memory(&mut config);
        ret
    }
}
