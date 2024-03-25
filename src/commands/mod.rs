use adoptium_api::error::AdoptiumError;
use clap::Args;

use crate::Installer;

pub mod install;
pub mod list;
pub mod uninstall;
pub mod update;
pub trait Command: Args {
    async fn execute(self, app: Installer) -> Result<(), AdoptiumError>;
}
