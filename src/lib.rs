#![allow(async_fn_in_trait)]

use crate::config::{get_config_directory, InstallConfig, Settings};
use crate::error::InstallerError;

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use tabled::Tabled;
use tokio::fs::{create_dir_all, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub mod config;

pub mod commands;
pub mod download;
pub mod error;
pub mod sys;
pub mod utils;

pub const ADOPTIUM_USER_AGENT: &str = "Adoptium Linux Installer(github.com/wherkamp/adoptium-rs)";

pub struct Install {
    pub install_file: PathBuf,
    pub config: InstallConfig,
}

impl PartialEq<InstallConfig> for Install {
    fn eq(&self, other: &InstallConfig) -> bool {
        self.config.to_string().eq(&other.to_string())
    }
}

impl Install {
    pub async fn update(&mut self) -> Result<(), InstallerError> {
        let mut file = OpenOptions::new()
            .create(true)
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

pub struct Installer {
    pub settings: Settings,
    pub installs: Vec<Install>,
}

impl Installer {
    pub fn does_install_exist(&self, config: &InstallConfig) -> bool {
        for x in self.installs.iter() {
            if x.eq(config) {
                return true;
            }
        }
        false
    }
    pub async fn add_install(&mut self, config: InstallConfig) -> Result<(), InstallerError> {
        let parents = get_config_directory().join("installs");
        if !parents.exists() {
            create_dir_all(&parents).await?;
        }
        let install_loc = parents.join(format!("{}.toml", &config));
        let mut file = OpenOptions::new()
            .create(true)
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

#[derive(Tabled)]
pub struct InstallTable<'a> {
    pub version: &'a String,
    pub location: &'a str,
    pub installed_on: String,
    pub id: String,
    pub up_to_date: UpToDate,
}

pub enum UpToDate {
    Yes,
    No(String),
}

impl Display for UpToDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UpToDate::Yes => {
                write!(f, "Yes")
            }
            UpToDate::No(value) => {
                write!(f, "No(Latest: {})", value)
            }
        }
    }
}

#[cfg(rel)]
#[cfg(feature = "mock_commands")]
async fn run_command(command: &mut Command) -> Result<u8, InstallerError> {
    println!("Imagine Running {:?}", command);
    Ok(0)
}

#[cfg(not(feature = "mock_commands"))]
async fn run_command(command: &mut Command) -> Result<u8, InstallerError> {
    Ok(command.spawn()?.wait().await?.code().unwrap_or(1) as u8)
}
