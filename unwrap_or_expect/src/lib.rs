pub enum Security {
	Unknown,
	High,
	Medium,
	Low,
	BlockServer,
}

pub fn fetch_data(server: Result<String, String>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap(), // Panic without custom message
        Security::High => server.expect("ERROR: program stops"), // Panic with error message
        Security::Medium => server.unwrap_or_else(|_| "WARNING: check the server".to_string()), // Return warning message
        Security::Low => server.unwrap_or_else(|err| format!("Not found: {}", err)), // Return not found message
        Security::BlockServer => {
            if let Ok(server_url) = server {
                panic!("{}", server_url) // Panic with the Ok value
            } else {
                server.unwrap() // Panic with the error message
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ERROR: program stops")]
    fn test_expect() {
        fetch_data(Err(String::new()), Security::High);
    }
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: \"ERROR CRITICAL\"")]
    fn test_unwrap() {
        fetch_data(Err("ERROR CRITICAL".to_string()), Security::Unknown);
    }
    #[test]
    #[should_panic(expected = "malicious_server.com")]
    fn test_unwrap_err() {
        fetch_data(
            Ok("malicious_server.com".to_string()),
            Security::BlockServer,
        );
    }
    #[test]
    fn test_unwrap_or() {
        assert_eq!(
            fetch_data(Err(String::new()), Security::Medium),
            "WARNING: check the server".to_string()
        );
    }
    #[test]
    fn test_unwrap_or_else() {
        assert_eq!(
            fetch_data(Err("another_server.com".to_string()), Security::Low),
            "Not found: another_server.com".to_string()
        );
    }
    #[test]
    fn test_ok() {
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::Low),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::Medium),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::High),
            "server.com"
        );
        assert_eq!(
            fetch_data(Ok("server.com".to_string()), Security::Unknown),
            "server.com"
        );
    }
}
