use sci::run;

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
