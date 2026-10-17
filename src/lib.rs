use std::io::Error;

use actix_web::{App, HttpResponse, HttpServer, dev::Server, web};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(address: &str) -> Result<Server, Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(address)?
        .run();

    Ok(server)
}

#[cfg(test)]
mod tests {
    use crate::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success())
    }
}
