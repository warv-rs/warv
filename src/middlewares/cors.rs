use crate::handler::Handler;
use crate::http::{Method, Request};
use crate::middleware::Middleware;
use crate::state::State;

use crate::http::Response;

pub struct CorsMiddleware {
    allowed_origins: Vec<String>,
    allowed_methods: Vec<Method>,
    allowed_headers: Vec<String>,
    allow_credentials: bool,
}

impl CorsMiddleware {
    pub fn new(
        allowed_origins: Vec<String>,
        allowed_methods: Vec<Method>,
        allowed_headers: Vec<String>,
        allow_credentials: bool,
    ) -> Self {
        CorsMiddleware {
            allowed_origins,
            allowed_methods,
            allowed_headers,
            allow_credentials,
        }
    }

    fn add_cors_headers(&self, response: &mut Response, origin: &str) {
        if self.allowed_origins.contains(&origin.to_string())
            || self.allowed_origins.contains(&"*".to_string())
        {
            response.insert_header("Access-Control-Allow-Origin".to_string(), origin.to_owned());

            response.insert_header(
                "Access-Control-Allow-Methods".to_string(),
                self.allowed_methods
                    .iter()
                    .map(|m| m.as_str())
                    .collect::<Vec<&str>>()
                    .join(", ")
                    .to_owned(),
            );

            response.insert_header(
                "Access-Control-Allow-Headers".to_string(),
                self.allowed_headers.join(", ").to_owned(),
            );

            if self.allow_credentials {
                response.insert_header(
                    "Access-Control-Allow-Credentials".to_string(),
                    "true".to_string(),
                );
            }
        }
    }
}

impl Middleware for CorsMiddleware {
    fn handle(&self, req: Request, state:State, next: &dyn Handler) -> Response {
        if let Some(origin) = req.headers().get("Origin") {
            let origin = origin.as_str();

            if req.method().clone() == Method::OPTIONS {
                // Handle preflight request
                let mut response = Response::no_content();
                self.add_cors_headers(&mut response, origin);
                return response;
            } else {
                // Handle actual request
                let mut response = next.handle(req.clone(), state);
                self.add_cors_headers(&mut response, origin);
                return response;
            }
        }

        // If there's no Origin header, just proceed to the next middleware or handler
        let response = next.handle(req, state);
        response
    }
}
