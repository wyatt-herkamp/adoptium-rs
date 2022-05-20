use adoptiummd::config::{get_installs, save_settings, Settings};

use adoptiummd::{config, install, list, uninstall, update, LinuxInstaller};
use clap::{Parser, Subcommand};

use std::path::PathBuf;




#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct AdoptiumClI {
    #[clap(subcommand)]
    command: Subcommands,
}

/// Doc comment
#[derive(Subcommand)]
enum Subcommands {
    Install(install::InstallCommand),
    Uninstall(uninstall::UninstallCommand),
    List(list::ListCommand),
    Update(update::UpdateCommand),
}

#[tokio::main]
async fn main() {
    if whoami::username().eq("root") {
        println!("This applications must be ran as root!");
        return;

    }
    let value = AdoptiumClI::parse();
    let option = config::get_settings().await.expect("Unable to load config");
    let config = if let Some(value) = option {
        value
    } else {
        let settings = Settings {
            install_location: PathBuf::from("/").join("usr").join("lib").join("jvm"),
            install_method: Default::default(),
        };
        save_settings(&settings)
            .await
            .expect("Unable to save config");
        settings
    };

    let vec = get_installs()
        .await
        .expect("Unable to load Installs")
        .into_iter()
        .map(|value| value.into())
        .collect();
    let app = LinuxInstaller {
        settings: config,
        installs: vec,
    };
    match value.command {
        Subcommands::Install(value) => install::execute(app, value).await,
        Subcommands::Uninstall(value) => uninstall::execute(app, value).await,
        Subcommands::List(value) => list::execute(app, value).await,
        Subcommands::Update(value) => update::execute(app, value).await,
    }
    .unwrap();
}
