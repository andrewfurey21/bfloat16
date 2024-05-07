#![allow(dead_code, unused_variables)]
use std::ops::{Add, Sub, Mul, Div};


#[allow(non_camel_case_types)]
struct bfloat16(u16);

impl bfloat16 {
    fn sqrt(self) -> Self {
        return self;
    }

    fn inverse_sqrt(self) -> Self {
        return self;
    }
}

impl Add<bfloat16> for bfloat16 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        return self;
    }
}

impl Sub<bfloat16> for bfloat16 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        return self;
    }
}

impl Mul<bfloat16> for bfloat16 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        return self;
    }
}

impl Div<bfloat16> for bfloat16 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        return self;
    }
}


