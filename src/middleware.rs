use crate::http::Request;
use std::sync::Arc;
use crate::handler::Handler;
use crate::http::Response;
use crate::state::State;

pub trait Middleware: Send + Sync {
    fn handle(
        &self,
        req: Request,
        state: Option<Arc<dyn State + 'static>>,
        next: &dyn Handler,
    ) -> Response;
}
