use std::io::Write;

use egui::Ui;
use egui_extras::RetainedImage;
use native_dialog::{FileDialog, MessageDialog};
use num_format::{Locale, ToFormattedString};
use opener::reveal;
use poll_promise::Promise;
use reqwest::{header::CONTENT_TYPE, Url};
use rusty_ytdl::{
    block_async, DownloadOptions, RequestOptions, Video, VideoOptions, VideoQuality,
    VideoSearchOptions,
};

use crate::{
    audio::convert,
    downloader::{insta, tiktok, yt},
    extras::{error, label_wrap},
    types::Action,
    PSKGui,
};

impl PSKGui {
    pub fn show_image(&mut self, ui: &mut Ui) {
        self.dwl_image
            .as_ref()
            .unwrap()
            .show_max_size(ui, ui.available_size());
    }
    pub fn downloaders(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        ui.horizontal_top(|ui| {
            ui.label("url: ");
            ui.text_edit_singleline(&mut self.url);
            let action = ui.add_enabled(self.tmp_bool, egui::Button::new("go!"));
            //let action = ui.button("go!")
            if action.clicked() {
                self.side_txt = String::new();
                println!("uh");
                self.tmp_bool = false;
                let promise = self.comms_video.get_or_insert((|| {
                    let ctx = ctx.clone();
                    let (sender, promise) = Promise::new();

                    match self.action {
                        Action::Tiktok => {
                            let res = tiktok(self.url.clone());
                            sender.send(res);
                        }
                        Action::Instagram => {
                            let res = insta(self.url.clone());
                            sender.send(res);
                        }
                        Action::Youtube => {
                            let res = yt(self.url.clone());
                            sender.send(res);
                        }
                        _ => {}
                    }
                    ctx.request_repaint();

                    promise
                })());
                match &self.comms_video {
                    Some(x) => {
                        self.tmp_bool = true;
                        match x.ready() {
                            Some(x) => match x {
                                Ok(x) => {
                                    //ui.label(&x.title);
                                    self.has_data = true;
                                    self.dwl_video = Some(x.clone());
                                    self.comms_video = None;
                                    //if self.dat.unwrap().thumbnail.is_some() {}
                                    //self.download_dat = Some(Ok(x.clone()));
                                }
                                Err(e) => {
                                    println!("err: {}", e);
                                    //ui.label(format!("error: {}", e));
                                    self.side_txt = format!("error: {}", e);
                                    self.comms_video = None;
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
        if !&self.side_txt.is_empty() {
            ui.label(&self.side_txt);
        }
        if self.has_data {
            //self.show_thumbnail(ctx, ui);
            if self.img_ready {
                self.show_image(ui);
            }
            egui::ScrollArea::vertical().show(ui, |ui| match self.dwl_video.as_ref() {
                Some(dat) => {
                    ui.horizontal(|ui| {
                        ui.label("title:");
                        label_wrap(ui, &dat.title);
                    });
                    ui.horizontal(|ui| {
                        ui.label("description:");
                        label_wrap(ui, &dat.description);
                    });
                    ui.horizontal(|ui| {
                        label_wrap(ui, format!("made by {}", dat.uploader));
                    });
                    match &dat.views {
                        Some(views) => {
                            ui.horizontal(|ui| {
                                ui.label(format!(
                                    "{} views",
                                    views.to_formatted_string(&Locale::en)
                                ));
                            });
                        }
                        None => {}
                    }
                    let dwl_button = ui.button("download");
                    if dwl_button.clicked() {
                        if self.action != Action::Youtube {
                            let chooser = FileDialog::new()
                                .set_location("~/Downloads/")
                                .set_filename(
                                    format!(
                                        "{}{}.mp3",
                                        match self.action {
                                            Action::Tiktok => "tik_",
                                            Action::Instagram => "ig_",
                                            _ => "",
                                        },
                                        &dat.title
                                    )
                                    .as_str(),
                                )
                                .show_save_single_file();

                            match chooser {
                                Ok(x) => match x {
                                    Some(x) => {
                                        self.dwl_path = Some(x);
                                        match reqwest::blocking::get(dat.url.clone().unwrap()) {
                                            Ok(x) => {
                                                let mp3 = self.dwl_path.clone().unwrap();
                                                let mut mp4 = mp3.clone();
                                                mp4.set_extension("mp4");
                                                let mut file = std::fs::File::create(&mp4).unwrap();
                                                file.write_all(&x.bytes().unwrap()).unwrap();
                                                match convert(&mp4, &mp3) {
                                                    Ok(mut x) => {
                                                        x.wait();
                                                        reveal(mp3);
                                                    }
                                                    Err(err) => {
                                                        MessageDialog::new()
                                                            .set_title("psk: converting error")
                                                            .set_text(&format!(
                                                                "wasnt able to convert: {}",
                                                                err
                                                            ))
                                                            .show_alert()
                                                            .unwrap();
                                                    }
                                                }
                                            }
                                            Err(e) => {
                                                println!("err: {}", e);
                                            }
                                        }
                                    }
                                    None => return,
                                },
                                Err(err) => {
                                    println!("error: {}", err);
                                }
                            }
                        } else {
                            let chooser = FileDialog::new()
                                .set_location("~/Downloads/")
                                .set_filename(format!("{}.mp3", &dat.title).as_str())
                                .show_save_single_file();
                            match chooser {
                                Ok(x) => match x {
                                    Some(path) => {
                                        let video_options = VideoOptions {
                                            quality: VideoQuality::Lowest,
                                            filter: VideoSearchOptions::Audio,
                                            ..Default::default()
                                        };
                                        let vid = Video::new_with_options(&self.url, video_options)
                                            .unwrap();
                                        match block_async!(vid.download(&path)) {
                                            Ok(x) => {
                                                reveal(path);
                                            }
                                            Err(err) => error(&err.to_string()),
                                        }
                                    }
                                    None => {}
                                },
                                Err(err) => println!("error: {}", err),
                            }
                        }
                    }
                }
                None => {}
            });
        }
    }

    // laggy piece of code
    pub fn show_thumbnail(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        match &self.dwl_video.as_ref().unwrap().thumbnail {
            Some(x) => {
                let promise = self.comms_image.get_or_insert((|| {
                    let ctx = ctx.clone();
                    let (sender, promise) = Promise::new();
                    //let res: Result<RetainedImage, String> = None;
                    //println!("{}", x);
                    let client = reqwest::blocking::Client::new();

                    match client.get(Url::parse(x).unwrap()).send() {
                        Ok(x) => {
                            let content_type =
                                x.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap();
                            //println!("{}", content_type);
                            if content_type.starts_with("image/") {
                                sender.send(Ok(RetainedImage::from_image_bytes(
                                    x.url().to_string(),
                                    &x.bytes().unwrap(),
                                )
                                .unwrap()));
                            } else {
                                sender.send(Err(format!(
                                    "unsupported content type: {}",
                                    content_type
                                )));
                            }
                            //ctx.request_repaint();
                            //sender.send(res);
                        }
                        Err(err) => {
                            //res = Err(format!("Error downloading image: {err:?}"));
                            sender.send(Err(format!("Error downloading image: {err:?}")));
                        }
                    }
                    promise
                })());
                match &self.comms_image {
                    Some(x) => match x.ready() {
                        Some(x) => match x {
                            Ok(x) => {
                                x.show_scaled(ui, 1.0);
                            }
                            Err(err) => {
                                ui.label(format!("err: {}", err));
                            }
                        },
                        None => {}
                    },
                    None => {
                        ui.label("fetching...");
                    }
                }
            }
            None => {
                ui.label("no image :/");
            }
        }
    }
    pub fn fetch_thumbnail(&mut self) {}
}
