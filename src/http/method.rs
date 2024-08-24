use std::str::FromStr;

///HTTP Methods
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    OPTIONS,
    DELETE,
    TRACE,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(input: &str) -> Result<Method, Self::Err> {
        match input {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "DELETE" => Ok(Method::DELETE),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(()),
        }
    }
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::OPTIONS => "OPTIONS",
            Method::DELETE => "DELETE",
            Method::TRACE => "TRACE",
        }
    }
}
