pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {

    let result: Vec<u32> = s
        .split_whitespace() // Séparer la chaîne en sous-chaînes par les espaces
        .map(|s| {
            if let Some(pos) = s.find('k') {
                // Si la sous-chaîne contient 'k', convertir la partie numérique en f32 et multiplier par 1000
                let num_str = &s[..pos];
                let num: f32 = num_str.parse().expect("Failed to parse number");
                (num * 1000.0) as u32
            } else {
                // Sinon, convertir simplement la sous-chaîne en u32
                s.parse().expect("Failed to parse number")
            }
        })
        .collect(); // Collecter les résultats dans un vecteur
    Box::new(result)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_transform() {
        let new_str = String::from("5.5k 8.9k 32");
        let new_str_1 = String::from("6.8k 13.5k");
        let new_str_2 = String::from("20.3k 3.8k 7.7k 992");

        let a = transform_and_save_on_heap(new_str);
        let b = transform_and_save_on_heap(new_str_1);
        let c = transform_and_save_on_heap(new_str_2);

        assert_eq!(a, Box::new(vec![5500, 8900, 32]));
        assert_eq!(b, Box::new(vec![6800, 13500]));
        assert_eq!(c, Box::new(vec![20300, 3800, 7700, 992]));
        assert_eq!(mem::size_of_val(&a), 8);
        assert_eq!(mem::size_of_val(&b), 8);
        assert_eq!(mem::size_of_val(&c), 8);
    }

    #[test]
    fn test_take_value_from_box() {
        let new_str = String::from("5.5k 8.9k 32");
        let new_str_1 = String::from("6.8k 13.5k");
        let new_str_2 = String::from("20.3k 3.8k 7.7k 992");
        let a = take_value_ownership(transform_and_save_on_heap(new_str));
        let b = take_value_ownership(transform_and_save_on_heap(new_str_1));
        let c = take_value_ownership(transform_and_save_on_heap(new_str_2));

        assert_eq!(a, vec![5500, 8900, 32]);
        assert_eq!(b, vec![6800, 13500]);
        assert_eq!(c, vec![20300, 3800, 7700, 992]);
        assert_eq!(mem::size_of_val(&a), 24);
        assert_eq!(mem::size_of_val(&b), 24);
        assert_eq!(mem::size_of_val(&c), 24);
    }
}
