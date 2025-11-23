use eframe::egui;
use egui::{Color32, Rangef, Sense, Shape, Stroke, Vec2};

use crate::sim::EulerSim;

impl eframe::App for dyn EulerSim {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::from_rgb(78, 97, 128)))
            .show(ctx, |ui| ());

        egui::Area::new(egui::Id::new("Center Area"))
            .fixed_pos(egui::pos2(50.0, 50.0))
            .default_size(egui::vec2(1000.0, 1000.0))
            .show(ctx, |ui| {
                ui.set_max_size(egui::vec2(1000.0, 1000.0).min(ctx.viewport_rect().size()) );

                let (resp, painter) = ui.allocate_painter(
                    Vec2::new(ui.available_width(), 
                        ui.available_height()), 
                    Sense::empty());
                painter.rect_filled(
                    ui.max_rect(),
                    egui::CornerRadius::default(), 
                    Color32::DARK_GRAY);
                add_grid((50,50), painter, ui.max_rect(), 2.0, Color32::WHITE);
            });
    }
}

fn add_grid(dims: (usize, usize), painter: egui::Painter, area: egui::Rect, width: f32, color: Color32) {
    let x_step = area.x_range().span() / (dims.0 as f32 - 1.0);
    let y_step = area.y_range().span() / (dims.1 as f32 - 1.0);

    for x in 0..dims.0 {
        painter.add(Shape::vline(
                    (x as f32) * x_step + area.min.x, 
                    area.y_range(), 
                    Stroke::new(width, color)));
    } 
    for y in 0..dims.1 {
        painter.add(Shape::hline(
                area.x_range(), 
                (y as f32) * y_step + area.min.y, 
                Stroke::new(width, color)));  
    }
}

#[macro_export]
macro_rules! impl_app {
    ($S:ty) => {
        impl eframe::App for $S {
            fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
                (self as &mut dyn EulerSim).update(ctx, frame)
            }
        } 
    };
}