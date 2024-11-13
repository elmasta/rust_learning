pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut to_vec = String::new();
    let mut to_return: Vec<String>= vec![];
    for name in names {
        let words: Vec<&str> = name.split_whitespace().collect();
        for word in words {
            to_vec.push_str(&word.chars().next().map(|c| c.to_string()).unwrap_or_default());
            to_vec.push_str(". ")
        }
        to_vec.pop();
        to_return.push(to_vec.clone());
        to_vec.clear();
    }
    return to_return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    struct Test<'a> {
        names: Vec<&'a str>,
        result: Vec<&'a str>,
    }

    #[test]
    fn test_function() {
        let cases = vec![
            Test {
                names: vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"],
                result: vec!["H. P.", "S. E.", "J. L.", "B. O."],
            },
            Test {
                names: vec![
                    "James John",
                    "David Joseph",
                    "Matthew Brian",
                    "Jacob Sousa",
                    "Bruce Banner",
                    "Scarlett Johansson",
                    "Graydon Hoare",
                ],
                result: vec![
                    "J. J.", "D. J.", "M. B.", "J. S.", "B. B.", "S. J.", "G. H.",
                ],
            },
        ];

        for v in cases {
            assert_eq!(
                initials(v.names),
                v.result
                    .iter()
                    .map(|ele| ele.to_string())
                    .collect::<Vec<String>>()
            );
        }
    }
}
