use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use tabled::Tabled;


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


#[cfg_attr(test, derive(Tabled))]
#[derive(Serialize, Deserialize)]
pub struct VersionData {
    pub major: i64,
    pub build: i64,
    pub minor: i64,
    pub security: i64,
    /// Equal to {major}.{minor}.{security}+{build}
    pub semver: String,
    #[cfg_attr(test, tabled(skip))]
    pub openjdk_version: String,
    #[cfg_attr(test, tabled(skip))]
    pub adopt_build_number: Option<i64>,
}

impl From<(i64, i64, i64, i64)> for VersionData {
    fn from((major, build, minor, security): (i64, i64, i64, i64)) -> Self {
        Self {
            major,
            build,
            minor,
            security,
            semver: format!("{major}.{minor}.{security}+{build}"),
            openjdk_version: "".to_string(),
            adopt_build_number: None,
        }
    }
}

impl PartialEq<Self> for VersionData {
    fn eq(&self, other: &Self) -> bool {
        self.semver.eq(&other.semver)
    }
}

impl Eq for VersionData {}


impl PartialOrd for VersionData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VersionData {
    fn cmp(&self, other: &Self) -> Ordering {
        let values = vec![(self.major, other.major), (self.minor, other.minor), (self.security, other.security), (self.build, other.build)];
        for (a, b) in values {
            match a.cmp(&b) {
                Less => {
                    return Less;
                }
                Greater => {
                    return Greater;
                }
                _ => {}
            }
        }
        Equal
    }
}

#[cfg(test)]
pub mod version_test {
    use tabled::{Style, Table};
    use crate::response::VersionData;

    #[test]
    pub fn test() {
        let mut vec = vec![VersionData::from((8, 0, 1, 5)),
                           VersionData::from((11, 0, 1, 5)),
                           VersionData::from((11, 0, 5, 44)),
                           VersionData::from((5, 5, 1, 5)),
                           VersionData::from((17, 0, 5, 1))];
        vec.sort();
        println!("{}", Table::new(&vec).with(Style::ascii()));
    }
}

impl Display for VersionData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.semver)
    }
}