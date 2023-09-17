use egui::Ui;
use egui_extras::RetainedImage;
use poll_promise::Promise;

use crate::{
    downloader::{insta, tiktok},
    types::{Action, DownloadableVideo, LayoutSettings},
};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(defaul)] // if we add new fields, give them default values when deserializing old state
pub struct PSKGui {
    // Example stuff:
    pub url: String,
    pub action: Action,
    pub comms_video: Option<Promise<Result<DownloadableVideo, String>>>,
    pub comms_image: Option<Promise<Result<RetainedImage, String>>>,
    pub download_vid: Option<Result<DownloadableVideo, String>>,
    pub download_img: Option<Result<RetainedImage, String>>,
    pub side_txt: String,
    pub dwl_video: Option<DownloadableVideo>,
    pub dwl_image: Option<RetainedImage>,
    pub dwl_path: Option<PathBuf>,
    pub has_data: bool,
    pub tmp_bool: bool,
    pub img_ready: bool,
}

impl Default for PSKGui {
    fn default() -> Self {
        Self {
            url: String::new(),
            action: Action::Home,
            comms_video: None,
            comms_image: None,
            dwl_video: None,
            side_txt: String::new(),
            dwl_path: None,
            download_vid: None,
            has_data: false,
            tmp_bool: true,
            dwl_image: None,
            download_img: None,
            img_ready: false,
        }
    }
}

impl PSKGui {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            //return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn check(&mut self) {}
}

fn centerer(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        let id = ui.id().with("_centerer");
        let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(id));
        if let Some(last_width) = last_width {
            ui.add_space((ui.available_width() - last_width) / 2.0);
        }
        let res = ui
            .scope(|ui| {
                add_contents(ui);
            })
            .response;
        let width = res.rect.width();
        ui.memory_mut(|mem| mem.data.insert_temp(id, width));

        // Repaint if width changed
        match last_width {
            None => ui.ctx().request_repaint(),
            Some(last_width) if last_width != width => ui.ctx().request_repaint(),
            Some(_) => {}
        }
    });
}
impl eframe::App for PSKGui {
    ///// Called by the frame work to save state before shutdown.
    //fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //    eframe::set_value(storage, eframe::APP_KEY, self);
    //}

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //let Self { label, value } = self;
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.selectable_value(&mut self.action, Action::Home, "🏠");
                ui.heading("Download sources");
                ui.selectable_value(&mut self.action, Action::Tiktok, "Tiktok");
                ui.selectable_value(&mut self.action, Action::Instagram, "Instagram");
                ui.selectable_value(&mut self.action, Action::Youtube, "Youtube");
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.label("psk");
            if self.action == Action::Home {
                //ui.label("hh");
                //ui.with_layout(
                //    egui::Layout::from_main_dir_and_cross_align(
                //        egui::Direction::LeftToRight,
                //        egui::Align::Center,
                //    ),
                //    |ui| {
                //        ui.label("d");
                //    },
                //);
                ui.with_layout(LayoutSettings::center().layout(), |ui| {
                    centerer(ui, |ui| {
                        //ui.label("d");
                        ui.heading("psk - producer swiss knife");
                        //ui.label("¯\\_(ツ)_/¯");
                    });
                });
                ui.with_layout(
                    egui::Layout::from_main_dir_and_cross_align(
                        egui::Direction::BottomUp,
                        egui::Align::BOTTOM,
                    ),
                    |ui| {
                        use egui::special_emojis::GITHUB;
                        ui.hyperlink_to(format!("{GITHUB} psk"), "https://github.com/gempoint/psk");
                    },
                );
            } else {
                self.downloaders(ctx, ui);
                //if ui.button("panic!").clicked() {
                //    panic!("idk why not panic")
                //}
            }
        });
    }
}
