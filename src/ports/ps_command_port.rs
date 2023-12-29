/// `PsCommandPort` Trait
///
/// Defines an interface for executing the `ps` command to monitor system processes.
/// This trait abstracts the functionality for running the `ps` command, allowing
/// different implementations to tailor process monitoring and CPU usage analysis.
///
/// Implementors can provide specific strategies for process monitoring, adhering to
/// the Dependency Inversion Principle for flexibility and loose coupling in application design.
pub trait PsCommandPort {
    /// Executes the `ps` command to gather process statistics.
    ///
    /// Implement this method to perform process monitoring using the `ps` command.
    /// The implementation should gather details about running processes, including
    /// their CPU usage, and may include features for sorting, filtering, or limiting
    /// the output for specific monitoring requirements.
    ///
    /// # Returns
    /// A `Result` containing either the command output as a `String` or an error.
    fn execute_ps_command(&self) -> Result<String, String>;

    /// Writes the output of the `ps` command to a specified file.
    ///
    /// Implement this method to save the output of the `ps` command to a file.
    /// This can be useful for logging, analysis, or real-time monitoring purposes.
    ///
    /// # Arguments
    /// * `output` - The output string from the `ps` command.
    /// * `file_path` - The path to the file where the output should be written.
    ///
    /// # Returns
    /// A `Result` indicating the success or failure of the write operation.
    fn write_to_file(&self, output: String, file_path: &str) -> Result<(), String>;

    // Placeholder for additional methods related to `ps` command management, e.g., custom sorting or filtering.
    // fn sort_processes(&self, criteria: &str) -> Result<Vec<ProcessInfo>, String>;
    // fn filter_processes(&self, filter: &str) -> Result<Vec<ProcessInfo>, String>;

    // Periodically executes the `ps` command to gather CPU statistics and writes to a file.
    ///
    /// # Arguments
    /// * `output_file_path` - The path to the file where the command output will be saved.
    fn collect_cpu_statistics(&self, output_file_path: &str);
}
