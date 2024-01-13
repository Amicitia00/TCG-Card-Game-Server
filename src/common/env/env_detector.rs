use dotenv::dotenv;
use std::env;

pub struct EnvDetector;

impl EnvDetector {
    pub fn get_var(key: &str) -> Option<String> {
        dotenv().ok();

        match env::var(key) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    pub fn get_port() -> Option<String> {
        Self::get_var("TARET_PORT")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_port() {
        env::set_var("TARET_PORT", "1234");

        assert_eq!(EnvDetector::get_port(), Some("1234".to_string()));
    }
}