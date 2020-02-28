use crate::Config;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub photoset: Photoset,
}

#[derive(Deserialize, Debug)]
pub struct Photoset {
    pub id: String,
    pub primary: String,
    pub owner: String,
    pub ownername: String,
    pub photo: Vec<Photo>,
}

#[derive(Deserialize, Debug)]
pub struct Photo {
    pub id: String,
    pub secret: String,
    pub server: String,
    pub title: String,
    pub isprimary: String,
}

impl Response {
    pub fn from_response(response: String) -> Result<Response> {
        let res = serde_json::from_str(&response)?;
        Ok(res)
    }

    pub async fn get(config: Config, id: String) -> Result<Photoset> {
        let url = config.photosets_get_url(id);
        let resp = reqwest::get(&url).await?.text().await?;
        let resp = crate::strip_js_function(resp);

        let ps = Response::from_response(resp)?;
        Ok(ps.photoset)
    }
}

impl Photoset {
    pub fn ids(&self) -> Vec<String> {
        self.photo
            .iter()
            .map(|p| p.id.to_owned())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ids() {
        let ps = Photoset {
            id: "1234".to_owned(),
            primary: "abcd".to_owned(),
            owner: "abcd".to_owned(),
            ownername: "abcd".to_owned(),
            photo: vec![
                Photo {
                    id: "1".to_string(),
                    secret: "abcd".to_owned(),
                    server: "abcd".to_owned(),
                    title: "abcd".to_owned(),
                    isprimary: "true".to_owned(),
                },
                Photo {
                    id: "2".to_string(),
                    secret: "abcd".to_owned(),
                    server: "abcd".to_owned(),
                    title: "abcd".to_owned(),
                    isprimary: "true".to_owned(),
                },
            ],
        };
        let expected = vec!["1".to_owned(), "2".to_owned()];

        assert_eq!(ps.ids(), expected)
    }

    #[test]
    fn parse_get_photosets_response() {
        let got = Response::from_response(get_photosets_response()).unwrap();

        assert_eq!(got.photoset.id, "72157712467772973");
        assert_eq!(got.photoset.photo.len(), 1);
    }

    fn get_photosets_response() -> String {
        "{
  \"photoset\": {
    \"id\": \"72157712467772973\",
    \"primary\": \"49312283763\",
    \"owner\": \"54171525@N00\",
    \"ownername\": \"aaronosau.us\",
    \"photo\": [
      {
        \"id\": \"49312283763\",
        \"secret\": \"ABCD1234\",
        \"server\": \"65535\",
        \"farm\": 66,
        \"title\": \"A lonely tree has keept some leaves 1/366\",
        \"isprimary\": \"1\",
        \"ispublic\": 1,
        \"isfriend\": 0,
        \"isfamily\": 0
      }
    ],
    \"page\": 1,
    \"per_page\": 500,
    \"perpage\": 500,
    \"pages\": 1,
    \"title\": \"2020 - 366 project\",
    \"total\": 46
  },
  \"stat\": \"ok\"
}"
        .to_owned()
    }
}
