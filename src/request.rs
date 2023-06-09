use clap::{ValueEnum, Args};
use http::StatusCode;
use hyper::client::HttpConnector;
use hyper::{Client, Request};
use hyper_openssl::HttpsConnector;
//use std::thread::__FastLocalKeyInner;
use std::process::exit;
use std::time::SystemTime;
use std::time::Duration;

use crate::auth::sign_request;

#[derive(ValueEnum, Clone)]
enum Method {
    #[value(alias("GET"))]
    GET,
    #[value(alias("PUT"))]
    PUT,
    #[value(alias("POST"))]
    POST,
    #[value(alias("DELETE"))]
    DELETE
}

#[derive(Args)]
/// Send request with given args to endpoint
pub struct Commands {
    #[arg(short, long)]
    endpoint: Option<String>,

    #[arg(short, long, value_enum)]
    method: Option<Method>,

    #[arg(short, long)]
    body: Option<String>,
}

async fn request(endpoint: String, method: Method, body: String) {
    let method: &str = match method {
        Method::GET    => "GET",
        Method::POST   => "POST",
        Method::DELETE => "DELETE",
        Method::PUT    => "PUT"
    };

    let mut endpoint = endpoint;

    if !endpoint.contains("://") {
        endpoint = format!("https://{}", endpoint);
    }

    // Create the request to sign
    let request: Request<String> = match Request::builder()
    .method(method)
    .uri(endpoint)
    .body(body) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to create request. Error was: {}", e); exit(2)
        }
    };

    let request = match sign_request(request) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to sign request, error was {}", e);
            exit(4)
        }
    };

    //let connector = HttpConnector::new();
    let client: Client<HttpsConnector<HttpConnector>, String> = Client::builder().pool_idle_timeout(Duration::from_secs(30))
    .http2_only(true).build(HttpsConnector::new().unwrap());

    match client.request(request).await {
        Ok(r) => {
            let status = r.status();
            let body = match hyper::body::to_bytes(r.into_body()).await {
                Ok(b) => {
                    String::from_utf8(b.to_vec()).unwrap()
                },
                Err(_e) => {
                    eprintln!("Error: Failed to get bytes from response body. ");
                    exit(2)
                }
            };

            match status {
                StatusCode::OK => { println!("{}", body) },
                _ => { eprintln!("Request failed with code {}", status); println!("{}", body) }
            }
           
                        
        },
        Err(e) => {
            eprintln!("Request failed. Error was {}", e);
        }
    };
}

pub async fn handle(args: Commands) {

    let method = args.method.unwrap_or(Method::GET);

    let body = args.body.unwrap_or("".into());

    let endpoint = match args.endpoint {
        Some(e) => e,
        None => "".into()
    };

    request(endpoint, method, body).await
}
