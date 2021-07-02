pub struct Config<V> {
    port: u16,
    max_threads: u8,
    routes: std::collections::BTreeMap<String, V>,
}

impl<V> Config<V> {
    pub fn default() -> Result<Self, std::io::Error> {
        let config_file_path = std::env::current_dir()?.join("config.yaml");
        let config_file = std::fs::File::open(config_file_path)?;
        Self::load_config(config_file)
    }

    fn load_config(path: std::fs::File) -> Result<Self, std::io::Error> {
        Ok(Self {
            port: 3000,
            max_threads: 1,
            routes: std::collections::BTreeMap::new(),
        })
    }

    pub fn from_path<T: AsRef<std::path::Path>>(path: T) -> Result<Self, std::io::Error> {
        let config_file = std::fs::File::open(path)?;
        Self::load_config(config_file)
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use std::io::Write;

    #[test]
    #[should_panic]
    fn test_default_without_file() {
        let _: Config<String> = Config::default().unwrap();
    }

    #[test]
    fn test_default() {
        let file_data = r#"version: Test"#;
        let _ = set_file(
            std::env::current_dir().unwrap().join("config.yaml"),
            file_data.as_bytes().iter().map(|&x| x).collect(),
        );

        let config: Config<String> = Config::default().unwrap();
        assert_eq!(config.routes.len(), 0);

        let _ = clean_file(std::env::current_dir().unwrap().join("config.yaml"));
    }

    #[test]
    fn test_load_config() {}

    fn set_file<T: AsRef<std::path::Path>>(s: T, data: Vec<u8>) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::create(s)?;
        file.write(&data)?;
        Ok(())
    }

    fn clean_file<T: AsRef<std::path::Path>>(s: T) -> Result<(), std::io::Error> {
        let _ = std::fs::remove_file(s)?;
        Ok(())
    }
}
