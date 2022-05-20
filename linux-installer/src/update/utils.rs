use adoptium_api::Adoptium;
use adoptium_api::requests::AdoptiumRequest;
use adoptium_api::requests::release_information::ReleaseInformationDatum;
use adoptium_api::types::{Sort, SortMethod, SortOrder, SystemProperties};
use crate::{ADOPTIUM_USER_AGENT, InstallConfig, InstallerError};
use crate::config::InstallSettings;

pub async fn get_latest_version(config: &InstallSettings)->Result<ReleaseInformationDatum, InstallerError>{
    let adoptium = Adoptium::new(ADOPTIUM_USER_AGENT);
    let mut request = adoptium
        .release_information_request(config.feature_version)
        .local_system(SystemProperties::default())
        .image_type(config.image_type.clone())
        .jvm_impl(config.jvm_impl.clone())
        .release_type(config.release_type.clone())
        .sort(Sort {
            sort_order: SortOrder::Descending,
            sort_method: SortMethod::Default,
            page: 0,
            page_size: 1,
        })
        .execute()
        .await?;

    Ok(request.remove(0))
}