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
        <html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>System Diagnostics Dashboard - Dark Mode</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <style>
        /* Dark mode styles */
        body {
            background-color: #1A1A1A; /* Dark background for body */
            color: #CCCCCC; /* Light text color for body */
        }

        .gradient-background {
            background: linear-gradient(145deg, #4C0099 0%, #190033 100%);
        }

        .text-color {
            color: #BB86FC; /* Light purple text color for better visibility in dark mode */
        }

        .card {
            background: #242424; /* Darker background for cards */
            border-radius: 0.5rem;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.25); /* Darker shadow for cards */
            padding: 1.5rem;
            border: none; /* Remove border for dark mode */
        }

        .button {
            background-color: #BB86FC; /* Light purple background for buttons */
            color: #1A1A1A; /* Dark text color for buttons */
            padding: 0.5rem 1rem;
            border-radius: 0.375rem;
            font-weight: 600;
            border: none; /* Remove border for buttons */
        }

        .chart-placeholder {
            background-color: #333333; /* Darker background for chart placeholders */
            border: 1px solid #444444; /* Slight border for chart placeholders */
        }
    </style>
</head>
<body class="font-sans leading-normal tracking-normal">
<div class="container mx-auto px-4">
    <div class="py-8">
        <div class="gradient-background text-white p-8 rounded-lg shadow-md">
            <h2 class="text-3xl font-semibold leading-tight">System Diagnostics Dashboard</h2>
        </div>
        <div class="mt-6">
            <div class="card">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <!-- Power Utilization -->
                    <div class="md:col-span-1">
                        <h3 class="text-lg font-semibold text-color">Power Utilization</h3>
                        <p id="power-util" class="mt-1 text-sm">Loading...</p>
                        <button class="button mt-2">View Details</button>
                    </div>
                    <!-- Memory Utilization -->
                    <div class="md:col-span-1">
                        <h3 class="text-lg font-semibold text-color">Memory Utilization</h3>
                        <p id="memory-util" class="mt-1 text-sm">Loading...</p>
                        <button class="button mt-2">View Details</button>
                    </div>
                    <!-- CPU/GPU Utilization -->
                    <div class="md:col-span-1">
                        <h3 class="text-lg font-semibold text-color">CPU/GPU Utilization</h3>
                        <p id="cpu-gpu-util" class="mt-1 text-sm">Loading...</p>
                        <button class="button mt-2">View Details</button>
                    </div>
                </div>
            </div>
        </div>
        <div class="mt-6">
            <div class="card">
                <h3 class="text-lg font-semibold text-color">Real-time Graphs</h3>
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mt-4">
                    <!-- Power Chart Placeholder -->
                    <div class="rounded-lg h-64 chart-placeholder"></div>
                    <!-- Memory Chart Placeholder -->
                    <div class="rounded-lg h-64 chart-placeholder"></div>
                    <!-- CPU/GPU Chart Placeholder -->
                    <div class="rounded-lg h-64 chart-placeholder"></div>
                </div>
            </div>
        </div>
    </div>
</div>

<script>
    // Placeholder for JavaScript to fetch and update real-time data
    document.addEventListener('DOMContentLoaded', function () {
        document.getElementById('power-util').textContent = '50% (50W)';
        document.getElementById('memory-util').textContent = '75% (12GB / 16GB)';
        document.getElementById('cpu-gpu-util').textContent = 'CPU: 60%, GPU: 40%';
    });
</script>
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
                .route("/dashboard", web::get().to(show_dashboard)) // Route for show_dashboard
        })
        .bind("127.0.0.1:8000")?
        .run();

        tokio::spawn(server);

        // Wait indefinitely, or until you have another condition to close the application
        futures::future::pending::<()>().await;

        Ok(())
    }
}
