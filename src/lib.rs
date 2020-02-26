mod photosets_get;
mod photosets_list;

const BASE_URL: &str = "https://www.flickr.com/services/rest/";

pub struct Config {
    pub user_id: String,
    pub api_key: String,
}

/// Config for the application
impl Config {
    fn base_url(&self, method: String) -> String {
        format!(
            "{}?method={}&format=json&api_key={}",
            BASE_URL, method, self.api_key
        )
    }

    /// Builds the authenticated URL for pulling the Albums, called
    /// photosets by the API
    pub fn photosets_list_url(&self) -> String {
        format!(
            "{}&user_id={}",
            self.base_url("flickr.photosets.getList".to_owned()),
            self.user_id
        )
    }

    pub fn photosets_get_url(&self, id: String) -> String {
        format!(
            "{}&user_id={}&photoset_id={}",
            self.base_url("flickr.photosets.getPhotos".to_owned()),
            self.user_id,
            id
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
