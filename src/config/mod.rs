use crate::error::InstallerError;
use crate::sys::SysConfig;
use adoptium_api::response::VersionData;
use adoptium_api::types::{
    AdoptiumJvmImpl, CLib, HeapSize, ImageType, Project, ReleaseType, Vendor,
};
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use tokio::fs::{create_dir_all, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio_stream::wrappers::ReadDirStream;
use tokio_stream::StreamExt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstallSettings {
    pub heap_size: HeapSize,
    pub image_type: ImageType,
    pub jvm_impl: AdoptiumJvmImpl,
    pub vendor: Vendor,
    pub project: Project,
    pub c_lib: Option<CLib>,
    pub release_type: ReleaseType,
    pub feature_version: i64,
}

#[derive(Serialize, Deserialize)]
pub struct InstallConfig {
    pub install_location: PathBuf,
    pub install_time: DateTime<Local>,
    pub install_settings: InstallSettings,
    pub current_version: VersionData,
}
impl InstallConfig {
    pub fn human_date_time(&self) -> DelayedFormat<StrftimeItems<'_>> {
        self.install_time.format("%Y-%m-%d %H:%M")
    }
}

impl Display for InstallConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{}-{}-{}",
            self.current_version.major,
            self.install_settings.image_type,
            self.install_settings.heap_size,
            self.install_settings.jvm_impl
        )
    }
}

impl PartialEq<String> for InstallConfig {
    fn eq(&self, other: &String) -> bool {
        self.to_string().eq(other)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub install_location: PathBuf,
    pub default_version: Option<i64>,
    pub system: SysConfig,
}

pub fn get_config_directory() -> PathBuf {
    std::env::var("ADOPTIUM_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/etc").join("adoptium"))
}

pub async fn get_installs() -> Result<Vec<(PathBuf, InstallConfig)>, InstallerError> {
    let installs = get_config_directory().join("installs");
    if !installs.exists() {
        return Ok(vec![]);
    }
    let mut values = Vec::new();
    let dir = tokio::fs::read_dir(&installs).await?;
    let mut stream = ReadDirStream::new(dir);
    while let Some(value) = stream.next().await {
        match value {
            Ok(value) => {
                let value = value.path();
                match read_install(&value).await {
                    Ok(ok) => values.push((value, ok)),
                    Err(error) => {
                        println!("Unable to read File {}", error);
                    }
                }
            }
            Err(error) => {
                println!("Unable to read File {}", error);
            }
        }
    }
    Ok(values)
}

pub async fn read_install(path: &PathBuf) -> Result<InstallConfig, InstallerError> {
    let value = tokio::fs::read_to_string(path).await?;
    toml::from_str(&value).map_err(InstallerError::from)
}

pub async fn get_settings() -> Result<Option<Settings>, InstallerError> {
    let config = get_config_directory().join("adoptium.toml");
    if !config.exists() {
        return Ok(None);
    }
    let value = tokio::fs::read_to_string(config).await?;
    toml::from_str(&value)
        .map(Some)
        .map_err(InstallerError::from)
}

pub async fn save_settings(settings: &Settings) -> Result<(), InstallerError> {
    let config = get_config_directory();
    if !config.exists() {
        create_dir_all(&config).await?;
    }
    let config = config.join("adoptium.toml");
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(config)
        .await?;
    let string = toml::to_string_pretty(&settings)?;
    file.write_all(string.as_bytes()).await?;
    Ok(())
}
