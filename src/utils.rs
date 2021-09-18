use crate::request::RequestWithMetadata;
use anyhow::anyhow;
use hyper::{Body, Request, Response};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn file_as_body(filename: &str) -> Result<Body, anyhow::Error> {
    // Serve a file by asynchronously reading it by chunks using tokio-util crate.

    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(body);
    }
    Err(anyhow!("Failed"))
}
