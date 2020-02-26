use serde::Deserialize;
/// Container for the JSON returned from the API for a photoset Should
/// probably not have this be public. This is an artifact of my not
/// knowing Serde well enough.
#[derive(Deserialize, Debug)]
pub struct ListPhotosetsResult {
    pub photosets: Photosets,
}

/// Container for photosets
#[derive(Deserialize, Debug)]
pub struct Photosets {
    pub page: u64,
    pub pages: u64,
    pub perpage: u64,
    pub total: u64,
    pub photoset: Vec<Photoset>,
}

#[derive(Deserialize, Debug)]
pub struct Photoset {
    pub id: String,
    pub owner: String,
    pub username: String,
    pub primary: String,
    pub secret: String,
    pub server: String,
    pub farm: u64,
    pub count_views: String,
    pub count_comments: String,
    pub count_photos: u64,
    pub count_videos: u64,
    pub title: Title,
    pub date_create: String,
    pub date_update: String,
}

#[derive(Deserialize, Debug)]
pub struct Title {
    pub _content: String,
}
