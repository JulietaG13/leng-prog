type Fraction = (i32, i32);

/// Add 2 fractions
pub fn add((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let n = n1 * d2 + n2 * d1;
    let d = d1 * d2;
    simplify(n, d)
}

/// Subtract 2 fractions
pub fn sub((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    let n = n1 * d2 - n2 * d1;
    let d = d1 * d2;
    simplify(n, d)
}

/// Multiply 2 fractions
pub fn mul((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    simplify(n1 * n2, d1 * d2)
}

/// Divide 2 fractions
pub fn divide((n1, d1): Fraction, (n2, d2): Fraction) -> Fraction {
    simplify(n1 * d2, d1 * n2)
}

/// Calculate the Highest common factor between 2 numbers
pub fn hcf(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    hcf(b, a % b)
}

/// Create a fraction simplifying with the arguments simplified by the `hcf`
pub fn simplify(n: i32, d: i32) -> Fraction {
    let mcm = hcf(n, d);
    (n / mcm, d / mcm)
}
