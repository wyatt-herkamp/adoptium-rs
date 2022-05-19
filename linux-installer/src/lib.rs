use crate::config::{get_config_directory, InstallConfig, Settings};
use crate::error::InstallerError;
use clap::{ArgEnum, PossibleValue};
use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

pub mod config;

pub mod download;
pub mod error;
pub mod install;
pub mod list;
pub mod uninstall;
pub mod update;

pub const ADOPTIUM_USER_AGENT: &str = "Adoptium Linux Installer(github.com/wherkamp/adoptium-rs)";

pub struct Install {
    pub install_file: PathBuf,
    pub config: InstallConfig,
}

impl PartialEq<InstallConfig> for Install {
    fn eq(&self, other: &InstallConfig) -> bool {
        return self.config.to_string().eq(&other.to_string());
    }
}

impl Install {
    pub async fn update(&mut self) -> Result<(), InstallerError> {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.install_file)
            .await?;
        let value = toml::to_string_pretty(&self.config)?;
        file.write_all(value.as_bytes()).await?;
        Ok(())
    }
}

impl From<(PathBuf, InstallConfig)> for Install {
    fn from((file, config): (PathBuf, InstallConfig)) -> Self {
        Install {
            install_file: file,
            config,
        }
    }
}

pub struct LinuxInstaller {
    pub settings: Settings,
    pub installs: Vec<Install>,
}

impl LinuxInstaller {
    pub fn does_install_exist(&self, config: InstallConfig) -> bool {
        for x in self.installs.iter() {
            if x.eq(&config) {
                return true;
            }
        }
        return false;
    }
    pub async fn add_install(&mut self, config: InstallConfig) -> Result<(), InstallerError> {
        let install_loc = get_config_directory().join("installs").join(format!("{}.toml", &config));
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&install_loc)
            .await?;
        let value = toml::to_string_pretty(&config)?;
        file.write_all(value.as_bytes()).await?;
        self.installs.push(Install {
            install_file: install_loc,
            config,
        });
        Ok(())
    }
}
