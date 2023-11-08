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

pub struct FunctionInputConfig {
    input_type: ParamTypes
}

pub struct FunctionInput {
    config: FunctionInputConfig
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
        let outer_response = ui.allocate_rect(
            Rect { min: self.config.position, max: Pos2 { 
                x: self.config.position.x + self.config.sizes.x, 
                y: self.config.position.y + self.config.sizes.y 
            }},
            Sense::drag()
        );

        if ui.is_rect_visible(outer_response.rect) {
            let stroke = ui.visuals().widgets.hovered.bg_stroke;
            let painter = ui.painter();

            if outer_response.hovered() {
                ui.output_mut(|o| { o.cursor_icon = CursorIcon::Grab })
            }

            if outer_response.dragged() || outer_response.drag_released() {
                ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

                if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                    let delta = pointer_pos - outer_response.rect.center();
                    let moved_rect = outer_response.rect.translate(delta);

                    painter.rect(
                        moved_rect, 
                        Rounding::same(5.0), 
                        Color32::from_rgb(195, 255, 104), 
                        stroke
                    );

                    painter.rect( 
                        Rect { 
                            min: Pos2 {
                                x: moved_rect.min.x, 
                                y: moved_rect.min.y + 5.0, 
                            },
                            max: Pos2 {
                                x: moved_rect.min.x + 10.0, 
                                y: moved_rect.min.y + 5.0 + 15.0, 
                            }
                        }, 
                        Rounding::ZERO, Color32::from_rgb(128, 0, 0), stroke
                    );

                    let moved_response = ui.allocate_rect(moved_rect, Sense::drag());

                    self.config.position = moved_rect.left_top();

                    return moved_response;
                }
            }
        
            painter.rect(
                outer_response.rect, 
                Rounding::same(5.0), 
                Color32::from_rgb(195, 255, 104), 
                stroke
            );

            painter.rect( 
                Rect { 
                    min: Pos2 {
                        x: outer_response.rect.min.x, 
                        y: outer_response.rect.min.y + 5.0, 
                    },
                    max: Pos2 {
                        x: outer_response.rect.min.x + 10.0, 
                        y: outer_response.rect.min.y + 5.0 + 15.0, 
                    }
                }, 
                Rounding::ZERO, Color32::from_rgb(128, 0, 0), stroke
            );
        }

        outer_response
    }
}
