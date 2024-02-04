use wasm_bindgen::prelude::*;

// Define a struct to hold process data for demonstration purposes.
// In a real-world scenario, this would be replaced with appropriate data structures.
#[wasm_bindgen]
#[derive(Debug)]
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

// A helper function to parse the `ps` command output and create a `ProcessData` instance.
// This function is private and not exposed to JavaScript.
fn parse_ps_output(output: &str) -> Result<ProcessData, JsValue> {
    // Split the output into lines and iterate over them.
    // In a real-world scenario, you would have more complex parsing logic here.
    for line in output.lines() {
        // Split each line into whitespace-separated fields.
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() > 10 {
            // Create a `ProcessData` instance from the fields.
            let process_data = ProcessData::new(
                fields[0].to_string().parse().unwrap(),
                fields[2]
                    .parse()
                    .map_err(|_| JsValue::from_str("Error parsing CPU usage"))?,
                fields[3]
                    .parse()
                    .map_err(|_| JsValue::from_str("Error parsing memory usage"))?,
                fields[10..].join(" "),
            );
            // Return the `ProcessData` instance.
            return Ok(process_data);
        }
    }
    // If no data could be parsed, return an error.
    Err(JsValue::from_str("No process data found"))
}
