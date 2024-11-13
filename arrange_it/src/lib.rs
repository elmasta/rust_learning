pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split_whitespace().collect();
    let mut word_list: Vec<String>= vec![String::new(); words.len()];
    for word in words {
        if let Some(index) = word.find(char::is_numeric) {
            let numeric_char = word[index..].chars().next().unwrap();
            if let Ok(number) = numeric_char.to_string().parse::<usize>() {
                let string_without_number = format!("{}{}", &word[..index], &word[index + 1..]);
                word_list[number-1] = string_without_number;
            } else {
                println!("error");
            }
        }
    }
    let mut to_return = String::new();
    println!("Contenu du vecteur : {:?}", word_list);
    for (index, value) in word_list.iter().enumerate() {
        to_return = to_return + value;
        if index != word_list.len()-1 {
            to_return += " "
        }
    }
    return to_return
}
