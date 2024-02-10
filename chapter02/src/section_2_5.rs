// 値オブジェクトを採用するモチベーション

#![allow(dead_code)]

use anyhow::Error;
use std::fmt;

#[derive(Debug)]
struct ModelNumber {
    product_code: String,
    branch: String,
    lot: String,
}

impl ModelNumber {
    fn new(product_code: &str, branch: &str, lot: &str) -> Result<Self, Error> {
        Ok(ModelNumber {
            product_code: product_code.to_string(),
            branch: branch.to_string(),
            lot: lot.to_string(),
        })
    }
}

impl fmt::Display for ModelNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.product_code, self.branch, self.lot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_model_number() {
        let model_number = ModelNumber::new("a", "b", "c");
        assert!(model_number.is_ok());
    }

    #[test]
    fn is_valid_output() {
        let model_number = ModelNumber::new("a", "b", "c").unwrap();
        assert_eq!(format!("{model_number}"), "a-b-c");
    }
}
