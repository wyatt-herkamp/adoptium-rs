use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub checksum: String,
    pub checksum_link: Option<String>,
    pub download_count: i64,
    pub link: String,
    pub metadata_link: String,
    pub name: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    link: String,
    name: String,
    size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct VersionData {
    pub build: i64,
    pub major: i64,
    pub minor: i64,
    pub openjdk_version: String,
    pub security: i64,
    pub semver: String,
    pub adopt_build_number: Option<i64>,
}
