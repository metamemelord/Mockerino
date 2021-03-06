use anyhow::Result;
use std::io::Write;

static BOILERPLATE_SOURCE_BASE: &str =
    "https://raw.githubusercontent.com/metamemelord/Mockerino/main/boilerplate";

pub fn init() -> Result<()> {
    log::info!("Initializing the spec and config in current directory");
    log::info!("Downloading sample spec. The spec can be found at https://gaurav.app/r/github/Mockerino/tree/main/boilerplate");

    log::debug!("Creating directories");
    std::fs::create_dir_all("spec/hello")?;
    std::fs::create_dir_all("data")?;

    let files = vec![
        "config.yaml",
        "spec/root.yaml",
        "spec/hello/root.yaml",
        "spec/hello/world.yaml",
        "data/hello-world.json",
    ];
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        for file in files {
            if let Err(e) = download_file(file).await {
                log::error!("Error downloading spec file '{}': {}", file, e);
            }
        }
    });
    log::info!("Done! Run mockerino to start the engine.");
    Ok(())
}

async fn download_file(file_path: &str) -> Result<()> {
    let download_link = format!("{}/{}", BOILERPLATE_SOURCE_BASE, file_path);
    log::debug!("Downloading file '{}'", file_path);
    let response = reqwest::get(download_link).await?;
    let path = std::path::Path::new(file_path);
    let mut file = std::fs::File::create(&path)?;
    let content = response.text().await?;
    file.write_all(content.as_bytes())?;
    log::debug!("Downloaded boilerplate code at '{}'", file_path);
    Ok(())
}
