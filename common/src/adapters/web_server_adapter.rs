use std::future;
use std::sync::Arc;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tokio::io;

use crate::ports::log_port::LoggerPort;
// web_server_adapter.rs
use crate::ports::web_server_port::WebServerPort;

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
async fn show_console() -> impl Responder {
    // Return the HTML content for the dashboard
    HttpResponse::Ok().body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>System Diagnostics</title>
    <script src="https://cdn.tailwindcss.com"></script>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/5.15.3/css/all.min.css">
    <style>
        :root {
            --background-color: #121212; /* Dark background */
            --card-background-color: #1F1F1F; /* Darker card background */
            --highlight-color: #5D55FA; /* Highlight color */
            --text-color: #FFFFFF; /* Text color */
            --status-up-color: #48BB78; /* Status up color */
            --status-down-color: #F56565; /* Status down color */
            --glow-color: #7F9CF5; /* Glow color */
        }
        body {
            background: var(--background-color);
            color: var(--text-color);
        }
        .card {
            background: var(--card-background-color);
            border-radius: 0.5rem;
            padding: 1.5rem;
            /* Removed margin-bottom to manage spacing with grid gap */
            position: relative;
            overflow: hidden;
        }
        .card::before {
            content: '';
            position: absolute;
            top: -50%;
            right: -50%;
            bottom: -50%;
            left: -50%;
            z-index: -1;
            background: var(--glow-color);
            border-radius: 2rem;
            opacity: 0;
            transition: opacity 0.3s ease-in-out;
        }
        .card:hover::before {
            opacity: 1;
        }
        .card h2 {
            color: var(--highlight-color);
        }
        .status-indicator {
            height: 10px;
            width: 10px;
            border-radius: 50%;
            display: inline-block;
            margin-right: 0.5rem;
        }
        .status-up {
            background-color: var(--status-up-color);
        }
        .status-down {
            background-color: var(--status-down-color);
        }
        .refresh-button {
            background-color: var(--highlight-color);
            color: var(--text-color);
            padding: 0.5rem 1rem;
            border-radius: 0.375rem;
            font-weight: 600;
            margin-left: auto;
            display: block;
            cursor: pointer;
        }
    </style>
</head>
<body class="font-sans leading-normal tracking-normal">
<div class="flex flex-wrap">
    <!-- Sidebar -->
    <div class="sidebar w-full md:w-1/4 p-4">
        <h2 class="font-semibold text-lg mb-4">System Info</h2>
        <p>Hostname: <span>example-host</span></p>
        <p>IP: <span>192.168.1.1</span></p>
        <h2 class="font-semibold text-lg mt-4 mb-2">Ledger Status</h2>
        <p>Hedera DLT: <span class="status-indicator status-up"></span>Connected</p>
        <p>Cardano: <span class="status-indicator status-down"></span>Disconnected</p>
        <p>Bitcoin: <span class="status-indicator status-up"></span>Connected</p>
        <p>Ether: <span class="status-indicator status-down"></span>Disconnected</p>
    </div>
<div class="container mx-auto px-4 py-5">
    <!-- Grid container with 3 columns -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <!-- Memory Card -->
        <div class="card">
            <h2 class="font-semibold text-lg">Memory Info</h2>
            <div class="flex items-center mt-2">
                <span class="status-indicator status-up"></span>
                <span class="ml-2">16GB Used / 32GB Total</span>
            </div>
            <p class="mt-2">Swap Usage: 2GB / 4GB</p>
        </div>
        
        <!-- Storage Device Card -->
<div class="card">
    <h2 class="font-semibold text-lg">Storage Device Info</h2>
    <p class="mt-2">Device Model: Kingston NVMe SSD</p>
    <div class="flex items-center mt-2">
        <span class="status-indicator status-up"></span>
        <span class="ml-2">Read Speed: 3.5 GB/s</span>
    </div>
    <div class="flex items-center mt-2">
        <span class="status-indicator status-up"></span>
        <span class="ml-2">Write Speed: 2.8 GB/s</span>
    </div>
    <div class="flex items-center mt-2">
        <span class="status-indicator status-up"></span>
        <span class="ml-2">IOPS: 1,000,000</span>
    </div>
    <div class="flex items-center mt-2">
        <span class="status-indicator status-up"></span>
        <span class="ml-2">Capacity: 2 TB</span>
    </div>
</div>

        <!-- CPU Card -->
        <div class="card">
            <h2 class="font-semibold text-lg">CPU Usage</h2>
            <div class="flex items-center mt-2">
                <span class="status-indicator status-up"></span>
                <span class="ml-2">35% Load</span>
            </div>
            <p class="mt-2">Processor Info: Intel Xeon E5-2678 v3 @ 2.50GHz</p>
        </div>

        <!-- GPU Card -->
        <div class="card">
            <h2 class="font-semibold text-lg">GPU Load</h2>
            <div class="flex items-center mt-2">
                <span class="status-indicator status-up"></span>
                <span class="ml-2">NVIDIA RTX 3080: 60% Load</span>
            </div>
            <p class="mt-2">GPU Info: 10GB GDDR6X, 8704 CUDA cores</p>
        </div>

        <!-- Network Bandwidth Card -->
        <div class="card">
            <h2 class="font-semibold text-lg">Network Bandwidth</h2>
            <div class="flex items-center mt-2">
                <span class="status-indicator status-up"></span>
                <span class="ml-2">500Mbps In / 250Mbps Out</span>
            </div>
            <p class="mt-2">Network Speed: 1Gbps link</p>
        </div>

        <!-- Public Network Availability Status Card -->
        <div class="card">
            <h2 class="font-semibold text-lg">Public Network Status</h2>
            <div class="flex items-center mt-2">
                <span class="status-indicator status-up"></span>
                <span class="ml-2">Online</span>
            </div>
            <p class="mt-2">No interruptions detected</p>
        </div>

        <!-- Operating System Information Card -->
        <div class="card">
            <h2 class="font-semibold text-lg">Operating System</h2>
            <div class="flex items-center mt-2">
                <span class="status-indicator status-up"></span>
                <span class="ml-2">Ubuntu 20.04 LTS</span>
            </div>
            <p class="mt-2">Kernel version: 5.4.0-42-generic</p>
        </div>

        <!-- Motherboard Information Card -->
        <div class="card">
            <h2 class="font-semibold text-lg">Motherboard Information</h2>
            <div class="flex items-center mt-2">
                <span class="status-indicator status-up"></span>
                <span class="ml-2">ASUS ROG STRIX Z390-E</span>
            </div>
            <p class="mt-2">BIOS version: 1302, Release Date: 05/10/2019</p>
        </div>

        <!-- Other cards follow the same structure -->
        <!-- ... -->
    </div>
</div>

<script>
    // JavaScript for dynamic updates and interactions can be added here
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
                .route("/console", web::get().to(show_console)) // Route for show console
        })
        .workers(1) // set the number of workers
        .bind("127.0.0.1:8000")?
        .run();

        tokio::spawn(server);

        // Wait indefinitely, or until you have another condition to close the application
        future::pending::<()>().await;

        Ok(())
    }
}
