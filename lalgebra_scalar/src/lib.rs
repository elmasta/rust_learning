use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: Add<Output = Self> 
                + Sub<Output = Self> 
                + Mul<Output = Self> 
                + Div<Output = Self>
                + Copy
                + Clone
                + PartialEq 
                + PartialOrd {
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for u64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for i64 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

impl Scalar for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_u32() {
        let a: u32 = u32::zero();
        assert_eq!(a, 0 as u32);

        let b = u32::one();
        assert_eq!(b, 1 as u32);
    }

    #[test]
    fn scalar_u64() {
        let a = u64::zero();
        assert_eq!(a, 0 as u64);

        let b = u64::one();
        assert_eq!(b, 1 as u64);
    }

    #[test]
    fn scalar_i32() {
        let a: i32 = i32::zero();
        assert_eq!(a, 0 as i32);

        let b = i32::one();
        assert_eq!(b, 1 as i32);
    }

    #[test]
    fn scalar_i64() {
        let a: i64 = i64::zero();
        assert_eq!(a, 0 as i64);

        let b = i64::one();
        assert_eq!(b, 1 as i64);
    }

    #[test]
    fn scalar_f32() {
        let zero = f32::zero();
        assert_eq!(zero, 0.0);
        let one = f32::one();
        assert_eq!(one, 1.0);
    }

    #[test]
    fn scalar_f64() {
        let zero = f64::zero();
        assert_eq!(zero, 0.0);
        let one = f64::one();
        assert_eq!(one, 1.0);
    }
}
