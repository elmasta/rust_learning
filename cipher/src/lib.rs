#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub expected: String,
    pub validation: bool,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError{expected, validation}
    }
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let original_rev: String = original.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_lowercase() {
                    (b'z' - (c as u8 - b'a')) as char
                } else {
                    (b'Z' - (c as u8 - b'A')) as char
                }
            } else {
                c
            }
        })
        .collect();
    if original_rev == ciphered {
        Some(Ok(true))
    } else if original.len() == 0 || ciphered.len() == 0 {
        None
    } else {
        Some(Err(CipherError::new(false, original_rev)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok_values() {
        assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Some(Ok(true)));
        assert_eq!(cipher("asdasd", "zhwzhw"), Some(Ok(true)));
        assert_eq!(cipher("3(/&%fsd 32das", "3(/&%uhw 32wzh"), Some(Ok(true)));
    }

    #[test]
    fn test_empty_values() {
        assert_eq!(cipher("", "1Svool 2dliow!"), None);
        assert_eq!(cipher("1Hello 2world!", ""), None);
    }

    #[test]
    fn test_errors() {
        assert_eq!(
            cipher("1Hello 2world!", "1svool 2dliow!"),
            Some(Err(CipherError {
                validation: false,
                expected: String::from("1Svool 2dliow!")
            }))
        );
        assert_eq!(
            cipher("asdasd", "lkdas"),
            Some(Err(CipherError {
                validation: false,
                expected: String::from("zhwzhw")
            }))
        );
        assert_eq!(
            cipher("3(/&%sd 32das", "3(/&%uhw 32wzh"),
            Some(Err(CipherError {
                validation: false,
                expected: String::from("3(/&%hw 32wzh")
            }))
        );
    }
}
