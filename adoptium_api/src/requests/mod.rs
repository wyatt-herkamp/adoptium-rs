pub mod release_information;

use crate::error::{AdoptiumError, IntoResult};

use crate::Adoptium;
use reqwest::Url;
use serde::de::DeserializeOwned;

use std::borrow::Cow;
use tracing::trace;

pub trait AdoptiumRequest {
    type Output: DeserializeOwned;
    fn get_client(&self) -> Adoptium;

    fn get_url(&self) -> Cow<'_, str>;

    async fn execute(&self) -> Result<Self::Output, AdoptiumError> {
        let url = self.get_url();
        trace!(path=?url, "Making request at URL");
        let client = self.get_client();
        let full_url = client.build_url(url.as_ref());
        trace!(path=?full_url, "Full URL");

        let url = Url::parse(full_url.as_str()).map_err(AdoptiumError::from)?;

        let request = client.client.get(url);

        let response = request.send().await?.into_result()?;
        trace!(status=?response.status(), "Response Status");
        let data = response.json::<Self::Output>().await?;
        Ok(data)
    }
}

