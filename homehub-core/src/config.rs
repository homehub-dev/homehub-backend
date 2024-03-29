#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub access_token_private_key: String,
    pub access_token_public_key: String,
    pub refresh_token_private_key: String,
    pub refresh_token_public_key: String,
    pub access_token_max_age: i64,
    pub refresh_token_max_age: i64,
}

fn get_env_var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{} not set", key))
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: get_env_var("DATABASE_URL"),
            access_token_private_key: get_env_var("ACCESS_TOKEN_PRIVATE_KEY"),
            access_token_public_key: get_env_var("ACCESS_TOKEN_PUBLIC_KEY"),
            refresh_token_private_key: get_env_var("REFRESH_TOKEN_PRIVATE_KEY"),
            refresh_token_public_key: get_env_var("REFRESH_TOKEN_PUBLIC_KEY"),
            access_token_max_age: get_env_var("ACCESS_TOKEN_MAX_AGE")
                .parse()
                .expect("ACCESS_TOKEN_MAX_AGE must be an integer"),
            refresh_token_max_age: get_env_var("REFRESH_TOKEN_MAX_AGE")
                .parse()
                .expect("REFRESH_TOKEN_MAX_AGE must be an integer"),
        }
    }
}
