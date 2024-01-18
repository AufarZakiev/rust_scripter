use indexmap::IndexMap;
use rhai::Map;
use egui::{Pos2, widgets::Widget, Sense, Color32, Rect, Vec2, Order, LayerId, Id, Align, Label, Window, Key, KeyboardShortcut, Modifiers, Button, Rounding, TextEdit};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LinkVertex {
    pub function_name: String,
    pub entry_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionInputConfig {
    pub type_name: String,
    pub pos: Pos2,
    pub should_be_deleted: bool,
    pub is_edited: bool,
}

impl Default for FunctionInputConfig {
    fn default() -> Self {
        Self {
            type_name: "String".to_string(), 
            pos: Pos2::default(),
            should_be_deleted: false,
            is_edited: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Runnable {
    pub name: String,
    pub code: String,
    #[serde(with = "vectorize")]
    pub inputs: IndexMap<String, FunctionInputConfig>,
    #[serde(with = "vectorize")]
    pub outputs: IndexMap<String, FunctionInputConfig>,
}

impl Default for Runnable {
    fn default() -> Self {
        let mut inputs = IndexMap::new();
        inputs.insert("Input1".into(), FunctionInputConfig::default());
        inputs.insert("Input2".into(), FunctionInputConfig::default());
        inputs.insert("Input3".into(), FunctionInputConfig::default());

        let mut outputs = IndexMap::new();
        outputs.insert("Output1".into(), FunctionInputConfig::default());
        outputs.insert("Output2".into(), FunctionInputConfig::default());
        Self { 
            name: "Function #0".to_owned(), 
            code: 
r#"let val = #{test: Input1};
val"#
            .to_string(),
            inputs,
            outputs,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FunctionConfig {
    pub runnable: Runnable,
    pub position: Pos2,
    pub interactive_size: Vec2,
    pub code_size: Vec2,
    pub is_open: bool,
    pub is_collapsed: bool,
    pub has_vertex: Option<LinkVertex>,
    pub mode: WidgetMode,
    #[serde(skip, default = "rhai::Engine::new")]
    pub engine: rhai::Engine,
}

impl Default for FunctionConfig {
    fn default() -> FunctionConfig {
        let default_runnable = Runnable::default();

        FunctionConfig::new(default_runnable, Pos2 {x: 120.0, y: 40.0}, true, true)
    }
}

#[derive(PartialEq, Deserialize, Serialize)]
pub enum WidgetMode {
    Code,
    Signature
}

impl FunctionConfig {
    pub fn default_with_pos(initial_pos: Pos2, name: String) -> Self {
        let mut def = FunctionConfig::default();
        def.runnable.name = name;
        def.position = initial_pos;
        def
    }

    pub fn get_entry(self: &Self, entry_name: &String) -> Option<&FunctionInputConfig> {
        return self.runnable.inputs.get(entry_name).or_else(|| {self.runnable.outputs.get(entry_name)});
    }

    pub fn new(
        runnable: Runnable,
        initial_pos: Pos2, 
        is_open: bool,
        is_collapsed: bool
    ) -> Self {
        Self { 
            position: initial_pos, 
            interactive_size: Vec2 {x: 200.0, y: 100.0},
            code_size: Vec2 {x: 400.0, y: 100.0},
            runnable,
            is_open,
            is_collapsed,
            has_vertex: None,
            mode: WidgetMode::Signature,
            engine: rhai::Engine::new(),
        }
    }
}

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct FunctionWidget<'a> {
    pub config: &'a mut FunctionConfig,
}

impl<'a> FunctionWidget<'a> {
    pub fn new(config: &'a mut FunctionConfig) -> Self {
        Self { 
            config,
        }
    }
}

impl Widget for &mut FunctionWidget<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut window = Window::new(&self.config.runnable.name)
            .open(&mut self.config.is_open)
            .collapsible(true);

        if self.config.mode == WidgetMode::Signature {
            window = window.fixed_size(self.config.interactive_size);
        } else {
            window = window.fixed_size(self.config.code_size);
        }

        self.config.runnable.inputs.retain(|_, input| {
            !input.should_be_deleted
        });

        self.config.runnable.outputs.retain(|_, output| {
            !output.should_be_deleted
        });

        let pointer = ui.ctx().pointer_latest_pos();

        let window_response = window.show(ui.ctx(), |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.config.mode, WidgetMode::Signature, "Signature");
                ui.selectable_value(&mut self.config.mode, WidgetMode::Code, "Code");
            });

            if self.config.mode == WidgetMode::Code {
                let language = "rs";
                let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());

                let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                    // REIMPLEMENT FOR FILE SUPPORT & MAKE A PR FOR GUI EXTRAS
                    let mut layout_job =
                        egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, language);
                    layout_job.wrap.max_width = wrap_width;
                    ui.fonts(|f| f.layout_job(layout_job))
                };
    
                ui.add(egui::TextEdit::multiline(&mut self.config.runnable.code)
                        .font(egui::TextStyle::Monospace) // for cursor height
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter));
            } else {
                let circle_painter = ui.ctx()
                    .layer_painter(LayerId::new(Order::Foreground, Id::new(self.config.runnable.name.clone())));
                
                let stroke = ui.visuals().widgets.hovered.bg_stroke;
                
                ui.columns(3, |columns| {
                    for ele in self.config.runnable.inputs.iter_mut() {
                        let label_response = if !ele.1.is_edited { 
                            columns[0].add(Label::new(ele.0.clone()).sense(Sense::click())) 
                        } else {
                            columns[0].add(TextEdit::singleline(&mut ele.0.clone()))
                        };

                        let circle_rect = Rect::from_center_size(
                            label_response.rect.left_center() + Vec2 { x: -6.0, y: 0.0 },
                            Vec2 { x: 10.0, y: 10.0 }
                        );
                        circle_painter.circle(
                            circle_rect.center(),
                            5.0,
                            Color32::from_rgb(128, 0, 0), 
                            stroke
                        );
                        ele.1.pos = circle_rect.center();

                        if label_response.clicked() {
                            self.config.has_vertex = Some(LinkVertex { function_name: self.config.runnable.name.clone(), entry_name: ele.0.clone() });
                        }

                        if label_response.hovered() {
                            columns[0].painter().rect(label_response.rect.expand(1.5), Rounding::same(2.0), Color32::TRANSPARENT, stroke)
                        }

                        if label_response.double_clicked() {
                            ele.1.is_edited = true;
                            self.config.has_vertex = None;
                        }

                        if label_response.lost_focus() || label_response.clicked_elsewhere() {
                            ele.1.is_edited = false;
                        }

                        columns[0].input(|i| { 
                            if i.key_pressed(Key::Escape) {
                                ele.1.is_edited = false;       
                            }
                        });

                        let is_circle_hovered = pointer.is_some() && circle_rect.contains(pointer.unwrap());
                        if label_response.hovered() || is_circle_hovered {                            
                            circle_painter.circle(
                                circle_rect.center(),
                                2.5,
                                Color32::from_rgb(255, 255, 255), 
                                stroke
                            )
                        }

                        label_response.context_menu(|ui| {
                            let btn  = Button::new("Edit").shortcut_text("Double-click");
                            if ui.add(btn).clicked() {
                                ele.1.is_edited = true;
                                ui.close_menu();
                            }
                            if ui.button("Delete").clicked() {
                                ele.1.should_be_deleted = true;
                                ui.close_menu();
                            }
                        });
                    };
                    if columns[0].button("Add...").clicked() {
                        self.config.runnable.inputs.insert("New...".to_string(), FunctionInputConfig::default());
                    };
                    let run_button = egui::Button::new("▶").rounding(5.0);
                    columns[1].with_layout(egui::Layout::top_down(Align::Center), |ui| { 
                        let run_button_response = ui.add(run_button);
                        if run_button_response.hovered() {
                            let prepend_code = format!(
                                "{}{}{}",
                                "let ",
                                self.config.runnable.inputs.keys().cloned().collect::<Vec<String>>().join(" = 3; let "),
                                " = 3;"
                            ); 
                            if let Ok(result) = self.config.engine.eval::<Map>(
                                format!("{} {}", prepend_code, &self.config.runnable.code).as_str()
                            ) {
                                if let Some(val) = result.get("test") {
                                    if val.is_int() {
                                        ui.label(val.clone().as_int().unwrap().to_string());
                                    }
                                }
                            }
                        }
                    });
                    for ele in self.config.runnable.outputs.iter_mut() {
                        let label_response = columns[2].with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                            if !ele.1.is_edited { 
                                ui.add(Label::new(ele.0.clone()).sense(Sense::click())) 
                            } else {
                                ui.add(TextEdit::singleline(&mut ele.0.clone()))
                            }
                        });

                        let circle_rect = Rect::from_center_size(
                            label_response.response.rect.right_center() + Vec2 { x: 8.0, y: 0.0 },
                            Vec2 { x: 10.0, y: 10.0 }
                        );
                        circle_painter.circle(
                            circle_rect.center(),
                            5.0,
                            Color32::from_rgb(128, 0, 0), 
                            stroke
                        );
                        ele.1.pos = circle_rect.center();

                        if label_response.inner.clicked() {
                            self.config.has_vertex = Some(LinkVertex { function_name: self.config.runnable.name.clone(), entry_name: ele.0.clone() });
                        }

                        if label_response.inner.hovered() {
                            columns[2].painter().rect(label_response.response.rect.expand(1.5), Rounding::same(2.0), Color32::TRANSPARENT, stroke)
                        }

                        if label_response.inner.double_clicked() {
                            ele.1.is_edited = true;
                            self.config.has_vertex = None;
                        }

                        if label_response.inner.lost_focus() || label_response.inner.clicked_elsewhere() {
                            ele.1.is_edited = false;
                        }

                        columns[2].input(|i| { 
                            if i.key_pressed(Key::Escape) {
                                ele.1.is_edited = false;       
                            }
                        });

                        let is_circle_hovered = pointer.is_some() && circle_rect.contains(pointer.unwrap());
                        if label_response.inner.hovered() || is_circle_hovered {  
                            circle_painter.circle(
                                circle_rect.center(),
                                2.5,
                                Color32::from_rgb(255, 255, 255), 
                                stroke
                            )
                        }

                        label_response.inner.context_menu(|ui| {
                            let btn  = Button::new("Edit").shortcut_text("Double-click");
                            if ui.add(btn).clicked() {
                                ele.1.is_edited = true;
                                ui.close_menu();
                            }
                            if ui.button("Delete").clicked() {
                                ele.1.should_be_deleted = true;
                                ui.close_menu();
                            }
                        });
                    }
                    if columns[2].button("Add...").clicked() {
                        self.config.runnable.outputs.insert("New...".to_string(), FunctionInputConfig::default());
                    };
                });
            }
        }).unwrap();

        ui.input_mut(|i| { 
            if pointer.is_some() && 
                window_response.response.rect.contains(pointer.unwrap()) && 
                i.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::Q)) 
            {
                self.config.mode = if self.config.mode == WidgetMode::Signature { WidgetMode::Code } else { WidgetMode::Signature };
            }
        });

        self.config.is_collapsed = window_response.inner.is_none();
        self.config.position = window_response.response.rect.left_top();

        window_response.response
    }
}