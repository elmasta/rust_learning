pub fn rev_str(input: &str) -> String {
    let mut to_return = String::new();
    for (_, c) in input.char_indices().rev() {
        to_return.push(c);
    }
    return to_return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rev_str("ex: this is an example água");
        assert_eq!(result, "augá elpmaxe na si siht :xe");
    }
}
