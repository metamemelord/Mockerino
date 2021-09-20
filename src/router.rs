use crate::request::RequestWithMetadata;
use anyhow::{anyhow, Result};
use hyper::header::HeaderMap;
use hyper::StatusCode;
use hyper::{Body, Request, Response};
use routerify::{Router, RouterBuilder, RouterService};
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use tokio_util::codec::{BytesCodec, FramedRead};

pub fn get_service(routes: Vec<RequestWithMetadata>) -> Result<RouterService<Body, anyhow::Error>> {
    let router: RouterBuilder<Body, anyhow::Error> =
        Router::builder().middleware(routerify::Middleware::pre(logging_middleware));
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

        let response_headers: HeaderMap = route
            .headers()
            .iter()
            .filter_map(|(k, v)| {
                if let (Ok(k), Ok(v)) = (
                    hyper::header::HeaderName::from_str(k),
                    hyper::header::HeaderValue::from_str(v),
                ) {
                    Option::from(Ok((k, v)))
                } else {
                    None
                }
            })
            .collect::<Result<HeaderMap>>()
            .unwrap();

        let status_code =
            StatusCode::from_u16(route.status_code().unwrap_or(200)).unwrap_or_default();

        if let Some(raw_b) = route.raw_body() {
            let handler = get_raw_body_handler(raw_b, response_headers, status_code, route.sleep());
            router = router.add(
                route.path(),
                vec![hyper::Method::from_str(route.method())?],
                handler,
            );
        } else if let Some(file_path) = route.file() {
            let handler = get_file_handler(file_path, response_headers, status_code, route.sleep());
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
    headers: HeaderMap,
    status_code: StatusCode,
    sleep: u64,
) -> impl Fn(
    Request<Body>,
) -> Pin<Box<dyn Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync>> {
    move |_: Request<Body>| -> Pin<Box<dyn Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync>> {
      let response_headers = headers.clone();
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
    headers: HeaderMap,
    status_code: StatusCode,
    sleep: u64,
) -> impl Fn(
    Request<Body>,
) -> Pin<Box<dyn Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync>> {
    move |_: Request<Body>| -> Pin<Box<dyn Future<Output = Result<Response<Body>, anyhow::Error>> + Send + Sync>> {
                let response_headers = headers.clone();
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

async fn logging_middleware(req: Request<Body>) -> Result<Request<Body>, anyhow::Error> {
    log::debug!("{} {}", req.method(), req.uri().path());
    Ok(req)
}
