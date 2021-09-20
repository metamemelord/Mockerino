use anyhow::Result;

#[derive(serde::Deserialize)]
pub struct Config {
    port: u16,
    max_threads: u8,
    log_level: String,
    base_dir: String,
}

impl Config {
    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn max_threads(&self) -> u8 {
        self.max_threads
    }

    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    pub fn base_dir(&self) -> &str {
        &self.base_dir
    }

    pub fn default() -> Result<Self> {
        println!("[WARN] No config provided, looking for config in './config.yaml'");
        let config_file_path = std::env::current_dir()?.join("config.yaml");
        let config_file = std::fs::File::open(config_file_path)?;
        Self::load_config(config_file)
    }

    fn load_config(file: std::fs::File) -> Result<Self> {
        serde_yaml::from_reader(file).map_err(|e| e.into())
    }

    pub fn from_path<T: AsRef<std::path::Path>>(path: T) -> Result<Self> {
        let config_file = std::fs::File::open(path)?;
        Self::load_config(config_file)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use anyhow::Result;
    use std::io::Write;

    #[test]
    #[should_panic]
    fn test_default_without_file() {
        let _: Config = Config::default().unwrap();
    }

    #[test]
    fn test_default() {
        let file_data = r#"port: 8080
max_threads: 2
log_level: DEBUG
base_dir: files"#;
        let _ = set_file(
            std::env::current_dir().unwrap().join("config.yaml"),
            file_data.as_bytes().iter().map(|&x| x).collect(),
        );

        let _config: Config = Config::default().unwrap();

        let _ = clean_file(std::env::current_dir().unwrap().join("config.yaml"));
    }

    #[test]
    fn test_load_config() {}

    fn set_file<T: AsRef<std::path::Path>>(s: T, data: Vec<u8>) -> Result<()> {
        let mut file = std::fs::File::create(s)?;
        file.write(&data)?;
        Ok(())
    }

    fn clean_file<T: AsRef<std::path::Path>>(s: T) -> Result<()> {
        let _ = std::fs::remove_file(s)?;
        Ok(())
    }
}
