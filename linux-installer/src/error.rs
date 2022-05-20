use adoptium_api::error::AdoptiumError;


use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstallerError {
    #[error("Failed to Serialize {0}")]
    TOMLSerError(toml::ser::Error),
    #[error("Failed to Deserialize {0}")]
    TOMLDeError(toml::de::Error),
    #[error("Internal Error {0}")]
    Custom(String),
    #[error("{0}")]
    IOError(std::io::Error),
    #[error("{0}")]
    Adoptium(AdoptiumError),
}

impl From<std::io::Error> for InstallerError {
    fn from(err: std::io::Error) -> InstallerError {
        InstallerError::IOError(err)
    }
}
impl From<AdoptiumError> for InstallerError {
    fn from(value: AdoptiumError) -> Self {
        InstallerError::Adoptium(value)
    }
}

impl From<toml::de::Error> for InstallerError {
    fn from(err: toml::de::Error) -> InstallerError {
        InstallerError::TOMLDeError(err)
    }
}

impl From<toml::ser::Error> for InstallerError {
    fn from(err: toml::ser::Error) -> InstallerError {
        InstallerError::TOMLSerError(err)
    }
}
impl From<reqwest::Error> for InstallerError {
    fn from(err: reqwest::Error) -> InstallerError {
        InstallerError::Custom(err.to_string())
    }
}
