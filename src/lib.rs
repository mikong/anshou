extern crate regex;
extern crate publicsuffix;
extern crate url;

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use publicsuffix::List;
use url::Url;

lazy_static! {
    static ref SUFFIX_LIST: List = List::fetch().unwrap();
}

pub fn is_valid(password: &str) -> bool {
    match password.chars().nth(0) {
        Some(c) => {
            c.is_lowercase() &&
            password.chars().any(|c| c.is_uppercase()) &&
            password.chars().any(|c| c.is_numeric())
        },
        None => false,
    }
}

pub fn has_scheme(s: &str) -> bool {
    let re = Regex::new(r"^([a-zA-Z][a-zA-Z0-9+-.]*)://").unwrap();

    re.is_match(s)
}

pub fn get_domain_root(input: &str) -> Result<String, &'static str> {
    let input = if has_scheme(input) {
        let input_url = match Url::parse(input) {
            Ok(url) => url,
            Err(_) => return Err("can't parse URL"),
        };
        match input_url.host_str() {
            Some(s) => s.to_string(),
            None => return Err("can't parse URL"),
        }
    } else {
        input.to_string()
    };

    let domain = match SUFFIX_LIST.parse_domain(&input) {
        Ok(d) => d,
        Err(_) => return Err("can't parse domain"),
    };

    match domain.root() {
        Some(root) => Ok(root.to_string()),
        None => Err("can't parse domain"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn valid_password() {
        assert!(is_valid("ri39yFvq7c"));
    }

    #[test]
    fn invalid_passwords() {
        let empty = "";
        assert!(!is_valid(empty));

        let starts_with_uppercase = "Ri39yFvq7c";
        assert!(!is_valid(starts_with_uppercase));

        let no_numeric = "riEgyFvqlc";
        assert!(!is_valid(no_numeric));

        let no_uppercase = "ri39yfvq7c";
        assert!(!is_valid(no_uppercase));
    }

    #[test]
    fn check_scheme() {
        let http = "http://www.example.com";
        assert!(has_scheme(http));

        let https = "https://example.com";
        assert!(has_scheme(https));

        let complex = "HtT+p-50.00://complex.com";
        assert!(has_scheme(complex));

        let number = "9ttp://example.com";
        assert!(!has_scheme(number));

        let none = "example.com";
        assert!(!has_scheme(none));
    }

    #[test]
    fn extract_domain_root() {
        let domain = "www.google.com";
        assert_eq!(get_domain_root(domain).unwrap(), "google.com".to_string());

        let country_coded = "www.amazon.co.jp";
        assert_eq!(get_domain_root(country_coded).unwrap(), "amazon.co.jp".to_string());

        let url = "http://www.google.com";
        assert_eq!(get_domain_root(url).unwrap(), "google.com".to_string());
    }
}
