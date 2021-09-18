use std::collections::HashMap;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Request {
    description: String,
    method: String,
    headers: Option<HashMap<String, String>>,
    raw_body: Option<String>,
    status_code: Option<u16>,
    file: Option<String>,
    sleep: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RequestWithMetadata {
    inner: Request,
    full_path: String,
}

impl RequestWithMetadata {
    pub fn new(request: Request, full_path: String) -> Self {
        Self {
            inner: request,
            full_path,
        }
    }

    pub fn method(&self) -> &str {
        &self.inner.method
    }

    pub fn path(&self) -> &str {
        &self.full_path
    }

    pub fn status_code(&self) -> Option<u16> {
        self.inner.status_code
    }

    pub fn headers(&self) -> HashMap<String, String> {
        self.inner.headers.clone().unwrap_or_default()
    }

    pub fn raw_body(&self) -> Option<String> {
        self.inner.raw_body.clone()
    }

    pub fn file(&self) -> Option<String> {
        self.inner.file.clone()
    }

    pub fn sleep(&self) -> u64 {
        self.inner.sleep.unwrap_or_default()
    }
}

lazy_static::lazy_static! {}
