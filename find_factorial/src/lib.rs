pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        return 1;
    } else {
        let mut to_return: u64 = 1;
        for nmb in 1..num+1 {
            to_return = to_return * nmb;
        }
        return to_return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = factorial(10);
        assert_eq!(result, 3628800);
    }
}
