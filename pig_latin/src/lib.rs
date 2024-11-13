pub fn pig_latin(word: &str) -> String {
    if word.starts_with("qu") {
        return format!("{}qay", &word[1..]);
    }

    let mut first_vowel_index = None;
    for (i, c) in word.chars().enumerate() {
        if is_vowel(c) {
            first_vowel_index = Some(i);
            break;
        }
    }

    match first_vowel_index {
        Some(index) if index == 0 => format!("{}ay", word),
        Some(index) => {
            let (consonants, rest) = word.split_at(index);
            if rest.starts_with("u") && consonants.ends_with('q') {
                format!("{}{}uay", &rest[1..], &consonants)
            } else {
                format!("{}{}ay", rest, consonants)
            }
        }
        None => word.to_string(),
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_beginning_with_vowel() {
        assert_eq!(pig_latin(&String::from("apple")), "appleay");
        assert_eq!(pig_latin(&String::from("ear")), "earay");
        assert_eq!(pig_latin(&String::from("igloo")), "iglooay");
        assert_eq!(pig_latin(&String::from("object")), "objectay");
        assert_eq!(pig_latin(&String::from("under")), "underay");
        assert_eq!(pig_latin(&String::from("equal")), "equalay");
    }

    #[test]
    fn test_word_beginning_with_consonant() {
        assert_eq!(pig_latin(&String::from("queen")), "ueenqay");
        assert_eq!(pig_latin(&String::from("square")), "aresquay");
        assert_eq!(pig_latin(&String::from("pig")), "igpay");
        assert_eq!(pig_latin(&String::from("koala")), "oalakay");
        assert_eq!(pig_latin(&String::from("yellow")), "ellowyay");
        assert_eq!(pig_latin(&String::from("xenon")), "enonxay");
        assert_eq!(pig_latin(&String::from("qat")), "atqay");
        assert_eq!(pig_latin(&String::from("chair")), "airchay");
        assert_eq!(pig_latin(&String::from("therapy")), "erapythay");
        assert_eq!(pig_latin(&String::from("thrush")), "ushthray");
        assert_eq!(pig_latin(&String::from("school")), "oolschay");
        assert_eq!(pig_latin(&String::from("british")), "itishbray");
    }
}
