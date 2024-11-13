pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut to_return = String::new();
    let words: Vec<&str> = input.split_whitespace().collect();
    for (i, word) in words.clone().into_iter().enumerate() {
        to_return.push_str(&capitalize_first(word));
        if i != words.len()-1 {
            to_return.push(' ');
        }
    }
    return to_return
}

pub fn change_case(input: &str) -> String {
    let mut to_return = String::new();
    for c in input.chars() {
        if c.is_uppercase() {
            to_return.push_str(&c.to_lowercase().to_string());
        } else if c.is_lowercase() {
            to_return.push_str(&c.to_uppercase().to_string());
        } else {
            to_return.push_str(&c.to_string());
        }
    }
    return to_return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("this is working"), "This is working");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("this is a tittle"), "This Is A Tittle");
        assert_eq!(title_case("hello my name is carl"), "Hello My Name Is Carl");
    }

    #[test]
    fn test_change_case() {
        assert_eq!(change_case("PROgraming"), "proGRAMING");
        assert_eq!(change_case("heLLo THere"), "HEllO thERE");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
        assert_eq!(title_case(""), "");
        assert_eq!(change_case(""), "");
    }
}
