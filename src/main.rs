use aws_sigv4::http_request::{sign, SigningSettings, SigningParams, SignableRequest};
use clap::{Parser, ValueEnum, Args};
use hyper::client::HttpConnector;
use hyper::{Client, Request};
use hyper_openssl::HttpsConnector;

use std::collections::HashMap;
use std::env;
//use std::thread::__FastLocalKeyInner;
use std::process::exit;
use std::time::SystemTime;
use std::time::Duration;

mod config;

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

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
enum Cli {
    #[command(name = "request")]
	Request(RequestCommand),
	#[command(name = "config", subcommand)]
    Config(config::Commands) // or set and get
}

#[derive(Args)]
/// Send request with given args to endpoint
struct RequestCommand {
    #[arg(short, long)]
    endpoint: Option<String>,

    #[arg(short, long, value_enum)]
    method: Option<Method>,

    #[arg(short, long)]
    body: Option<String>
}

async fn request(args: RequestCommand) {
    let vars: HashMap<String, String> = env::vars().filter(|(key, _val)|
        match key.as_str() {
            "ACCESS_KEY"|"SECRET_KEY" => true,
            _ => false
        }
    ).collect();

    if ! vars.contains_key("ACCESS_KEY") || ! vars.contains_key("SECRET_KEY") {
        eprintln!("Error: Missing ACCESS_KEY or SECRET_KEY. Please specify in environment variables. "); // TODO: Change when added args etc. 
        exit(1)
    }

    let method: &str = match args.method.unwrap_or(Method::GET) {
        Method::GET    => "GET",
        Method::POST    => "POST",
        Method::DELETE => "DELETE",
        Method::PUT    => "PUT"
    };

    // Create the request to sign
    let mut request: Request<String> = match Request::builder()
    .method(method) // TODO: add arg
    .uri(args.endpoint.unwrap()) // TODO: fix
    .body(args.body.unwrap_or("".into())) {// TODO: add arg
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to create request. Error was: {}", e); exit(2)
        }
    };

    let signing_settings = SigningSettings::default();
    let signing_params = SigningParams::builder()
        .access_key(vars.get("ACCESS_KEY").unwrap()) // TODO: Fetch from aws config
        .secret_key(vars.get("SECRET_KEY").unwrap())
        .region("us-east-1") // TODO: Variablise
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


#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args {
        Cli::Request(r) => { request(r).await },
        Cli::Config(c)  => { 
            match config::handle(c).await {
                Ok(_) => {},
                Err(e) => { println!("Error: {}", e) }
        } }
    }   
}
