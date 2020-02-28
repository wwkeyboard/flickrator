use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub photo: Photo,
}

#[derive(Deserialize, Debug)]
pub struct Photo {
    pub id: String,
    pub tags: Vec<Tag>,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    fn test_parse_get_photo_info() {
        let got = Response::parse(get_photo_info_response());
        println!(got);
    }

    fn get_photo_info_response() -> String {
        let file = File::open("../test_data/get_photo.json").unwrap();
    }
}
