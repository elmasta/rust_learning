pub fn talking(input: &str) -> &str {
    let trimmed_input = input.trim();

    if trimmed_input.is_empty() {
        "Just say something!"
    } else if is_yelling(trimmed_input) {
        if trimmed_input.ends_with('?') {
            "Quiet, I am thinking!"
        } else {
            "There is no need to yell, calm down!"
        }
    } else if trimmed_input.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
}

fn is_yelling(input: &str) -> bool {
    let mut has_letters = false;
    let mut all_uppercase = true;

    for c in input.chars() {
        if c.is_alphabetic() {
            has_letters = true;
            if !c.is_uppercase() {
                all_uppercase = false;
                break;
            }
        }
    }

    has_letters && all_uppercase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yell() {
        assert_eq!(
            talking("JUST DO IT!"),
            "There is no need to yell, calm down!"
        );
        assert_eq!(
            talking("1, 2, 3 GO!"),
            "There is no need to yell, calm down!"
        );
        assert_eq!(
            talking("I LOVE YELLING"),
            "There is no need to yell, calm down!"
        );
        assert_eq!(
            talking("WJDAGSAF ASVF EVA VA"),
            "There is no need to yell, calm down!"
        );
    }

    #[test]
    fn test_question() {
        assert_eq!(talking("Hello how are you?"), "Sure.");
        assert_eq!(talking("Are you going to be OK?"), "Sure.");
        assert_eq!(talking("7?"), "Sure.");
        assert_eq!(talking("Like 15?"), "Sure.");
    }

    #[test]
    fn test_question_yelling() {
        assert_eq!(talking("WHAT'S GOING ON?"), "Quiet, I am thinking!");
        assert_eq!(talking("ARE YOU FINISHED?"), "Quiet, I am thinking!");
        assert_eq!(talking("WHAT DID I DO?"), "Quiet, I am thinking!");
        assert_eq!(talking("ARE YOU COMING?"), "Quiet, I am thinking!");
    }

    #[test]
    fn test_interesting() {
        assert_eq!(talking("something"), "Interesting");
        assert_eq!(talking("Wow that's good!"), "Interesting");
        assert_eq!(talking("Run far"), "Interesting");
        assert_eq!(talking("1 2 3 go!"), "Interesting");
        assert_eq!(talking("This is not ? a question."), "Interesting");
    }

    #[test]
    fn test_empty() {
        assert_eq!(talking(""), "Just say something!");
        assert_eq!(talking("										"), "Just say something!");
        assert_eq!(talking("          "), "Just say something!");
    }
}