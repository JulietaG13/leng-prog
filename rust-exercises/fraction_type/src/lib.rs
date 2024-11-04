use std::ops;

#[derive(Debug, PartialEq, Eq)]
pub struct Fraction(pub i32, pub i32);

impl Fraction {
    pub fn add(&self, other: Fraction) -> Fraction {
        let n = self.0 * other.1 + other.0 * self.1;
        let d = self.1 * other.1;
        simplify(n, d)
    }

    pub fn sub(&self, other: Fraction) -> Fraction {
        let n = self.0 * other.1 - other.0 * self.1;
        let d = self.1 * other.1;
        simplify(n, d)
    }

    pub fn mul(&self, other: Fraction) -> Fraction {
        simplify(self.0 * other.0, self.1 * other.1)
    }

    pub fn divide(&self, other: Fraction) -> Fraction {
        simplify(self.0 * other.1, self.1 * other.0)
    }
}

impl ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, other: Fraction) -> Fraction {
        let n = self.0 * other.1 + other.0 * self.1;
        let d = self.1 * other.1;
        simplify(n, d)
    }
}

/// Calculate the Highest common factor between 2 numbers
fn hcf(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { hcf(b, a % b) }
}

fn simplify(n: i32, d: i32) -> Fraction {
    let h = hcf(n, d);
    Fraction(n/h, d/h)
}