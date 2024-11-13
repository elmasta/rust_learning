pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    vec[index].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let groceries: Vec<String> = vec![
            "milk".to_string(),
            "bread".to_string(),
            "water".to_string(),
            "wine".to_string(),
        ];
        assert_eq!(at_index(&groceries, 0), "milk");
    }

    #[test]
    fn test_insertions() {
        let mut groceries = Vec::new();
        insert(&mut groceries, "milk".to_string());
        assert_eq!(groceries, ["milk"]);
        insert(&mut groceries, "bread".to_string());
        assert_eq!(groceries, ["milk", "bread"]);
    }
}
