
pub struct OwnConfig {
    pub db_user_name: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_own_db_name: String,

    pub immich_url: String,
}

impl OwnConfig {
    pub fn default() -> Self {
        Self {
            db_user_name: "postgres".to_string(),
            db_password: "q1w2e3".to_string(),
            db_host: "localhost".to_string(),
            db_port: 5445,
            db_own_db_name: "immich_space_saver".to_string(),

            immich_url: "http://localhost:2283".to_string(),
        }
    }
}
