use anyhow::Result;
use log;
use reqwest;
use std::io::Write;
use tokio;

static BOILERPLATE_SOURCE: &str =
    "https://codeload.github.com/metamemelord/Crawly/legacy.zip/master";

pub fn init() -> Result<()> {
    log::info!("Initializing the spec and config in current directory");
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(download_boilerplate())?;

    log::info!("Writing config file at './config.yaml'");
    log::info!("Creating spec directory at './spec'");
    log::info!("Creting data directoru './config.yaml'");
    Ok(())
}

async fn download_boilerplate() -> Result<()> {
    log::info!("Downloading sample spec from https://gaurav.app/r/github/mockerino-boilerplate");
    let local_path = "/tmp/mockerino-boilerplate.zip";
    let response = reqwest::get(BOILERPLATE_SOURCE).await?;
    let path = std::path::Path::new(local_path);
    let mut file = std::fs::File::create(&path)?;
    let content = response.text().await?;
    file.write_all(content.as_bytes())?;
    log::info!("Downloaded boilerplate code at '{}'", local_path);
    Ok(())
}
