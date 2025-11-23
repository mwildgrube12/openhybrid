use eframe::App;
use openhybrid::sim::{DummySim, EulerSim};

fn main() -> eframe::Result {
    println!("Hello, world!");
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };

    let sim = DummySim{ width: 100, height: 100 };

    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(sim) as Box<dyn App>)),
    )
}
