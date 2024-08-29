use crate::state::State;
use crate::http::Request;
use crate::http::Response;


pub trait Handler: Send + Sync {
    fn handle(
        &self,
        req: Request,
        state: State,
    ) -> Response;
}

/// Enum for Handler types
/// Each of the added route functions need to be either of the Enums
/// The function or "Handler" gets wrapped in the enum to determine if should be able to access a shared state.
pub enum HandlerType {
    Stateless(
        Box<
            dyn Fn(
                    Request,

                ) -> Response
                + Send
                + Sync,
        >,
    ),
    Stateful(
        Box<
            dyn Fn(
                    Request,
                    State,
                ) -> Response
                + Send
                + Sync,
        >,
    ),
}

impl Handler for HandlerType {
    fn handle(
        &self,
        req: Request,
        state: State,
    ) -> Response {
        match self {
            HandlerType::Stateless(handler) => handler(req ),
            HandlerType::Stateful(handler) => handler(req, state),
        }
    }
}
