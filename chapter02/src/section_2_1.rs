// システム固有の値を表現する「値オブジェクト」

#![allow(dead_code)]

#[derive(Debug)]
struct FullName {
    first_name: String,
    last_name: String,
}

impl FullName {
    fn new(first_name: &str, last_name: &str) -> Self {
        FullName {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    fn first_name(&self) -> &str {
        &self.first_name
    }

    fn last_name(&self) -> &str {
        &self.last_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_full_name() {
        let first_name = "John";
        let last_name = "Smith";
        let full_name = FullName::new(first_name, last_name);

        assert_eq!(full_name.first_name(), "John");
        assert_eq!(full_name.last_name(), "Smith");
    }
}
