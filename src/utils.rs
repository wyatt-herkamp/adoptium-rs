use crate::config::InstallSettings;
use crate::{InstallerError, ADOPTIUM_USER_AGENT};
use adoptium_api::requests::release_information::{
    ReleaseInformationDatum, ReleaseInformationParamsBuilder,
};
use adoptium_api::requests::AdoptiumRequest;
use adoptium_api::types::{SortMethod, SortOrder, SystemProperties, WithSort};
use adoptium_api::Adoptium;

pub async fn get_latest_version(
    config: &InstallSettings,
) -> Result<ReleaseInformationDatum, InstallerError> {
    let adoptium = Adoptium::new(ADOPTIUM_USER_AGENT);

    let request = ReleaseInformationParamsBuilder::default()
        .feature_version(config.feature_version)
        .release_type(config.release_type)
        .with_query_builder(|query| {
            query
                .image_type(Some(config.image_type))
                .jvm_impl(Some(config.jvm_impl))
                .local_system(Some(SystemProperties::default()))
                .with_sort(|sort| {
                    sort.sort_order(SortOrder::Descending)
                        .sort_method(SortMethod::Default)
                        .page(0)
                        .page_size(1);
                });
        })
        .build()
        .expect("Failed to build ReleaseInformationParams");

    let mut request = adoptium
        .release_information_request(request)
        .execute()
        .await?;

    Ok(request.remove(0))
}
