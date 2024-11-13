pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    s.strip_prefix(prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_prefix() {
        assert_eq!(delete_prefix("john", "john wick"), Some(" wick"));

        assert_eq!(delete_prefix("ab", "b"), None);

        assert_eq!(delete_prefix("aa", "ab"), None);

        assert_eq!(delete_prefix("á©", "á©ab"), Some("ab"));
    }
}
