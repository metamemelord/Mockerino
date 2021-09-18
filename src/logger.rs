use simplelog::{ColorChoice, Config as SConfig, LevelFilter, TermLogger, TerminalMode};
use anyhow::Result;
use crate::config::Config;
use std::str::FromStr;

pub fn init(cfg: Option<&Config>) -> Result<()> {
    let log_level = {
        match cfg {
            Some(ref c) => c.log_level(),
            None => "INFO"
        }
    };

    TermLogger::init(
        LevelFilter::from_str(log_level)?,
        SConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    Ok(())
}