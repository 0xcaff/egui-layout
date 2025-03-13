use eframe::epaint::Margin;
use egui::{CentralPanel, Context, Id, RichText, TextEdit};
use egui_layout::layout::{Alignment, Draw, Layout, LayoutDirection, LayoutParams, Measure};
use egui_layout::widgets::frame::Frame;
use egui_layout::widgets::lazy::LazyMeasuredWidget;
use egui_layout::widgets::text::Text;

struct App;

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        CentralPanel::default()
            .frame(egui::Frame {
                fill: ctx.style().visuals.panel_fill,
                inner_margin: Margin::symmetric(8, 4),
                ..Default::default()
            })
            .show(ctx, |ui| {
                Layout::new(LayoutParams {
                    direction: LayoutDirection::Column,
                    main_axis_alignment: Alignment::Center,
                    cross_axis_alignment: Alignment::Center,
                })
                    // .with_child(Text::new(RichText::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.")))
                    .with_child(Text::new("short"))
                    .measure(ui.available_size(), ui)
                    .1
                    .draw(ui.available_rect_before_wrap(), ui);
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native("text", options, Box::new(|cc| Ok(Box::new(App))))?;

    Ok(())
}
