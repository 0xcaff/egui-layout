use eframe::epaint::Margin;
use egui::{CentralPanel, Context};

struct App {
    counter: i32,
}

impl App {
    fn new() -> Self {
        Self {
            counter: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(egui::Frame {
                fill: ctx.style().visuals.panel_fill,
                inner_margin: Margin::symmetric(8, 4),
                ..Default::default()
            })
            .show(ctx, |ui| {
                if ui.button("hello world").clicked() {
                    self.counter += 1;
                }

                for i in 0..self.counter {
                    ui.label(format!("hello {}", i));
                }
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native("simple_button", options, Box::new(|cc| Ok(Box::new(App::new()))))?;

    Ok(())
}
