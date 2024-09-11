use log::{error, info};
use rcgen::{generate_simple_self_signed, CertifiedKey};
use rustls::server::ServerConfig;
use warv::{middlewares::LoggingMiddleware, router::Router};

fn main() {
    let subject_alt_names = vec!["hello.world.example".to_string(), "localhost".to_string()];
    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();

    let tlsconfig = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            vec![cert.der().clone()],
            rustls::pki_types::PrivateKeyDer::try_from(key_pair.serialize_der()).unwrap(),
        )
        .unwrap();
    let addr = "127.0.0.1:3000";

    let mut router = Router::new();
    _ = router.add_stateless_route(warv::http::Method::GET, "/", index);

    router.add_middleware(LoggingMiddleware {});
    let mut server = warv::server::Server::new();
    server.add_router(router);

    match server.run_tls(addr, tlsconfig.into()) {
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
