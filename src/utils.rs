use crate::request::RequestWithMetadata;
use hyper::{Body, Request, Response};
use tokio_util::codec::{BytesCodec, FramedRead};
use anyhow::anyhow;
use tokio::fs::File;

pub async fn file_as_body(filename: &str) -> Result<Body, anyhow::Error> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(body);
    }
    Err(anyhow!("Failed"))

}
