use eframe::epaint::Margin;
use egui::{CentralPanel, Context, RichText};
use egui_layout::layout::{Alignment, Draw, Layout, LayoutDirection, LayoutParams, Measure};
use egui_layout::widgets::frame::Frame;
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
                    main_axis_alignment: Alignment::Start,
                    cross_axis_alignment: Alignment::Center,
                })
                    .with_child(Text::new(RichText::from("extern_traces").heading()))
                    .with_child(Text::new(
                        {
                            let version = env!("CARGO_PKG_VERSION");
                            let git_short_sha = "7642d77";

                            format!("v{version} @ {git_short_sha}")
                        }
                    ))
                    .with_child(Frame::new(
                        Layout::new(LayoutParams {
                            direction: LayoutDirection::Column,
                            main_axis_alignment: Alignment::Center,
                            ..Default::default()
                        })
                            .with_child(Text::new("more text"))
                            .with_child(Text::new("more text 22")),
                    ))
                    .with_child(Frame::new(
                        Layout::new(LayoutParams {
                            direction: LayoutDirection::Column,
                            main_axis_alignment: Alignment::Center,
                            ..Default::default()
                        })
                            .with_child(Text::new("more text"))
                            .with_child(Text::new("more text 22 hello world rhs")),
                    ))
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

    eframe::run_native(
        "extern_traces",
        options,
        Box::new(|cc| {
            Ok(Box::new(App))
        }),
    )?;

    Ok(())
}
