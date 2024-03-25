#![allow(async_fn_in_trait)]

use std::{fmt::Debug, sync::Arc};

use derive_more::{AsRef, Deref};
use requests::release_information::{ReleaseInformationParams, ReleaseInformationRequest};
use reqwest::{Client, ClientBuilder};

pub mod error;
pub mod requests;
pub mod response;
pub mod types;
pub use types::*;
#[derive(AsRef, Deref, Debug, Clone)]
pub struct Adoptium(pub Arc<Inner>);

/// Inner struct for Adoptium
#[doc(hidden)]
pub struct Inner {
    pub client: Client,
}
impl Debug for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inner").finish()
    }
}

impl Adoptium {
    /// Lists of information about builds that match the query
    /// [api.adoptium.net](https://api.adoptium.net/q/swagger-ui/#/Assets/searchReleases)
    pub fn release_information_request(
        &self,
        params: impl Into<ReleaseInformationParams>,
    ) -> ReleaseInformationRequest {
        ReleaseInformationRequest {
            client: self.clone(),
            params: params.into(),
        }
    }
    pub fn new<V: AsRef<str>>(user_agent: V) -> Adoptium {
        let client = ClientBuilder::new()
            .user_agent(user_agent.as_ref())
            .build()
            .unwrap();
        Adoptium(Arc::new(Inner { client }))
    }

    /// Builds a URL
    pub fn build_url(&self, dest: &str) -> String {
        format!("https://api.adoptium.net/v3/{}", dest)
    }
}
