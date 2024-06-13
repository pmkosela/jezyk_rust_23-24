use std::ops::{Add, Mul, MulAssign};

#[derive(Debug, Clone, Eq)]
pub struct Fraction {
    numerator: u32,
    denominator: u32
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Fraction {
        let gcd = Fraction::gcd(numerator, denominator);
        let numerator = numerator / gcd;
        let denominator = denominator / gcd;
        Fraction{numerator, denominator}
    }

    pub fn numerator(&self) -> u32 {
        self.numerator
    }

    pub fn denominator(&self) -> u32 {
        self.denominator
    }

    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
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

// operator*
impl Mul for Fraction {
    type Output = Self;

    //fn mul(self, other: Fraction) -> <Self as Mul<fraction::Fraction>>::Output {
    fn mul(self, other: Fraction) -> Self {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        Fraction::new(numerator, denominator)
    }
}

// operator*=
impl MulAssign for Fraction {
    fn mul_assign(&mut self, other: Fraction) {
        self.numerator *= other.numerator;
        self.denominator *= other.denominator;
    }
}

impl Add for Fraction {
    type Output = Fraction;
    fn add(self, other: Fraction) -> Self {
        self
    }
}

// operator ==
impl PartialEq for Fraction {
    fn eq(&self, other: &Fraction) -> bool {
        self.numerator == other.numerator &&
        self.denominator == other.denominator
    }
}

impl From<u32> for Fraction {
    fn from(n: u32) -> Self {
        Self::new(n, 1)
    }
}

impl From<(u32, u32)> for Fraction {
    fn from(n: (u32, u32)) -> Self {
        Self::new(n.0, n.1)
    }
}

impl TryFrom<f32> for Fraction {
    type Error = String;

    fn try_from(a: f32) -> Result<Self, String> {
        let s = a.to_string();
        println!("{}", s);
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err("NaN".to_string());
        }
        let p = parts[1].len() as u32;
        let d0: u32 = parts[0].to_string()
                              .parse::<u32>()
                              .unwrap();
        let d1: u32 = parts[1].to_string()
                              .parse::<u32>()
                              .unwrap();
        let f1: Fraction = p.into();
        let f2: Fraction = Fraction::from((d1, u32::pow(10, p)));
        Ok(f1 + f2)
    }
}

fn main() {
    let f1 = Fraction::from(7);
    println!("{:?}", f1);
    let f1 = Fraction::from((7, 2));
    println!("{:?}", f1);
    let f1 = Fraction::try_from(1.5);
    println!("{:?}", f1);
}
