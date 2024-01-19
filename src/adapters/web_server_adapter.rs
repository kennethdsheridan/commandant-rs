// web_server_adapter.rs
use crate::ports::log_port::LoggerPort;
use crate::ports::web_server_port::WebServerPort;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use std::sync::Arc;

use tokio::io;

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
// show_dashboard
///
/// A comprehensive dashboard for the OneForAll application. It displays the current
/// status of the server, the current stress test, and the current processes.
/// // Define the show_dashboard handler function
/// // This function returns a String, which is the HTML content for the dashboard.
/// // The HTML content is hardcoded in this example, but it can be generated dynamically
/// // using a templating engine such as Handlebars.
/// // The show_dashboard function is defined as an async function, which allows it to
/// // perform asynchronous operations such as reading from a database or making an HTTP request.
async fn show_dashboard() -> impl Responder {
    // Return the HTML content for the dashboard
    HttpResponse::Ok().body(
        r#"
        <html>
            <head>
                <title>OneForAll Dashboard</title>
            </head>
            <body>
                <h1>OneForAll Dashboard</h1>
                <h2>Server Status</h2>
                <p>Server is running</p>
                <h2>Current Stress Test</h2>
                <p>None</p>
                <h2>Current Processes</h2>
                <p>None</p>
            </body>
        </html>
        "#,
    )
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
