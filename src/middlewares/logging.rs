use crate::http::Request;
use crate::middleware::Middleware;
use log::info;
use crate::handler::Handler;
use crate::http::Response;
use crate::state::State;


pub struct LoggingMiddleware;

impl LoggingMiddleware {
    pub fn new() -> Self {
        LoggingMiddleware
    }
}

impl Middleware for LoggingMiddleware {
    fn handle(
        &self,
        req: Request,
        //  path_params: HashMap<String, String>,
        //  query_params: Option<HashMap<String, String>>,
        state: State,
        next: &dyn Handler,
    ) -> Response {
        // Log request details
        info!("Received request: {:?}", req);

        // Call the next handler (could be stateful or stateless)
        let response = next.handle(req, state);

        // Log response details
        info!("Response: {:?}", response);

        // Return the response
        response
    }
}
