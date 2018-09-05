extern crate regex;

use regex::Regex;

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
}