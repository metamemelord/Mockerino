extern crate anyhow;
extern crate clap;
extern crate hyper;
extern crate lazy_static;
extern crate log;
extern crate notify;
extern crate regex;
extern crate serde;
extern crate serde_yaml;
extern crate simplelog;
extern crate tokio;
extern crate tokio_util;
extern crate walkdir;

mod config;
mod endpoint;
mod request;
mod router;
mod spec_parser;

use anyhow::Result;
use clap::{clap_app, crate_authors, crate_version};
use hyper::server::Server;
use simplelog::{ColorChoice, Config as SConfig, LevelFilter, TermLogger, TerminalMode};
use std::net::SocketAddr;
use std::str::FromStr;

pub fn init_app<'a, 'b>() -> Result<clap::App<'a, 'b>> {
    Ok(clap_app!(
      Mockerino =>
      (version: crate_version!())
      (about: "A YAML based REST API mocking engine.")
      (author: crate_authors!())
      (@arg config: -c --config +takes_value "Path to config file. Default is ./config.yaml")
    ))
}

fn main() -> Result<()> {
    let app = init_app()?;
    let app_name = app.get_name().to_owned();
    let app_matches = app.get_matches();

    let cfg = {
        match app_matches.value_of("config") {
            Some(ref v) => config::Config::from_path(v)?,
            None => config::Config::default()?,
        }
    };

    TermLogger::init(
        LevelFilter::from_str(cfg.log_level())?,
        SConfig::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    log::info!("{} v{}", app_name, clap::crate_version!());

    let routes = spec_parser::parse(cfg.base_dir())?;

    let service = router::get_service(routes)?;

    let rt = tokio::runtime::Builder::new_multi_thread()
        .thread_name("Mockerino thread")
        .worker_threads(cfg.max_threads().into())
        .enable_all()
        .on_thread_start(|| {
            log::debug!("Starting a new thread");
        })
        .on_thread_stop(|| {
            log::debug!("Stopped a thread");
        })
        .build()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port()));
    let _guard = rt.enter();

    log::info!("Starting on port {}", cfg.port());

    rt.block_on(Server::bind(&addr).serve(service))
        .map_err(|e| e.into())
}
