use dotenv::dotenv;
use std::env;

pub struct AppConfig {
    pub gitlab_token: String,
    pub gitlab_base_url: String,
}

impl AppConfig {
    pub fn build() -> Self {
        dotenv().ok();

        Self {
            gitlab_token: env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN need be set"),
            gitlab_base_url: env::var("GITLAB_BASE_URL").expect("GITLAB_BASE_URL need be set"),
        }
    }
}
