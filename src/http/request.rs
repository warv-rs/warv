use crate::http::Method;

use crate::http::Uri;
use crate::http::Version;
use std::collections::HashMap;

/// HTTP Request
#[derive(Debug, Clone)]
pub struct Request {
    parts: Parts,
    pub body: Vec<u8>,
}
impl Request {
    pub fn new(method: Method) -> Self {
        Request {
            parts: Parts {
                method: method,
                uri: Uri {
                    path: "".to_owned(),
                    query: None,
                    pattern_params: None,
                },
                version: Version::HTTP1_1,
                headers: HashMap::new(),
            },
            body: Vec::new(),
        }
    }
    /// Returns the Method of the request
    pub fn method(&self) -> &Method {
        &self.parts.method
    }

    /// Returns the HTTP Verions
    pub fn version(&self) -> &Version{
        return &self.parts.version;
    }

    /// Returns the headers
    pub fn headers(&self) -> &HashMap<String, String> {
        return &self.parts.headers;
    }
    /// Sets the request body
    pub fn body(&mut self, body: Vec<u8>) -> &Self {
        self.body = body;
        return self;
    }
    /// Adds headers to the requests
    pub fn insert_header(&mut self, k: &str, v: &str) -> &Self {
        self.parts.headers.insert(k.to_owned(), v.to_owned());
        return self;
    }
    /// Set the URI for the request.
    pub fn set_uri(&mut self, path: &str) -> &Self {
        let uri = Uri::new(path);
        self.parts.uri = uri;
        self
    }
    ///Returns the URI
    pub fn uri(&self) -> &Uri {
        &self.parts.uri
    }
    /// Collects route parameters
    pub fn add_params(&mut self, params: HashMap<String, String>) -> &Self {
        self.parts.uri.pattern_params = Some(params.clone());
        self
    }
}

#[derive(Clone, Debug)]
struct Parts {
    method: Method,
    uri: Uri,
    version: Version,
    headers: HashMap<String, String>,
}
