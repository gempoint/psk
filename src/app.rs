use poll_promise::Promise;

use crate::{
    downloader::{insta, tiktok},
    types::{Action, DownloadableVideo},
};
use std::{
    sync::{Arc, Mutex},
    thread,
};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(defaul)] // if we add new fields, give them default values when deserializing old state
pub struct PSKGui {
    // Example stuff:
    url: String,
    action: Action,
    communication: Option<Promise<Result<DownloadableVideo, String>>>,
    download_dat: Option<Result<DownloadableVideo, String>>,
    side_txt: String,
    dat: Option<DownloadableVideo>,
    has_data: bool,
    tmp_bool: bool,
}

impl Default for PSKGui {
    fn default() -> Self {
        Self {
            url: String::new(),
            action: Action::Instagram,
            communication: None,
            dat: None,
            side_txt: String::new(),
            download_dat: None,
            has_data: false,
            tmp_bool: true,
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
            //ui.heading("Left Panel");
            ui.selectable_value(&mut self.action, Action::Tiktok, "Tiktok");
            ui.selectable_value(&mut self.action, Action::Instagram, "Instagram");
            ui.selectable_value(&mut self.action, Action::Youtube, "Youtube");
            ui.horizontal(|ui| {
                //ui.add(egui::Slider::new(&mut self.url, 0.0..=1.0));
                //ui.add(egui::Label::new(&self.url));
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            //ui.label("psk");
            ui.horizontal_top(|ui| {
                ui.label("url: ");
                ui.text_edit_singleline(&mut self.url);
                let action = ui.add_enabled(self.tmp_bool, egui::Button::new("go!"));
                ui.label(&self.side_txt);
                //let action = ui.button("go!")
                if action.clicked() {
                    println!("uh");
                    self.tmp_bool = false;
                    let promise = self.communication.get_or_insert((|| {
                        let ctx = ctx.clone();
                        let (sender, promise) = Promise::new();
                        if self.action == Action::Tiktok || self.action == Action::Instagram {
                            if self.action == Action::Tiktok {
                                let res = tiktok(self.url.clone());
                                sender.send(res);
                            } else {
                                let res = insta(self.url.clone());
                                sender.send(res);
                            }
                            ctx.request_repaint();
                            //self.has_data = true;
                        }
                        promise
                    })());
                    match &self.communication {
                        Some(x) => {
                            self.tmp_bool = true;
                            match x.ready() {
                                Some(x) => match x {
                                    Ok(x) => {
                                        //ui.label(&x.title);
                                        self.has_data = true;
                                        self.dat = Some(x.clone());
                                        self.communication = None;
                                        //if self.dat.unwrap().thumbnail.is_some() {}
                                        //self.download_dat = Some(Ok(x.clone()));
                                    }
                                    Err(e) => {
                                        println!("err: {}", e);
                                        //ui.label(format!("error: {}", e));
                                        self.side_txt = format!("error: {}", e);
                                        self.communication = None;
                                    }
                                },
                                None => {
                                    //ui.label("fetching...");
                                    self.side_txt = "fetching...".to_string();
                                }
                            }
                        }
                        None => {}
                    }
                }
            });
            if self.has_data {
                ui.horizontal(|ui| {
                    ui.label("title: ");
                    ui.label(&self.dat.as_ref().unwrap().title);
                });
                ui.horizontal(|ui| {
                    ui.label("url: ");
                    ui.label(&self.dat.as_ref().unwrap().url);
                    //});
                    //ui.horizontal(|ui| {
                    //    ui.label("thumbnail: ");
                    //    ui.label(&self.dat.as_ref().unwrap().thumbnail);
                    //});
                    //ui.horizontal(|ui| {
                    //    ui.label("duration: ");
                    //    ui.label(&self.dat.as_ref().unwrap().duration);
                    //});
                    //ui.horizontal(|ui| {
                    //    ui.label("views: ");
                    //    ui.label(&self.dat.as_ref().unwrap().views);
                });
            }
        });
    }
}
