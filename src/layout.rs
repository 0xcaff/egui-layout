use egui::{Rect, Ui, Vec2, vec2};

pub trait Draw {
    fn draw(self, region: Rect, ui: &mut Ui);
}

trait DrawDyn {
    fn draw(self: Box<Self>, region: Rect, ui: &mut Ui);
}

impl<T: Draw + 'static> DrawDyn for T {
    fn draw(self: Box<Self>, region: Rect, ui: &mut Ui) {
        (*self).draw(region, ui);
    }
}

pub trait Measure {
    type Measured: Draw;

    fn measure(self, max_size: Vec2, ui: &Ui) -> (Vec2, Self::Measured);
}

trait MeasureDyn {
    fn measure(self: Box<Self>, max_size: Vec2, ui: &Ui) -> (Vec2, Box<dyn DrawDyn>);
}

impl<T: Measure + 'static> MeasureDyn for T {
    fn measure(self: Box<Self>, max_size: Vec2, ui: &Ui) -> (Vec2, Box<dyn DrawDyn>) {
        let (vec, measured) = (*self).measure(max_size, ui);
        (vec, Box::new(measured))
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum LayoutDirection {
    Row,
    Column,
}

impl LayoutDirection {
    fn main_axis(&self, vec2: Vec2) -> f32 {
        match self {
            LayoutDirection::Row => vec2.x,
            LayoutDirection::Column => vec2.y,
        }
    }

    fn cross_axis(&self, vec2: Vec2) -> f32 {
        match self {
            LayoutDirection::Row => vec2.y,
            LayoutDirection::Column => vec2.x,
        }
    }
}

trait Vec2Ext {
    fn with_main_axis(self, direction: LayoutDirection, value: f32) -> Vec2;
    fn with_cross_axis(self, direction: LayoutDirection, value: f32) -> Vec2;
    fn main_axis(self, direction: LayoutDirection) -> f32;
    fn cross_axis(self, direction: LayoutDirection) -> f32;
}

impl Vec2Ext for Vec2 {
    fn with_main_axis(self, direction: LayoutDirection, value: f32) -> Vec2 {
        match direction {
            LayoutDirection::Row => Vec2::new(value, self.y),
            LayoutDirection::Column => Vec2::new(self.x, value),
        }
    }

    fn with_cross_axis(self, direction: LayoutDirection, value: f32) -> Vec2 {
        match direction {
            LayoutDirection::Row => Vec2::new(self.x, value),
            LayoutDirection::Column => Vec2::new(value, self.y),
        }
    }

    fn main_axis(self, direction: LayoutDirection) -> f32 {
        match direction {
            LayoutDirection::Row => self.x,
            LayoutDirection::Column => self.y,
        }
    }

    fn cross_axis(self, direction: LayoutDirection) -> f32 {
        match direction {
            LayoutDirection::Row => self.y,
            LayoutDirection::Column => self.x,
        }
    }
}

pub enum Alignment {
    Start,
    End,
    Center,
}

pub struct LayoutParams {
    pub direction: LayoutDirection,
    pub main_axis_alignment: Alignment,
    pub cross_axis_alignment: Alignment,
}

impl Default for LayoutParams {
    fn default() -> Self {
        LayoutParams {
            direction: LayoutDirection::Row,
            main_axis_alignment: Alignment::Center,
            cross_axis_alignment: Alignment::Start,
        }
    }
}

pub struct Layout {
    params: LayoutParams,
    children: Vec<Box<dyn MeasureDyn>>,
}

impl Layout {
    pub fn new(params: LayoutParams) -> Self {
        Self {
            children: vec![],
            params,
        }
    }

    pub fn with_child(mut self, child: impl Measure + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl Measure for Layout {
    type Measured = MeasuredLayout;

    fn measure(self, max_size: Vec2, ui: &Ui) -> (Vec2, Self::Measured) {
        let child_size = match self.params.direction {
            LayoutDirection::Row => vec2(max_size.x / self.children.len() as f32, max_size.y),
            LayoutDirection::Column => vec2(max_size.x, max_size.y / self.children.len() as f32),
        };

        let measured_children: Vec<_> = self
            .children
            .into_iter()
            .map(|it| it.measure(child_size, ui))
            .collect();

        let bounding_box = match self.params.direction {
            LayoutDirection::Row => vec2(
                measured_children.iter().map(|it| it.0.x).sum(),
                max_partial(measured_children.iter().map(|it| it.0.y)).unwrap(),
            ),
            LayoutDirection::Column => vec2(
                max_partial(measured_children.iter().map(|it| it.0.x)).unwrap(),
                measured_children.iter().map(|it| it.0.y).sum(),
            ),
        };

        (bounding_box, MeasuredLayout {
            params: self.params,
            children: measured_children,
        })
    }
}

pub struct MeasuredLayout {
    params: LayoutParams,
    children: Vec<(Vec2, Box<dyn DrawDyn>)>,
}

impl Draw for MeasuredLayout {
    fn draw(mut self, available_space: Rect, ui: &mut Ui) {
        let calculate_cross_axis =
            |cross_axis_size: f32, element_size: f32| match self.params.cross_axis_alignment {
                Alignment::Start => 0.,
                Alignment::End => cross_axis_size - element_size,
                Alignment::Center => (cross_axis_size - element_size) / 2.0,
            };

        let rects: Vec<_> = match self.params.main_axis_alignment {
            Alignment::Start => self
                .children
                .into_iter()
                .scan(0_f32, |current_main_axis, (measurement, child)| {
                    let rect = Rect::from_min_size(
                        available_space.min
                            + Vec2::ZERO
                                .with_main_axis(self.params.direction, *current_main_axis)
                                .with_cross_axis(
                                    self.params.direction,
                                    calculate_cross_axis(
                                        available_space.size().cross_axis(self.params.direction),
                                        measurement.cross_axis(self.params.direction),
                                    ),
                                ),
                        measurement,
                    );

                    *current_main_axis += measurement.main_axis(self.params.direction);

                    Some((rect, child))
                })
                .collect(),
            Alignment::End => {
                let consumed_space: f32 = self
                    .children
                    .iter()
                    .map(|it| self.params.direction.main_axis(it.0))
                    .sum();
                let spacing_width =
                    self.params.direction.main_axis(available_space.size()) - consumed_space;

                self.children
                    .into_iter()
                    .scan(0_f32, |current_main_axis, (measurement, child)| {
                        let rect = Rect::from_min_size(
                            available_space.min
                                + Vec2::ZERO
                                    .with_main_axis(
                                        self.params.direction,
                                        *current_main_axis + spacing_width,
                                    )
                                    .with_cross_axis(
                                        self.params.direction,
                                        calculate_cross_axis(
                                            available_space
                                                .size()
                                                .cross_axis(self.params.direction),
                                            measurement.cross_axis(self.params.direction),
                                        ),
                                    ),
                            measurement,
                        );

                        *current_main_axis += measurement.main_axis(self.params.direction);

                        Some((rect, child))
                    })
                    .collect()
            }
            Alignment::Center => {
                let consumed_space: f32 = self
                    .children
                    .iter()
                    .map(|it| self.params.direction.main_axis(it.0))
                    .sum();
                let spacing_size =
                    self.params.direction.main_axis(available_space.size()) - consumed_space;
                let starting_offset = (spacing_size / 2.0);

                self.children
                    .into_iter()
                    .scan(0_f32, |current_main_axis, (measurement, child)| {
                        let rect = Rect::from_min_size(
                            available_space.min
                                + Vec2::ZERO
                                    .with_main_axis(
                                        self.params.direction,
                                        *current_main_axis + starting_offset,
                                    )
                                    .with_cross_axis(
                                        self.params.direction,
                                        calculate_cross_axis(
                                            available_space
                                                .size()
                                                .cross_axis(self.params.direction),
                                            measurement.cross_axis(self.params.direction),
                                        ),
                                    ),
                            measurement,
                        );

                        *current_main_axis += measurement.main_axis(self.params.direction);

                        Some((rect, child))
                    })
                    .collect()
            }
        };

        for (rect, child) in rects {
            let mut ui = ui.child_ui(rect, egui::Layout::default(), None);

            child.draw(rect, &mut ui);
        }
    }
}

fn max_partial<T: PartialOrd>(values: impl Iterator<Item = T>) -> Option<T> {
    values.fold(None, |max, x| match max {
        Some(m) if m.partial_cmp(&x) == Some(std::cmp::Ordering::Greater) => Some(m),
        _ => Some(x),
    })
}
