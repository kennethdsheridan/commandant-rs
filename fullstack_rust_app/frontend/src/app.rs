struct MyApp {
    // Your state here
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
            // Build your UI here
        });
    }
}
