use sci::configuration::get_config;
use std::net::TcpListener;

mod routes;
mod startup;
#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Failed reading the config.");
    let addr = format!("127.0.0.1:{}", config.app_port);
    let listener = TcpListener::bind(addr).expect("Failed binding to random port");
    startup::run(listener)?.await
}
