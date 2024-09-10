#![allow(dead_code)]
use std::os::raw::c_int;

use frclib_core::units::time::Time;

use crate::{devices::DeviceIdentifier, error::StatusCodeType, spn::SPN, Status};

use super::{SPNValue, SignalValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignalSpecifier {
    pub hash: u32,
    pub spn: SPN,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SignalValueResponse {
    pub value: f64,
    pub can_timestamp: f64,
    pub software_timestamp: f64,
    pub device_timestamp: f64,
}
impl SignalValueResponse {
    pub fn try_cast<T: SPNValue>(self) -> Status<SignalValue<T>> {
        Ok(SignalValue {
            value: T::try_from_f64(self.value)?,
            can_timestamp: self.can_timestamp,
            software_timestamp: self.software_timestamp,
            device_timestamp: self.device_timestamp,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SignalMeta {
    pub can_bus: String,
    pub timeout: f64,
}
impl SignalMeta {
    fn new(network: String) -> Self {
        Self {
            can_bus: network,
            timeout: crate::DEFAULT_TIMEOUT,
        }
    }

    fn new_with_timeout(network: String, timeout: impl Time) -> Self {
        Self {
            can_bus: network,
            timeout: timeout.to_seconds().value(),
        }
    }
}

pub fn request_signal_values<const N: usize>(
    meta: SignalMeta,
    signals: [SignalSpecifier; N],
) -> Status<[SignalValueResponse; N]> {
    let mut hashes = [0u32; N];
    let mut spns = [0u32; N];
    for (i, req) in signals.iter().enumerate() {
        hashes[i] = req.hash;
        spns[i] = req.spn as u32;
    }

    let mut values = [0f64; N];
    let mut hw_timestamps = [0f64; N];
    let mut sw_timestamps = [0f64; N];
    let mut can_timestamps = [0f64; N];
    unsafe {
        ctre_phoenix6_sys::c_ctre_phoenix6_get_signal_simplified(
            meta.can_bus.as_ptr() as *const i8,
            c_int::from(meta.timeout > 0.0),
            meta.timeout,
            N as u32,
            hashes.as_mut_ptr(),
            spns.as_mut_ptr(),
            values.as_mut_ptr(),
            hw_timestamps.as_mut_ptr(),
            sw_timestamps.as_mut_ptr(),
            can_timestamps.as_mut_ptr(),
        )
        .to_result()?;
    }

    let mut responses = [SignalValueResponse::default(); N];
    for (i, resp) in responses.iter_mut().enumerate() {
        resp.value = values[i];
        resp.can_timestamp = hw_timestamps[i];
        resp.software_timestamp = sw_timestamps[i];
        resp.device_timestamp = can_timestamps[i];
    }

    Ok(responses)
}

pub fn request_signal_values_dynamic(
    meta: SignalMeta,
    signals: &[SignalSpecifier],
) -> Status<Vec<SignalValueResponse>> {
    let mut hashes = Vec::with_capacity(signals.len());
    let mut spns = Vec::with_capacity(signals.len());
    for req in signals {
        hashes.push(req.hash);
        spns.push(req.spn as u32);
    }

    let mut values = Vec::with_capacity(signals.len());
    let mut can_timestamps = Vec::with_capacity(signals.len());
    let mut software_timestamps = Vec::with_capacity(signals.len());
    let mut device_timestamps = Vec::with_capacity(signals.len());
    unsafe {
        ctre_phoenix6_sys::c_ctre_phoenix6_get_signal_simplified(
            meta.can_bus.as_ptr() as *const i8,
            c_int::from(meta.timeout > 0.0),
            meta.timeout,
            signals.len() as u32,
            hashes.as_mut_ptr(),
            spns.as_mut_ptr(),
            values.as_mut_ptr(),
            can_timestamps.as_mut_ptr(),
            software_timestamps.as_mut_ptr(),
            device_timestamps.as_mut_ptr(),
        )
        .to_result()?;
    }

    let mut responses = Vec::with_capacity(signals.len());
    for i in 0..signals.len() {
        responses.push(SignalValueResponse {
            value: values[i],
            can_timestamp: can_timestamps[i],
            software_timestamp: software_timestamps[i],
            device_timestamp: device_timestamps[i],
        });
    }

    Ok(responses)
}

pub fn request_signal_value_single(
    meta: SignalMeta,
    signal: SignalSpecifier,
) -> Status<SignalValueResponse> {
    request_signal_values(meta, [signal]).map(|v| v[0])
}

pub fn set_update_freq(meta: SignalMeta, signal: SignalSpecifier, freq_hz: f64) -> Status<()> {
    unsafe {
        ctre_phoenix6_sys::c_ctre_phoenix6_SetUpdateFrequency(
            0,
            meta.can_bus.as_ptr() as *const i8,
            signal.hash,
            signal.spn as u16,
            freq_hz,
            meta.timeout,
        )
        .to_result()
    }
}

pub fn set_update_freq_all<T: Time>(
    signals: &[(String, SignalSpecifier)],
    freq_hz: f64,
    timeout: Option<T>,
) -> Status<()> {
    let mut c_signal_list: Vec<ctre_phoenix6_sys::network_signal_t> =
        Vec::with_capacity(signals.len());
    for (network, signal) in signals {
        c_signal_list.push(ctre_phoenix6_sys::network_signal_t {
            network: network.as_ptr() as *const i8,
            signal: ctre_phoenix6_sys::signal_values_t {
                deviceHash: signal.hash,
                spn: signal.spn as u32,
            },
        });
    }
    unsafe {
        ctre_phoenix6_sys::c_ctre_phoenix6_SetUpdateFrequencyForAll(
            0,
            c_signal_list.as_mut_ptr(),
            signals.len(),
            freq_hz,
            timeout
                .map(|t| t.to_seconds().value())
                .unwrap_or(crate::DEFAULT_TIMEOUT),
        )
        .to_result()
    }
}

pub fn optimize_signals(meta: SignalMeta, device: DeviceIdentifier) -> Status<()> {
    unsafe {
        ctre_phoenix6_sys::c_ctre_phoenix6_OptimizeUpdateFrequencies(
            0,
            meta.can_bus.as_ptr() as *const i8,
            device.hash.0,
            meta.timeout,
        )
        .to_result()
    }
}

pub fn resend_freq_updates(meta: SignalMeta, device: DeviceIdentifier) -> Status<()> {
    unsafe {
        ctre_phoenix6_sys::c_ctre_phoenix6_ResendUpdateFrequencies(
            0,
            meta.can_bus.as_ptr() as *const i8,
            device.hash.0,
            meta.timeout,
        )
        .to_result()
    }
}
