use std::ops::{Mul, MulAssign, Add, AddAssign, Div, DivAssign, Sub, SubAssign};

pub struct Fraction {
    numerator: u32,
    denominator: u32
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Fraction {
        let gcd = Fraction::gcd(numerator, denominator);
        Fraction{numerator: numerator / gcd,
                 denominator: denominator / gcd}
    }

    fn as_f64(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }

    fn numerator(&self) -> u32 {
        self.numerator
    }

    fn denominator(&self) -> u32 {
        self.denominator
    }

    fn gcd(a: u32, b: u32) -> u32 {
        let mut a = a;
        let mut b = b;
        while b > 0 {
            let tmp = a;
            a = b;
            b = tmp % b;
        }
        a
    }
}

impl std::ops::Mul for Fraction {
    type Output = Fraction;

    //fn mul(self, other: Fraction) -> Self {
    fn mul(self, other: Fraction) -> <Self as Mul<Fraction>>::Output {
        Fraction::new(self.numerator * other.numerator,
                      self.denominator * other.denominator)
    }
}

impl std::ops::MulAssign for Fraction {
    fn mul_assign(&mut self, other: Fraction) {
        self.numerator *= other.numerator;
        self.denominator *= other.denominator;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_fraction() {
        let f = Fraction::new(1, 2);
        assert_eq!(f.as_f64(), 0.5);
        //assert_ne!
        //assert!(false);
    }

    #[test]
    fn multiplication() {
        let mut a = Fraction::new(1, 2);
        let b = Fraction::new(2, 5);
        let c = a * b;
        //a *= b;
        assert_eq!(c.numerator, 2);
        assert_eq!(c.denominator, 10);
    }

    #[test]
    fn gcd() {
        assert_eq!(Fraction::gcd(81, 18), 9);
    }
}
