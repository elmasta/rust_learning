pub fn rotate(input: &str, key: i8) -> String {
    input.chars()
    .map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let offset = c as u8 - base;
            let rotated = (offset as i8 + key) % 26;
            let rotated = if rotated < 0 { rotated + 26 } else { rotated };
            (base + rotated as u8) as char
        } else {
            c
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!("z", rotate("m", 13));
        assert_eq!("n", rotate("m", 1));
        assert_eq!("a", rotate("a", 26));
        assert_eq!("z", rotate("a", 25));
        assert_eq!("b", rotate("l", 16));
        assert_eq!("j", rotate("j", 0));
        assert_eq!("RNXX", rotate("MISS", 5));
        assert_eq!("M J Q Q T", rotate("H E L L O", 5));
    }

    #[test]
    fn test_all_letters() {
        assert_eq!(
            "Gur svir obkvat jvmneqf whzc dhvpxyl.",
            rotate("The five boxing wizards jump quickly.", 13)
        );
    }

    #[test]
    fn test_numbers_punctuation() {
        assert_eq!(
            "Xiwxmrk amxl ryqfivw 1 2 3",
            rotate("Testing with numbers 1 2 3", 4)
        );
        assert_eq!("Gzo\'n zvo, Bmviyhv!", rotate("Let\'s eat, Grandma!", 21));
    }

    #[test]
    fn test_negative() {
        assert_eq!("z", rotate("a", -1));
        assert_eq!("W", rotate("A", -4));
        assert_eq!("Fqefuzs", rotate("Testing", -14));
    }
}