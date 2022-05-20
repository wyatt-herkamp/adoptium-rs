use crate::{InstallConfig, InstallerError};
use futures_util::StreamExt;
use std::path::PathBuf;
use std::process::{ExitCode, Termination};
use tokio::fs::{create_dir_all, read_dir, remove_dir_all, rename};
use tokio::process::Command;
use tokio_stream::wrappers::ReadDirStream;
use adoptium_api::types::ImageType;
use crate::config::InstallMethod;

pub struct Installer<'a> {
    pub extracted_data: PathBuf,
    pub install_data: &'a InstallConfig,
}

impl<'a> Installer<'a> {
    pub fn new(data: &'a InstallConfig, extracted: PathBuf) -> Installer {
        Installer {
            extracted_data: extracted,
            install_data: data,
        }
    }
    pub async fn find_internal_data(&mut self) -> Result<(), InstallerError> {
        let mut stream = ReadDirStream::new(read_dir(&self.extracted_data).await?);
        while let Some(value) = stream.next().await {
            let entry = value?;
            if entry.metadata().await?.is_dir() {
                if entry.path().join("bin").exists() {
                    self.extracted_data = entry.path();
                    drop(stream);
                    break;
                }
            }
        }
        Ok(())
    }
    pub async fn move_data(&self) -> Result<(), InstallerError> {
        let install_location = self.install_data.install_location.clone();
        if install_location.exists() {
            remove_dir_all(&install_location).await?;
        }
        create_dir_all(&install_location).await?;
        let mut stream = ReadDirStream::new(read_dir(&self.extracted_data).await?);
        while let Some(value) = stream.next().await {
            let entry = value?;
            rename(entry.path(), install_location.join(entry.file_name())).await?;
        }
        Ok(())
    }
    pub async fn update_system(&self, install: &InstallMethod) -> Result<(), InstallerError> {
        Installer::run_command(Command::new("chmod").arg("-Rv").arg("755").arg(self.install_data.install_location.as_os_str())).await?;

        match install {
            InstallMethod::UpdateAlternatives(value) => {
                let paths = match self.install_data.install_settings.image_type {
                    ImageType::JDK => {
                        &value.jdk_paths
                    }
                    ImageType::JRE => {
                        &value.jre_paths
                    }
                    _ => {
                        return Ok(());
                    }
                };
                for value in paths {
                    let path = self.install_data.install_location.join("bin").join(&value.exec_name);
                    let code = Installer::run_command(
                        Command::new("update-alternatives").arg("--install").arg(&value.system_path).arg(&value.exec_name).arg(path.as_os_str()).arg("1")).await?;
                    if code != 0 {
                        //TODO handle Command Error
                    }
                }
            }
        }
        Ok(())
    }
    #[cfg(feature = "mock_commands")]
    async fn run_command(command: &mut Command) -> Result<u8, InstallerError> {
        println!("Imagine Running {:?}", command);
        Ok(0)
    }
    #[cfg(not(feature = "mock_commands"))]
    async fn run_command(command: &mut Command) -> Result<u8, InstallerError> {
        Ok(command.spawn()?.wait().await?.code().unwrap_or(1) as u8)
    }
}
