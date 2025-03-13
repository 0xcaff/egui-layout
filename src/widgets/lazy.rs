use egui::{Rect, Ui, Vec2};
use crate::layout::{Draw, Measure};

pub struct LazyMeasuredWidget<W> {
    widget: W,
    id: egui::Id,
}

#[derive(Clone)]
struct LazyMeasuredWidgetState {
    last_size: Vec2,
}

impl<W: egui::Widget> Measure for LazyMeasuredWidget<W> {
    type Measured = LazyMeasuredWidgetMeasured<W>;

    fn measure(self, max_size: Vec2, ui: &Ui) -> (Vec2, Self::Measured) {
        if let Some(widget) = ui
            .ctx()
            .data(|r| r.get_temp::<LazyMeasuredWidgetState>(ui.id()))
        {
            return (widget.last_size, LazyMeasuredWidgetMeasured {
                widget: self.widget,
                id: self.id,
                skip_render: false,
            });
        }

        (max_size, LazyMeasuredWidgetMeasured {
            widget: self.widget,
            id: self.id,
            skip_render: true,
        })
    }
}

pub struct LazyMeasuredWidgetMeasured<W> {
    widget: W,
    id: egui::Id,
    skip_render: bool,
}

impl<W: egui::Widget> Draw for LazyMeasuredWidgetMeasured<W> {
    fn draw(self, _region: Rect, ui: &mut Ui) {
        let response = self.widget.ui(ui);
        let size = response.rect;

        ui.ctx().data_mut(|it| {
            it.insert_temp(self.id, LazyMeasuredWidgetState {
                last_size: size.size(),
            })
        });

        if self.skip_render {
            ui.ctx().request_discard("layout");
        }
    }
}

// todo: need a way to host regular egui components as children
// todo: interactive components like buttons and text edit
