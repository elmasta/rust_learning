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

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut mat = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            mat[i][i] = T::one();
        }
        Matrix(mat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_property() {
        let matrix: Matrix<u32> = Matrix::zero(3, 4);
        let expected: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
        assert_eq!(matrix, expected);

        let matrix: Matrix<u32> = Matrix::zero(2, 2);
        let expected: Matrix<u32> = Matrix(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn identity_matrix() {
        let matrix: Matrix<u32> = Matrix::identity(2);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(matrix, expected);

        let matrix: Matrix<u32> = Matrix::identity(3);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
        assert_eq!(matrix, expected);
    }
}
