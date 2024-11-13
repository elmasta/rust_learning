pub fn first_subword(mut s: String) -> String {
    let mut to_return = String::new();
    for (i, c) in s.char_indices() {  
        if (c.is_uppercase() || c == '_') && i != 0 {
            break;
        } else {
            to_return.push(c);
        }
    }
    return to_return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = first_subword("Change_you".to_string());
        assert_eq!(result, "Change");
    }
}
