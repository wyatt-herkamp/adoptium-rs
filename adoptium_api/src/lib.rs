
use reqwest::{Client, ClientBuilder, Response};







pub mod error;
pub mod requests;
pub mod response;
pub mod types;

pub struct Adoptium {
    pub client: Client,
}

impl Adoptium {
    pub fn new<V: AsRef<str>>(user_agent: V) -> Adoptium {
        let client = ClientBuilder::new()
            .user_agent(user_agent.as_ref())
            .build()
            .unwrap();
        Adoptium { client }
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
