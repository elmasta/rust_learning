use std::ops::{Add, Sub, Mul};

pub trait Scalar: Add<Output = Self> 
                + Sub<Output = Self> 
                + Mul<Output = Self>
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

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
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

    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n]).collect()
    }
}

impl<T: Scalar + Mul<Output = T> + Add<Output = T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.number_of_cols() != other.number_of_rows() {
            return None;
        }

        let m = self.number_of_rows();
        let n = self.number_of_cols();
        let p = other.number_of_cols();

        let mut result = Matrix::zero(m, p);

        for i in 0..m {
            for j in 0..p {
                let mut sum = T::zero();
                for k in 0..n {
                    sum = sum + self.0[i][k] * other.0[k][j];
                }
                result.0[i][j] = sum;
            }
        }

        Some(result)
    }
}
