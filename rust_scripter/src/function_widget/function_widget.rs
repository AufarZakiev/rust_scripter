use egui::{
    vec2, widgets::Widget, Align, Align2, Button, Color32, Id, Key, KeyboardShortcut, Label,
    LayerId, Modifiers, Order, Pos2, Rect, Rounding, Sense, TextEdit, TextStyle, Vec2, Window,
};
use indexmap::IndexMap;
use rhai::Map;
use serde::{Deserialize, Serialize};
use tinyid::TinyId;

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum ParamType {
    Input,
    Output,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LinkVertex {
    pub function_name: String,
    pub param_id: TinyId,
}

#[derive(Serialize, Deserialize)]
pub struct FunctionParam {
    pub param_name: String,
    pub type_name: String,
    pub pos: Pos2,
    pub should_be_deleted: bool,
    pub is_editing: bool,
    pub last_value: Option<rhai::Dynamic>,
}

impl Default for FunctionParam {
    fn default() -> Self {
        Self {
            param_name: "New...".to_string(),
            type_name: "String".to_string(),
            pos: Pos2::default(),
            should_be_deleted: false,
            is_editing: false,
            last_value: None,
        }
    }
}

impl FunctionParam {
    fn default_with_name(name: &str) -> Self {
        Self {
            param_name: name.to_string(),
            type_name: "String".to_string(),
            pos: Pos2::default(),
            should_be_deleted: false,
            is_editing: false,
            last_value: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Runnable {
    pub name: String,
    pub code: String,
    pub inputs: IndexMap<TinyId, FunctionParam>,
    pub outputs: IndexMap<TinyId, FunctionParam>,
}

impl Default for Runnable {
    fn default() -> Self {
        let inputs = IndexMap::from_iter([
            (TinyId::random(), FunctionParam::default_with_name("Input1")),
            (TinyId::random(), FunctionParam::default_with_name("Input2")),
            (TinyId::random(), FunctionParam::default_with_name("Input3")),
        ]);

        let outputs = IndexMap::from_iter([
            (
                TinyId::random(),
                FunctionParam::default_with_name("Output1"),
            ),
            (
                TinyId::random(),
                FunctionParam::default_with_name("Output2"),
            ),
        ]);
        Self {
            name: "Function #0".to_owned(),
            code: r#"let val = #{Output1: Input1, Output2: Input2};
val"#
                .to_string(),
            inputs,
            outputs,
        }
    }
}

impl Runnable {
    pub fn get_entry_by_vertex(&self, vertex: &LinkVertex) -> Pos2 {
        if let Some(input) = self.inputs.get(&vertex.param_id) {
            return input.pos;
        }
        if let Some(output) = self.outputs.get(&vertex.param_id) {
            return output.pos;
        }
        panic!(
            "No vertex found with {}, {}",
            vertex.function_name, vertex.param_id
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct RenameOptions {
    pub rename_id: TinyId,
    pub param_type: ParamType,
    pub new_name: String,
}

impl Default for RenameOptions {
    fn default() -> Self {
        Self {
            rename_id: Default::default(),
            param_type: ParamType::Input,
            new_name: Default::default(),
        }
    }
}

#[derive(PartialEq, Deserialize, Serialize)]
pub enum WidgetMode {
    Code,
    Signature,
}

#[derive(Deserialize, Serialize)]
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct FunctionWidget {
    // Actual code of function
    pub runnable: Runnable,
    // Visual state
    pub position: Pos2,
    pub interactive_size: Vec2,
    pub code_size: Vec2,
    pub is_open: bool,
    pub is_collapsed: bool,
    pub has_vertex: Option<LinkVertex>,
    pub mode: WidgetMode,
    // Temp values
    pub entry_rename: Option<RenameOptions>,
    // Engine to run the code
    #[serde(skip, default = "rhai::Engine::new")]
    pub engine: rhai::Engine,
}

impl Default for FunctionWidget {
    fn default() -> FunctionWidget {
        let default_runnable = Runnable::default();

        FunctionWidget::new(default_runnable, Pos2 { x: 120.0, y: 40.0 }, true, true)
    }
}

impl FunctionWidget {
    pub fn default_with_pos(initial_pos: Pos2, name: String) -> Self {
        let mut def = FunctionWidget::default();
        def.runnable.name = name;
        def.position = initial_pos;
        def
    }

    pub fn new(runnable: Runnable, initial_pos: Pos2, is_open: bool, is_collapsed: bool) -> Self {
        Self {
            position: initial_pos,
            interactive_size: Vec2 { x: 200.0, y: 100.0 },
            code_size: Vec2 { x: 400.0, y: 100.0 },
            runnable,
            is_open,
            is_collapsed,
            has_vertex: None,
            mode: WidgetMode::Signature,
            entry_rename: None,
            engine: rhai::Engine::new(),
        }
    }
}

impl Widget for &mut FunctionWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut window = Window::new(&self.runnable.name)
            .open(&mut self.is_open)
            .collapsible(true);

        if self.mode == WidgetMode::Signature {
            window = window.fixed_size(self.interactive_size);
        } else {
            window = window.fixed_size(self.code_size);
        }

        self.runnable
            .inputs
            .retain(|_, input| !input.should_be_deleted);

        if let Some(ref rename_options) = self.entry_rename {
            if rename_options.param_type == ParamType::Input {
                self.runnable
                    .inputs
                    .get_mut(&rename_options.rename_id)
                    .expect(
                        format!(
                            "Rename options are invalid: {}, {}",
                            rename_options.rename_id, rename_options.new_name
                        )
                        .as_str(),
                    )
                    .param_name = rename_options.new_name.clone();
            } else {
                self.runnable
                    .outputs
                    .get_mut(&rename_options.rename_id)
                    .expect(
                        format!(
                            "Rename options are invalid: {}, {}",
                            rename_options.rename_id, rename_options.new_name
                        )
                        .as_str(),
                    )
                    .param_name = rename_options.new_name.clone();
            }
        }

        self.runnable
            .outputs
            .retain(|_, output| !output.should_be_deleted);

        let pointer = ui.ctx().pointer_latest_pos();
        let font_id = TextStyle::Body.resolve(ui.style());
        let visuals = ui.visuals();

        let window_response = window
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.mode, WidgetMode::Signature, "Signature");
                    ui.selectable_value(&mut self.mode, WidgetMode::Code, "Code");
                });

                if self.mode == WidgetMode::Code {
                    let language = "rs";
                    let theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());

                    let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
                        // REIMPLEMENT FOR FILE SUPPORT & MAKE A PR FOR GUI EXTRAS
                        let mut layout_job = egui_extras::syntax_highlighting::highlight(
                            ui.ctx(),
                            &theme,
                            string,
                            language,
                        );
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };

                    ui.add(
                        egui::TextEdit::multiline(&mut self.runnable.code)
                            .font(egui::TextStyle::Monospace) // for cursor height
                            .code_editor()
                            .desired_rows(10)
                            .lock_focus(true)
                            .desired_width(f32::INFINITY)
                            .layouter(&mut layouter),
                    );
                } else {
                    let painter = ui.ctx().layer_painter(LayerId::new(
                        Order::Foreground,
                        Id::new(self.runnable.name.clone()),
                    ));

                    let stroke = ui.visuals().widgets.hovered.bg_stroke;

                    ui.columns(3, |columns| {
                        for (input_id, input) in self.runnable.inputs.iter_mut() {
                            let label_response = if !input.is_editing {
                                columns[0].add(
                                    Label::new(input.param_name.clone())
                                        .sense(Sense::click())
                                        .wrap(true),
                                )
                            } else {
                                if self.entry_rename.is_none() {
                                    self.entry_rename = Some(RenameOptions {
                                        rename_id: input_id.clone(),
                                        param_type: ParamType::Input,
                                        new_name: input.param_name.clone(),
                                    });
                                }
                                columns[0].add(TextEdit::singleline(
                                    &mut self
                                        .entry_rename
                                        .as_mut()
                                        .expect("Entry rename was not inited")
                                        .new_name,
                                ))
                            };

                            let circle_rect = Rect::from_center_size(
                                label_response.rect.left_center() + Vec2 { x: -6.0, y: 0.0 },
                                Vec2 { x: 10.0, y: 10.0 },
                            );
                            painter.circle(
                                circle_rect.center(),
                                5.0,
                                Color32::from_rgb(128, 0, 0),
                                stroke,
                            );
                            input.pos = circle_rect.center();

                            if let Some(ref last_value) = input.last_value {
                                if last_value.is_int() {
                                    let layout = painter.layout_no_wrap(
                                        last_value.clone().as_int().unwrap().to_string(),
                                        font_id.clone(),
                                        visuals.text_color(),
                                    );
                                    painter.rect(
                                        Rect::from_center_size(
                                            circle_rect.center() + Vec2 { x: -15.0, y: 0.0 },
                                            layout.rect.size(),
                                        )
                                        .expand2(vec2(1.0, 0.0)),
                                        Rounding::same(1.5),
                                        Color32::LIGHT_GRAY,
                                        stroke,
                                    );
                                    painter.text(
                                        circle_rect.center() + Vec2 { x: -15.0, y: 0.0 },
                                        Align2::CENTER_CENTER,
                                        last_value.clone().as_int().unwrap().to_string(),
                                        font_id.clone(),
                                        visuals.text_color(),
                                    );
                                }
                            }

                            if label_response.clicked() && !input.is_editing {
                                self.has_vertex = Some(LinkVertex {
                                    function_name: self.runnable.name.clone(),
                                    param_id: input_id.clone(),
                                });
                            }

                            if label_response.hovered() {
                                columns[0].painter().rect(
                                    label_response.rect.expand(1.5),
                                    Rounding::same(2.0),
                                    Color32::TRANSPARENT,
                                    stroke,
                                )
                            }

                            if label_response.double_clicked() {
                                input.is_editing = true;
                                self.has_vertex = None;
                            }

                            if label_response.clicked_elsewhere() {
                                input.is_editing = false;
                                self.entry_rename = None;
                            }

                            if input.is_editing {
                                columns[0].input(|i| {
                                    if i.key_pressed(Key::Escape) {
                                        input.is_editing = false;
                                        self.entry_rename = None;
                                    }
                                    if i.key_pressed(Key::Enter) {
                                        input.is_editing = false;
                                    }
                                });
                            }

                            let is_circle_hovered =
                                pointer.is_some() && circle_rect.contains(pointer.unwrap());
                            if label_response.hovered() || is_circle_hovered {
                                painter.circle(
                                    circle_rect.center(),
                                    2.5,
                                    Color32::from_rgb(255, 255, 255),
                                    stroke,
                                )
                            }

                            label_response.context_menu(|ui| {
                                let btn = Button::new("Edit").shortcut_text("Double-click");
                                if ui.add(btn).clicked() {
                                    input.is_editing = true;
                                    ui.close_menu();
                                }
                                if ui.button("Delete").clicked() {
                                    input.should_be_deleted = true;
                                    ui.close_menu();
                                }
                            });
                        }
                        if columns[0].button("Add...").clicked() {
                            self.runnable
                                .inputs
                                .insert(TinyId::random(), FunctionParam::default());
                        };
                        let run_button = egui::Button::new("▶").rounding(5.0);
                        columns[1].with_layout(egui::Layout::top_down(Align::Center), |ui| {
                            let run_button_response = ui.add(run_button);
                            if run_button_response.clicked() {
                                let prepend_code = format!(
                                    "{}{}{}",
                                    "let ",
                                    self.runnable
                                        .inputs
                                        .iter()
                                        .map(|input| input.1.param_name.clone())
                                        .collect::<Vec<String>>()
                                        .join(" = 3; let "),
                                    " = 3;"
                                );
                                if let Ok(result) = self.engine.eval::<Map>(
                                    format!("{} {}", prepend_code, &self.runnable.code).as_str(),
                                ) {
                                    for ele in self.runnable.outputs.iter_mut() {
                                        if let Some(val) = result.get(ele.1.param_name.as_str()) {
                                            ele.1.last_value = Some(val.clone());
                                        }
                                    }
                                }
                            }
                        });
                        for (output_id, output) in self.runnable.outputs.iter_mut() {
                            let label_response = columns[2].with_layout(
                                egui::Layout::right_to_left(Align::Min),
                                |ui| {
                                    if !output.is_editing {
                                        ui.add(
                                            Label::new(output.param_name.clone())
                                                .sense(Sense::click())
                                                .wrap(true),
                                        )
                                    } else {
                                        if self.entry_rename.is_none() {
                                            self.entry_rename = Some(RenameOptions {
                                                rename_id: output_id.clone(),
                                                param_type: ParamType::Output,
                                                new_name: output.param_name.clone(),
                                            });
                                        }
                                        ui.add(TextEdit::singleline(
                                            &mut self
                                                .entry_rename
                                                .as_mut()
                                                .expect("Entry rename was not inited")
                                                .new_name,
                                        ))
                                    }
                                },
                            );

                            let circle_rect = Rect::from_center_size(
                                label_response.response.rect.right_center()
                                    + Vec2 { x: 6.0, y: 0.0 },
                                Vec2 { x: 10.0, y: 10.0 },
                            );
                            painter.circle(
                                circle_rect.center(),
                                5.0,
                                Color32::from_rgb(128, 0, 0),
                                stroke,
                            );
                            output.pos = circle_rect.center();

                            if let Some(ref last_value) = output.last_value {
                                if last_value.is_int() {
                                    let layout = painter.layout_no_wrap(
                                        last_value.clone().as_int().unwrap().to_string(),
                                        font_id.clone(),
                                        visuals.text_color(),
                                    );
                                    painter.rect(
                                        Rect::from_center_size(
                                            circle_rect.center() + Vec2 { x: 15.0, y: 0.0 },
                                            layout.rect.size(),
                                        )
                                        .expand2(vec2(1.0, 0.0)),
                                        Rounding::same(1.5),
                                        Color32::LIGHT_GRAY,
                                        stroke,
                                    );
                                    painter.text(
                                        circle_rect.center() + Vec2 { x: 15.0, y: 0.0 },
                                        Align2::CENTER_CENTER,
                                        last_value.clone().as_int().unwrap().to_string(),
                                        font_id.clone(),
                                        visuals.text_color(),
                                    );
                                }
                            }

                            if label_response.inner.clicked() && !output.is_editing {
                                self.has_vertex = Some(LinkVertex {
                                    function_name: self.runnable.name.clone(),
                                    param_id: output_id.clone(),
                                });
                            }

                            if label_response.inner.hovered() {
                                columns[2].painter().rect(
                                    label_response.response.rect.expand(1.5),
                                    Rounding::same(2.0),
                                    Color32::TRANSPARENT,
                                    stroke,
                                )
                            }

                            if label_response.inner.double_clicked() {
                                output.is_editing = true;
                                self.has_vertex = None;
                            }

                            if label_response.inner.lost_focus()
                                || label_response.inner.clicked_elsewhere()
                            {
                                output.is_editing = false;
                            }

                            columns[2].input(|i| {
                                if i.key_pressed(Key::Escape) {
                                    output.is_editing = false;
                                    self.entry_rename = None;
                                }
                                if i.key_pressed(Key::Enter) {
                                    output.is_editing = false;
                                }
                            });

                            let is_circle_hovered =
                                pointer.is_some() && circle_rect.contains(pointer.unwrap());
                            if label_response.inner.hovered() || is_circle_hovered {
                                painter.circle(
                                    circle_rect.center(),
                                    2.5,
                                    Color32::from_rgb(255, 255, 255),
                                    stroke,
                                )
                            }

                            label_response.inner.context_menu(|ui| {
                                let btn = Button::new("Edit").shortcut_text("Double-click");
                                if ui.add(btn).clicked() {
                                    output.is_editing = true;
                                    ui.close_menu();
                                }
                                if ui.button("Delete").clicked() {
                                    output.should_be_deleted = true;
                                    ui.close_menu();
                                }
                            });
                        }
                        if columns[2].button("Add...").clicked() {
                            self.runnable
                                .outputs
                                .insert(TinyId::random(), FunctionParam::default());
                        };
                    });
                }
            })
            .unwrap();

        ui.input_mut(|i| {
            if pointer.is_some()
                && window_response.response.rect.contains(pointer.unwrap())
                && i.consume_shortcut(&KeyboardShortcut::new(Modifiers::CTRL, Key::Q))
            {
                self.mode = if self.mode == WidgetMode::Signature {
                    WidgetMode::Code
                } else {
                    WidgetMode::Signature
                };
            }
        });

        self.is_collapsed = window_response.inner.is_none();
        self.position = window_response.response.rect.left_top();

        window_response.response
    }
}
