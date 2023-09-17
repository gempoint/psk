#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// https://github.com/emilk/egui/discussions/1574#discussioncomment-5840144
pub fn load_icon() -> eframe::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let icon = include_bytes!("../assets/icon.png");
        let image = image::load_from_memory(icon)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    eframe::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use std::{
        backtrace::Backtrace,
        process::{exit, Command},
    };

    use native_dialog::{MessageDialog, MessageType};

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
                        // check for ffmpeg bc apparently i cant figure out how to convert shit with code

    let check = Command::new("ffmpeg").arg("-version").spawn();
    match check {
        Ok(x) => {}
        Err(err) => {
            MessageDialog::new()
                .set_title("psk: missing exec")
                .set_text("missing ffmpeg executable")
                .show_alert()
                .unwrap();
            open::that("https://ffmpeg.org/download.html").unwrap();
            exit(1);
        }
    }

    let res = std::panic::catch_unwind(|| {
        let native_options = eframe::NativeOptions {
            initial_window_size: Some([640.0, 512.0].into()),
            icon_data: Some(load_icon()),
            ..Default::default()
        };

        eframe::run_native(
            "psk",
            native_options,
            Box::new(|cc| Box::new(psf::PSKGui::new(cc))),
        );
    });
    match res {
        Ok(x) => {}
        Err(err) => {
            //idk what to do here
            println!("{:#?}", err.type_id());
            println!("{:#?}", err);
            let msg = MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("psk: crash error")
                .set_text(format!("{:#?}", Backtrace::capture()).as_str())
                .show_alert()
                .unwrap();
        }
    }
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
