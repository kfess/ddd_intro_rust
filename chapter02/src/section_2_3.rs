// 値オブジェクトにする基準
// FirstName, LastName を struct Name として宣言

#![allow(dead_code)]

use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Name {
    value: String,
}

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if NAME_REGEX.is_match(&value) {
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
    fn new(first_name: &str, last_name: &str) -> Result<Self, anyhow::Error> {
        Ok(FullName {
            first_name: first_name.parse()?,
            last_name: last_name.parse()?,
        })
    }

    fn first_name(&self) -> &str {
        &self.first_name.value
    }

    fn last_name(&self) -> &str {
        &self.last_name.value
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

    #[test]
    fn is_invalid_name() {
        let invalid_first_name = "John123".parse::<Name>();
        assert_eq!(invalid_first_name.is_err(), true);
    }

    #[test]
    fn is_empty_name() {
        let empty_name = "".parse::<Name>();
        assert_eq!(empty_name.is_err(), true);
    }

    #[test]
    fn generate_full_name() {
        assert!(FullName::new("John123", "Smith123").is_err());
    }
}
