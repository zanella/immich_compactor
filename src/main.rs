use std::process::exit;
use chrono::DateTime;
use immich_client::apis::configuration::{ApiKey, Configuration};
use immich_client::models::MetadataSearchDto;

/**
  * § Overall plan / architecture:
  *
  * - Able to communicate with Immich's Rest API
  * -- I don't intend to hammer the server, so, sequential is probably
  *      the way to go, re-encoding videos will already saturate the local CPUS
  *
  * - Own DB (Postgres)
  * -- Keep track of assets from the server
  *
  * - Web UI (Dioxus ?)
  * -- Add user's API key through the UI, and then check if it's able to connect
  * -- I want to see the progress, stats, etc
  **/
#[tokio::main]
async fn main() {
    blah().await;
}

async fn blah() {
    let cfg = Configuration {
        base_path: String::from("http://localhost:2283/api"),
        api_key: Some(
            ApiKey {
                prefix: None,
                key: String::from("wrmIFc2FMOwNTUyyUdyBOBgwnvfNRsuY8xyDiy0E")
            }
        ),
        ..Configuration::default()
    };

    ///////////////////////////////////////////////////////////////////////////

    match immich_client::apis::server_api::get_about_info(&cfg).await {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Unable to connect / use the API, check the URL and KEY are correct");
            eprintln!("Error: {}", e);
            exit(1);
        }
    }

    // 1. Create Utc DateTime from timestamp (returns Option)
    // 2. Map the inner Utc datetime to a FixedOffset datetime
    let created_after = DateTime::from_timestamp(1, 0)
        .map(|utc_dt| DateTime::from(utc_dt));

    // Get everything
    // TODO: how does it paginate ?
    match immich_client::apis::search_api::search_assets(
        &cfg,
        MetadataSearchDto {
            created_after,
            with_deleted: Some(false),
            ..MetadataSearchDto::new()
        }
    ).await {
        Ok(search_response) => {
            println!("Search Result: {:?}", search_response);
        }

        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
