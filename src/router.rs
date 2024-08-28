use crate::http::Method;
use crate::http::Request;
use crate::middleware::Middleware;

use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::handler::Handler;
use crate::http::Response;
use crate::middlewarewrapper::MiddlewareWrapper;
use crate::state::State;
use crate::handler::HandlerType;
use std::any::Any;


/// Router struct
#[derive(Clone)]
pub struct Router {
    routes: HashMap<Method, Vec<(Regex, Arc<dyn Handler>)>>,
    middlewares: Vec<Arc<dyn Middleware>>,
    state: State,
    static_dir: Option<String>,
    default_handler: Option<Arc<dyn Handler>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
            middlewares: Vec::new(),
            state: State::new(),
            static_dir: None,
            default_handler: None,
        }
    }
    /// Add a stateless route / handler function.
    pub fn add_stateless_route<F>(&mut self, method: Method, path: &str, handler: F) -> Result<(),Box<dyn Error>> 
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        let regex = Regex::new(&format!(
            "^{}$",
            path.replace("{", "(?P<").replace("}", ">[^/]+)")
        ))?;

        let handler_type = HandlerType::Stateless(Box::new(handler));
        self.routes
        .entry(method)
        .or_insert_with(Vec::new)
        .push((regex, Arc::new(handler_type)));
        Ok(())
    }
    /// Add a stateful route / handler function.
    pub fn add_stateful_route<F>(&mut self, method: Method, path: &str, handler: F) -> Result<(),Box<dyn Error>> 
    where
        F: Fn(Request, State) -> Response + Send + Sync + 'static,
    {
        let regex = Regex::new(&format!(
            "^{}$",
            path.replace("{", "(?P<").replace("}", ">[^/]+)")
        ))?;

        let handler_type = HandlerType::Stateful(Box::new(handler));
        self.routes
        .entry(method)
        .or_insert_with(Vec::new)
        .push((regex, Arc::new(handler_type)));
        Ok(())
    }
    /// Add a route / handler function 
    /// Requires the use of HandlerType Enums
      pub fn add_route(&mut self, method: Method, path: &str, handler: impl Handler + 'static) -> Result<(),Box<dyn Error>> {
        let regex = Regex::new(&format!(
            "^{}$",
            path.replace("{", "(?P<").replace("}", ">[^/]+)")
        ))?;
        self.routes
            .entry(method)
            .or_insert_with(Vec::new)
            .push((regex, Arc::new(handler)));
            Ok(())
    }
    /// Add a middleware
    /// Multiple middlewares can be added.
    pub fn add_middleware<M>(&mut self, middleware: M)
    where
        M: Middleware + 'static,
    {
        self.middlewares.push(Arc::new(middleware));
    }
    /// Add a state to the router and the defined routes
    //TODO 
    pub fn set_state<T: 'static + Send + Sync>(&mut self, state:T )
    {
        self.state.set(state);
    }
    /// Serve a static directory
    pub fn set_static_dir(&mut self, dir: String) {
        self.static_dir = Some(dir);
    }
    /// Set a default route for all requests missing a route
    pub fn set_default_handler<F>(&mut self, handler: impl Handler + 'static)
    where
        F: Fn(Request, State) -> Response + Send + Sync + 'static,
    {
        self.default_handler = Some(Arc::new(handler));
    }

    /// The main router component managing the requests received
    pub fn handle_request(&self, mut req: Request) -> Option<Response> {
        // Check for route match
        if let Some(handlers) = self.routes.get(&req.method()) {
            for (pattern, handler) in handlers {
                if let Some(captures) = pattern.captures(req.uri().path()) {
                    let mut params = HashMap::new();
                    for name in pattern.capture_names().flatten() {
                        if let Some(value) = captures.name(name) {
                            params.insert(name.to_string(), value.as_str().to_string());
                        }
                    }
                    req.add_params(params);

                    // Apply middlewares in sequence
                    let mut final_handler: Arc<dyn Handler> = handler.clone();
                    for middleware in self.middlewares.iter().rev() {
                        final_handler = Arc::new(MiddlewareWrapper {
                            middleware: Arc::clone(middleware),
                            next: final_handler.clone(),
                        });
                    }

                    return Some(final_handler.handle(req, /*params,*/ self.state.clone()));
                }
            }
        }

        // Handle static file serving TODO make sure directory traversal doesn't work
        if let Some(dir) = &self.static_dir {
            let path = Path::new(dir).join(req.uri().path().trim_start_matches('/'));
            if path.is_file() {
                match fs::read(&path) {
                    Ok(file_content) => {
                        let mut response = Response::ok();
                        response.body(file_content);
                        return Some(response);
                    }
                    Err(_) => {
                        return Some(Response::internal_server_error());
                    }
                }
            }
        }

        // Call the default handler if set TODO add method check?
        if let Some(handler) = &self.default_handler {
            return Some(handler.handle(req , self.state.clone()));
        }
        None
        //Response::not_found()
    }
}
