use chrono::DateTime;
use immich_client::apis::Error;
use immich_client::apis::configuration;
use immich_client::apis::search_api::SearchAssetsError;
use immich_client::models::{MetadataSearchDto, SearchResponseDto};

/// Runs on the first time a user "registers", afterwards only the delta is necessary
pub async fn get_all_assets(
    configuration: &configuration::Configuration
) -> Result<SearchResponseDto, Error<SearchAssetsError>> {
    // 1. Create Utc DateTime from timestamp (returns Option)
    // 2. Map the inner Utc datetime to a FixedOffset datetime
    let created_after = DateTime::from_timestamp(1, 0)
        .map(|utc_dt| DateTime::from(utc_dt));

    // Get everything
    // TODO: how does it paginate ?
    immich_client::apis::search_api::search_assets(
            &configuration,
            MetadataSearchDto {
                created_after,
                with_deleted: Some(false),
                ..MetadataSearchDto::new()
            }
    ).await
}
