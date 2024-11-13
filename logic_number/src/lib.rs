pub fn number_logic(mut num: u32) -> bool {
    let num_copy = num;
    let mut digits = vec![];
    loop {
        if num % 10 == 0 && num != 10 {
            break;
        } else if num == 10 {
            digits.push(1);
            digits.push(0);
        } else {
            digits.push(num % 10);
        }
        num = num / 10;
    }
    let mut res = 0;
    println!("{:?}", digits);
    for nmb in &digits {
        println!("loop");
        res += nmb.pow(digits.len().try_into().unwrap());
    }
    println!("{}", res);
    num_copy == res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        assert!(number_logic(0))
    }

    #[test]
    fn test_single_digit_numbers() {
        assert!(number_logic(5));
        assert!(number_logic(9))
    }

    #[test]
    fn test_two_digit_numbers() {
        assert!(!number_logic(10))
    }

    #[test]
    fn test_three_or_more_digit_number() {
        assert!(number_logic(153));
        assert!(!number_logic(100));
        assert!(number_logic(9474));
        assert!(!number_logic(9475));
        assert!(number_logic(9_926_315));
        assert!(!number_logic(9_926_316))
    }
}
