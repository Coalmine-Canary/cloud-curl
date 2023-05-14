use aws_sigv4::http_request::{sign, SigningSettings, SigningParams, SignableRequest};
use clap::{ValueEnum, Args};
use hyper::client::HttpConnector;
use hyper::{Client, Request};
use hyper_openssl::HttpsConnector;
//use std::thread::__FastLocalKeyInner;
use std::process::exit;
use std::time::SystemTime;
use std::time::Duration;

use crate::auth::get_credentials;

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

    #[arg(short, long)]
    region: Option<String>
}

async fn sign_request(region: &str) {} // service_name, creds

async fn request(endpoint: String, region: String, method: Method, body: String) {
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
    let mut request: Request<String> = match Request::builder()
    .method(method)
    .uri(endpoint)
    .body(body) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to create request. Error was: {}", e); exit(2)
        }
    };

    let credentials = match get_credentials() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed fetching credentials. Error: {}", e);
            exit(3)
        }
    };

    let signing_settings = SigningSettings::default();
    let signing_params = SigningParams::builder()
        .access_key(credentials.0.as_str()) // TODO: Fetch from aws config
        .secret_key(credentials.1.as_str())
        .region(region.as_str()) // TODO: Variablise
        .service_name("es")
        .time(SystemTime::now())
        .settings(signing_settings)
        .build()
        .unwrap();
    // Convert the HTTP request into a signable request
    let signable_request = SignableRequest::from(&request);

    // Sign and then apply the signature to the request
    let (signing_instructions, _signature) = sign(signable_request, &signing_params).unwrap().into_parts();
    signing_instructions.apply_to_request(&mut request);

    //let connector = HttpConnector::new();
    let client: Client<HttpsConnector<HttpConnector>, String> = Client::builder().pool_idle_timeout(Duration::from_secs(30))
    .http2_only(true).build(HttpsConnector::new().unwrap());

    match client.request(request).await {
        Ok(r) => {
            match hyper::body::to_bytes(r.into_body()).await {
                Ok(b) => {
                    println!("{}", String::from_utf8(b.to_vec()).unwrap());
                },
                Err(_e) => {
                    eprintln!("Error: Failed to get bytes from response body. ")
                }
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

    let region = args.region.unwrap_or("us-east-1".into());

    request(endpoint, region, method, body).await
}
