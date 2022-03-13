extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
pub fn construct_osu_profile_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
    let osu_profile_url = format!("https://osu.ppy.sh/u/{}", encoded_query);
    osu_profile_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_osu_profile_url() {
        let fake_query = "osu chocomint";
        assert_eq!(
            construct_osu_profile_url(fake_query),
            String::from("https://osu.ppy.sh/u/chocomint")
        );
    }

    #[test]
    fn test_construct_osu_profile_url_with_space() {
        let fake_query = "osu flower shop";
        assert_eq!(
            construct_osu_profile_url(fake_query),
            String::from("https://osu.ppy.sh/u/flower%20shop")
        );
    }
}