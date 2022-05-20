use std::str::FromStr;
use crate::types::AdoptiumJvmImpl::HotSpot;
use crate::types::HeapSize::Normal;
use serde::Serialize;
use serde::{Deserialize};


use strum_macros::{Display, EnumString};

#[cfg(feature = "time_converter")]
pub mod time_converter {
    pub const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%:z";

    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CLib {
    MUSL,
    GLIBC,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum AdoptiumJvmImpl {
    HotSpot,
}

impl Default for AdoptiumJvmImpl {
    fn default() -> Self {
        HotSpot
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum HeapSize {
    Normal,
    Large,
}

impl Default for HeapSize {
    fn default() -> Self {
        Normal
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Project {
    JDK,
    Valhalla,
    Metropolis,
    JRF,
    Shenandoah,
}

impl Default for Project {
    fn default() -> Self {
        Project::JDK
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ImageType {
    JDK,
    JRE,
    TestImage,
    DebugImage,
    StaticLibs,
    Sources,
}

impl Default for ImageType {
    fn default() -> Self {
        ImageType::JDK
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
pub enum ReleaseType {
    #[serde(rename = "ga")]
    #[strum(serialize = "ga")]
    GeneralAvailability,
    #[serde(rename = "ea")]
    #[strum(serialize = "ea")]
    EarlyAccess,
}

impl Default for ReleaseType {
    fn default() -> Self {
        ReleaseType::GeneralAvailability
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Vendor {
    Eclipse,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Architecture {
    #[strum(serialize = "x64", serialize = "x86_64")]
    X64,
    X86,
    X32,
    PPC64,
    PPC64LE,
    S390X,
    AArch64,
    ARM,
    SparcV9,
    RISCV64,
}

impl Default for Architecture {
    fn default() -> Self {
        match Architecture::from_str(std::env::consts::ARCH) {
            Ok(value) => value,
            Err(_error) => {
                panic!("Unsupported Architecture {}", std::env::consts::ARCH)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum OS {
    Linux,
    #[serde(rename = "alpine-linux")]
    AlpineLinux,
    Windows,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "UPPERCASE")]
pub enum SortMethod {
    Default,
    Date,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
pub enum SortOrder {
    #[serde(rename = "DESC")]
    Descending,
    #[serde(rename = "ASC")]
    Ascending,
}

impl Default for OS {
    fn default() -> Self {
        OS::from_str(std::env::consts::OS).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct SystemProperties {
    pub os: OS,
    pub architecture: Architecture,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Sort {
    pub sort_order: SortOrder,
    pub sort_method: SortMethod,
    pub page: i64,
    pub page_size: i64,
}

impl Default for Sort {
    fn default() -> Self {
        Sort {
            sort_order: SortOrder::Descending,
            sort_method: SortMethod::Default,
            page: 0,
            page_size: 10,
        }
    }
}
