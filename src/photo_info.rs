use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub photo: Photo,
}

#[derive(Deserialize, Debug)]
pub struct Photo {
    pub id: String,
    pub tags: Tags,
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

impl Response {
    pub fn parse(response: String) -> Result<Response> {
        let res = serde_json::from_str(&response)?;
        Ok(res)
    }

    pub fn tags(&self) -> Vec<String> {
        self.photo
            .tags
            .tag
            .iter()
            .map(|t| t.raw.to_owned())
            .collect::<Vec<String>>()
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
