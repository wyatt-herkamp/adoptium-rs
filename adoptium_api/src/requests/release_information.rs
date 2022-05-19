use crate::error::{AdoptiumError, IntoResult};
use crate::requests::AdoptiumRequest;
use crate::response::{Package, Source, VersionData};
use crate::types::SortMethod::Default;
use crate::types::{
    AdoptiumJvmImpl, Architecture, CLib, HeapSize, ImageType, Project, ReleaseType, Sort,
    SystemProperties, Vendor, OS,
};
use crate::Adoptium;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

impl Adoptium {
    /// Lists of information about builds that match the query
    /// [api.adoptium.net](https://api.adoptium.net/q/swagger-ui/#/Assets/searchReleases)
    pub fn release_information_request(&self, feature_version: i64) -> ReleaseInformationRequest {
        ReleaseInformationRequest {
            client: self,
            feature_version,
            release_types: None,
            query_params: ReleaseInformationQueryParams::default(),
        }
    }
}

#[derive(Clone, Serialize, Default)]
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

impl<'a> ReleaseInformationRequest<'a> {
    pub fn apply_defaults(mut self) -> Self {
        use core::default::Default;
        self.query_params.jvm_impl = Some(Default::default());
        self.query_params.local_system = Some(Default::default());
        self.query_params.project = Some(Default::default());
        self.query_params.image_type = Some(Default::default());
        self.query_params.sort = Some(Default::default());
        self.query_params.heap_size = Some(Default::default());
        self
    }
    pub fn release_type(mut self, release_type: ReleaseType) -> Self {
        self.release_types = Some(release_type);
        self
    }
    pub fn local_system(mut self, local: SystemProperties) -> Self {
        self.query_params.local_system = Some(local);
        self
    }
    pub fn image_type(mut self, value: ImageType) -> Self {
        self.query_params.image_type = Some(value);
        self
    }
    pub fn vendor(mut self, value: Vendor) -> Self {
        self.query_params.vendor = Some(value);
        self
    }
    pub fn jvm_impl(mut self, value: AdoptiumJvmImpl) -> Self {
        self.query_params.jvm_impl = Some(value);
        self
    }
    pub fn sort(mut self, value: Sort) -> Self {
        self.query_params.sort = Some(value);
        self
    }
}

impl<'a> Display for ReleaseInformationRequest<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "assets/feature_releases/{}/{}?{}",
            self.feature_version,
            self.release_types
                .as_ref()
                .unwrap_or(&ReleaseType::default()),
            serde_qs::to_string(&self.query_params).unwrap()
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReleaseInformationDatum {
    pub binaries: Vec<Binary>,
    pub download_count: i64,
    pub id: String,
    pub release_link: String,
    pub release_name: String,
    pub release_type: ReleaseType,
    pub source: Option<Source>,
    pub timestamp: String,
    pub updated_at: String,
    pub vendor: Vendor,
    pub version_data: VersionData,
}

#[derive(Serialize, Deserialize)]
pub struct Binary {
    pub architecture: Architecture,
    pub download_count: i64,
    pub heap_size: HeapSize,
    pub image_type: ImageType,
    pub jvm_impl: AdoptiumJvmImpl,
    pub os: OS,
    pub package: Package,
    pub project: Project,
    pub scm_ref: String,
    pub updated_at: String,
    pub installer: Option<Package>,
}

impl<'a> AdoptiumRequest<'a, Vec<ReleaseInformationDatum>> for ReleaseInformationRequest<'a> {
    fn get_client(&self) -> &'a Adoptium {
        self.client
    }
}
