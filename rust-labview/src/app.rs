use egui::{Pos2, Window, Sense};
use function_widget::{FunctionWidget, Runnable, ParamTypes, FunctionConfig};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    #[serde(skip)] // This how you opt-out of serialization of a field
    rects: Vec<FunctionConfig>
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            rects: vec![ 
                FunctionConfig::default(),
                FunctionConfig::default_with_pos(Pos2 {x: 165.0, y: 40.0})
            ]
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        cc.egui_ctx.set_pixels_per_point(2.0);

        egui_extras::install_image_loaders(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::left("Toolbox").show(ctx, |ui| {
            let icon = 
                egui::Image::new(egui::include_image!("../assets/function-icon.png"))
                .rounding(5.0).max_height(48.0).sense(Sense::click());
            let icon_response = ui.add(icon);
            if icon_response.clicked() {
                self.rects.push(FunctionConfig::default());
            }
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            powered_by_egui_and_eframe(ui)
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for ele in self.rects.iter_mut() {
                    ui.add(&mut FunctionWidget::new(ele));   
                }
            });            
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
