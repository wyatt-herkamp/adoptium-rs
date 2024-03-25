//! Lists of information about builds that match the query
//! [api.adoptium.net](https://api.adoptium.net/q/swagger-ui/#/Assets/searchReleases)
//!
use crate::requests::AdoptiumRequest;
use crate::response::{Package, Source, VersionData};
use std::borrow::Cow;

use crate::types::{
    AdoptiumJvmImpl, Architecture, CLib, HeapSize, ImageType, Project, ReleaseType, Sort,
    SystemProperties, Vendor, WithSort, OS,
};
use crate::Adoptium;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ReleaseInformationRequest {
    pub client: Adoptium,
    pub params: ReleaseInformationParams,
}

#[derive(Debug, Clone, Builder)]
#[builder(default)]
pub struct ReleaseInformationParams {
    pub feature_version: i64,
    pub release_type: ReleaseType,
    pub query_params: ReleaseInformationQueryParams,
}
impl Default for ReleaseInformationParams {
    fn default() -> Self {
        Self {
            feature_version: 21,
            release_type: Default::default(),
            query_params: Default::default(),
        }
    }
}
impl From<i64> for ReleaseInformationParams {
    fn from(feature_version: i64) -> Self {
        Self {
            feature_version,
            ..Default::default()
        }
    }
}
impl ReleaseInformationParamsBuilder {
    pub fn with_query_builder(
        &mut self,
        query_builder: impl FnOnce(&mut ReleaseInformationQueryParamsBuilder),
    ) -> &mut Self {
        let mut query_params = ReleaseInformationQueryParamsBuilder::default();
        query_builder(&mut query_params);
        self.query_params(query_params.build());
        self
    }
}

#[derive(Debug, Clone, Serialize, Default, Builder)]
#[builder(
    default,
    build_fn(
        private,
        name = "try_build",
        error = "::derive_builder::UninitializedFieldError"
    )
)]
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
impl ReleaseInformationQueryParamsBuilder {
    pub fn build(&self) -> ReleaseInformationQueryParams {
        self.try_build().expect("Infallible")
    }
}
impl WithSort for ReleaseInformationQueryParamsBuilder {
    fn set_sort(&mut self, sort: Sort) {
        self.sort = Some(Some(sort));
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

impl AdoptiumRequest for ReleaseInformationRequest {
    type Output = Vec<ReleaseInformationDatum>;

    fn get_client(&self) -> Adoptium {
        self.client.clone()
    }

    fn get_url(&self) -> Cow<'_, str> {
        let url = format!(
            "assets/feature_releases/{}/{}?{}",
            self.params.feature_version,
            self.params.release_type,
            serde_qs::to_string(&self.params.query_params).unwrap()
        );
        Cow::Owned(url)
    }
}
