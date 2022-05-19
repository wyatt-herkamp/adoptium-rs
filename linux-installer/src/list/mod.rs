use crate::error::InstallerError;
use crate::LinuxInstaller;
use clap::Args;

#[derive(Args)]
pub struct ListCommand {}

pub async fn execute(mut app: LinuxInstaller, install: ListCommand) -> Result<(), InstallerError> {
    println!("List");
    return Ok(());
}
