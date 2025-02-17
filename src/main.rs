mod app;
mod config;

use config::Config;

use hyprland::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::get().await;

    app::App::run(config).await
}
