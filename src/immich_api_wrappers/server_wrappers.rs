use anyhow::Context;
use immich_client::apis::{configuration};

pub async fn check_api_accessible(
    configuration: &configuration::Configuration
) -> Option<anyhow::Error> {
    immich_client::apis::server_api::get_about_info(configuration)
        .await
        .context("Unable to connect / use the API, check the URL and KEY are correct.")
        .err()
}
