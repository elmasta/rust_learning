use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    for x in list {
        sum = sum + x;
    }

    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {

    let mut sorted_list = list.clone();
    sorted_list.sort();
    let mid = sorted_list.len() / 2;
    if sorted_list.len() % 2 == 0 {
        (sorted_list[mid - 1] + sorted_list[mid]) / 2
    } else {
        sorted_list[mid]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut occurrences = HashMap::new();

    for &num in list {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    let mut mode = None;
    let mut max_count = 0;

    for (&num, &count) in &occurrences {
        if count > max_count {
            max_count = count;
            mode = Some(num);
        }
    }

    return mode.expect("REASON");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_mean() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];
        assert!(approx_eq(mean(&v), 3.857142857142857));
    }

    #[test]
    fn test_median() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];
        assert_eq!(median(&v), 4);

        let aux1 = vec![2, 1, 5, 2, 7, 4];
        assert_eq!(median(&aux1), 3, "tested with {:?}", aux1);

        let aux2 = vec![1, 7, 5, 5, 6, 4];
        assert_eq!(median(&aux2), 5, "tested with {:?}", aux2);
    }

    #[test]
    fn test_mode() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];
        assert_eq!(mode(&v), 5);
    }
}
