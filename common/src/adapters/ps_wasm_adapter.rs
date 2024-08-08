use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;


// Define a struct to hold process data for demonstration purposes.
// In a real-world scenario, this would be replaced with appropriate data structures.
#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessData {
    cpu_usage: f32,
    memory_usage: f32,
    command: String,
}

// Implement the `ProcessData` struct with a new function to create instances.
#[wasm_bindgen]
impl ProcessData {
    #[wasm_bindgen(constructor)]
    pub fn new(cpu_usage: f32, memory_usage: f32, command: String, string: String) -> ProcessData {
        ProcessData {
            cpu_usage,
            memory_usage,
            command,
        }
    }


/// The `write_to_wasm` function is a public function that takes a string output from the `ps` command,
/// parses it into process data, serializes the process data into a JSON string, and then converts the JSON string
/// into a `JsValue` that can be passed to the WebAssembly context.
///
/// # Arguments
///
/// * `output` - A string that holds the output of the `ps` command.
///
/// # Returns
///
/// * `Ok(JsValue)` - If the function executes successfully, it returns a `JsValue` that contains the serialized process data.
/// * `Err(JsValue)` - If an error occurs during the execution of the function, it returns an `Err` result with a `JsValue` that contains the error message.
#[wasm_bindgen]
pub fn write_to_wasm(output: String) -> Result<JsValue, JsValue> {
    // Parse the output of the `ps` command to extract process data.
    let process_data = parse_ps_output(&output)?;

    // Serialize the `process_data` to a JSON string.
    let json = serde_json::to_string(&process_data)
        .map_err(|_| JsValue::from_str("Failed to serialize process data"))?;

    // Convert the JSON string to a `JsValue`.
    let js_value = JsValue::from_str(&json);

    // Return the `JsValue`.
    Ok(js_value)
}

/// Parses the output of the `ps` command and returns a `ProcessData` instance.
///
/// This function iterates over each line of the `ps` command output, splits each line into whitespace-separated fields,
/// and attempts to create a `ProcessData` instance from these fields. If the line has more than 10 fields and the fields
/// can be successfully parsed into the appropriate data types, a `ProcessData` instance is created and returned.
///
/// # Arguments
///
/// * `output` - A string slice that holds the output of the `ps` command.
///
/// # Returns
///
/// * `Ok(ProcessData)` - If the function executes successfully, it returns a `ProcessData` instance.
/// * `Err(JsValue)` - If an error occurs during the execution of the function, it returns an `Err` result with a `JsValue` that contains the error message.
fn parse_ps_output(output: &str) -> Result<ProcessData, JsValue> {
    // Iterate over each line in the output
    for line in output.lines() {
        // Split the line into whitespace-separated fields
        let fields: Vec<&str> = line.split_whitespace().collect();
        // Check if the line has more than 10 fields
        if fields.len() > 10 {
            // Attempt to create a `ProcessData` instance from the fields
            let process_data = ProcessData::new(
                // Parse the first field into an integer
                fields[0]
                    .to_string()
                    .parse()
                    .map_err(|_| JsValue::from_str("Error parsing field[0]"))?,
                // Parse the third field into a float
                fields[2]
                    .parse()
                    .map_err(|_| JsValue::from_str("Error parsing CPU usage"))?,
                // Parse the fourth field into a float
                fields[3]
                    .parse()
                    .map_err(|_| JsValue::from_str("Error parsing memory usage"))?,
                // Join the remaining fields into a single string
                fields[10..].join(" "),
            );
            // Return the `ProcessData` instance
            return Ok(process_data);
        }
    }
    // If no data could be parsed, return an error
    Err(JsValue::from_str("No process data found"))
}
}

/// The `write_to_wasm` function is a public function that takes a string output from the `ps` command,
/// parses it into process data, serializes the process data into a JSON string, and then converts the JSON string
/// into a `JsValue` that can be passed to the WebAssembly context.
///
/// # Arguments
///
/// * `output` - A string that holds the output of the `ps` command.
///
/// # Returns
///
/// * `Ok(JsValue)` - If the function executes successfully, it returns a `JsValue` that contains the serialized process data.
/// * `Err(JsValue)` - If an error occurs during the execution of the function, it returns an `Err` result with a `JsValue` that contains the error message.
#[wasm_bindgen]
pub fn write_to_wasm(output: String) -> Result<JsValue, JsValue> {
    // Parse the output of the `ps` command to extract process data.
    let process_data = parse_ps_output(&output)?;

    // Serialize the `process_data` to a JSON string.
    let json = serde_json::to_string(&process_data)
        .map_err(|_| JsValue::from_str("Failed to serialize process data"))?;

    // Convert the JSON string to a `JsValue`.
    let js_value = JsValue::from_str(&json);

    // Return the `JsValue`.
    Ok(js_value)
}

/// Parses the output of the `ps` command and returns a `ProcessData` instance.
///
/// This function iterates over each line of the `ps` command output, splits each line into whitespace-separated fields,
/// and attempts to create a `ProcessData` instance from these fields. If the line has more than 10 fields and the fields
/// can be successfully parsed into the appropriate data types, a `ProcessData` instance is created and returned.
///
/// # Arguments
///
/// * `output` - A string slice that holds the output of the `ps` command.
///
/// # Returns
///
/// * `Ok(ProcessData)` - If the function executes successfully, it returns a `ProcessData` instance.
/// * `Err(JsValue)` - If an error occurs during the execution of the function, it returns an `Err` result with a `JsValue` that contains the error message.
fn parse_ps_output(output: &str) -> Result<ProcessData, JsValue> {
    // Iterate over each line in the output
    for line in output.lines() {
        // Split the line into whitespace-separated fields
        let fields: Vec<&str> = line.split_whitespace().collect();
        // Check if the line has more than 10 fields
        if fields.len() > 10 {
            // Attempt to create a `ProcessData` instance from the fields
            let process_data = ProcessData::new(
                // Parse the first field into an integer
                fields[0]
                    .to_string()
                    .parse()
                    .map_err(|_| JsValue::from_str("Error parsing field[0]"))?,
                // Parse the third field into a float
                fields[2]
                    .parse()
                    .map_err(|_| JsValue::from_str("Error parsing CPU usage"))?,
                // Parse the fourth field into a float
                fields[3]
                    .parse()
                    .map_err(|_| JsValue::from_str("Error parsing memory usage"))?,
                // Join the remaining fields into a single string
                fields[10..].join(" "),
            );
            // Return the `ProcessData` instance
            return Ok(process_data);
        }
    }
    // If no data could be parsed, return an error
    Err(JsValue::from_str("No process data found"))
}
