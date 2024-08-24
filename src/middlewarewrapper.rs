use crate::http::Request;
use std::sync::Arc;
use crate::handler::Handler;
use crate::http::Response;
use crate::state::State;
use crate::middleware::Middleware;

pub struct MiddlewareWrapper {
    pub middleware: Arc<dyn Middleware>,
    pub next: Arc<dyn Handler>,
}

impl Handler for MiddlewareWrapper {
    fn handle(
        &self,
        req: Request,
        state: Option<Arc<dyn State + 'static>>,
    ) -> Response {
        self.middleware.handle(req, state, &*self.next)
    }
}
