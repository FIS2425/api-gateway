mod config;
mod utils;

use clap::{Arg, Command};
use config::logger::Logger;
use config::openapi::OpenApiMerger;
use config::parser::{load_config, GatewayConfig, NoAuthEndpoints, ServiceConfig};
use http_body_util::BodyExt;
use hyper::body::{Bytes, Incoming};
use hyper::header::HeaderValue;
use hyper::header::CONTENT_TYPE;
use hyper::http::request::Parts;
use hyper::server::conn::http1;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::client::legacy::Client;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::service::TowerToHyperService;
use iptools::ipv4;
use iptools::ipv6;
use openapiv3::OpenAPI;
use reqwest::header::{HeaderMap, COOKIE};
use std::net::SocketAddr;
use std::result::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{AllowOrigin, CorsLayer};
use utils::http::{full, BoxBody};
use uuid::Uuid;

type GenericError = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    let matches = Command::new("HyperGate")
        .version("0.1.0")
        .author("@adrrf @AntonioRodriguezRuiz @alvarobernal2412")
        .about("An API Gateway built with Rust and Hyper")
        .subcommand(
            Command::new("merge")
                .about("Merge OpenAPI specs")
                .arg(
                    Arg::new("specs")
                        .long("specs")
                        .required(true)
                        .help("Directory of OpenAPI specs to merge."),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .required(true)
                        .help("Output path of the HTML OpenAPI merged spec."),
                ),
        )
        .subcommand(
            Command::new("serve")
                .about("Serve the API Gateway")
                .arg(
                    Arg::new("conf")
                        .long("conf")
                        .required(true)
                        .help("Path to the configuration file."),
                )
                .arg(
                    Arg::new("spec")
                        .long("specs")
                        .required(true)
                        .help("Path to the OpenAPI spec."),
                )
                .arg(
                    Arg::new("html")
                        .long("html")
                        .required(true)
                        .help("Path to the OpenAPI HTML."),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("merge", sub_matches)) => {
            let specs = sub_matches
                .get_one::<String>("specs")
                .expect("Specs directory is required.");
            let output = sub_matches
                .get_one::<String>("output")
                .expect("Output path is required.");
            println!("Merging OpenAPI specs...");
            if let Err(err) = merge_openapi_specs(specs, output).await {
                eprintln!("Error merging OpenAPI specs: {:?}", err);
            }
        }
        Some(("serve", sub_matches)) => {
            let config = sub_matches
                .get_one::<String>("conf")
                .expect("Config file is required.");
            let spec = sub_matches
                .get_one::<String>("spec")
                .expect("OpenAPI spec is required.");
            let html = sub_matches
                .get_one::<String>("html")
                .expect("HTML path is required.");
            if let Err(err) = api_gateway(config, spec, html).await {
                println!("Error: {:?}", err);
            }
        }
        _ => println!("Invalid command"),
    }
}

async fn api_gateway(
    config_path: &str,
    openapi_spec: &str,
    html_path: &str,
) -> Result<(), GenericError> {
    let config = Arc::new(load_config(config_path));
    let logger = Arc::new(Logger::from_config(&config.logger_config));

    let url = format!(
        "{}://{}",
        if config.is_https { "https" } else { "http" },
        config.api_gateway_url
    );

    println!(
        "HyperGate🚀 listening at {} \n \tOpenAPI specification at {}/docs",
        url, url
    );
    let listener = TcpListener::bind(&config.api_gateway_url).await?;

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_origin(AllowOrigin::mirror_request())
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    loop {
        // Accept incoming connections
        let (stream, conn_addr) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let config = config.clone();
        let logger = logger.clone();
        let html = html_path.to_string();
        let spec = openapi_spec.to_string();
        let cors = cors.clone();

        tokio::task::spawn(async move {
            let request_id = Uuid::new_v4().to_string();
            let html = &html;
            let spec = &spec;
            let cors = cors.clone();

            logger.info(
                "New connection",
                &[
                    ("request_id", &request_id),
                    ("ip", conn_addr.ip().to_string().as_str()),
                ],
            );

            let service = ServiceBuilder::new().layer(cors).service_fn(move |req| {
                handle_request(
                    req,
                    conn_addr,
                    config.clone(),
                    logger.clone(),
                    request_id.to_owned(),
                    spec,
                    html,
                )
            });
            let service = TowerToHyperService::new(service);

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn merge_openapi_specs(
    docs_path: &str,
    output_path: &str,
) -> Result<OpenAPI, Box<dyn std::error::Error + Send + Sync>> {
    let mut merger = OpenApiMerger::new(docs_path, output_path);
    merger.load_specs()?;
    let merged_spec = merger.merge()?;
    merger.generate_swagger_ui()?;
    println!("OpenAPI specs merged successfully.");
    Ok(merged_spec)
}

async fn handle_request(
    req: Request<Incoming>,
    conn_addr: SocketAddr,
    config: Arc<GatewayConfig>,
    logger: Arc<Logger>,
    request_id: String,
    openapi_path: &str,
    html_path: &str,
) -> Result<Response<BoxBody>, GenericError> {
    if req.method() == Method::OPTIONS {
        let response = Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(full(Bytes::new()))
            .unwrap();
        return Ok(response);
    }

    let path = req.uri().path();

    match path {
        "/docs/spec" => return serve_openapi_spec(openapi_path).await,
        "/docs" => return serve_swagger_ui(html_path).await,
        _ => (),
    }

    let service_config = match get_service_config(path, &config.services) {
        Some(service_config) => service_config,
        None => {
            logger.warn(
                &format!("Path not found: {}", path),
                &[
                    ("request_id", &request_id),
                    ("ip", conn_addr.ip().to_string().as_str()),
                    ("method", req.method().as_str()),
                    ("url", req.uri().path().to_string().as_str()),
                    ("params", req.uri().query().unwrap_or("")),
                ],
            );
            return not_found();
        }
    };

    if needs_auth(path, req.method().as_str(), &config.endpoints_without_auth) {
        match authorize_user(req.headers(), &config.authorization_api_url).await {
            Ok(res) if !res.status().is_success() => {
                logger.info(
                    "Connection closed",
                    &[
                        ("request_id", &request_id),
                        ("ip", conn_addr.ip().to_string().as_str()),
                        ("status", res.status().as_str()),
                    ],
                );
                return Ok(res);
            }
            Ok(_) => (),
            Err(_) => {
                logger.err(
                    &format!(
                        "Failed to connect to Authorization API: {}",
                        &config.authorization_api_url
                    ),
                    &[
                        ("request_id", &request_id),
                        ("ip", conn_addr.ip().to_string().as_str()),
                        ("method", req.method().as_str()),
                        ("url", req.uri().path().to_string().as_str()),
                        ("params", req.uri().query().unwrap_or("")),
                    ],
                );
                return service_unavailable("Failed to connect to Authorization API");
            }
        };
    }

    let (parts, body) = req.into_parts();

    // For logging
    let cloned_parts = parts.clone();

    let downstream_req =
        build_downstream_request(parts, body, conn_addr, &request_id, service_config).await?;

    match forward_request(downstream_req).await {
        Ok(res) => {
            logger.info(
                "Connection closed",
                &[
                    ("request_id", &request_id),
                    ("ip", conn_addr.ip().to_string().as_str()),
                    ("status", res.status().as_str()),
                ],
            );
            Ok(res)
        }
        Err(_) => {
            logger.err(
                &format!(
                    "Failed to connect to downstream service {}",
                    &service_config.target_service
                ),
                &[
                    ("request_id", &request_id),
                    ("ip", conn_addr.ip().to_string().as_str()),
                    ("method", cloned_parts.method.as_str()),
                    ("url", cloned_parts.uri.path().to_string().as_str()),
                    ("params", cloned_parts.uri.query().unwrap_or("")),
                ],
            );
            service_unavailable("Failed to connect to downstream service")
        }
    }
}

async fn serve_openapi_spec(openapi_path: &str) -> Result<Response<BoxBody>, GenericError> {
    match tokio::fs::read_to_string(openapi_path).await {
        Ok(content) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/yaml")
                .body(full(Bytes::from(content)))
                .unwrap();
            Ok(response)
        }
        Err(_) => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(full(Bytes::from("OpenAPI Specification Not Found")))
                .unwrap();
            Ok(response)
        }
    }
}

async fn serve_swagger_ui(html_path: &str) -> Result<Response<BoxBody>, GenericError> {
    match tokio::fs::read_to_string(&html_path).await {
        Ok(content) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(full(Bytes::from(content)))
                .unwrap();
            Ok(response)
        }
        Err(_) => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(full(Bytes::from("Swagger UI Not Found")))
                .unwrap();
            Ok(response)
        }
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
    conn_addr: SocketAddr,
    request_id: &str,
    service_config: &ServiceConfig,
) -> Result<Request<BoxBody>, GenericError> {
    let uri = format!(
        "{}:{}{}?{}",
        service_config.target_service,
        service_config.target_port,
        parts.uri.path(),
        parts.uri.query().unwrap_or("")
    );

    let forwarded_for = match parts.headers.get("x-forwarded-for") {
        Some(value)
            if ipv4::validate_ip(value.to_str().unwrap_or_default())
                || ipv6::validate_ip(value.to_str().unwrap_or_default()) =>
        {
            value.clone()
        }
        _ => HeaderValue::from_str(&conn_addr.ip().to_string()).unwrap(),
    };

    let request_id_header = HeaderValue::from_str(request_id).unwrap();

    parts.uri = uri.parse().unwrap();
    parts.headers.insert("x-forwarded-for", forwarded_for);
    parts.headers.insert("x-request-id", request_id_header);

    // Rebuild the request with the new URI and headers
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
