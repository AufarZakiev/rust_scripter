use std::cmp::max;

pub use runnable::{Runnable, ParamTypes};
use egui::{widgets::Widget, vec2, Sense, Rounding, Color32};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct FunctionWidget {
    function: Runnable,
}

impl FunctionWidget {
    pub fn new(function: Runnable) -> Self {
        Self { function }
    }

    pub fn function(mut self, function: Runnable) -> Self {
        self.function = function;
        self
    }
}

impl Widget for FunctionWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let FunctionWidget {
            function
        } = self;

        let inputs = function.inputs;
        let outputs = function.outputs;

        let size = vec2(
            30.0,
            max(inputs.len(), outputs.len()) as f32 * 15.0,
        );

        let (_, response) = ui.allocate_at_least(size, Sense::drag());

        if ui.is_rect_visible(response.rect) {
            let stroke = ui.visuals().widgets.hovered.bg_stroke;
            let painter = ui.painter();

            painter.rect(
                response.rect, 
                Rounding::same(1.0), 
                Color32::from_rgb(195, 255, 104), 
                stroke
            )
        }

        response
    }
}
