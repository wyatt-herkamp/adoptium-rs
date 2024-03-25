use adoptium_api::{
    error::AdoptiumError, requests::release_information::ReleaseInformationParamsBuilderError,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstallerError {
    #[error("Failed to Serialize {0}")]
    TOMLSerError(#[from] toml::ser::Error),
    #[error("Failed to Deserialize {0}")]
    TOMLDeError(#[from] toml::de::Error),
    #[error("Internal Error {0}")]
    Custom(String),
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("{0}")]
    Adoptium(#[from] AdoptiumError),
    #[error("Missing parameter {0}")]
    MissingParameter(#[from] ReleaseInformationParamsBuilderError),
}

impl From<reqwest::Error> for InstallerError {
    fn from(err: reqwest::Error) -> InstallerError {
        InstallerError::Custom(err.to_string())
    }
}
