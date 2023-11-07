use std::cmp::max;

pub use runnable::{Runnable, ParamTypes};
use egui::{widgets::Widget, Sense, Rounding, Color32, CursorIcon, Rect, Pos2};

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct FunctionWidget<'a> {
    pub rect: &'a mut Rect
}

impl<'a> FunctionWidget<'a> {
    pub fn new(rect: &'a mut Rect) -> Self {
        Self { 
            rect
        }
    }
}

impl Widget for &mut FunctionWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui.allocate_rect(*self.rect, Sense::drag());

        if ui.is_rect_visible(response.rect) {
            let stroke = ui.visuals().widgets.hovered.bg_stroke;
            let painter = ui.painter();

            if response.hovered() {
                ui.output_mut(|o| { o.cursor_icon = CursorIcon::Grab })
            }

            if response.dragged() || response.drag_released() {
                ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

                if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                    let delta = pointer_pos - response.rect.center();
                    let moved_rect = response.rect.translate(delta);

                    painter.rect(
                        moved_rect, 
                        Rounding::same(1.0), 
                        Color32::from_rgb(195, 255, 104), 
                        stroke
                    );

                    let moved_response = ui.allocate_rect(moved_rect, Sense::drag());

                    *self.rect = moved_rect;

                    return moved_response;
                }
            }
        
            painter.rect(
                *self.rect, 
                Rounding::same(1.0), 
                Color32::from_rgb(195, 255, 104), 
                stroke
            )
        }

        response
    }
}
