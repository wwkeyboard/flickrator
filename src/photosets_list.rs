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

#[cfg(test)]
mod tests {
    //use super::*;

    fn list_photosets_response() -> String {
        "{
            \"photosets\": {
                \"page\": 1,
                \"pages\": 1,
                \"perpage\": 500,
                \"total\": 1,
                \"photoset\": [
                    {
                        \"id\": \"72157712467772973\",
                        \"owner\": \"54171525@N00\",
                        \"username\": \"aaronosau.us\",
                        \"primary\": \"49312283763\",
                        \"secret\": \"4f8618f484\",
                        \"server\": \"65535\",
                        \"farm\": 66,
                        \"count_views\": \"5\",
                        \"count_comments\": \"0\",
                        \"count_photos\": 46,
                        \"count_videos\": 0,
                        \"title\": {
                            \"_content\": \"2020 - 366 project\"
                        },
                        \"description\": {
                            \"_content\": \"\"
                        },
                        \"can_comment\": 0,
                        \"date_create\": \"1577922046\",
                        \"date_update\": \"1581821103\",
                        \"photos\": 46,
                        \"videos\": 0,
                        \"visibility_can_see_set\": 1,
                        \"needs_interstitial\": 0
                    }
                ]
            }
        }"
        .to_owned()
    }
}
