use reqwest::{StatusCode, Response, Body, Client, ClientBuilder};
use std::error::Error;
use std::fmt::{Formatter, Display};
use serde::de::DeserializeOwned;
use reqwest::header::{USER_AGENT, HeaderValue, HeaderMap};

use std::path::{Path, PathBuf};
use std::num::ParseIntError;

pub mod types;
pub mod error;
pub mod requests;
pub mod response;

pub struct Adoptium {
    client: Client,
}

impl Adoptium {
    pub fn new(
        user_agent: String,
    ) -> Adoptium {
        let client = ClientBuilder::new().user_agent(user_agent).build().unwrap();
        Adoptium {
            client,
        }
    }



    pub(crate) async fn get(&self, url: &str) -> Result<Response, reqwest::Error> {
        let string = self.build_url(url);
        self.client.get(string).send().await
    }
    /// Builds a URL
    pub fn build_url(&self, dest: &str) -> String {
        format!("https://api.adoptium.net/v3/{}", dest)
    }
}