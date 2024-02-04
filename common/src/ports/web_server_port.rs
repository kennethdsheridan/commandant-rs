// src/ports/web_server_port.rs

use std::io;

use async_trait::async_trait;

/// WebServerPort trait defines the interface for web server operations.
#[async_trait]
pub trait WebServerPort {
    async fn start_server(&self) -> io::Result<()>;
}
