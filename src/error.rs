use crate::http::Response;
use std::error;
use std::fmt;


#[derive(Debug)]
pub enum Error {
    BadRequest,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::BadRequest => write!(f, "Bad Request"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::BadRequest => None,
        }
    }
}

impl Error {
    pub fn http_response(&self) -> Response {
        let response = match *self {
            Error::BadRequest => Response::bad_request(),
        };
        return response;
    }
}
