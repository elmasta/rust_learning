pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut sorted_s1 = s1.chars().collect::<Vec<char>>();
    let mut sorted_s2 = s2.chars().collect::<Vec<char>>();

    sorted_s1.sort();
    sorted_s2.sort();

    sorted_s1 == sorted_s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
