//! PS (Process Status) Domain Entity
//!
//! This module provides a domain entity for the `ps` command, a tool for displaying
//! current process statuses on Unix-based systems like Linux and macOS.

use std::collections::HashMap;

/// Represents the configuration for a `ps` command execution.
/// This struct is used to configure and manage parameters for the `ps` command
/// to monitor processes and their resource utilization.
pub struct PsConfig {
    /// The sorting criteria for processes, e.g., CPU usage or memory usage.
    pub sort_by: String,

    /// The maximum number of processes to display.
    /// This value limits the output to the top N processes based on the sorting criteria.
    pub limit: usize,

    /// Include processes of all users, not just the current user.
    pub all_users: bool,

    /// Show full command line for each process instead of just the command name.
    pub full_format: bool,

    /// Custom options for the `ps` command.
    /// This allows specifying additional flags or arguments to customize the `ps` output.
    pub custom_options: HashMap<String, String>,
}

impl PsConfig {
    /// Creates a new `ps` command configuration with specified parameters.
    ///
    /// This constructor initializes a `PsConfig` instance with detailed settings
    /// for running the `ps` command. Each parameter can be customized to tailor the process
    /// monitoring to specific requirements or interests.
    ///
    /// # Arguments
    ///
    /// * `sort_by` - Criteria for sorting processes.
    /// * `limit` - Maximum number of processes to display.
    /// * `all_users` - Whether to include processes from all users.
    /// * `full_format` - Show full command line for each process.
    /// * `custom_options` - Additional custom options for `ps`.
    ///
    /// # Returns
    ///
    /// * `PsConfig` - A new instance with the provided configuration.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = PsConfig::new("cpu", 10, true, false, HashMap::new());
    /// // Use `config` to run and parse `ps` command
    /// ```
    pub fn new(
        sort_by: String,
        limit: usize,
        all_users: bool,
        full_format: bool,
        custom_options: HashMap<String, String>,
    ) -> Self {
        Self {
            sort_by,
            limit,
            all_users,
            full_format,
            custom_options,
        }
    }

    // Additional methods to execute and parse the `ps` command can be added here...
}
