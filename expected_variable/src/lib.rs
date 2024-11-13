use case::CaseExt;

fn is_camel_case(s: &str) -> bool {
    let words: Vec<&str> = s.split(|c: char| !c.is_alphanumeric()).collect();
    if words.is_empty() || words[0].is_empty() {
        return false;
    }
    
    // Vérifier le premier mot (peut commencer par une majuscule ou une minuscule)
    if !words[0].chars().next().unwrap().is_alphabetic() {
        return false;
    }
    
    // Vérifier les mots suivants (doivent commencer par une majuscule)
    words.iter().skip(1).all(|word| {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            first_char.is_uppercase() && chars.all(|c| c.is_lowercase())
        } else {
            false
        }
    })
}

fn is_snake_case(s: &str) -> bool {
    s.split('_').all(|word| word.chars().all(|c| c.is_lowercase() || c.is_uppercase()))
}

fn edit_distance(source: &str, target: &str) -> f64 {
    let source = source.to_lowercase();
    let target = target.to_lowercase();
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let m = source_chars.len();
    let n = target_chars.len();

    // Créer une matrice pour stocker les distances
    let mut distances = vec![vec![0; n + 1]; m + 1];

    // Initialiser les distances pour les cas de base
    for i in 0..=m {
        distances[i][0] = i;
    }
    for j in 0..=n {
        distances[0][j] = j;
    }

    // Remplir la matrice avec les distances
    for i in 1..=m {
        for j in 1..=n {
            let cost = if source_chars[i - 1] == target_chars[j - 1] { 0 } else { 1 };
            distances[i][j] = *[
                distances[i - 1][j] + 1,     // Suppression
                distances[i][j - 1] + 1,     // Insertion
                distances[i - 1][j - 1] + cost, // Substitution
            ].iter().min().unwrap();
        }
    }

    let edit_distance = distances[m][n];
    let max_len = m.max(n);
    let similarity_score = 1.0 - (edit_distance as f64 / max_len as f64);
    similarity_score * 100.0
}

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    println!("{}", compare);
    println!("{:?} {:?}", is_camel_case(compare), is_snake_case(compare));
    if is_camel_case(compare) || is_snake_case(compare) {
        let percent = edit_distance(compare, expected).round() as u8;
        //println!("{:?} {:?}", nmb_of_change, expected.len());
        //let mut percent = ((1.0 / expected.len() as f64) * 100.0).floor() as u8;
        //println!("{:?}", percent);
        //percent = 100 - percent;
        println!("{:?}", percent);
        
        //let percent = nmb_of_change / compare.len() * 100.0;
        
        if percent > 50 {
            //percent = 100 - percent;
            //println!("{:?}", percent);
            Some(format!("{}%", percent))
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_variable_case() {
        let mut result = expected_variable("It is simply not a variable case", "gonnaFail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("do-not-use-dashes", "do-not-use-dashes");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("Not a variable case", "needs to fail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("This should be None", "needs to fail");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("Do not use spaces", "Do not use spaces");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );
    }

    #[test]
    fn incorrect_names() {
        let mut result = expected_variable("it_is_done", "almost_there");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );

        result = expected_variable("frankenstein", "Dracula");
        assert!(
            result.is_none(),
            "Should have been None and not, {:?}",
            result
        );
    }

    #[test]
    fn one_hundred_percent() {
        assert_eq!(
            "100%",
            expected_variable("great_variable", "great_variable").unwrap()
        );
        assert_eq!("100%", expected_variable("SpOtON", "SpotOn").unwrap());
        assert_eq!(
            "100%",
            expected_variable("Another_great_variable", "Another_great_variable").unwrap()
        );
    }

    #[test]
    fn passing_tests() {
        assert_eq!("88%", expected_variable("soClose", "So_Close").unwrap());
        assert_eq!("73%", expected_variable("lets_try", "lets_try_it").unwrap());
        assert_eq!("64%", expected_variable("GoodJob", "VeryGoodJob").unwrap());
        assert_eq!("64%", expected_variable("GoodJob", "VeryGoodJob").unwrap());
        assert_eq!(
            "67%",
            expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch").unwrap()
        );
    }
}
