mod settings;
use settings::app::AppConfig;
fn main() {
    let cfg: AppConfig = AppConfig::build();
    println!("{}", cfg.gitlab_token)
}
