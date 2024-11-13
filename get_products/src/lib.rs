pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut to_return: Vec<usize> = vec![];
    for (index, _) in arr.iter().enumerate() {
        let mut total: usize = 0;
        for (i, nmb) in arr.iter().enumerate() {
            if i != index {
                if total == 0 {
                    total = *nmb;
                } else {
                    total = total * *nmb;
                }
            }
        }
        if total != 0 {
            to_return.push(total);
        }
    }
    to_return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple() {
        let arr: Vec<usize> = vec![1, 7, 3, 4];
        let output = get_products(arr);
        let arr2: Vec<usize> = vec![10, 3, 5, 6, 2];
        let output2 = get_products(arr2);
        assert_eq!(output, vec![84, 12, 28, 21]);
        assert_eq!(output2, vec![180, 600, 360, 300, 900]);
    }

    #[test]
    fn test_empty_case() {
        let arr: Vec<usize> = Vec::new();
        let output = get_products(arr);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_single_case() {
        let arr: Vec<usize> = vec![2];
        let output = get_products(arr);
        assert_eq!(output, vec![]);
    }
}
