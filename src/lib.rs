#![allow(dead_code, unused_variables)]
use std::ops::{Add, Sub, Mul, Div};


#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod bfloat16_add_tests {
    use super::bfloat16;

    #[test]
    fn one_add_one() {
        let a = bfloat16(0x3f80);
        let b = bfloat16(0x3f80);
        let result = a + b;
        assert_eq!(result, bfloat16(0x4000));
    }
}

#[cfg(test)]
mod bfloat16_sub_tests {
    use super::bfloat16;

    #[test]
    fn one_sub_one() {
        let a = bfloat16(0x3f80);
        let b = bfloat16(0x3f80);
        let result = a + b;
        assert_eq!(result, bfloat16(0));
    }
}

#[cfg(test)]
mod bfloat_mul_tests {
    use super::bfloat16;

    #[test]
    fn one_mul_two() {
        let a = bfloat16(0x3f80);
        let b = bfloat16(0x4000);
        let result = a * b;
        assert_eq!(result, bfloat16(0x4000));
    }
}

#[cfg(test)]
mod bfloat_div_tests {
    use super::bfloat16;

    #[test]
    fn one_div_two() {
        let a = bfloat16(0x3f80);
        let b = bfloat16(0x4000);
        let result = a / b;
        assert_eq!(result, bfloat16(0x3f00));
    }
}

