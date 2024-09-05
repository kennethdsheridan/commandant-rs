// src/ports/web_server_port.rs
use async_trait::async_trait;
use std::io;
use std::sync::Arc;

use crate::ports::log_port::LoggerPort;

/// WebServerPort trait defines the interface for web server operations.
#[async_trait]
pub trait WebServerPort {
    async fn start_server(&self) -> io::Result<()>;

    async fn start_websocket(&self, logger: Arc<dyn LoggerPort>) -> io::Result<()>;
}
