pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        assert_eq!(to_url("a b c d"), "a%20b%20c%20d");
    }
}
