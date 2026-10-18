use tribune::run;

use std::io::Error;

use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    run(listener)?.await
}
