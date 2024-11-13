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

impl<T: Scalar> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let mut result = self.0.clone();
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result[i][j] = result[i][j] + other.0[i][j];
            }
        }
        Some(Matrix(result))
    }
}

impl<T: Scalar> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }

        let mut result = self.0.clone();
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                result[i][j] = result[i][j] - other.0[i][j];
            }
        }
        Some(Matrix(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let expected = Matrix(vec![vec![2, 2], vec![2, 2]]);
        assert_eq!(matrix + matrix_2, Some(expected));

        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        assert_eq!(matrix + matrix_2, None);
    }

    #[test]
    fn subtraction() {
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let expected = Matrix(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(matrix - matrix_2, Some(expected));

        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        assert_eq!(matrix - matrix_2, None);
    }
}
