use crate::http::StatusCode;
use crate::http::Version;
use chrono::prelude::*;
use std::collections::HashMap;

///HTTP Response
#[derive(Debug, Clone)]
pub struct Response {
    parts: Parts,
    body: Vec<u8>,
}
impl Response {
    /// Creates a response struct with set status code.
    pub fn new(status: StatusCode) -> Self{
        Response {
            parts: Parts {
                status: status,
                version: Version::HTTP1_1,
                headers: HashMap::new(),
            },
            body: Vec::new(),
        }
    }

    ///Pre-confgured OK HTTP Response
    pub fn ok() -> Self {
        Response {
            parts: Parts {
                status: StatusCode::OK,
                version: Version::HTTP1_1,
                headers: HashMap::new(),
            },
            body: Vec::new(),
        }
    }
    ///Pre-confgured Bad Request HTTP Response
    pub fn bad_request() -> Self {
        Response {
            parts: Parts {
                status: StatusCode::BadRequest,
                version: Version::HTTP1_1,
                headers: HashMap::new(),
            },
            body: Vec::new(),
        }
    }
    ///Pre-confgured No Content HTTP Response
    pub fn no_content() -> Self {
        Response {
            parts: Parts {
                status: StatusCode::NoContent,
                version: Version::HTTP1_1,
                headers: HashMap::new(),
            },
            body: Vec::new(),
        }
    }
    ///Pre-confgured Not Found HTTP Response
    pub fn not_found() -> Self {
        Response {
            parts: Parts {
                status: StatusCode::NotFound,
                version: Version::HTTP1_1,
                headers: HashMap::new(),
            },
            body: b"Not Found".to_vec(),
        }
    }
    ///Pre-confgured Internal Server Error HTTP Response
    pub fn internal_server_error() -> Self {
        Response {
            parts: Parts {
                status: StatusCode::InternalServerError,
                version: Version::HTTP1_1,
                headers: HashMap::new(),
            },
            body: Vec::new(),
        }
    }
    /// Returns the status
    pub fn status(&self) -> &StatusCode {
        return &self.parts.status;
    }
    // Sets the Body
    pub fn body(&mut self, body: Vec<u8>) -> &Self {
        self.body = body;
        return self;
    }
    /// Returns HTTP Version
    pub fn version(&self) -> &Version {
        &self.parts.version
    }
    /// Add headers
    pub fn insert_header(&mut self, k: String, v: String) -> &Self {
        self.parts.headers.insert(k, v);
        return self;
    }
    /// Formats the response to be sent
    pub fn format(&self) -> Vec<u8> {
        let dt = Utc::now();
        let mut response_str = format!(
            "{} {} {}\r\nContent-Length: {}\r\nServer: warv\r\nDate: {} \r\n",
            self.version().as_str(),
            self.status().as_u16(),
            self.status().reason(),
            self.body.len(),
            dt.to_rfc2822(),
        );
        for (k, v) in self.parts.headers.iter() {
            response_str = format!("{}{}: {}\r\n", response_str, k, v);
        }
        response_str = format!("{}\r\n", response_str);
        let mut response_bytes = response_str.into_bytes();
        response_bytes.extend_from_slice(&self.body);
        response_bytes
    }
}

#[derive(Debug, Clone)]
struct Parts {
    status: StatusCode,
    version: Version,
    headers: HashMap<String, String>,
    // pub extensions: HashMap<String, String>,
}
