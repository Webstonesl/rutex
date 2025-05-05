use crate::error::Error;
use std::fmt::{Debug, Display};
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::panic::{RefUnwindSafe, UnwindSafe};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorKind {
    DivideByZero,
}
impl Into<crate::error::ErrorKind> for ErrorKind {
    fn into(self) -> crate::error::ErrorKind {
        return crate::error::ErrorKind::MathsError(self);
    }
}
pub trait Zero {
    const ZERO: Self;
}
pub trait One {
    const ONE: Self;
}
pub trait Trig: NumberFunctions {
    type Output;
    fn sin(self) -> <Self as Trig>::Output;
    fn cos(self) -> <Self as Trig>::Output;
    fn tan(self) -> <Self as Trig>::Output;
}

pub trait NumberFunctions:
    Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Output = Self>
    + Clone
    + Copy
    + Debug
    + Display
    + Sized
    + Zero
    + One
    + Sum
    + Neg<Output = Self>
    + PartialEq
    + PartialOrd
{
}

impl One for u128 {
    const ONE: Self = 1;
}

impl One for u64 {
    const ONE: Self = 1;
}

impl One for u32 {
    const ONE: Self = 1;
}

impl One for u16 {
    const ONE: Self = 1;
}
impl One for u8 {
    const ONE: Self = 1;
}

impl One for usize {
    const ONE: Self = 1;
}

impl One for i128 {
    const ONE: Self = 1;
}

impl One for i64 {
    const ONE: Self = 1;
}

impl One for i32 {
    const ONE: Self = 1;
}

impl One for i16 {
    const ONE: Self = 1;
}
impl One for i8 {
    const ONE: Self = 1;
}

impl One for isize {
    const ONE: Self = 1;
}

impl Zero for u128 {
    const ZERO: Self = 0;
}

impl Zero for u64 {
    const ZERO: Self = 0;
}

impl Zero for u32 {
    const ZERO: Self = 0;
}

impl Zero for u16 {
    const ZERO: Self = 0;
}
impl Zero for u8 {
    const ZERO: Self = 0;
}

impl Zero for usize {
    const ZERO: Self = 0;
}

impl Zero for i128 {
    const ZERO: Self = 0;
}

impl Zero for i64 {
    const ZERO: Self = 0;
}

impl Zero for i32 {
    const ZERO: Self = 0;
}

impl Zero for i16 {
    const ZERO: Self = 0;
}
impl Zero for i8 {
    const ZERO: Self = 0;
}

impl Zero for isize {
    const ZERO: Self = 0;
}

impl Zero for f32 {
    const ZERO: Self = 0.0;
}

impl Zero for f64 {
    const ZERO: Self = 0.0;
}

impl One for f32 {
    const ONE: Self = 1.0;
}

impl One for f64 {
    const ONE: Self = 1.0;
}

impl<T> NumberFunctions for T where
    T: Add<Self, Output = Self>
        + Sub<Self, Output = Self>
        + Mul<Output = Self>
        + Clone
        + Copy
        + Debug
        + Display
        + Sized
        + Zero
        + One
        + Sum
        + Neg<Output = Self>
        + PartialEq
        + PartialOrd
{
}
impl Trig for f64 {
    type Output = Self;

    fn sin(self) -> <Self as Trig>::Output {
        self.sin()
    }

    fn cos(self) -> <Self as Trig>::Output {
        self.cos()
    }

    fn tan(self) -> <Self as Trig>::Output {
        self.tan()
    }
}

#[derive(PartialEq, Clone)]
pub struct Matrix<T: NumberFunctions, const N: usize, const M: usize>([[T; M]; N]);

impl<T: NumberFunctions, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn new_other(numbers: &[&[T]]) -> Self {
        let mut m = Self([[T::ZERO; M]; N]);
        for i in 0..N {
            for j in 0..M {
                m.0[i][j] = numbers[i][j];
            }
        }
        m
    }
    pub fn new(numbers: [[T; M]; N]) -> Self {
        Matrix(numbers)
    }
    pub fn zeros() -> Self {
        Matrix([[T::ZERO; M]; N])
    }
    pub fn ones() -> Self {
        Matrix([[T::ONE; M]; N])
    }
}

impl<T: NumberFunctions, const N: usize, const M: usize> UnwindSafe for Matrix<T, N, M> where
    T: UnwindSafe
{
}
impl<T: NumberFunctions, const N: usize, const M: usize> RefUnwindSafe for Matrix<T, N, M> where
    T: RefUnwindSafe
{
}
unsafe impl<T: NumberFunctions, const N: usize, const M: usize> Send for Matrix<T, N, M> where
    T: Send
{
}

unsafe impl<T: NumberFunctions, const N: usize, const M: usize> Sync for Matrix<T, N, M> where
    T: Sync
{
}

impl<T: NumberFunctions, const N: usize, const M: usize> From<[[T; M]; N]> for Matrix<T, N, M> {
    fn from(value: [[T; M]; N]) -> Self {
        Matrix(value)
    }
}
impl<T: NumberFunctions, const N: usize> From<[T; N]> for Matrix<T, 1, N> {
    fn from(value: [T; N]) -> Self {
        Matrix([value])
    }
}

impl<T: NumberFunctions, const N: usize, const M: usize, const P: usize> Mul<Matrix<T, N, P>>
    for Matrix<T, N, M>
{
    type Output = Matrix<T, N, P>;

    fn mul(self, rhs: Matrix<T, N, P>) -> Self::Output {
        let mut a = [[T::ZERO; P]; N];
        for n in 0..N {
            for p in 0..P {
                a[n][p] = (0..M).map(|m| self.0[n][m] * rhs.0[m][p]).sum();
            }
        }
        return Matrix(a);
    }
}

impl<T: NumberFunctions, const N: usize, const M: usize> Sub for Matrix<T, N, M> {
    type Output = Matrix<T, N, M>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut r = [[T::ZERO; M]; N];
        for n in 0..N {
            for m in 0..M {
                r[n][m] = self.0[n][m] - rhs.0[n][m]
            }
        }
        Matrix(r)
    }
}
impl<T: NumberFunctions, const N: usize, const M: usize> Add for Matrix<T, N, M> {
    type Output = Matrix<T, N, M>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut r = [[T::ZERO; M]; N];
        for n in 0..N {
            for m in 0..M {
                r[n][m] = self.0[n][m] + rhs.0[n][m]
            }
        }
        Matrix(r)
    }
}
impl<T: NumberFunctions, const N: usize> Matrix<T, N, N> {
    pub fn identity() -> Self {
        let mut r = [[T::ZERO; N]; N];
        for i in 0..N {
            r[i][i] = T::ONE;
        }
        return Matrix(r);
    }
}
impl<T: NumberFunctions, const N: usize, const M: usize> Matrix<T, N, M> {
    pub fn transpose(&self) -> Matrix<T, M, N> {
        let mut r = [[T::ZERO; N]; M];
        for n in 0..N {
            for m in 0..M {
                r[m][n] = self.0[n][m];
            }
        }
        Matrix::<T, M, N>(r)
    }
}
pub type TransformationMatrix<T> = Matrix<T, 3, 3>;
impl<T: NumberFunctions + Trig<Output = T>> Matrix<T, 3, 3> {
    pub fn translation(tx: T, ty: T) -> Self {
        Matrix([
            [T::ONE, T::ZERO, T::ZERO],
            [T::ZERO, T::ONE, T::ZERO],
            [tx, ty, T::ONE],
        ])
    }
    pub fn scale(sx: T, sy: T) -> Self {
        Matrix([
            [sx, T::ZERO, T::ZERO],
            [T::ZERO, sy, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE],
        ])
    }
    pub fn rotation(theta: T) -> Self {
        Matrix([
            [theta.cos(), theta.sin(), T::ZERO],
            [-theta.sin(), theta.cos(), T::ZERO],
            [T::ZERO, T::ZERO, T::ONE],
        ])
    }
    pub fn skew(alpha: T, beta: T) -> Self {
        Matrix([
            [T::ONE, alpha.tan(), T::ZERO],
            [beta.tan(), T::ONE, T::ZERO],
            [T::ZERO, T::ZERO, T::ONE],
        ])
    }
}

pub struct TranformationSpace<T: NumberFunctions>(T, T, T, T, T, T);
impl<T: NumberFunctions> From<TranformationSpace<T>> for Matrix<T, 3, 3> {
    fn from(ts: TranformationSpace<T>) -> Self {
        Matrix([
            [ts.0, ts.1, T::ZERO],
            [ts.2, ts.3, T::ZERO],
            [ts.4, ts.4, T::ONE],
        ])
    }
}
impl<T: NumberFunctions, const N: usize, const M: usize> Debug for Matrix<T, N, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix ({}x{}) {:?}", N, M, &self.0)
    }
}

impl<T: NumberFunctions, const N: usize, const M: usize> Copy for Matrix<T, N, M> {}

#[derive(Clone, Copy, PartialEq)]
pub struct CartesianPoint<T: NumberFunctions> {
    pub x: T,
    pub y: T,
}

impl<T: NumberFunctions> Into<Matrix<T, 1, 3>> for CartesianPoint<T> {
    fn into(self) -> Matrix<T, 1, 3> {
        Matrix::from([[self.x, self.y, T::ONE]])
    }
}
impl<T: NumberFunctions + Div<Output = T>> TryFrom<Matrix<T, 1, 3>> for CartesianPoint<T> {
    type Error = Error;

    fn try_from(value: Matrix<T, 1, 3>) -> Result<Self, Self::Error> {
        let [[x, y, f]] = value.0;
        if f == T::ZERO {
            return Err(Error::new_with_message(
                ErrorKind::DivideByZero.into(),
                "The last element of the co-oordinate is 0.0",
            ));
        }
        return Ok(CartesianPoint { x: x / f, y: y / f });
    }
}

impl<T: NumberFunctions + Div<Output = T>> Mul<TransformationMatrix<T>> for CartesianPoint<T> {
    type Output = Result<CartesianPoint<T>, Error>;

    fn mul(self, rhs: TransformationMatrix<T>) -> Self::Output {
        let result = rhs * Into::<Matrix<T, 1, 3>>::into(self).transpose();
        result.transpose().try_into()
    }
}
pub trait Apply<T: NumberFunctions, S: NumberFunctions, const N: usize, const M: usize> {
    fn apply(&self, a: impl Fn(T) -> S) -> Matrix<S, N, M>;
    fn apply_mut(&self, a: impl FnMut(T) -> S) -> Matrix<S, N, M>;
}

impl<T: NumberFunctions, S: NumberFunctions, const N: usize, const M: usize> Apply<T, S, N, M>
    for Matrix<T, N, M>
{
    fn apply(&self, a: impl Fn(T) -> S) -> Matrix<S, N, M> {
        let mut r = [[S::ZERO; M]; N];
        for n in 0..N {
            for m in 0..M {
                r[n][m] = a(self.0[n][m]);
            }
        }
        return Matrix(r);
    }
    fn apply_mut(&self, mut a: impl FnMut(T) -> S) -> Matrix<S, N, M> {
        let mut r = [[S::ZERO; M]; N];
        for n in 0..N {
            for m in 0..M {
                r[n][m] = a(self.0[n][m]);
            }
        }
        return Matrix(r);
    }
}

#[test]
fn test() {
    let m = Matrix::rotation(0.0);
    let n = Matrix::new([[2f64, 0f64, 1.0]]).transpose();
    let k = m * n;
    println!("{:?}", k);
}
