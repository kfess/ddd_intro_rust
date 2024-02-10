// 値の性質と値オブジェクトの実装

#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq, Eq)]
struct FullName {
    first_name: String,
    last_name: String,
    middle_name: String,
}

impl FullName {
    fn new(first_name: &str, last_name: &str, middle_name: &str) -> Self {
        FullName {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            middle_name: middle_name.to_string(),
        }
    }

    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name
    }

    fn middle_name(&self) -> &str {
        &self.middle_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_full_names() {
        let full_name_1 = FullName::new("John", "Smith", "A");
        let full_name_2 = FullName::new("Naruse", "Masanobu", "A");

        assert_eq!(full_name_1, full_name_1.clone());
        assert_ne!(full_name_1, full_name_2);
    }
}
