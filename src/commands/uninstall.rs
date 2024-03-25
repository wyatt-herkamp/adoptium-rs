use crate::error::InstallerError;
use crate::sys::SysInstaller;
use crate::Installer;
use clap::Args;
use tokio::fs::{remove_dir_all, remove_file};

#[derive(Args)]
pub struct UninstallCommand {
    #[clap(short, long)]
    pub version: String,
}

pub async fn execute(mut app: Installer, install: UninstallCommand) -> Result<(), InstallerError> {
    match app
        .installs
        .iter_mut()
        .find(|value| value.config.eq(&install.version))
    {
        None => {
            println!("Install Not found");
        }
        Some(value) => {
            println!("Uninstalling");
            SysInstaller::remove_install(&app.settings.system, &value).await?;
            remove_dir_all(&value.config.install_location).await?;

            println!("Removing Config");
            remove_file(&value.install_file).await?;
        }
    }

    Ok(())
}
