use crate::layout::{Draw, Measure};
use egui::{Rect, Ui, Vec2};

pub struct Frame<Child> {
    child: Child,
}

impl<Child: Measure> Frame<Child> {
    pub fn new(child: Child) -> Self {
        Self { child }
    }
}

impl<Child: Measure> Measure for Frame<Child> {
    type Measured = MeasuredFrame<Child::Measured>;

    fn measure(self, max_size: Vec2, ui: &Ui) -> (Vec2, Self::Measured) {
        let (_ignored_child_size, child) = self.child.measure(max_size, ui);

        (max_size, MeasuredFrame { child })
    }
}

pub struct MeasuredFrame<Child> {
    child: Child,
}

impl<Child: Draw> Draw for MeasuredFrame<Child> {
    fn draw(self, region: Rect, ui: &mut Ui) {
        self.child.draw(region, ui);
    }
}
