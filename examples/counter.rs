use eframe::epaint::Margin;
use egui::{CentralPanel, Context};

struct App {
    counter1: i32,
    counter2: i32,
}

impl App {
    fn new() -> Self {
        Self {
            counter1: 0,
            counter2: 0,
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
                ui.heading("Counter App");

                ui.label(format!("Counter 1: {}", self.counter1));
                ui.label(format!("Counter 2: {}", self.counter2));

                let disabled = self.counter1 > 10 && self.counter2 > 10;

                let status = if disabled {
                    "Status: Max count hit"
                } else if self.counter1 % 5 == 0 || self.counter2 % 5 == 0 {
                    "Status: Milestone reached!"
                } else {
                    "Status: Counting..."
                };

                ui.label(status);

                if ui
                    .add_enabled(!disabled, egui::Button::new("Increment Counter 1"))
                    .clicked()
                {
                    self.counter1 += 1;
                }
                if ui
                    .add_enabled(!disabled, egui::Button::new("Increment Counter 2"))
                    .clicked()
                {
                    self.counter2 += 1;
                }
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native("button", options, Box::new(|cc| Ok(Box::new(App::new()))))?;

    Ok(())
}
