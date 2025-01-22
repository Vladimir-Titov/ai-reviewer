use dotenv::dotenv;
use std::env;

pub struct AppConfig {
    pub gitlab_token: String,
}

impl AppConfig {
    pub fn build() -> Self {
        dotenv().ok();

        Self {
            gitlab_token: env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN need be set"),
        }
    }
}
