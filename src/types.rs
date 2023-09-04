#[derive(Debug, Clone)]
pub struct DownloadableVideo {
    pub title: String,
    pub views: Option<i64>,
    pub description: Option<String>,
    pub url: String,
    pub thumbnail: Option<String>,
    pub duration: Option<i64>,
    pub uploader: String,
}

#[derive(PartialEq)]
pub enum Action {
    Tiktok,
    Youtube,
    Instagram,
}
