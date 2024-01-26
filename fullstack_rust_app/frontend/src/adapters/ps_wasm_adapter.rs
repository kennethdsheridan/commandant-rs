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

// The `write_to_wasm` function takes the output of the `ps` command as a string
// and writes it to WASM storage. This function is exposed to JavaScript using the
// `#[wasm_bindgen]` attribute, allowing it to be called from the JavaScript context.
#[wasm_bindgen]
pub fn write_to_wasm(output: String) -> Result<(), JsValue> {
    // Parse the output of the `ps` command to extract process data.
    // This is a simplified example and assumes the output is in a specific format.
    let process_data = parse_ps_output(&output)?;

    // Here you would write the `process_data` to WASM storage.
    // The actual implementation of writing to WASM storage will depend on your specific use case.
    // For demonstration purposes, we'll just log the data to the console using `web_sys`.
    // See https://rustwasm.github.io/docs/wasm-bindgen/examples/console-log.html for more details.
    web_sys::console::log_1(&JsValue::from_str(&format!(
        "Process data: {:?}",
        process_data
    )));

    // Return `Ok` to indicate success.
    Ok(())
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
