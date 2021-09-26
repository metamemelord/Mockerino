use crate::request::Request;

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    #[serde(skip)]
    is_root: bool,
    api_version: String,
    kind: String,
    spec: Spec,
}

#[derive(serde::Deserialize, Debug)]
pub struct Spec {
    requests: Vec<Request>,
}

impl Endpoint {
    pub fn request(&self) -> &Vec<Request> {
        &self.spec.requests
    }

    pub fn kind(&self) -> String {
        self.kind.clone()
    }
}
