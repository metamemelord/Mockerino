use crate::request::RequestWithMetadata;
use anyhow::{anyhow, Result};
use hyper::{Body, Request, Response};
use routerify::{Router, RouterBuilder, RouterService};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use tokio_util::codec::{BytesCodec, FramedRead};

pub fn get_service(routes: Vec<RequestWithMetadata>) -> Result<RouterService<Body, anyhow::Error>> {
    let router: RouterBuilder<Body, anyhow::Error> = Router::builder();
    let router = add_routes(router, routes)?
        .build()
        .map_err(|e| anyhow!("{}", e))?;
    RouterService::new(router).map_err(|e| anyhow!("{}", e))
}

fn add_routes(
    mut router: RouterBuilder<Body, anyhow::Error>,
    routes: Vec<RequestWithMetadata>,
) -> Result<RouterBuilder<Body, anyhow::Error>> {
    for route in routes.iter() {
        log::info!("{} {}", route.method(), route.path());
        if let Some(raw_b) = route.raw_body() {
            let handler =
                get_raw_body_handler(raw_b, route.headers(), route.status_code(), route.sleep());
            router = router.add(
                route.path(),
                vec![hyper::Method::from_str(route.method())?],
                handler,
            );
        } else if let Some(file_path) = route.file() {
            let handler = get_file_handler(
                file_path,
                route.headers(),
                route.status_code(),
                route.sleep(),
            );
            router = router.add(
                route.path(),
                vec![hyper::Method::from_str(route.method())?],
                handler,
            );
        }
    }
    Ok(router)
}

fn get_raw_body_handler(
    raw_body: String,
    headers: HashMap<String, String>,
    status_code: Option<u16>,
    sleep: u64,
) -> impl Fn(
    Request<Body>,
) -> Pin<Box<dyn Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync>> {
    let response_headers: hyper::header::HeaderMap = headers
        .iter()
        .filter_map(|(k, v)| {
            hyper::header::HeaderName::from_str(k).ok().and_then(|k| {
                hyper::header::HeaderValue::from_str(v)
                    .ok()
                    .and_then(|v| Some((k, v)))
            })
        })
        .collect();

    let status_code = hyper::StatusCode::from_u16(status_code.unwrap_or(200)).unwrap_or_default();

    move |_: Request<Body>| -> Pin<Box<dyn Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync>> {
      let response_headers = response_headers.clone();
      let raw_body = raw_body.clone();
      Box::pin(async move {
        if sleep > 0 {
          log::debug!("Sleeping for {}ms", sleep);
          tokio::time::sleep(std::time::Duration::from_millis(sleep)).await;
        }

        let mut response = Response::new(Body::from(raw_body));
        *response.status_mut() = status_code;
        *response.headers_mut() = response_headers;
        Ok(response)
      })
    }
}

fn get_file_handler(
    file_path: String,
    headers: HashMap<String, String>,
    status_code: Option<u16>,
    sleep: u64,
) -> impl Fn(
    Request<Body>,
) -> Pin<Box<dyn Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync>> {
    let response_headers: hyper::header::HeaderMap = headers
        .iter()
        .map(|(k, v)| {
            (
                hyper::header::HeaderName::from_str(k).ok(),
                hyper::header::HeaderValue::from_str(v).ok(),
            )
        })
        .filter(|(k, v)| (k.is_some() && v.is_some()))
        .map(|(k, v)| (k.unwrap(), v.unwrap()))
        .collect();

    let status_code = hyper::StatusCode::from_u16(status_code.unwrap_or(200)).unwrap_or_default();

    move |_: Request<Body>| -> Pin<Box<dyn Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync>> {
                let response_headers = response_headers.clone();
                let file_path = file_path.clone();
                Box::pin(async move {
                    if let Ok(file) = tokio::fs::File::open(file_path.clone()).await {
                      if sleep > 0 {
                        log::info!("Sleeping for {}ms", sleep);
                        tokio::time::sleep(std::time::Duration::from_millis(sleep)).await;
                      }

                      let stream = FramedRead::new(file, BytesCodec::new());
                      let body = Body::wrap_stream(stream);
                      let mut response = Response::new(body);
                      *response.headers_mut() = response_headers;
                      *response.status_mut() = status_code;
                      Ok(response)
                    } else {
                      log::error!("File not found \"{}\"", file_path);
                      panic!();
                    }
                })
            }
}
