mod client;
mod settings;

use client::gitlab::GitlabHttpClient;
use settings::app::AppConfig;

fn main() {
    run()
}

fn run() {
    let cfg: AppConfig = AppConfig::build();
    let gitlab_client: GitlabHttpClient =
        GitlabHttpClient::new(&cfg.gitlab_token, &cfg.gitlab_base_url);
    println!("{} {}", cfg.gitlab_token, cfg.gitlab_base_url)
}
