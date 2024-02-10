// 値オブジェクトにする基準
// FirstName, LastName を struct Name として宣言

#![allow(dead_code)]

use anyhow::{bail, Result};
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Name {
    value: String,
}

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^[a-zA-Z]+$").unwrap();
        if re.is_match(&value) {
            Ok(Name {
                value: value.to_string(),
            })
        } else {
            bail!("許可されていない文字が含まれています。")
        }
    }
}

#[derive(Debug)]
struct FullName {
    first_name: Name,
    last_name: Name,
}

impl FullName {
    fn new(first_name: &str, last_name: &str) -> Self {
        FullName {
            first_name: Name {
                value: first_name.to_string(),
            },
            last_name: Name {
                value: last_name.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_name() {
        let valid_first_name = "John".parse::<Name>();
        assert_eq!(valid_first_name.is_err(), false);
    }

    fn is_invalid_name() {
        let invalid_first_name = "John123".parse::<Name>();
        assert_eq!(invalid_first_name.is_err(), true);
    }

    fn is_empty_name() {
        let empty_name = "".parse::<Name>();
        assert_eq!(empty_name.is_err(), true);
    }

    fn generate_full_name() {
        let full_name = FullName::new("John", "Smith");
        println!("{:?}", full_name);
    }
}
