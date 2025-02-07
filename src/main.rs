use std::net::TcpListener;
mod routes;
mod startup;
#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed binding to random port");
    startup::run(listener)?.await
}
