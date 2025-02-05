use std::net::TcpListener;

use sci::run;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed binding to random port");
    run(listener)?.await
}
