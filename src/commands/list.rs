use crate::error::InstallerError;
use crate::utils::get_latest_version;
use crate::{InstallTable, Installer, UpToDate};
use clap::Args;
use tabled::settings::Style;
use tabled::Table;
use tracing::warn;

#[derive(Args)]
pub struct ListCommand {}

pub async fn execute(app: Installer, _install: ListCommand) -> Result<(), InstallerError> {
    let mut versions = Vec::new();
    for install in app.installs.iter() {
        let datum = get_latest_version(&install.config.install_settings).await?;
        let up_to_date = if datum.version_data > install.config.current_version {
            UpToDate::No(datum.version_data.semver)
        } else {
            UpToDate::Yes
        };
        let location = install
            .config
            .install_location
            .as_os_str()
            .to_str()
            .unwrap_or_else(|| {
                warn!("Non UTF-8 Path: {:?}", install.config.install_location);
                "Unknown"
            });
        versions.push(InstallTable {
            version: &install.config.current_version.semver,
            location,
            installed_on: install.config.human_date_time().to_string(),
            id: install.config.to_string(),
            up_to_date,
        })
    }
    println!("{}", Table::new(&versions).with(Style::ascii()));
    Ok(())
}
