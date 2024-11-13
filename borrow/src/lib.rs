pub fn str_len<T: AsRef<str>>(s: T) -> usize {
    return s.as_ref().chars().count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
