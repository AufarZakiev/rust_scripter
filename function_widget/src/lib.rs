use std::cmp::max;

pub use runnable::{Runnable, ParamTypes};
use egui::{widgets::Widget, Sense, Rounding, Color32, Rect, Pos2, Vec2, Response, Window, Order, Area, LayerId, Id, text::LayoutJob, TextFormat, Align, RichText};

pub struct FunctionConfig {
    pub runnable: Runnable,
    pub position: Pos2,
    pub sizes: Vec2,
    pub is_open: bool,
}

impl Default for FunctionConfig {
    fn default() -> FunctionConfig {
        let default_runnable = Runnable::default();

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
}

impl Widget for &mut FunctionWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let window_response = Window::new(&self.config.runnable.name)
            .fixed_size(Vec2 {x: 160.0, y: self.config.runnable.inputs.len() as f32 * 10.0 + 10.0})
            .open(&mut self.config.is_open)
            .collapsible(true)
            .show(ui.ctx(), |ui| {                
                let circle_painter = ui.ctx()
                    .layer_painter(LayerId::new(Order::Foreground, Id::new(self.config.runnable.name.clone())));
                
                let stroke = ui.visuals().widgets.hovered.bg_stroke;
                
                ui.columns(2, |columns| {
                    for ele in self.config.runnable.inputs.iter() {
                        let label_response = columns[0].label(ele.0.clone());

                        let circle_rect = Rect::from_center_size(
                            label_response.rect.left_center() + Vec2 { x: -7.0, y: 0.0 },
                            Vec2 { x: 5.0, y: 5.0 }
                        );
                        circle_painter.circle(
                            circle_rect.center(),
                            5.0,
                            Color32::from_rgb(128, 0, 0), 
                            stroke
                        );

                        if let Some(pointer_pos) = columns[0].ctx().pointer_interact_pos() {
                            if circle_rect.contains(pointer_pos) {
                                let tooltip_pos = circle_rect.right_bottom() + Vec2{x: 4.0, y: 4.0};
                    
                                columns[0].painter().error(
                                    tooltip_pos,
                                    "Click to start drawing a connection"
                                );
                            }
                        }
                    };
                    for ele in self.config.runnable.outputs.iter() {
                        let label_response = columns[1].with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                            ui.label(ele.0.clone());
                        });

                        let circle_rect = Rect::from_center_size(
                            label_response.response.rect.right_center() + Vec2 { x: 7.0, y: 0.0 },
                            Vec2 { x: 5.0, y: 5.0 }
                        );
                        circle_painter.circle(
                            circle_rect.center(),
                            5.0,
                            Color32::from_rgb(128, 0, 0), 
                            stroke
                        );

                        if let Some(pointer_pos) = columns[1].ctx().pointer_interact_pos() {
                            if circle_rect.contains(pointer_pos) {
                                let tooltip_pos = circle_rect.right_bottom() + Vec2{x: 4.0, y: 4.0};
                    
                                columns[1].painter().error(
                                    tooltip_pos,
                                    "Click to start drawing a connection"
                                );
                            }
                        }
                    }
                })
        }).unwrap();

        window_response.response
    }
}