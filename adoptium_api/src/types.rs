use crate::types::AdoptiumJvmImpl::HotSpot;
use crate::types::HeapSize::Normal;
use derive_builder::Builder;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use strum::EnumIter;
use strum::IntoEnumIterator;
use tracing::warn;

use strum::{Display, EnumString};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CLib {
    MUSL,
    GLIBC,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display, Copy)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display, Copy)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display, Copy)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display, Copy)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Vendor {
    Eclipse,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumString, Display, Copy, EnumIter)]
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
                let supported_architectures = Architecture::iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");

                warn!(
                    "Unsupported Architecture {}. Supported Architectures {supported_architectures}",
                    std::env::consts::ARCH
                );
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Builder)]
#[builder(default, setter(into))]
#[builder(build_fn(
    private,
    name = "try_build",
    error = "::derive_builder::UninitializedFieldError"
))]
pub struct Sort {
    pub sort_order: SortOrder,
    pub sort_method: SortMethod,
    pub page: i64,
    pub page_size: i64,
}
pub trait WithSort {
    fn set_sort(&mut self, sort: Sort);

    fn with_sort(&mut self, sort_builder: impl FnOnce(&mut SortBuilder)) -> &mut Self {
        let mut sort = SortBuilder::default();
        sort_builder(&mut sort);
        self.set_sort(sort.build());
        self
    }
}
impl SortBuilder {
    pub fn build(&self) -> Sort {
        self.try_build().expect("Failed to build Sort")
    }
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
