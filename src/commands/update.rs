use std::env::temp_dir;
use std::time::SystemTime;

use crate::download::download;
use crate::error::InstallerError;
use crate::sys::SysInstaller;
use crate::utils::get_latest_version;
use crate::{Install, InstallTable, Installer, UpToDate};
use clap::Args;

use tabled::settings::Style;
use tabled::Table;
use url::Url;

#[derive(Args)]
pub struct UpdateCommand {
    #[clap(short, long)]
    pub list: bool,
    #[clap(short, long, required_unless_present = "list")]
    pub update: Option<String>,
}

pub async fn execute(app: Installer, value: UpdateCommand) -> Result<(), InstallerError> {
    if value.list {
        list_updates(app, value).await
    } else if value.update.is_some() {
        update(app, value).await
    } else {
        // HELP!
        Ok(())
    }
}

async fn list_updates(app: Installer, _install: UpdateCommand) -> Result<(), InstallerError> {
    let mut versions = Vec::new();
    for install in app.installs.iter() {
        let datum = get_latest_version(&install.config.install_settings).await?;
        let up_to_date = if datum.version_data > install.config.current_version {
            UpToDate::No(datum.version_data.semver)
        } else {
            continue;
        };
        versions.push(InstallTable {
            version: &install.config.current_version.semver,
            location: install
                .config
                .install_location
                .as_os_str()
                .to_str()
                .unwrap(),
            installed_on: install.config.human_date_time().to_string(),
            id: install.config.to_string(),
            up_to_date,
        })
    }
    println!("{}", Table::new(&versions).with(Style::ascii()));
    Ok(())
}

async fn update(mut app: Installer, install: UpdateCommand) -> Result<(), InstallerError> {
    let value = install.update.unwrap();
    if value.eq("all") {
        for install in app.installs.iter_mut() {
            update_internal(install).await?;
        }
    } else {
        match app.installs.iter_mut().find(|v| v.config.eq(&value)) {
            None => {
                println!("Installation by that name not found")
            }
            Some(value) => {
                update_internal(value).await?;
            }
        }
    }
    Ok(())
}

async fn update_internal(install: &mut Install) -> Result<(), InstallerError> {
    let datum = get_latest_version(&install.config.install_settings).await?;
    if datum.version_data <= install.config.current_version {
        println!(
            "{} is already on the latest version {}",
            &install.config, &datum.version_data.semver
        );
        return Ok(());
    } else {
        println!(
            "Updating {} to version {}",
            &install.config, &datum.version_data.semver
        );
    }

    install.config.current_version = datum.version_data;
    install.config.install_time = SystemTime::now().into();

    let temp_file = temp_dir().join(install.config.to_string());
    let binary = datum.binaries.first().unwrap();
    download(
        Url::try_from(binary.package.link.as_ref()).unwrap(),
        binary.package.size as u64,
        temp_file.clone(),
    )
    .await?;
    println!("Download Complete. Moving Files");
    let mut installer = SysInstaller::new(&install.config, temp_file);
    installer.find_internal_data().await?;
    installer.move_data().await?;
    install.update().await?;
    Ok(())
}
