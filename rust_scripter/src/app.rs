use egui::epaint::QuadraticBezierShape;
use egui::{epaint::CubicBezierShape, Key, Label, Pos2, Rect, Sense, Vec2};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tinyid::TinyId;

use crate::function_widget::function_widget::WidgetMode;
use crate::function_widget::function_widget::{FunctionWidget, LinkVertex};

#[derive(Deserialize, Serialize)]
struct Link {
    start: LinkVertex,
    end: LinkVertex,
    should_be_deleted: bool,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    functions: IndexMap<TinyId, FunctionWidget>,
    links: Vec<Link>,
    last_rect_id: usize,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let function1 = FunctionWidget::default();
        let function2 =
            FunctionWidget::default_with_pos(Pos2 { x: 180.0, y: 40.0 }, "Function #1".to_owned());
        let param_id1 = function1.runnable.outputs.get_index(0).unwrap().0.clone();
        let param_id2 = function2.runnable.inputs.get_index(0).unwrap().0.clone();

        Self {
            links: vec![Link {
                start: LinkVertex {
                    function_id: function1.id.clone(),
                    param_id: param_id1,
                },
                end: LinkVertex {
                    function_id: function2.id.clone(),
                    param_id: param_id2,
                },
                should_be_deleted: false,
            }],
            functions: IndexMap::from([(function1.id, function1), (function2.id, function2)]),
            last_rect_id: 3,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // cc.egui_ctx.set_debug_on_hover(true);

        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            if let Some(mut storage) = eframe::get_value::<TemplateApp>(storage, eframe::APP_KEY) {
                if let Some((_, last_rect)) = storage.functions.last() {
                    storage.last_rect_id = last_rect
                        .runnable
                        .name
                        .chars()
                        .last()
                        .unwrap()
                        .to_digit(10)
                        .unwrap() as usize
                        + 1;
                    return storage;
                }
            }
        }

        Default::default()
    }

    fn delete_old_links(&mut self) {
        self.links.retain(|link| !link.should_be_deleted);
    }

    fn render_links(&mut self, ui: &mut egui::Ui, stroke: egui::Stroke) {
        let collapsed_window_width = 160.0;

        for current_link in self.links.iter_mut() {
            let start_point_widget = self.functions.get(&current_link.start.function_id);
            let end_point_widget = self.functions.get(&current_link.end.function_id);

            if start_point_widget.is_none() || end_point_widget.is_none() {
                current_link.should_be_deleted = true;
                continue;
            };

            let start_point_widget = start_point_widget.unwrap();
            let end_point_widget = end_point_widget.unwrap();

            let start_param = start_point_widget
                .runnable
                .outputs
                .get(&current_link.start.param_id);

            let end_param = end_point_widget
                .runnable
                .inputs
                .get(&current_link.end.param_id);

            if start_param.is_none() || end_param.is_none() {
                current_link.should_be_deleted = true;
                continue;
            };

            let start_point = match (start_point_widget.is_collapsed, &start_point_widget.mode) {
                (true, _) => Pos2 {
                    x: start_point_widget.position.x + collapsed_window_width,
                    y: start_point_widget.position.y + 16.0,
                },
                (false, WidgetMode::Code) => Pos2 {
                    x: start_point_widget.position.x + start_point_widget.code_size.x,
                    y: start_point_widget.position.y + 16.0,
                },
                (false, WidgetMode::Signature) => start_param.unwrap().pos,
            };

            let end_point = match (end_point_widget.is_collapsed, &end_point_widget.mode) {
                (true, _) => Pos2 {
                    x: end_point_widget.position.x,
                    y: end_point_widget.position.y + 16.0,
                },
                (false, WidgetMode::Code) => Pos2 {
                    x: end_point_widget.position.x,
                    y: end_point_widget.position.y + 16.0,
                },
                (false, WidgetMode::Signature) => end_param.unwrap().pos,
            };

            let signum = (end_point.y - start_point.y).signum();
            match end_point.x - start_point.x {
                diff if diff <= 0.0 => {
                    {
                        let second_point = Pos2 {
                            x: start_point.x + 150.0,
                            y: start_point.y,
                        };
                        let third_point = Pos2 {
                            x: start_point.x + 150.0,
                            y: start_point.y + signum * 150.0,
                        };
                        let end_point = Pos2 {
                            x: start_point.x,
                            y: start_point.y + signum * 150.0,
                        };

                        let points: [Pos2; 4] = [start_point, second_point, third_point, end_point];
                        let curve = CubicBezierShape::from_points_stroke(
                            points,
                            false,
                            Default::default(),
                            stroke,
                        );
                        ui.painter().add(curve);
                    }
                    {
                        let curve = CubicBezierShape::from_points_stroke(
                            [
                                Pos2 {
                                    x: start_point.x,
                                    y: start_point.y + signum * 150.0,
                                },
                                Pos2 {
                                    x: (start_point.x + end_point.x) / 2.0,
                                    y: start_point.y + signum * 150.0,
                                },
                                Pos2 {
                                    x: (start_point.x + end_point.x) / 2.0,
                                    y: end_point.y - signum * 150.0,
                                },
                                Pos2 {
                                    x: end_point.x,
                                    y: end_point.y - signum * 150.0,
                                },
                            ],
                            false,
                            Default::default(),
                            stroke,
                        );
                        ui.painter().add(curve);
                    }
                    {
                        let start_point = Pos2 {
                            x: end_point.x,
                            y: end_point.y - signum * 150.0,
                        };
                        let second_point = Pos2 {
                            x: end_point.x - 150.0,
                            y: end_point.y - signum * 150.0,
                        };
                        let third_point = Pos2 {
                            x: end_point.x - 150.0,
                            y: end_point.y,
                        };

                        let points: [Pos2; 4] = [start_point, second_point, third_point, end_point];
                        let curve = CubicBezierShape::from_points_stroke(
                            points,
                            false,
                            Default::default(),
                            stroke,
                        );
                        ui.painter().add(curve);
                    }
                }
                diff if diff > 0.0 && diff < 100.0 => {
                    {
                        let second_point = Pos2 {
                            x: start_point.x + 50.0,
                            y: start_point.y,
                        };
                        let third_point = Pos2 {
                            x: start_point.x + 50.0,
                            y: start_point.y + signum * 50.0,
                        };
                        // let end_point = Pos2 { x: start_point.x, y: (start_point.y + end_point.y)/2.0 };

                        let points = [start_point, second_point, third_point];
                        let curve = QuadraticBezierShape::from_points_stroke(
                            points,
                            false,
                            Default::default(),
                            stroke,
                        );
                        ui.painter().add(curve);
                    }
                    {
                        let curve = CubicBezierShape::from_points_stroke(
                            [
                                Pos2 {
                                    x: start_point.x + 50.0,
                                    y: start_point.y + signum * 50.0,
                                },
                                Pos2 {
                                    x: (start_point.x + 50.0 + end_point.x) / 2.0,
                                    y: (start_point.y + end_point.y) / 2.0,
                                },
                                Pos2 {
                                    x: (start_point.x - 50.0 + end_point.x) / 2.0,
                                    y: (start_point.y + end_point.y) / 2.0,
                                },
                                Pos2 {
                                    x: end_point.x - 50.0,
                                    y: end_point.y - signum * 50.0,
                                },
                            ],
                            false,
                            Default::default(),
                            stroke,
                        );
                        ui.painter().add(curve);
                    }
                    {
                        //let start_point = Pos2 { x: end_point.x, y: (start_point.y + end_point.y)/2.0 };
                        let second_point = Pos2 {
                            x: end_point.x - 50.0,
                            y: end_point.y - signum * 50.0,
                        };
                        let third_point = Pos2 {
                            x: end_point.x - 50.0,
                            y: end_point.y,
                        };

                        let points = [second_point, third_point, end_point];
                        let curve = QuadraticBezierShape::from_points_stroke(
                            points,
                            false,
                            Default::default(),
                            stroke,
                        );
                        ui.painter().add(curve);
                    }
                }
                _ => {
                    let second_point = Pos2 {
                        x: (start_point.x + end_point.x) / 2.0,
                        y: start_point.y,
                    };
                    let third_point = Pos2 {
                        x: (start_point.x + end_point.x) / 2.0,
                        y: end_point.y,
                    };

                    let points: [Pos2; 4] = [start_point, second_point, third_point, end_point];
                    let curve = CubicBezierShape::from_points_stroke(
                        points,
                        false,
                        Default::default(),
                        stroke,
                    );
                    ui.painter().add(curve);
                }
            }

            if let Some(cursor_pos) = ui.ctx().pointer_latest_pos() {
                let delete_icon_point = Pos2 {
                    x: (start_point.x + end_point.x) / 2.0,
                    y: (start_point.y + end_point.y) / 2.0,
                };
                let delete_icon_rect = Rect {
                    min: delete_icon_point - Vec2 { x: 30.0, y: 30.0 },
                    max: delete_icon_point + Vec2 { x: 30.0, y: 30.0 },
                };

                if delete_icon_rect.contains(cursor_pos) {
                    let delete_icon_response = ui.allocate_ui_at_rect(delete_icon_rect, |ui| {
                        let delete_icon = Label::new("❌").sense(Sense::click());
                        ui.with_layout(
                            egui::Layout::centered_and_justified(egui::Direction::TopDown),
                            |ui| ui.add(delete_icon),
                        )
                    });
                    if delete_icon_response.inner.inner.clicked() {
                        current_link.should_be_deleted = true;
                    }
                }
            }
        }
    }

    fn create_finished_links(&mut self) {
        if let Some((_, link_start_widget)) = self
            .functions
            .iter()
            .find(|(_, widget)| widget.has_vertex.is_some())
        {
            if let Some((_, link_end_widget)) = self
                .functions
                .iter()
                .rev()
                .find(|(_, widget)| widget.has_vertex.is_some())
            {
                if link_start_widget.runnable.name != link_end_widget.runnable.name {
                    let (link_start, link_end) = if link_start_widget
                        .runnable
                        .inputs
                        .get(&link_start_widget.has_vertex.as_ref().unwrap().param_id)
                        .is_some()
                    {
                        (
                            link_end_widget.has_vertex.clone().unwrap(),
                            link_start_widget.has_vertex.clone().unwrap(),
                        )
                    } else {
                        (
                            link_start_widget.has_vertex.clone().unwrap(),
                            link_end_widget.has_vertex.clone().unwrap(),
                        )
                    };

                    self.links.push(Link {
                        start: link_start,
                        end: link_end,
                        should_be_deleted: false,
                    });

                    if let Some((_, link_start_widget)) = self
                        .functions
                        .iter_mut()
                        .find(|(_, widget)| widget.has_vertex.is_some())
                    {
                        link_start_widget.has_vertex.take();
                    }

                    if let Some((_, link_end)) = self
                        .functions
                        .iter_mut()
                        .find(|(_, widget)| widget.has_vertex.is_some())
                    {
                        link_end.has_vertex.take();
                    }
                }
            }
        }
    }

    fn create_unfinished_link_if_clicked(&mut self, ui: &mut egui::Ui, stroke: egui::Stroke) {
        if let Some((_, link_start_widget)) = self
            .functions
            .iter()
            .find(|(_, widget)| widget.has_vertex.is_some())
        {
            if let Some(link_end) = ui.ctx().pointer_latest_pos() {
                let link_start = link_start_widget.has_vertex.clone().unwrap();
                let link_start_pos = link_start_widget.runnable.get_param_by_vertex(&link_start);

                let second_point = Pos2 {
                    x: (link_start_pos.x + link_end.x) / 2.0,
                    y: link_start_pos.y,
                };
                let third_point = Pos2 {
                    x: (link_start_pos.x + link_end.x) / 2.0,
                    y: link_end.y,
                };

                let points: [Pos2; 4] = [link_start_pos, second_point, third_point, link_end];
                let curve =
                    CubicBezierShape::from_points_stroke(points, false, Default::default(), stroke);

                ui.painter().add(curve);
            }
        }
    }

    fn cancel_link_if_esc(&mut self, ui: &mut egui::Ui) {
        if let Some((_, link_start_widget)) = self
            .functions
            .iter_mut()
            .find(|(_, widget)| widget.has_vertex.is_some())
        {
            ui.input(|i| {
                if i.key_pressed(Key::Escape) {
                    link_start_widget.has_vertex.take();
                }
            });
        }
    }

    fn render_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("Toolbox")
            .exact_width(80.0)
            .show(ctx, |ui| {
                ui.add_space(5.0);
                let btn_response = ui.add(
                    egui::Button::image_and_text(
                        egui::include_image!("../assets/function-icon.png"),
                        "Add function",
                    )
                    .rounding(5.0),
                );
                if btn_response.clicked() {
                    self.functions.insert(
                        TinyId::random(),
                        FunctionWidget::default_with_pos(
                            Pos2 { x: 0.0, y: 0.0 },
                            format!("Function #{}", self.last_rect_id),
                        ),
                    );
                    self.last_rect_id += 1;
                }
                ui.add_space(5.0);
                let btn_response = ui.add(egui::Button::new("▶ Run all").rounding(5.0));
                // if btn_response.clicked() {
                //     self.functions
                // }
            });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        self.render_side_panel(ctx);

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| powered_by_egui_and_eframe(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            self.functions.retain(|_, ele| ele.is_open);
            self.delete_old_links();

            let stroke = ui.visuals().widgets.hovered.bg_stroke;

            for (_, ele) in self.functions.iter_mut() {
                ui.add(ele);
            }

            self.cancel_link_if_esc(ui);
            self.create_unfinished_link_if_clicked(ui, stroke);
            self.create_finished_links();
            self.render_links(ui, stroke);

            self.delete_old_links();
            for current_link in self.links.iter_mut() {
                let start_point_param = self
                    .functions
                    .get(&current_link.start.function_id)
                    .unwrap()
                    .runnable
                    .outputs
                    .get(&current_link.start.param_id)
                    .unwrap()
                    .last_value
                    .clone();

                if start_point_param.is_some() {
                    if let Some(last_value) = start_point_param {
                        let end_point_param = self
                            .functions
                            .get_mut(&current_link.end.function_id)
                            .unwrap()
                            .runnable
                            .inputs
                            .get_mut(&current_link.end.param_id)
                            .unwrap();

                        end_point_param.last_value = Some(last_value);
                    }
                }
            }
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Powered by");
        ui.hyperlink_to("egui. ", "https://github.com/emilk/egui");
        ui.label("Source code");
        ui.hyperlink_to("on GitHub", "https://github.com/AufarZakiev/rust_scripter");
        ui.label(".")
    });
}
