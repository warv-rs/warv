use log::{error, info};
use warv::{middlewares::LoggingMiddleware, router::Router};

fn main() {
    let addr = "127.0.0.1:3000";

    let mut router = Router::new();
    _ = router.add_stateless_route(warv::http::Method::GET, "/", index);

    router.add_middleware(LoggingMiddleware {});
    let mut server = warv::server::Server::new();
    server.add_router(router);

    match server.run(addr) {
        Ok(_) => info!("Clean Exit"),
        Err(e) => error!("{}", e),
    }
}

fn index(_req: warv::http::Request) -> warv::http::Response {
    let mut resp = warv::http::Response::ok();
    resp.insert_header("Content-Type".to_string(), "text/html".to_string());
    resp.body(format!("Hello World").into());
    resp
}
