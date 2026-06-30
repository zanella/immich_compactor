use derive_more::Constructor;
use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgPoolOptions;
use crate::config::OwnConfig;

pub async fn db_connect(own_config: &OwnConfig) -> Result<PgPool, sqlx::Error> {
    let database_url = format!(
        "postgres://{0}:{1}@{2}:{3}/{4}",
        own_config.db_user_name,
        own_config.db_password,
        own_config.db_host,
        own_config.db_port,
        own_config.db_own_db_name
    );

    // 2. Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("✅ Successfully connected to PostgreSQL!");

    sqlx::query("SELECT 1 + 1").fetch_one(&pool).await?;

    Ok(pool)
}

pub async fn test_db_connection(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut conn = pool.acquire().await?;

    // Executes a minimal dummy query on the database
    sqlx::query("SELECT 1").execute(&mut *conn).await?;

    println!("Database connection is healthy!");

    Ok(())
}

pub async fn db_create(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("db/migrations").run(pool).await?;

    println!("Database migrations applied successfully!");
    Ok(())
}

pub async fn nuke_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 1. Drop the public schema and everything inside it
    sqlx::query("DROP SCHEMA public CASCADE;")
        .execute(pool)
        .await?;

    // 2. Recreate the empty public schema so the database is usable again
    sqlx::query("CREATE SCHEMA public;")
        .execute(pool)
        .await?;

    // 3. Grant default permissions back to the public schema
    sqlx::query("GRANT ALL ON SCHEMA public TO public;")
        .execute(pool)
        .await?;

    println!("Database successfully nuked!");
    Ok(())
}

///////////////////////////

pub struct User {
    pub id: i64,
    pub name: String,
    pub api_key: String
}

impl User {
    pub async fn create(
        name: impl AsRef<str>,
        api_key: impl AsRef<str>,
        pool: &PgPool
    ) -> Result<Self, sqlx::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, api_key)
            VALUES ($1, $2)
            RETURNING id, name, api_key
            "#,
            name.as_ref(),
            api_key.as_ref()
        )
            .fetch_one(pool)
            .await?;

        Ok(row)
    }
}
///////////////////////////

#[derive(Constructor, Debug, FromRow)]
pub struct StagingAreaEntry {
    pub user_id: i64,
    pub id: String,
    pub checksum: String
}

impl StagingAreaEntry {
    pub async fn persist(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO staging_area (user_id, id, checksum)
            VALUES ($1, $2, $3)
            "#,
            self.user_id,
            self.id,
            self.checksum
        )
            .execute(pool)
            .await?;

        Ok(())
    }
}
