use crate::layout::{Draw, Measure};
use egui::text::TextWrapping;
use egui::{Align, FontSelection, Galley, Rect, Ui, Vec2, WidgetText, epaint};
use std::sync::Arc;

pub struct Text {
    text: WidgetText,
    wrapping: Option<TextWrapping>,
}

impl Text {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        Self {
            text: text.into(),
            wrapping: Some(TextWrapping::default()),
        }
    }
}

impl Measure for Text {
    type Measured = MeasuredText;

    fn measure(self, max_size: Vec2, ui: &Ui) -> (Vec2, MeasuredText) {
        let mut layout_job =
            self.text
                .into_layout_job(ui.style(), FontSelection::Default, Align::Min);

        if let Some(mut wrapping) = self.wrapping {
            wrapping.max_width = max_size.x;
            layout_job.wrap = wrapping;
        }

        let galley = ui.fonts(|it| it.layout_job(layout_job));

        (galley.size(), MeasuredText { galley })
    }
}

pub struct MeasuredText {
    galley: Arc<Galley>,
}

impl Draw for MeasuredText {
    fn draw(self, rect: Rect, ui: &mut Ui) {
        let text_color = ui.style().visuals.text_color();

        ui.painter()
            .add(epaint::TextShape::new(rect.min, self.galley, text_color));
    }
}
