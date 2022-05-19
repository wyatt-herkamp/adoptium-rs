use crate::error::InstallerError;
use crate::LinuxInstaller;
use clap::Args;

#[derive(Args)]
pub struct UninstallCommand {}

pub async fn execute(
    mut app: LinuxInstaller,
    install: UninstallCommand,
) -> Result<(), InstallerError> {
    println!("Uninstall");

    return Ok(());
}
