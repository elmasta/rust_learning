pub fn add_curry(nmb: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| nmb + x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let z = add_curry(0);
        assert_eq!(z(1999), 1999);
    }

    #[test]
    fn test_negatives() {
        let neg = add_curry(-10);
        assert_eq!(neg(6), -4);
        assert_eq!(neg(10), 0);
        assert_eq!(neg(600), 590);
        assert_eq!(neg(1000), 990);
        assert_eq!(neg(463), 453);
        assert_eq!(neg(400000000), 399999990);
    }

    #[test]
    fn test_add10() {
        let add10 = add_curry(10);
        assert_eq!(add10(6), 16);
        assert_eq!(add10(10), 20);
        assert_eq!(add10(600), 610);
        assert_eq!(add10(1000), 1010);
        assert_eq!(add10(463), 473);
        assert_eq!(add10(400000000), 400000010);
    }

    #[test]
    fn test_add250() {
        let add250 = add_curry(250);
        assert_eq!(add250(6), 256);
        assert_eq!(add250(10), 260);
        assert_eq!(add250(600), 850);
        assert_eq!(add250(1000), 1250);
        assert_eq!(add250(463), 713);
        assert_eq!(add250(400000000), 400000250);
    }
    #[test]
    fn test_add3960() {
        let add3960 = add_curry(3960);
        assert_eq!(add3960(6), 3966);
        assert_eq!(add3960(10), 3970);
        assert_eq!(add3960(600), 4560);
        assert_eq!(add3960(1000), 4960);
        assert_eq!(add3960(463), 4423);
        assert_eq!(add3960(400000000), 400003960);
    }
}
