use std::cmp::max;

pub use runnable::{Runnable, ParamTypes};
use egui::{widgets::Widget, Sense, Rounding, Color32, Rect, Pos2, Vec2, Response, Window};
use std::collections::HashMap;

pub struct FunctionConfig {
    pub runnable: Runnable,
    pub position: Pos2,
    pub sizes: Vec2,
    pub is_collapsed: bool,
    pub is_open: bool,
}

impl Default for FunctionConfig {
    fn default() -> FunctionConfig {
        let default_runnable = Runnable {
            name: "Function #0".to_owned(),
            inputs: HashMap::from([
                ("Input1".into(), ParamTypes::String),
                ("Input2".into(), ParamTypes::Number),
                ("Input3".into(), ParamTypes::Bool),
            ]),
            outputs: HashMap::from([
                ("Output1".into(), ParamTypes::String),
                ("Output2".into(), ParamTypes::Bool),
            ]),
        };

        FunctionConfig::new(default_runnable, Pos2 {x: 120.0, y: 40.0}, false, true)
    }
}

impl FunctionConfig {
    pub fn default_with_pos(initial_pos: Pos2, name: String) -> Self {
        let mut def = FunctionConfig::default();
        def.runnable.name = name;
        def.position = initial_pos;
        def
    }

    pub fn new(runnable: Runnable, initial_pos: Pos2, is_collapsed: bool, is_open: bool) -> Self {
        Self { 
            position: initial_pos, 
            sizes: Vec2 { 
                x: 30.0, 
                y: 
                    5.0 + // place for angle radius
                    max(runnable.inputs.len(), runnable.outputs.len()) as f32 * 15.0
            },
            runnable,
            is_collapsed,
            is_open
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
    
        ui.painter().circle( 
            entry_response.rect.center(), 
            5.0,
            Color32::from_rgb(128, 0, 0), 
            stroke
        );
        entry_response
    }

    fn render(&mut self, ui: &mut egui::Ui, window_rect: Rect, stroke: egui::Stroke) {
        for (idx, el) in self.config.runnable.inputs.iter().enumerate() {
            self.render_entry(ui, window_rect, idx, stroke, true);        
        }

        for (idx, el) in self.config.runnable.outputs.iter().enumerate() {
            let output_response = self.render_entry(ui, window_rect, idx, stroke, false);        

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
        let window_response = Window::new(&self.config.runnable.name)
            .auto_sized()
            .collapsible(true)
            .show(ui.ctx(), |ui| {

            ui.horizontal(|ui| { 
                ui.vertical(|ui| {
                    for ele in self.config.runnable.inputs.iter() {
                        ui.label(ele.0.clone());
                    }
                });
                ui.vertical(|ui| {
                    for ele in self.config.runnable.outputs.iter() {
                        ui.label(ele.0.clone());
                    }
                });
            })
        }).unwrap();
        
        let stroke = ui.visuals().widgets.hovered.bg_stroke;

        let window_rect = window_response.response.rect;
        self.render(ui, window_rect, stroke);

        window_response.response
    }
}