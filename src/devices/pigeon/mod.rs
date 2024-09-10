pub mod config;
pub mod signals;

use std::sync::Arc;

use frclib_core::units::time::Time;
use parking_lot::RwLock;

use crate::Status;

use self::{config::PigeonConfigType, signals::PigeonCache};

use super::{config_native, propose_device, DeviceCanID, DeviceIdentifier};

//do not implement clone for this
#[derive(Debug)]
pub struct Pigeon2 {
    identifier: DeviceIdentifier,
    cache: Arc<RwLock<PigeonCache>>,
}
impl Pigeon2 {
    pub fn new(can_id: impl Into<DeviceCanID>, can_bus: String) -> Status<Self> {
        let identifier =
            DeviceIdentifier::try_new(can_id.into(), super::Model::pigeon2(), can_bus)?;
        propose_device(identifier.clone())?;
        let cache = Arc::new(RwLock::new(Default::default()));
        Ok(Self { identifier, cache })
    }

    //mutable so it holds a unique reference to the device
    pub fn configurator(&mut self) -> PigeonConfigurator {
        PigeonConfigurator {
            identifier: &mut self.identifier,
        }
    }
}

pub struct PigeonConfigurator<'hw> {
    //mutable so it holds a unique reference to the device
    identifier: &'hw mut DeviceIdentifier,
}
impl PigeonConfigurator<'_> {
    pub fn apply_config(&mut self, config: impl PigeonConfigType) -> Status<()> {
        config_native::set_config(
            self.identifier.clone(),
            config,
            crate::DEFAULT_TIMEOUT,
            true,
            true,
        )
    }
    pub fn apply_config_timeout(
        &mut self,
        config: impl PigeonConfigType,
        timeout: impl Time,
    ) -> Status<()> {
        config_native::set_config(
            self.identifier.clone(),
            config,
            timeout.to_seconds().value(),
            true,
            true,
        )
    }
    pub fn get_config<T: PigeonConfigType>(&self) -> Status<T> {
        config_native::get_config(self.identifier.clone(), crate::DEFAULT_TIMEOUT)
    }
    pub fn get_config_timeout<T: PigeonConfigType>(&self, timeout: impl Time) -> Status<T> {
        config_native::get_config(self.identifier.clone(), timeout.to_seconds().value())
    }
}
