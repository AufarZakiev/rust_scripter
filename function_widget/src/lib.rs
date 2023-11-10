use std::cmp::max;

pub use runnable::{Runnable, ParamTypes};
use egui::{widgets::Widget, Sense, Rounding, Color32, CursorIcon, Rect, Pos2, Vec2, Response};

pub struct FunctionConfig {
    pub runnable: Runnable,
    pub position: Pos2,
    pub sizes: Vec2,
}

impl Default for FunctionConfig {
    fn default() -> FunctionConfig {
        let default_runnable = Runnable {
            inputs: vec![
                ("Input1".into(), ParamTypes::String),
                ("Input2".into(), ParamTypes::Number),
                ("Input3".into(), ParamTypes::Bool),
            ],
            outputs: vec![
                ("Output1".into(), ParamTypes::String),
                ("Output2".into(), ParamTypes::Bool),
            ],
        };

        FunctionConfig::new(default_runnable, Pos2 {x: 120.0, y: 40.0})
    }
}

impl FunctionConfig {
    pub fn default_with_pos( initial_pos: Pos2) -> Self {
        let mut def = FunctionConfig::default();
        def.position = initial_pos;
        def
    }

    pub fn new(runnable: Runnable, initial_pos: Pos2) -> Self {
        Self { 
            position: initial_pos, 
            sizes: Vec2 { 
                x: 30.0, 
                y: 
                    5.0 + // place for angle radius
                    max(runnable.inputs.len(), runnable.outputs.len()) as f32 * 15.0
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

impl<'a> FunctionWidget<'a> {
    fn render_entry(&self, ui: &mut egui::Ui, moved_rect: Rect, idx: usize, stroke: egui::Stroke, is_input: bool) -> Response{
        let entry_response = ui.allocate_rect(Rect { 
            min: Pos2 {
                x: moved_rect.min.x + if is_input { 0.0 } else { 20.0 }, 
                y: moved_rect.min.y + 5.0 + (idx as f32 * 15.0), 
            },
            max: Pos2 {
                x: moved_rect.min.x + 10.0 + if is_input { 0.0 } else { 20.0 }, 
                y: moved_rect.min.y + 5.0 + 10.0 + (idx as f32 * 15.0), 
            }
        }, Sense::click());
    
        ui.painter().rect( 
            entry_response.rect, 
            Rounding::ZERO, Color32::from_rgb(128, 0, 0), stroke
        );
        entry_response
    }

    fn render(&mut self, ui: &mut egui::Ui, moved_rect: Rect, stroke: egui::Stroke) {
        ui.painter().rect(
            moved_rect, 
            Rounding::same(5.0), 
            Color32::from_rgb(195, 255, 104), 
            stroke
        );

        for (idx, el) in self.config.runnable.inputs.iter().enumerate() {
            self.render_entry(ui, moved_rect, idx, stroke, true);        
        }

        for (idx, el) in self.config.runnable.outputs.iter().enumerate() {
            let output_response = self.render_entry(ui, moved_rect, idx, stroke, false);        

            inject_tooltips(ui, output_response, stroke);
        }
    }
}

fn inject_tooltips(ui: &mut egui::Ui, output_response: Response, stroke: egui::Stroke) {
    if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
        if output_response.rect.contains(pointer_pos) {
            let tooltip_pos = output_response.rect.right_bottom() + Vec2{x: 4.0, y: 4.0};

            ui.painter().error(
                tooltip_pos,
                "Click to start drawing a connection"
            );
        }
    }

    ui.painter().rect( 
        output_response.rect, 
        Rounding::ZERO, Color32::from_rgb(128, 0, 0), stroke
    );
}

impl Widget for &mut FunctionWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let outer_response = ui.allocate_rect(
            Rect { min: self.config.position, max: Pos2 { 
                x: self.config.position.x + self.config.sizes.x, 
                y: self.config.position.y + self.config.sizes.y 
            }},
            Sense::click_and_drag()
        );

        if ui.is_rect_visible(outer_response.rect) {
            let stroke = ui.visuals().widgets.hovered.bg_stroke;

            if outer_response.hovered() {
                ui.output_mut(|o| { o.cursor_icon = CursorIcon::Grab })
            }

            if outer_response.dragged_by(egui::PointerButton::Primary) {
                ui.ctx().set_cursor_icon(CursorIcon::Grabbing);

                if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                    let delta = pointer_pos - outer_response.rect.center();
                    let moved_rect = outer_response.rect.translate(delta);

                    self.render(ui, moved_rect, stroke);
                    
                    let moved_response = ui.allocate_rect(moved_rect, Sense::drag());

                    self.config.position = moved_rect.left_top();

                    return moved_response;
                }
            }
        
            self.render(ui, outer_response.rect, stroke)
        }

        outer_response
    }
}