use tribune::run;

use std::io::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run("http://127.0.0.1:8000")?.await
}
