// src/ports/web_server_port.rs

use async_trait::async_trait;
use std::io;

/// WebServerPort trait defines the interface for web server operations.
#[async_trait]
pub trait WebServerPort {
    async fn start_server(&self) -> io::Result<()>;
}
