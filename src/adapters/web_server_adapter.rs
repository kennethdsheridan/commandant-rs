// web_server_adapter.rs
use crate::ports::log_port::LoggerPort;
use crate::ports::web_server_port::WebServerPort;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use async_trait::async_trait;
use std::sync::Arc;
use std::thread;
use tokio::io;
use tokio::task::LocalSet;

/// WebServerAdapter
///
/// Adapter for the web server, integrating a logging facility.
pub struct WebServerAdapter {
    logger: Arc<dyn LoggerPort>, // Use LoggerPort trait for the logger
}

// Implement the Sync trait for the WebServerAdapter struct.
impl WebServerAdapter {
    /// new
    ///
    /// Constructs a new WebServerAdapter instance.
    pub fn new(logger: Arc<dyn LoggerPort>) -> Self {
        Self { logger }
    }
}

/// get_status
///
/// A simple endpoint to check the server's status. It responds with "Server is running"
/// when the server is operational.
/// // Define the get_status handler function
async fn get_status() -> impl Responder {
    HttpResponse::Ok().body("Server is running")
}

// Implement the WebServerPort trait for the WebServerAdapter struct.
#[async_trait::async_trait]
impl WebServerPort for WebServerAdapter {
    async fn start_server(&self) -> io::Result<()> {
        let server = HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(HttpResponse::Ok)) // Default route
                .route("/status", web::get().to(get_status)) // Route for get_status
        })
        .bind("127.0.0.1:8000")?
        .run();

        tokio::spawn(server);

        // Wait indefinitely, or until you have another condition to close the application
        futures::future::pending::<()>().await;

        Ok(())
    }
}
