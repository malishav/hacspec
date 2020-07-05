#![allow(unused_variables)]
use crate::prelude::*;

// Note: Matrices are currently not supported.
// TODO: this won't allow us to split between signed and unsigned.
impl<T: Numeric + PublicInteger + Copy> Numeric for PublicSeq<T> {
    /// Return largest value that can be represented.
    fn max_val() -> Self {
        unimplemented!();
    }

    fn wrap_add(self, rhs: Self) -> Self {
        self + rhs
    }
    fn wrap_sub(self, rhs: Self) -> Self {
        self - rhs
    }
    fn wrap_mul(self, rhs: Self) -> Self {
        self * rhs
    }
    fn wrap_div(self, rhs: Self) -> Self {
        unimplemented!();
    }

    /// `self ^ exp` where `exp` is a `u32`.
    fn exp(self, exp: u32) -> Self {
        unimplemented!();
    }
    /// `self ^ exp` where `exp` is a `Self`.
    fn pow_self(self, exp: Self) -> Self {
        unimplemented!();
    }
    /// Division.
    fn divide(self, rhs: Self) -> Self {
        unimplemented!();
    }
    /// Invert self modulo n.
    fn inv(self, n: Self) -> Self {
        unimplemented!();
    }

    // Comparison functions returning bool.
    fn equal(self, other: Self) -> bool {
        unimplemented!();
    }
    fn greater_than(self, other: Self) -> bool {
        unimplemented!();
    }
    fn greater_than_or_qual(self, other: Self) -> bool {
        unimplemented!();
    }
    fn less_than(self, other: Self) -> bool {
        unimplemented!();
    }
    fn less_than_or_equal(self, other: Self) -> bool {
        unimplemented!();
    }

    // Comparison functions returning a bit mask (0x0..0 or 0xF..F).
    fn not_equal_bm(self, other: Self) -> Self {
        unimplemented!();
    }
    fn equal_bm(self, other: Self) -> Self {
        unimplemented!();
    }
    fn greater_than_bm(self, other: Self) -> Self {
        unimplemented!();
    }
    fn greater_than_or_equal_bm(self, other: Self) -> Self {
        unimplemented!();
    }
    fn less_than_bm(self, other: Self) -> Self {
        unimplemented!();
    }
    fn less_than_or_equal_bm(self, other: Self) -> Self {
        unimplemented!();
    }
}

/// Polynomial multiplication on ℤ\[x\]
impl<T: Numeric + PublicInteger + Copy> Mul for PublicSeq<T> {
    type Output = Self;
    #[cfg_attr(feature="use_attributes", library(hacspec))]
    fn mul(self, rhs: Self) -> Self::Output {
        vec_mul(self, rhs, T::default())
    }
}

/// Polynomial subtraction on ℤ\[x\]
impl<T: Numeric + PublicInteger + Copy> Sub for PublicSeq<T> {
    type Output = Self;
    #[cfg_attr(feature="use_attributes", library(hacspec))]
    fn sub(self, rhs: Self) -> Self::Output {
        vec_sub(self, rhs, T::default())
    }
}

/// Polynomial addition on ℤ\[x\]
impl<T: Numeric + PublicInteger + Copy> Add for PublicSeq<T> {
    type Output = Self;
    #[cfg_attr(feature="use_attributes", library(hacspec))]
    fn add(self, rhs: Self) -> Self::Output {
        vec_add(self, rhs, T::default())
    }
}

impl<T: Numeric + PublicInteger + Copy> Not for PublicSeq<T> {
    type Output = PublicSeq<T>;
    #[cfg_attr(feature="use_attributes", library(hacspec))]
    fn not(self) -> Self::Output {
        unimplemented!();
    }
}

impl<T: Numeric + PublicInteger + Copy> BitOr for PublicSeq<T> {
    type Output = PublicSeq<T>;
    #[cfg_attr(feature="use_attributes", library(hacspec))]
    fn bitor(self, rhs: Self) -> Self::Output {
        unimplemented!();
    }
}

impl<T: Numeric + PublicInteger + Copy> BitXor for PublicSeq<T> {
    type Output = PublicSeq<T>;
    #[cfg_attr(feature="use_attributes", library(hacspec))]
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut out = Self::default();
        for (a, (b, c)) in out.b.iter_mut().zip(self.b.iter().zip(rhs.b.iter())) {
            *a = *b ^ *c;
        }
        out
    }
}

impl<T: Numeric + PublicInteger + Copy> BitAnd for PublicSeq<T> {
    type Output = PublicSeq<T>;
    #[cfg_attr(feature="use_attributes", library(hacspec))]
    fn bitand(self, rhs: Self) -> Self::Output {
        unimplemented!();
    }
}

impl<T: Numeric + PublicInteger + Copy> Shr<usize> for PublicSeq<T> {
    type Output = PublicSeq<T>;
    #[cfg_attr(feature="use_attributes", library(hacspec))]
    fn shr(self, rhs: usize) -> Self::Output {
        unimplemented!();
    }
}

impl<T: Numeric + PublicInteger + Copy> Shl<usize> for PublicSeq<T> {
    type Output = PublicSeq<T>;
    #[cfg_attr(feature="use_attributes", library(hacspec))]
    fn shl(self, rhs: usize) -> Self::Output {
        unimplemented!();
    }
}
