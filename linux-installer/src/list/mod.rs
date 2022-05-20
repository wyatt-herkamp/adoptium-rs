pub mod list;

use std::ffi::{OsStr, OsString};
use std::fmt::{Display, Formatter};
use crate::error::InstallerError;
use crate::{LinuxInstaller, update};
use clap::Args;
use tabled::{Style, Table, Tabled};
use crate::list::list::{InstallTable, UpToDate};

#[derive(Args)]
pub struct ListCommand {}

pub async fn execute(mut app: LinuxInstaller, install: ListCommand) -> Result<(), InstallerError> {
    let mut versions = Vec::new();
    for install in app.installs.iter() {
        let datum = update::utils::get_latest_version(&install.config.install_settings).await?;
        let up_to_date = if datum.version_data> install.config.current_version{
            UpToDate::No(datum.version_data.semver)
        }else{
            UpToDate::Yes
        };
        versions.push(InstallTable {
            version: &install.config.current_version.semver,
            location: install.config.install_location.as_os_str().to_str().unwrap(),
            installed_on: install.config.install_time.format(adoptium_api::types::time_converter::FORMAT).to_string(),
            id: install.config.to_string(),
            up_to_date,
        })
    }
    println!("{}", Table::new(&versions).with(Style::ascii()).to_string());
    return Ok(());
}
