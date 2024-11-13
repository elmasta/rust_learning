pub fn identity<T>(v: T) -> T {
    return v;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_with_int() {
        assert_eq!(identity(3), 3);
    }

    #[test]
    fn test_with_float() {
        assert_eq!(identity(1.0), 1.0);
    }

    #[test]
    fn test_with_str() {
        assert_eq!(identity("you"), "you");
    }

    #[test]
    fn test_with_struct() {
        let s = Point { x: 1, y: 2 };
        assert_eq!(identity(&s), &s);
    }
}
