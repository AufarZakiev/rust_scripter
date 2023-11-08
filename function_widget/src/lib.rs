use std::cmp::max;

pub use runnable::{Runnable, ParamTypes};
use egui::{widgets::Widget, Sense, Rounding, Color32, CursorIcon, Rect, Pos2, Vec2};

pub struct FunctionConfig {
    pub runnable: Runnable,
    pub position: Pos2,
    pub sizes: Vec2,
}

impl FunctionConfig {
    pub fn new(runnable: Runnable, initial_pos: Pos2) -> Self {
        Self { 
            position: initial_pos, 
            sizes: Vec2 { 
                x: 30.0, 
                y: 
                    5.0 + // place for angle radius
                    max(runnable.inputs.len(), runnable.outputs.len()) as f32 * 15.0 +
                    5.0 // place for angle radius
            },
            runnable
        }
    }
}

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct FunctionWidget<'a> {
    pub config: &'a mut FunctionConfig
}

impl<'a> FunctionWidget<'a> {
    pub fn new(config: &'a mut FunctionConfig) -> Self {
        Self { 
            config
        }
    }
}

impl Widget for &mut FunctionWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let response = ui.allocate_rect(
            Rect { min: self.config.position, max: Pos2 { 
                x: self.config.position.x + self.config.sizes.x, 
                y: self.config.position.y + self.config.sizes.y 
            }},
            Sense::drag()
        );

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
                        Rounding::same(5.0), 
                        Color32::from_rgb(195, 255, 104), 
                        stroke
                    );

                    let moved_response = ui.allocate_rect(moved_rect, Sense::drag());

                    self.config.position = moved_rect.left_top();

                    return moved_response;
                }
            }
        
            painter.rect(
                response.rect, 
                Rounding::same(5.0), 
                Color32::from_rgb(195, 255, 104), 
                stroke
            )
        }

        response
    }
}
