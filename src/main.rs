mod config;

use config::parser::{load_config, GatewayConfig, NoAuthEndpoints, ServiceConfig};
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::http::request::Parts;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::client::legacy::Client;
use hyper_util::rt::{TokioExecutor, TokioIo};
use reqwest::header::{HeaderMap, COOKIE};
use std::result::Result;
use std::sync::Arc;
use tokio::net::TcpListener;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    let config = Arc::new(load_config("config.yaml"));

    let listener = TcpListener::bind(&config.api_gateway_url).await?;

    loop {
        // Accept incoming connections
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let config = config.clone();

        tokio::task::spawn(async move {
            let service = service_fn(move |req| handle_request(req, config.clone()));

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn handle_request(
    req: Request<Incoming>,
    config: Arc<GatewayConfig>,
) -> Result<Response<BoxBody>, GenericError> {
    let path = req.uri().path();
    let service_config = match get_service_config(path, &config.services) {
        Some(service_config) => service_config,
        None => {
            return not_found();
        }
    };

    if needs_auth(path, req.method().as_str(), &config.endpoints_without_auth) {
        match authorize_user(req.headers(), &config.authorization_api_url).await {
            Ok(res) if !res.status().is_success() => return Ok(res),
            Ok(_) => (),
            Err(_) => return service_unavailable("Failed to connect to Authorization API"),
        };
    }

    let (parts, body) = req.into_parts();
    let downstream_req = build_downstream_request(parts, body, service_config).await?;

    match forward_request(downstream_req).await {
        Ok(res) => Ok(res),
        Err(_) => service_unavailable("Failed to connect to downstream service"),
    }
}

fn get_service_config<'a>(path: &str, services: &'a [ServiceConfig]) -> Option<&'a ServiceConfig> {
    services.iter().find(|c| path.starts_with(&c.path))
}

fn needs_auth(path: &str, method: &str, no_auth_endpoints: &[NoAuthEndpoints]) -> bool {
    !no_auth_endpoints
        .iter()
        .any(|e| e.endpoint == path && e.method == method)
}

async fn authorize_user(headers: &HeaderMap, auth_api_url: &str) -> Result<Response<BoxBody>, ()> {
    let cookies_header_value = match headers.get(COOKIE) {
        Some(value) => value.to_str().unwrap_or_default(),
        None => "",
    };

    let auth_request = Request::builder()
        .uri(auth_api_url)
        .header(COOKIE, cookies_header_value)
        .body(BoxBody::default())
        .unwrap();

    match forward_request(auth_request).await {
        Ok(res) => Ok(res),
        Err(_) => Err(()),
    }
}

async fn build_downstream_request(
    mut parts: Parts,
    body: Incoming,
    service_config: &ServiceConfig,
) -> Result<Request<BoxBody>, GenericError> {
    let uri = format!(
        "{}:{}{}?{}",
        service_config.target_service,
        service_config.target_port,
        parts.uri.path(),
        parts.uri.query().unwrap_or("")
    );

    parts.uri = uri.parse().unwrap();

    // Rebuild the request with the new URI
    let req = Request::from_parts(parts, body.boxed());

    Ok(req)
}

async fn forward_request(req: Request<BoxBody>) -> Result<Response<BoxBody>, ()> {
    match Client::builder(TokioExecutor::new())
        .build_http()
        .request(req)
        .await
    {
        Ok(res) => {
            let (parts, body) = res.into_parts();
            Ok(Response::from_parts(parts, body.boxed()))
        }
        Err(_) => Err(()),
    }
}

fn not_found() -> Result<Response<BoxBody>, GenericError> {
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full(Bytes::from("Not Found")))
        .unwrap();
    Ok(response)
}

fn service_unavailable<T: Into<Bytes>>(reason: T) -> Result<Response<BoxBody>, GenericError> {
    let response = Response::builder()
        .status(StatusCode::SERVICE_UNAVAILABLE)
        .body(full(reason))
        .unwrap();
    Ok(response)
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}
