use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Package {
    checksum: String,
    checksum_link: Option<String>,
    download_count: i64,
    link: String,
    metadata_link: String,
    name: String,
    size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    link: String,
    name: String,
    size: i64,
}

#[derive(Serialize, Deserialize)]
pub struct VersionData {
    build: i64,
    major: i64,
    minor: i64,
    openjdk_version: String,
    security: i64,
    semver: String,
    adopt_build_number: Option<i64>,
}