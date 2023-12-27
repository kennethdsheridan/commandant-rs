use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use std::sync::Arc;
use crate::adapters::log_adapter::FernLogger;
use crate::ports::log_port::LoggerPort;
use crate::ports::web_server_port::WebServerPort;
use async_trait::async_trait;
use std::io;
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    status: String,
    uptime: u64,  // example field, replace with actual calculation
}

pub struct WebServerAdapter {
    logger: Arc<FernLogger>,
}

impl WebServerAdapter {
    /// Creates a new WebServerAdapter instance.
    pub fn new(logger: Arc<FernLogger>) -> Self {
        WebServerAdapter { logger: Arc::clone(&logger) }
    }

    /// An asynchronous function that serves as the handler for status endpoint.
    async fn get_status() -> impl Responder {
        HttpResponse::Ok()
            .json(Status {
                status: "OK".to_string(),
                uptime: 5000,  // replace with actual calculation
            })
    }
}

#[async_trait]
impl WebServerPort for WebServerAdapter {
    async fn start_server(&self) -> io::Result<()> {
        let logger = Arc::clone(&self.logger);
        logger.log_info("Starting Actix-web server...");

        // Ensure the closure and all resources it captures are thread-safe
        HttpServer::new(move || {
            App::new().route("/status", web::get().to(WebServerAdapter::get_status))
        })
            .bind("127.0.0.1:8000")?
            .run()
            .await
            .map_err(move |e| {
                logger.log_error(&format!("Failed to start Actix-web server: {}", e));
                std::io::Error::new(std::io::ErrorKind::Other, e)
            })
    }
}
