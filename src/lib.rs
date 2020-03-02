mod photo_info;
mod photosets_get;
mod photosets_list;

pub use photo_info::Response as PIGetResponse;
pub use photosets_get::Response as PSGetResponse;
pub use photosets_list::ListPhotosetsResult;

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

    pub fn photo_info_get_url(&self, id: String) -> String {
        format!(
            "{}&photo_id={}",
            self.base_url("flickr.photos.getInfo".to_owned()),
            id
        )
    }
}

fn strip_js_function(json: String) -> String {
    json[14..(json.len() - 1)].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_url() {
        assert_eq!(
            Config {
                user_id : "1234".to_string(),
                api_key : "abcd".to_string(),
            }.photosets_list_url(),
            "https://www.flickr.com/services/rest/?method=flickr.photosets.getList&format=json&api_key=abcd&user_id=1234"
        );
    }

    #[test]
    fn test_get_url() {
        assert_eq!(
            Config {
                user_id : "1234".to_string(),
                api_key : "abcd".to_string(),
            }.photosets_get_url("5678".to_string()),
            "https://www.flickr.com/services/rest/?method=flickr.photosets.getPhotos&format=json&api_key=abcd&user_id=1234&photoset_id=5678"
        );
    }

    #[test]
    fn test_get_photo_info_url() {
        assert_eq!(
            Config {
                user_id: "1234".to_string(),
                api_key: "abcd".to_string(),
            }.photo_info_get_url("5678".to_string()),
            "https://www.flickr.com/services/rest/?method=flickr.photos.getInfo&format=json&api_key=abcd&photo_id=5678"
        )
    }

    #[test]
    fn test_strip_js_function() {
        let dat = "jsonFlickrApi({\"foo\": \"bar\"})".to_string();
        let _got = strip_js_function(dat);
    }
}
