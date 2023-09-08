use egui::{Align, Direction, Layout};

#[derive(Debug, Clone)]
pub struct DownloadableVideo {
    pub title: String,
    pub views: Option<i64>,
    pub description: String,
    pub url: Option<String>,
    pub thumbnail: Option<String>,
    pub duration: Option<i64>,
    pub uploader: String,
}

#[derive(PartialEq)]
pub enum Action {
    Home,
    Tiktok,
    Youtube,
    Instagram,
}

pub struct LayoutSettings {
    // Similar to the contents of `egui::Layout`
    main_dir: Direction,
    main_wrap: bool,
    cross_align: Align,
    cross_justify: bool,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self::top_down()
    }
}

impl LayoutSettings {
    fn top_down() -> Self {
        Self {
            main_dir: Direction::TopDown,
            main_wrap: false,
            cross_align: Align::Min,
            cross_justify: false,
        }
    }

    pub fn center() -> Self {
        Self {
            main_dir: Direction::LeftToRight,
            main_wrap: false,
            cross_align: Align::Center,
            cross_justify: false,
        }
    }

    pub fn layout(&self) -> Layout {
        Layout::from_main_dir_and_cross_align(self.main_dir, self.cross_align)
            .with_main_wrap(self.main_wrap)
            .with_cross_justify(self.cross_justify)
    }
}
