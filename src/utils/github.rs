extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
pub fn direct_github_url(query: &str) -> String {
    if query == "gh" {
        String::from("https://github.com")
    } else {
        // Assume a page "gh <page>"
        construct_github_page_url(&query[3..])
    }
}

pub fn construct_github_page_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let github_page_url = format!("https://github.com/{}", encoded_query);
    github_page_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_github_url() {
        let fake_query = "gh";
        assert_eq!(
            direct_github_url(fake_query),
            String::from("https://github.com")
        );
    }

    #[test]
    fn test_direct_github_url_profile() {
        let fake_query = "gh erhuve";
        assert_eq!(
            direct_github_url(fake_query),
            String::from("https://github.com/erhuve")
        );
    }

    #[test]
    fn test_direct_github_url_repo() {
        let fake_query = "gh erhuve/flowershop";
        assert_eq!(
            direct_github_url(fake_query),
            String::from("https://github.com/erhuve/flowershop")
        );
    }

}