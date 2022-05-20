pub mod installer;

use crate::config::InstallSettings;
use crate::download::download;
use crate::error::InstallerError;
use crate::install::installer::Installer;
use crate::{InstallConfig, LinuxInstaller, ADOPTIUM_USER_AGENT};
use adoptium_api::requests::AdoptiumRequest;
use adoptium_api::types::SortMethod::Default;
use adoptium_api::types::{
    AdoptiumJvmImpl, CLib, HeapSize, ImageType, Project, ReleaseType, Sort, SortMethod, SortOrder,
    SystemProperties, Vendor,
};
use adoptium_api::Adoptium;
use chrono::Utc;
use clap::Args;
use std::env::temp_dir;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::fs::read_dir;
use url::Url;

#[derive(Args)]
pub struct InstallCommand {
    #[clap(short, long)]
    /// Defaults to JDK
    pub image_type: Option<ImageType>,
    #[clap(short, long)]
    pub jvm_impl: Option<AdoptiumJvmImpl>,

    #[clap(short, long)]
    /// Defaults to GA
    pub release_type: Option<ReleaseType>,
    #[clap(short, long)]
    /// A Java Version
    pub version: i64,
}

pub async fn execute(
    mut app: LinuxInstaller,
    install: InstallCommand,
) -> Result<(), InstallerError> {
    let adoptium = Adoptium::new(ADOPTIUM_USER_AGENT);
    let mut request = adoptium
        .release_information_request(install.version)
        .local_system(SystemProperties::default())
        .image_type(install.image_type.unwrap_or_default())
        .jvm_impl(install.jvm_impl.unwrap_or_default())
        .release_type(install.release_type.unwrap_or_default())
        .sort(Sort {
            sort_order: SortOrder::Descending,
            sort_method: SortMethod::Default,
            page: 0,
            page_size: 1,
        })
        .execute()
        .await?;
    println!("{}", serde_json::to_string_pretty(&request).unwrap());
    let mut release = request.remove(0);
    let binary = release.binaries.remove(0);
    let download_link = binary.package.link;
    let size = binary.package.size as u64;
    let release_name = release.release_name;
    let mut config = InstallConfig {
        install_settings: InstallSettings {
            heap_size: binary.heap_size,
            image_type: binary.image_type,
            jvm_impl: binary.jvm_impl,
            vendor: release.vendor,
            project: binary.project,
            c_lib: None,
            release_type: release.release_type,
            feature_version: install.version,
        },
        install_location: PathBuf::new(),
        install_time: SystemTime::now().into(),
        current_version: release.version_data,
    };
    if app.does_install_exist(&config) {
        println!("Install Already Exists");
        return Ok(());
    }
    config.install_location = app.settings.install_location.join(config.to_string());

    let temp_file = temp_dir().join(config.to_string());
    download(
        Url::try_from(download_link.as_ref()).unwrap(),
        size,
        temp_file.clone(),
    )
        .await?;
    let mut installer = Installer::new(&config, temp_file);
    installer.find_internal_data().await?;
    installer.move_data().await?;
    installer.update_system(&app.settings.install_method).await?;
    drop(installer);
    app.add_install(config).await?;
    return Ok(());
}
