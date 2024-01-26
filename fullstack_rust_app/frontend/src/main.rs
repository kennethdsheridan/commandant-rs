mod adapters;
mod ports;

// Defines the `MyApp` struct which will represent our application.
struct MyApp {
    // A field of the struct
    some_field: String,
}

// Implementation of the Default trait for MyApp.
// The Default trait allows us to create a “default value” for a type.
impl Default for MyApp {
    // When `default()` is called, a new instance of `MyApp` will be returned
    // with `some_field` set to "Default Value".
    fn default() -> Self {
        MyApp {
            some_field: "Default Value".to_string(),
        }
    }
}

// Implements the App trait from the eframe crate for `MyApp`
// This trait provides a framework for building egui applications.
impl eframe::App for MyApp {
    // This `update` method is called on each frame of the application's life cycle.
    // It takes a mutable reference to self, a reference to an egui `Context`, and
    // a mutable reference to an eframe `Frame`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // A UI panel is created and shown in the center of the screen.
        // The `show` method is passed a lambda function that modifies the UI.
        egui::CentralPanel::default().show(ctx, |ui| {
            // Adds a label to the UI with the content of `some_field`.
            ui.label(&self.some_field);
        });
    }
}

// Main function initializes and runs the application.
fn main() {
    // Prepare the default options for the native application.
    let options = eframe::NativeOptions::default();

    // Run the native application with specified options, supply `MyApp` struct instance,
    // which has been wrapped in Box for heap allocation, and wrap that in Box to satisfy the function signature
    eframe::run_native(
        "My Egui Application",                   // Set the name of the application
        options,                                 // Set the options
        Box::new(|_cc| Box::<MyApp>::default()), // Create a new instance of `MyApp` and run it
    )
    // If the application fails to run, display an error message.
    .expect("Failed to run native application");
}
