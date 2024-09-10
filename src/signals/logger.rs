use std::path::PathBuf;

use crate::{
    error::{StatusCode, StatusCodeType},
    Status,
};

/// Sets the destination for signal logging,
/// restarting logger if the path changed.
pub fn set_logger_path(path: PathBuf) -> Status<()> {
    unsafe {
        let path = path.to_str().ok_or(StatusCode::CouldNotSerialize)?;
        ctre_phoenix6_sys::c_ctre_phoenix6_platform_set_logger_path(
            path.as_ptr() as *const ::std::os::raw::c_char
        )
        .to_result()
    }
}

pub fn start() -> Status<()> {
    unsafe { ctre_phoenix6_sys::c_ctre_phoenix6_platform_start_logger().to_result() }
}

pub fn stop() -> Status<()> {
    unsafe { ctre_phoenix6_sys::c_ctre_phoenix6_platform_stop_logger().to_result() }
}

pub fn enable_auto_logging(enable: bool) -> Status<()> {
    unsafe { ctre_phoenix6_sys::c_ctre_phoenix6_platform_enable_auto_logging(enable).to_result() }
}

const MAX_LOG_PACKET_SIZE: usize = 64;

pub fn write_raw_to_log(name: String, data: &[u8]) -> Status<()> {
    unsafe {
        if data.len() > MAX_LOG_PACKET_SIZE {
            return Err(StatusCode::InvalidSize);
        }

        let data = std::mem::transmute::<&[u8], &[bool]>(data);

        ctre_phoenix6_sys::c_ctre_phoenix6_platform_write_boolean_array(
            name.as_ptr() as *const ::std::os::raw::c_char,
            data.as_ptr(),
            data.len() as u8,
        )
        .to_result()
    }
}
