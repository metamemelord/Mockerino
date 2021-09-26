use crate::endpoint::Endpoint;
use crate::request::RequestWithMetadata;
use anyhow::Result;
use regex::Regex;
use std::rc::Rc;
use walkdir::WalkDir;

pub fn parse(base: &str) -> Result<Vec<RequestWithMetadata>> {
    let base = base.trim_end_matches('/');
    log::info!("Looking for spec files in base dir '{}'", base);

    let base_dir = WalkDir::new(base);
    let path_param_regex: Rc<Regex> = Rc::new(Regex::new(r"_(\w|\d)+/").unwrap());

    let res = base_dir
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.metadata().unwrap().is_file()
                && (e.path().extension() == Some(std::ffi::OsStr::new("yaml"))
                    || e.path().extension() == Some(std::ffi::OsStr::new("yaml")))
        })
        .map(|e| {
            log::debug!("Found file '{}'", e.path().display());
            process_file(base, e.path().to_str().unwrap(), path_param_regex.clone())
        })
        .filter_map(|e| e.ok())
        .flatten()
        .collect();
    Ok(res)
}

fn process_file(
    base: &str,
    path: &str,
    path_param_regex: Rc<Regex>,
) -> Result<Vec<RequestWithMetadata>> {
    let os_root_file_path = format!("{}root", std::path::MAIN_SEPARATOR);
    let mut request_path = path
        .trim_start_matches(base)
        .trim_end_matches(".yaml")
        .trim_end_matches(".yml")
        .trim_end_matches(&os_root_file_path)
        .trim_end_matches('/')
        .to_owned();
    request_path.push('/');

    let endpoint = parse_yaml_file(path)?;

    match endpoint.kind().as_ref() {
        "Endpoint" => {
            // Handle dynamic routes here
            if path_param_regex.find(request_path.as_ref()).is_some() {
                // TODO: Add capability to use dynamic routes.
                process_dynamic_path(endpoint, request_path.as_ref())
            } else {
                Ok(endpoint
                    .request()
                    .iter()
                    .map(|req| RequestWithMetadata::new(req.clone(), request_path.to_string()))
                    .collect())
            }
        }
        _ => Err(anyhow::Error::msg("Unsupported 'kind' in spec file")),
    }
}

fn process_dynamic_path(
    endpoint: Endpoint,
    request_path: &str,
) -> Result<Vec<RequestWithMetadata>> {
    Ok(endpoint
        .request()
        .iter()
        .map(|req| RequestWithMetadata::new(req.clone(), request_path.to_string()))
        .collect())
}

fn parse_yaml_file(path: &str) -> Result<Endpoint> {
    let file = std::fs::File::open(path)?;
    serde_yaml::from_reader(file).map_err(|e| {
        log::warn!("Error reading file '{}': {}", path, e);
        e.into()
    })
}
