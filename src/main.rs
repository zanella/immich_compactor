use anyhow::Context;
use sqlx::PgPool;
use crate::immich_api_wrappers::{search_wrappers, server_wrappers};
use crate::database_repository::{test_db_connection, db_connect, db_create, nuke_database, StagingAreaEntry, User};

mod immich_api_wrappers;
mod database_repository;
mod config;

use immich_client::apis::configuration::{ApiKey, Configuration as ImmichApiConfiguration};
use immich_client::models::SearchResponseDto;
use crate::config::OwnConfig;

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
async fn main() -> Result<(), anyhow::Error> {

    let own_config = OwnConfig::default();

    let pg_pool = db_connect(&own_config).await.context("DB error")?;

    if let Ok(()) = test_db_connection(&pg_pool).await {
        // TODO: remove
        nuke_database(&pg_pool).await?;

        db_create(&pg_pool).await?;
    }

    /////// BEGIN_FAKE_USER_SETUP /////

    let user =
        User::create("rzanella", "wrmIFc2FMOwNTUyyUdyBOBgwnvfNRsuY8xyDiy0E", &pg_pool).await?;

    let immich_api_cfg = ImmichApiConfiguration {
        base_path: format!("{}/api", own_config.immich_url),
        api_key: Some(
            ApiKey {
                prefix: None,
                key: user.api_key.clone(),
            }
        ),
        ..ImmichApiConfiguration::default()
    };

    if let Some(x) = server_wrappers::check_api_accessible(&immich_api_cfg).await {
        panic!("Server returned: {:#}", x);
    }

    /////// END_FAKE_USER_SETUP /////

    blah(&immich_api_cfg, &user, &pg_pool).await.expect("wat ?");

    Ok(())
}

async fn blah(
    api_cfg: &ImmichApiConfiguration,
    user: &User,
    pg_pool: &PgPool
) -> Result<SearchResponseDto, anyhow::Error> {
    let search_response = search_wrappers::get_all_assets(api_cfg)
        .await
        .context("Failed to retrieve assets")?;

    // TODO: Filter out entries already in the Table :: already_converted

    // TODO: Process entries already staged

    for asset_response in &search_response.assets.items {
        StagingAreaEntry {
            user_id: user.id,
            id: asset_response.id.clone(),
            checksum: asset_response.checksum.clone(),
        }.persist(pg_pool).await?;
    }

    for asset_response in &search_response.assets.items {
        let download_response = immich_client::apis::assets_api::download_asset(
            api_cfg,
            asset_response.id.as_str(),
            None,
            None,
            None,
        ).await.context("Failed to download asset")?;

        println!("{:?}", download_response);

        let content_type = download_response.headers().get("content-type")
            .expect("Unable to get file content type header")
            .to_str()?;

        let file_extension = mime2ext::mime2ext(content_type)
            .expect("Unable to parse mime type");

        let tmp_file_name = format!("{}.{}", asset_response.id, file_extension);

        std::fs::write(tmp_file_name, download_response.bytes().await?)?

        // TODO: pass file through converter

        // TODO: upload
        
        // TODO: Transaction(move metadata to new file, delete old, remove StageEntry, Add already_converted)
    }

    Ok(search_response)
}
