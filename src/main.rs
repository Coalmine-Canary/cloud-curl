use clap::Parser;

mod config;
mod request;

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
enum Cli {
    #[command(name = "request")]
	Request(request::Commands),
	#[command(name = "config", subcommand)]
    Config(config::Commands) // or set and get
}


#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args {
        Cli::Request(r) => { request::request(r).await },
        Cli::Config(c)  => { 
            match config::handle(c).await {
                Ok(_) => {},
                Err(e) => { println!("Error: {}", e) }
        } }
    }   
}
