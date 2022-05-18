use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::Adoptium;
use crate::error::{AdoptiumError, IntoResult};
use crate::types::{AdoptiumJvmImpl, Architecture, CLib, HeapSize, ImageType, OS, Project, ReleaseType, Sort, SystemProperties, Vendor};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use crate::requests::AdoptiumRequest;
use crate::response::{Package, Source, VersionData};
use crate::types::SortMethod::Default;

#[derive(Clone, Serialize)]
pub struct ReleaseInformationQueryParams {
    #[serde(flatten)]
    pub local_system: Option<SystemProperties>,
    #[serde(flatten)]
    pub sort: Option<Sort>,
    pub heap_size: Option<HeapSize>,
    pub image_type: Option<ImageType>,
    pub jvm_impl: Option<AdoptiumJvmImpl>,
    pub vendor: Option<Vendor>,
    pub project: Option<Project>,
    pub c_lib: Option<CLib>,
}

#[derive(Clone)]
pub struct ReleaseInformationRequest<'a> {
    pub client: &'a Adoptium,
    pub feature_version: i64,
    pub release_types: Option<ReleaseType>,
    pub query_params: ReleaseInformationQueryParams,
}

impl<'a> Display for ReleaseInformationRequest<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "/assets/feature_release/{}/{}?{}", self.feature_version, self.release_types.as_ref().unwrap_or(&ReleaseType::default()), serde_qs::to_string(&self.query_params).unwrap())
    }
}


#[derive(Serialize, Deserialize)]
pub struct ReleaseInformationDatum {
    binaries: Vec<Binary>,
    download_count: i64,
    id: String,
    release_link: String,
    release_name: String,
    release_type: String,
    source: Option<Source>,
    timestamp: String,
    updated_at: String,
    vendor: String,
    version_data: VersionData,
}

#[derive(Serialize, Deserialize)]
pub struct Binary {
    architecture: Architecture,
    download_count: i64,
    heap_size: HeapSize,
    image_type: ImageType,
    jvm_impl: AdoptiumJvmImpl,
    os: OS,
    package: Package,
    project: Project,
    scm_ref: String,
    updated_at: String,
    installer: Option<Package>,
}


impl<'a> AdoptiumRequest<'a, Vec<ReleaseInformationDatum>> for ReleaseInformationRequest<'a> {
    fn get_client(&self) -> &'a Adoptium {
        self.client
    }
}