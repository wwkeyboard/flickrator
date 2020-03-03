use crate::Config;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub photo: Photo,
}

#[derive(Deserialize, Debug)]
pub struct Photo {
    pub id: String,
    pub title: Title,
    pub tags: Tags,
    pub dates: Dates,
    pub urls: URLs,
}

#[derive(Deserialize, Debug)]
pub struct Title {
    #[serde(rename = "_content")]
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct Dates {
    #[serde(with = "taken_date_format")]
    pub taken: DateTime<Utc>,
}

// This is because of the screwy JSON format, I'm sure there is
// a better way to have Serde handle it. But that's a TODO for later.
#[derive(Deserialize, Debug)]
pub struct Tags {
    pub tag: Vec<Tag>,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub raw: String,
    pub _content: String,
}

#[derive(Deserialize, Debug)]
pub struct URLs {
    pub url: Vec<URL>,
}

#[derive(Deserialize, Debug)]
pub struct URL {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "_content")]
    pub content: String,
}

impl Response {
    pub fn parse(response: String) -> Result<Response> {
        let res = serde_json::from_str(&response)?;
        Ok(res)
    }

    pub async fn get(config: &Config, id: String) -> Result<Photo> {
        let url = config.photo_info_get_url(id);
        let resp = reqwest::get(&url).await?.text().await?;
        let resp = crate::strip_js_function(resp);

        let info = Response::parse(resp)?;
        Ok(info.photo)
    }

    pub fn tags(&self) -> Vec<String> {
        self.photo
            .tags
            .tag
            .iter()
            .map(|t| t.raw.to_owned())
            .collect::<Vec<String>>()
    }

    pub fn title(&self) -> String {
        self.photo.title.content.to_owned()
    }

    pub fn url(&self) -> String {
        self.photo
            .urls
            .url
            .first()
            .map_or("".to_owned(), |u| u.content.to_owned())
    }
}

mod taken_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_get_photo_info() {
        let got = Response::parse(get_photo_info_response()).unwrap();
        assert_eq!(got.photo.id, "49527151056");

        assert_eq!(got.photo.dates.taken.date().to_string(), "2020-02-10UTC");

        assert_eq!(
            got.url(),
            "https://www.flickr.com/photos/54171525@N00/49527151056/"
        );

        assert_eq!(got.tags(), vec!("clock"));
        assert_eq!(got.title(), "Urbana Clock 41/366");
    }

    #[test]
    fn test_collect_tags() {
        let got = Response::parse(get_photo_info_response()).unwrap();
        assert_eq!(got.tags(), vec!["clock"]);
    }

    fn get_photo_info_response() -> String {
        fs::read_to_string("./test_data/get_photo.json").unwrap()
    }
}
