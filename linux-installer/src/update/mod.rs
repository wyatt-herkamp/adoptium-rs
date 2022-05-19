use crate::error::InstallerError;
use crate::LinuxInstaller;
use clap::Args;

#[derive(Args)]
pub struct UpdateCommand {}

pub async fn execute(
    mut app: LinuxInstaller,
    install: UpdateCommand,
) -> Result<(), InstallerError> {
    println!("Update");
    return Ok(());
}
