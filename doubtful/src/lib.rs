pub fn doubtful(s: &mut String) {
    s.push('?');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function() {
        let mut s = "hello".to_string();
        let s_copy = s.clone();

        doubtful(&mut s);

        assert_eq!(s, s_copy + "?");
    }
}
