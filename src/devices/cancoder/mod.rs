pub mod config;
pub mod signals;

use parking_lot::RwLock;
use std::sync::Arc;

use crate::Status;

use self::{config::CanCoderConfigType, signals::CanCoderCache};

use super::{close_device, config_native, propose_device, DeviceCanID, DeviceIdentifier};

//do not implement clone for this
#[derive(Debug)]
pub struct CanCoder {
    identifier: DeviceIdentifier,
    cache: Arc<RwLock<CanCoderCache>>,
}
impl CanCoder {
    pub fn new(can_id: impl Into<DeviceCanID>, can_bus: String) -> Status<Self> {
        let identifier =
            DeviceIdentifier::try_new(can_id.into(), super::Model::cancoder(), can_bus)?;
        propose_device(identifier.clone())?;
        let inner = Arc::new(RwLock::new(Default::default()));
        Ok(Self {
            identifier,
            cache: inner,
        })
    }

    pub fn configurator(&mut self) -> CanCoderConfigurator {
        CanCoderConfigurator {
            identifier: &mut self.identifier,
        }
    }
}

impl Drop for CanCoder {
    fn drop(&mut self) {
        close_device(&self.identifier);
    }
}

pub struct CanCoderConfigurator<'hw> {
    //mutable so it holds a unique reference to the device
    identifier: &'hw mut DeviceIdentifier,
}
impl CanCoderConfigurator<'_> {
    pub fn apply_config(&mut self, config: impl CanCoderConfigType) -> Status<()> {
        let fpc = config.future_proof_configs();
        config_native::set_config(
            self.identifier.clone(),
            config,
            crate::DEFAULT_TIMEOUT,
            fpc,
            true,
        )
    }
    pub fn apply_config_timeout(
        &mut self,
        config: impl CanCoderConfigType,
        timeout: f64,
    ) -> Status<()> {
        let fpc = config.future_proof_configs();
        config_native::set_config(self.identifier.clone(), config, timeout, fpc, true)
    }
    pub fn get_config<T: CanCoderConfigType>(&self) -> Status<T> {
        config_native::get_config(self.identifier.clone(), crate::DEFAULT_TIMEOUT)
    }
    pub fn get_config_timeout<T: CanCoderConfigType>(&self, timeout: f64) -> Status<T> {
        config_native::get_config(self.identifier.clone(), timeout)
    }
}
