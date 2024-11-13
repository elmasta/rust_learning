use std::collections::HashMap;

pub fn score(s: &str) -> u32 {
    let mut score_table = HashMap::new();
    score_table.insert('A', 1);
    score_table.insert('B', 3);
    score_table.insert('C', 3);
    score_table.insert('D', 2);
    score_table.insert('E', 1);
    score_table.insert('F', 4);
    score_table.insert('G', 2);
    score_table.insert('H', 4);
    score_table.insert('I', 1);
    score_table.insert('J', 8);
    score_table.insert('K', 5);
    score_table.insert('L', 1);
    score_table.insert('M', 3);
    score_table.insert('N', 1);
    score_table.insert('O', 1);
    score_table.insert('P', 3);
    score_table.insert('Q', 10);
    score_table.insert('R', 1);
    score_table.insert('S', 1);
    score_table.insert('T', 1);
    score_table.insert('U', 1);
    score_table.insert('V', 4);
    score_table.insert('W', 4);
    score_table.insert('X', 8);
    score_table.insert('Y', 4);
    score_table.insert('Z', 10);
    let mut total_score: u32 = 0;
    for c in s.chars() {
        if let Some(&value) = score_table.get(&c.to_ascii_uppercase()) {
            total_score += value;
        }
    }
    total_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(score("a"), 1);
        assert_eq!(score("A"), 1);
        assert_eq!(score("h"), 4);
        assert_eq!(score("at"), 2);
        assert_eq!(score("Yes"), 6);
        assert_eq!(score("cellphones"), 17);
    }

    #[test]
    fn test_empty() {
        assert_eq!(score(""), 0);
        assert_eq!(score(" "), 0);
    }

    #[test]
    fn test_special() {
        assert_eq!(score("in Portugal NÃO stands for: no"), 30);
        assert_eq!(score("This is a test, comparação, têm Água?"), 36);
    }

    #[test]
    fn test_long() {
        assert_eq!(score("ThiS is A Test"), 14);
        assert_eq!(score("long sentences are working"), 34);
        assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
    }
}
