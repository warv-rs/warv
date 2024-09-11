use crate::error::Error;
use crate::http::Method;
use crate::http::Request;
use crate::http::Response;
use crate::router::Router;
use log::error;
use log::info;

use may::net::TcpListener;
use rustls::server::ServerConfig;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::sync::Arc;

const BUFFER_SIZE: usize = 1024 * 30;

fn handle_client(mut stream: may::net::TcpStream, router: Vec<Router>) {
    let mut buffer = [0; BUFFER_SIZE];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => return, // Connection closed
            Ok(bytes_read) => match parse_request(&buffer[0..bytes_read]) {
                Ok(request) => {
                    let mut response = None;
                    for r in &router {
                        let resp = r.handle_request(request.clone());
                        if resp.is_some() {
                            response = resp.clone();
                            break;
                        }
                    }
                    let response = match response {
                        Some(r) => r,
                        None => Response::not_found(),
                    };
                    if let Err(e) = stream.write_all(&response.format()) {
                        error!("Failed to write to stream: {}", e);
                        return;
                    }
                }
                Err(e) => {
                    error!("Failed to parse request");
                    let response_bytes = e.http_response();
                    if let Err(e) = stream.write_all(&response_bytes.format()) {
                        error!("Failed to write to stream: {}", e);
                        return;
                    }
                }
            },
            Err(e) => {
                error!("Failed to read from stream: {}", e);
                return;
            }
        }
    }
}
fn handle_client_tls(
    mut stream: may::net::TcpStream,
    router: Vec<Router>,
    tls_config: Arc<ServerConfig>,
) {
    let mut buffer = vec![0; BUFFER_SIZE];

    let mut tls_conn =
        rustls::ServerConnection::new(tls_config).expect("Cannot create TLS connection");
    let mut tlsstream = rustls::Stream::new(&mut tls_conn, &mut stream);

    loop {
        match tlsstream.read(&mut buffer) {
            Ok(0) => return, // Connection closed
            Ok(bytes_read) => match parse_request(&buffer[0..bytes_read]) {
                Ok(request) => {
                    let mut response = None;
                    for r in &router {
                        let resp = r.handle_request(request.clone());
                        if resp.is_some() {
                            response = resp.clone();
                            break;
                        }
                    }
                    let response = match response {
                        Some(r) => r,
                        None => Response::not_found(),
                    };
                    if let Err(e) = tlsstream.write_all(&response.format()) {
                        error!("Failed to write to stream: {}", e);
                        return;
                    }
                }
                Err(e) => {
                    error!("Failed to parse request");
                    let response_bytes = e.http_response();
                    if let Err(e) = tlsstream.write_all(&response_bytes.format()) {
                        error!("Failed to write to stream: {}", e);
                        return;
                    }
                }
            },
            Err(e) => {
                error!("Failed to read from stream: {}", e);
                return;
            }
        }
    }
}

fn parse_request(buffer: &[u8]) -> Result<Request, Error> {
    let request_str = match from_utf8(buffer) {
        Ok(s) => s,
        Err(e) => {
            error!("{:?}", e);
            return Err(Error::BadRequest);
        }
    };
    let mut request_lines = request_str.split("\r\n");

    if request_lines.clone().count() == 0 {
        return Err(Error::BadRequest);
    }

    let request_parts = match request_lines.next() {
        Some(s) => s,
        None => return Err(Error::BadRequest),
    };
    let parts: Vec<&str> = request_parts.split_whitespace().collect();
    if parts.len() < 3 {
        return Err(Error::BadRequest);
    }

    let method = match parts[0].parse::<Method>() {
        Ok(m) => m,
        Err(e) => {
            error!("{:?}", e);
            return Err(Error::BadRequest);
        }
    };
    let uri = parts[1];

    let mut kv_headers = HashMap::new();
    for value in request_lines.clone() {
        request_lines.next();
        if value == "" {
            break;
        }
        let kv: Vec<_> = value.split(": ").collect();
        kv_headers.insert(kv[0], kv[1]);
    }

    let body = match request_lines.next() {
        Some(s) => s,
        None => return Err(Error::BadRequest),
    };

    let mut request = Request::new(method);
    request.set_uri(uri);
    for (k, v) in kv_headers {
        request.insert_header(k, v);
    }
    request.body(body.as_bytes().to_vec());
    Ok(request)
}
/// The server
pub struct Server {
    workers: usize,
    stack_size: usize,
    router: Vec<Router>,
}
impl Server {
    pub fn new() -> Self {
        Server {
            workers: 4,
            stack_size: 256 * 1024,
            router: Vec::new(),
        }
    }
    /// Add threads/workers
    /// Default 4 workers
    pub fn worker(&mut self, amount: usize) {
        self.workers = amount;
    }
    /// Define the stack size for the threads
    /// Default 256 kb
    pub fn stack(&mut self, size: usize) {
        self.stack_size = size;
    }
    ///Add a router to the server
    ///Additional routers can be added to the server.
    pub fn add_router(&mut self, router: Router) {
        self.router.push(router);
    }
    /// Start the server.
    pub fn run(&self, addr: &str) -> std::io::Result<()> {
        may::config().set_workers(self.workers);
        may::config().set_stack_size(self.stack_size);

        let listener = TcpListener::bind(addr)?;
        while let Ok((stream, saddr)) = listener.accept() {
            let router = self.router.clone();
            info!("Connection {:?}", saddr);
            go!(move || {
                handle_client(stream, router.clone());
            });
        }
        Ok(())
    }
    ///Start server with TLS configuration
    pub fn run_tls(&self, addr: &str, tls_config: Arc<ServerConfig>) -> std::io::Result<()> {
        may::config().set_workers(self.workers);
        may::config().set_stack_size(self.stack_size * 1024);

        let listener = TcpListener::bind(addr)?;

        while let Ok((stream, saddr)) = listener.accept() {
            let tls_config = tls_config.clone();
            let router = self.router.clone();
            info!("Connection {:?}", saddr);
            go!(move || {
                handle_client_tls(stream, router.clone(), tls_config.clone());
            });
        }
        Ok(())
    }
}
