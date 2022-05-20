pub mod release_information;

use crate::error::{AdoptiumError, IntoResult};

use crate::Adoptium;
use async_trait::async_trait;
use serde::de::DeserializeOwned;

use std::fmt::{Display};

#[async_trait]
pub trait AdoptiumRequest<'a, T: DeserializeOwned>: Display {
    fn get_client(&self) -> &'a Adoptium;
    async fn execute(&self) -> Result<T, AdoptiumError> {
        let string = self.to_string();
        self.get_client()
            .get(&string)
            .await?
            .into_result()?
            .json()
            .await
            .map_err(AdoptiumError::from)
    }
}
