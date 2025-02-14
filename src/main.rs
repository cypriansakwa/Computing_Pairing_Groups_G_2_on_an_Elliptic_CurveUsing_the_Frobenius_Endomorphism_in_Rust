use std::fmt;

/// A struct to represent elements of the field \( \mathbb{F}_{5^2} \)
#[derive(Clone, Copy, Debug, PartialEq)]
struct F5x2 {
    a: u8, // Coefficient for 1
    b: u8, // Coefficient for t
}

impl fmt::Display for F5x2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.a, self.b) {
            (0, 0) => write!(f, "0"),
            (a, 0) => write!(f, "{}", a),
            (0, b) => write!(f, "{}t", b),
            (a, b) => write!(f, "{} + {}t", a, b),
        }
    }
}

impl F5x2 {
    fn new(a: u8, b: u8) -> Self {
        F5x2 { a: a % 5, b: b % 5 }
    }

    fn add(self, other: F5x2) -> F5x2 {
        F5x2::new((self.a + other.a) % 5, (self.b + other.b) % 5)
    }

    fn sub(self, other: F5x2) -> F5x2 {
        F5x2::new((self.a + 5 - other.a) % 5, (self.b + 5 - other.b) % 5)
    }

    fn mul(self, other: F5x2) -> F5x2 {
        let a = self.a as i16;
        let b = self.b as i16;
        let c = other.a as i16;
        let d = other.b as i16;
        // Using the irreducible polynomial t^2 + 2 = 0, so t^2 = -2 ≡ 3 mod 5.
        // (a + b*t) * (c + d*t) = ac + (ad+bc)t + bd*t^2 = ac + (ad+bc)t + 3bd.
        let ac = (a * c) % 5;
        let bd = (b * d) % 5;
        let ad_plus_bc = (a * d + b * c) % 5;
        let new_a = (ac + 3 * bd) % 5;
        let new_b = ad_plus_bc % 5;
        F5x2::new(new_a as u8, new_b as u8)
    }

    fn div(self, other: F5x2) -> F5x2 {
        let inv = other.inverse();
        self.mul(inv)
    }

    fn inverse(self) -> F5x2 {
        // For u = a + b*t, its inverse is (a - b*t)/(a^2 - 3b^2) (since t^2 = 3 mod 5).
        let a = self.a as i16;
        let b = self.b as i16;
        let denominator = (a * a - 3 * b * b).rem_euclid(5) as u8;
        let inv_denominator = F5x2::mod_inverse(denominator, 5);
        let new_a = (a * inv_denominator as i16).rem_euclid(5) as u8;
        let new_b = (5 - (b * inv_denominator as i16).rem_euclid(5)) as u8;
        F5x2::new(new_a, new_b)
    }

    fn mod_inverse(x: u8, p: u8) -> u8 {
        for i in 1..p {
            if (x as u16 * i as u16) % p as u16 == 1 {
                return i;
            }
        }
        panic!("No modular inverse found!");
    }

    /// Frobenius automorphism: in characteristic 5, (a + b*t)^5 = a + b*t^5.
    /// Since t^5 = t^(4+1) = (t^2)^2 * t = 3^2 * t = 9t ≡ 4t (mod 5),
    /// we have: (a + b*t)^5 = a + 4b*t.
    fn frobenius(self) -> F5x2 {
        F5x2::new(self.a, (4 * self.b) % 5)
    }
}

/// A struct to represent a point on the elliptic curve.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: Option<F5x2>,
    y: Option<F5x2>,
}

impl Point {
    fn new(x: Option<F5x2>, y: Option<F5x2>) -> Self {
        Point { x, y }
    }

    fn is_at_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }

    fn at_infinity() -> Self {
        Point { x: None, y: None }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_at_infinity() {
            write!(f, "Point at infinity")
        } else {
            // Unwrap is safe here because the point is not at infinity.
            write!(f, "({}, {})", self.x.unwrap(), self.y.unwrap())
        }
    }
}

/// Point addition on the curve: y^2 = x^3 + ax + b.
fn point_add(p: Point, q: Point, a: F5x2) -> Point {
    if p.is_at_infinity() {
        return q;
    }
    if q.is_at_infinity() {
        return p;
    }
    let (x1, y1) = (p.x.unwrap(), p.y.unwrap());
    let (x2, y2) = (q.x.unwrap(), q.y.unwrap());
    if x1 == x2 && y1 != y2 {
        return Point::at_infinity();
    }
    let lambda = if x1 == x2 && y1 == y2 {
        // Point doubling: λ = (3*x1^2 + a) / (2*y1)
        let numerator = x1.mul(x1).mul(F5x2::new(3, 0)).add(a);
        let denominator = y1.mul(F5x2::new(2, 0));
        numerator.div(denominator)
    } else {
        // Point addition: λ = (y2 - y1) / (x2 - x1)
        let numerator = y2.sub(y1);
        let denominator = x2.sub(x1);
        numerator.div(denominator)
    };
    let x3 = lambda.mul(lambda).sub(x1).sub(x2);
    let y3 = lambda.mul(x1.sub(x3)).sub(y1);
    Point::new(Some(x3), Some(y3))
}

/// Scalar multiplication using double-and-add.
fn point_mul(n: u8, p: Point, a: F5x2) -> Point {
    let mut result = Point::at_infinity();
    let mut base = p;
    let mut k = n;
    while k > 0 {
        if k & 1 == 1 {
            result = point_add(result, base, a);
        }
        base = point_add(base, base, a);
        k >>= 1;
    }
    result
}

/// Applies the Frobenius endomorphism to a point:
/// (x, y) -> (x^5, y^5).
fn point_frobenius(p: Point) -> Point {
    if p.is_at_infinity() {
        p
    } else {
        let (x, y) = (p.x.unwrap(), p.y.unwrap());
        Point::new(Some(x.frobenius()), Some(y.frobenius()))
    }
}

/// Finds all full r‑torsion points (P such that rP = O) by iterating over the field.
fn find_full_r_torsion_points(
    r: u8,
    a: F5x2,
    b: F5x2,
    field_elements: &Vec<F5x2>,
) -> Vec<Point> {
    let mut torsion_points = Vec::new();
    for x in field_elements.iter() {
        let x_cubed = x.mul(*x).mul(*x);
        let rhs = x_cubed.add(a.mul(*x)).add(b);
        for y in field_elements.iter() {
            if y.mul(*y) == rhs {
                let point = Point::new(Some(*x), Some(*y));
                if point_mul(r, point, a).is_at_infinity() {
                    torsion_points.push(point);
                }
            }
        }
    }
    torsion_points.push(Point::at_infinity());
    torsion_points
}

/// Finds all points in G₂:
/// those points P for which (x^5, y^5) = 5P.
fn find_g2_points(a: F5x2, b: F5x2, field_elements: &Vec<F5x2>) -> Vec<Point> {
    let mut g2_points = Vec::new();
    for x in field_elements.iter() {
        let x_cubed = x.mul(*x).mul(*x);
        let rhs = x_cubed.add(a.mul(*x)).add(b);
        for y in field_elements.iter() {
            if y.mul(*y) == rhs {
                let p = Point::new(Some(*x), Some(*y));
                let frob = point_frobenius(p);
                let five_p = point_mul(5, p, a);
                if frob == five_p {
                    g2_points.push(p);
                }
            }
        }
    }
    // The point at infinity trivially satisfies the condition.
    g2_points.push(Point::at_infinity());
    g2_points
}

fn main() {
    // Create all elements of F(5^2)
    let mut field_elements = Vec::new();
    for a in 0..5 {
        for b in 0..5 {
            field_elements.push(F5x2::new(a, b));
        }
    }

    println!("Elements of F(5^2):");
    for elem in &field_elements {
        println!("{}", elem);
    }

    // Define the curve: y^2 = x^3 + x + 1 (a = 1, b = 1).
    let a_curve = F5x2::new(1, 0);
    let b_curve = F5x2::new(1, 0);

    println!("\nPoints on the elliptic curve y^2 = x^3 + x + 1:");
    for x in &field_elements {
        let x_cubed = x.mul(*x).mul(*x);
        let rhs = x_cubed.add(a_curve.mul(*x)).add(b_curve);
        for y in &field_elements {
            if y.mul(*y) == rhs {
                let point = Point::new(Some(*x), Some(*y));
                println!("{}", point);
            }
        }
    }

    // Find full 3-torsion points (points P with 3P = O).
    let r = 3;
    let torsion_points = find_full_r_torsion_points(r, a_curve, b_curve, &field_elements);
    println!("\nFull {}-torsion points (3P = O):", r);
    for point in torsion_points {
        println!("{}", point);
    }

    // Find group G₂: points P such that (x^5, y^5) = 5P.
    let g2_points = find_g2_points(a_curve, b_curve, &field_elements);
    println!("\nGroup G₂ points (P such that (x^5, y^5) = 5P):");
    for point in g2_points {
        println!("{}", point);
    }
}
