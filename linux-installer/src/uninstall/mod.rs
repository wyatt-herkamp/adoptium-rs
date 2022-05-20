use crate::error::InstallerError;
use crate::{LinuxInstaller, run_command};
use clap::Args;
use tokio::fs::{remove_dir_all, remove_file};
use tokio::process::Command;
use adoptium_api::types::ImageType;
use crate::config::InstallMethod;

#[derive(Args)]
pub struct UninstallCommand {
    #[clap(short, long)]
    pub version: String,
}

pub async fn execute(
    mut app: LinuxInstaller,
    install: UninstallCommand,
) -> Result<(), InstallerError> {
    match app.installs.iter_mut().find(|value| value.config.eq(&install.version)) {
        None => {
            println!("Install Not found");
        }
        Some(value) => {
            println!("Uninstalling");
            match app.settings.install_method {
                InstallMethod::UpdateAlternatives(alt) => {
                    let paths = match value.config.install_settings.image_type {
                        ImageType::JDK => {
                            Some(&alt.jdk_paths)
                        }
                        ImageType::JRE => {
                            Some(&alt.jre_paths)
                        }
                        _ => {
                            None
                        }
                    };
                    if let Some(paths) = paths {
                        for up_a in paths {
                            let path = value.config.install_location.join("bin").join(&up_a.exec_name);
                            let code = run_command(
                                Command::new("update-alternatives").arg("--remove").arg(&up_a.exec_name).arg(path.as_os_str())).await?;
                            if code != 0 {
                                //TODO handle Command Error
                            }
                        }
                    }
                }
            }
            println!("Removing Install");
            remove_dir_all(&value.config.install_location).await?;

            println!("Removing Config");
            remove_file(&value.install_file).await?;
        }
    }

    Ok(())
}
