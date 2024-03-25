#[doc(hidden)]
#[path = "linux/mod.rs"]
pub mod installer;

use serde::de::DeserializeOwned;

pub trait SystemInstaller {
    type Config: Default + Clone + DeserializeOwned;
}
#[cfg(target_os = "linux")]
pub use installer::{config::SysConfig, SysInstaller};
