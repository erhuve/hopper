extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
pub fn direct_twitter_url(query: &str) -> String {
    if query == "tw" {
        String::from("https://twitter.com")
    } else if &query[..4] == "tw @" {
        construct_twitter_profile_url(&query[4..])
    } else {
        // Assume a search, "tw <search>"
        construct_twitter_search_url(&query[3..])
    }
}

pub fn construct_twitter_profile_url(query: &str) -> String {
    let twitter_profile_url = format!("https://twitter.com/{}", query);
    twitter_profile_url
}

pub fn construct_twitter_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let twitter_search_url = format!("https://twitter.com/search?q={}", encoded_query);
    twitter_search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_twitter_url() {
        let fake_query = "tw";
        assert_eq!(
            direct_twitter_url(fake_query),
            String::from("https://twitter.com")
        );
    }

    #[test]
    fn test_direct_twitter_url_search() {
        let fake_query = "tw rust development";
        assert_eq!(
            direct_twitter_url(fake_query),
            String::from("https://twitter.com/search?q=rust%20development")
        );
    }

    #[test]
    fn test_direct_twitter_url_profile() {
        let fake_query = "tw @etheFlowerShop";
        assert_eq!(
            direct_twitter_url(fake_query),
            String::from("https://twitter.com/etheFlowerShop")
        );
    }

    #[test]
    fn test_construct_twitter_search_url() {
        let fake_query = "rust";
        assert_eq!(
            construct_twitter_search_url(fake_query),
            String::from("https://twitter.com/search?q=rust")
        );
    }

    #[test]
    fn test_construct_twitter_search_url_with_encoding() {
        let fake_query = "rust development";
        assert_eq!(
            construct_twitter_search_url(fake_query),
            String::from("https://twitter.com/search?q=rust%20development")
        );
    }

    #[test]
    fn test_construct_twitter_profile_url() {
        let fake_query = "etheFlowerShop";
        assert_eq!(
            construct_twitter_profile_url(fake_query),
            String::from("https://twitter.com/etheFlowerShop")
        );
    }
}