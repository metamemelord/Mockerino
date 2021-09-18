use crate::config::Config;
use anyhow::Result;
use simplelog::{ColorChoice, Config as SConfig, LevelFilter, TermLogger, TerminalMode};
use std::str::FromStr;

pub fn init(cfg: Option<&Config>) -> Result<()> {
    let log_level = {
        match cfg {
            Some(c) => c.log_level(),
            None => "INFO",
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
