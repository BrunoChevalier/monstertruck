//! Solvers for polynomial equations up to degree 4.
//!
//! Ported from `matext4cgmath::solver`. All functions return roots as
//! [`Complex<F>`] arrays and apply Newton refinement for numerical precision.

use crate::traits::BaseFloat;
use num_complex::Complex;

// Re-import `Float` under a private alias to disambiguate from nalgebra's
// `ComplexField` methods that shadow identically-named functions.
use num_traits::Float as Fl;

/// Solve x^2 + ax + b = 0.
///
/// Real solutions are returned sorted in ascending order.
/// Complex conjugate pairs are returned with positive imaginary part first.
pub fn solve_quadratic<F: BaseFloat>(a: F, b: F) -> [Complex<F>; 2] {
    let two = F::one() + F::one();
    let four = two + two;
    let det = a * a - four * b;
    match det >= F::zero() {
        true => {
            let h = Fl::sqrt(det);
            [
                Complex::new((-a - h) / two, F::zero()),
                Complex::new((-a + h) / two, F::zero()),
            ]
        }
        false => {
            let h = Fl::sqrt(-det);
            [Complex::new(-a, h) / two, Complex::new(-a, -h) / two]
        }
    }
}

/// Solve the depressed cubic x^3 + px + q = 0.
///
/// Uses Cardano's formula with cube-root-of-unity splitting, followed by
/// Newton refinement to polish each root.
pub fn pre_solve_cubic<F: BaseFloat>(p: F, q: F) -> [Complex<F>; 3] {
    let two = F::one() + F::one();
    let three = two + F::one();
    let sqrt3_2 = Fl::sqrt(three) / two;
    let omega = Complex::new(-F::one() / two, sqrt3_2);
    let omega2 = Complex::new(-F::one() / two, -sqrt3_2);
    let eps_2 = Fl::sqrt(Fl::epsilon());

    let p_3 = p / three;
    let q_2 = q / two;
    let alpha2 = q_2 * q_2 + p_3 * p_3 * p_3;
    let (x, y) = match alpha2 >= F::zero() {
        true => {
            let alpha = Fl::sqrt(alpha2);
            let tmpx = -q_2 - alpha;
            let tmpy = -q_2 + alpha;
            (
                Complex::new(
                    Fl::signum(tmpx) * Fl::powf(Fl::abs(tmpx), F::one() / three),
                    F::zero(),
                ),
                Complex::new(
                    Fl::signum(tmpy) * Fl::powf(Fl::abs(tmpy), F::one() / three),
                    F::zero(),
                ),
            )
        }
        false => {
            let alphai = Fl::sqrt(-alpha2);
            (
                Complex::powf(Complex::new(-q_2, alphai), F::one() / three),
                Complex::powf(Complex::new(-q_2, -alphai), F::one() / three),
            )
        }
    };
    let mut res = [x + y, omega * x + omega2 * y, omega2 * x + omega * y];
    // Refinement by Newton method.
    res.iter_mut().for_each(|x| {
        let mut f = *x * *x * *x + *x * p + q;
        let mut f_prime = *x * *x * three + p;
        while f.norm() > eps_2 * f_prime.norm() {
            if f_prime.norm() < eps_2 {
                return;
            }
            *x -= f / f_prime;
            f = *x * *x * *x + *x * p + q;
            f_prime = *x * *x * three + p;
        }
    });
    res
}

/// Solve x^3 + ax^2 + bx + c = 0.
///
/// Reduces to the depressed form via the substitution x = t - a/3, then
/// delegates to [`pre_solve_cubic`].
pub fn solve_cubic<F: BaseFloat>(a: F, b: F, c: F) -> [Complex<F>; 3] {
    let two = F::one() + F::one();
    let three = two + F::one();
    let twenty_seven = three * three * three;
    let p = b - a * a / three;
    let q = c - a * b / three + two * a * a * a / twenty_seven;
    let mut res = pre_solve_cubic(p, q);
    res.iter_mut().for_each(|x| {
        *x -= a / three;
    });
    res
}

/// Solve the depressed quartic x^4 + px^2 + qx + r = 0.
///
/// Uses resolvent cubic factorisation with sign-combination search, followed
/// by Newton refinement to polish each root.
pub fn pre_solve_quartic<F: BaseFloat>(p: F, q: F, r: F) -> [Complex<F>; 4] {
    let two = F::one() + F::one();
    let four = two + two;
    let eps_2 = Fl::sqrt(Fl::epsilon());

    let cubic_a = two * p;
    let cubic_b = p * p - four * r;
    let cubic_c = -q * q;
    let f = solve_cubic(cubic_a, cubic_b, cubic_c);
    let sa = f[0].sqrt() / two;
    let sb = f[1].sqrt() / two;
    let sc = f[2].sqrt() / two;

    // Try all 8 sign combinations for the three square roots and keep the one
    // that minimises the worst-case residual across all four candidate roots.
    // SAFETY: The iterator always yields exactly 8 elements, so `min_by`
    // always returns `Some`.
    let mut res = (0..8i32)
        .map(|i| {
            let a = sa * Fl::powi(-F::one(), i % 2);
            let b = sb * Fl::powi(-F::one(), (i / 2) % 2);
            let c = sc * Fl::powi(-F::one(), (i / 4) % 2);
            [-a - b - c, -a + b + c, a - b + c, a + b - c]
        })
        .map(|x| {
            let max_residual = x
                .iter()
                .map(|t| (t * t * t * t + t * t * p + t * q + r).norm_sqr())
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap();
            (x, max_residual)
        })
        .min_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
        .unwrap()
        .0;
    // Refinement by Newton method.
    res.iter_mut().for_each(|x| {
        let mut f = *x * *x * *x * *x + *x * *x * p + *x * q + r;
        let mut f_prime = *x * *x * *x * four + *x * p * two + q;
        while f.norm() > eps_2 * f_prime.norm() {
            if f_prime.norm() < eps_2 {
                return;
            }
            *x -= f / f_prime;
            f_prime = *x * *x * *x * four + *x * p * two + q;
            f = *x * *x * *x * *x + *x * *x * p + *x * q + r;
        }
    });
    res
}

/// Solve x^4 + ax^3 + bx^2 + cx + d = 0.
///
/// Reduces to the depressed form via the substitution x = t - a/4, then
/// delegates to [`pre_solve_quartic`].
pub fn solve_quartic<F: BaseFloat>(a: F, b: F, c: F, d: F) -> [Complex<F>; 4] {
    let two = F::one() + F::one();
    let three = F::one() + two;
    let four = two + two;
    let six = two * three;
    let eight = four + four;

    let a_4 = a / four;
    let p = b - six * a_4 * a_4;
    let q = c - two * b * a_4 + eight * a_4 * a_4 * a_4;
    let r = d - c * a_4 + b * a_4 * a_4 - three * a_4 * a_4 * a_4 * a_4;
    let mut res = pre_solve_quartic(p, q, r);
    res.iter_mut().for_each(|x| *x -= a_4);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1.0e-10;
    const EPS_QUARTIC: f64 = 1.0e-7;

    #[test]
    fn test_solve_quadratic_real_roots() {
        // x^2 + 3x - 4 = 0 => roots: -4, 1.
        let mut res = solve_quadratic(3.0, -4.0);
        res.sort_by(|x, y| x.re.partial_cmp(&y.re).unwrap());
        let ans = [Complex::from(-4.0), Complex::from(1.0)];
        for (x, y) in res.iter().zip(ans.iter()) {
            assert!(Complex::norm(x - y) < EPS, "got {x}, expected {y}");
        }
    }

    #[test]
    fn test_solve_quadratic_complex_roots() {
        // x^2 + 2x + 5 = 0 => roots: -1 +/- 2i.
        let mut res = solve_quadratic(2.0, 5.0);
        res.sort_by(|x, y| {
            x.re.partial_cmp(&y.re)
                .unwrap()
                .then(x.im.partial_cmp(&y.im).unwrap())
        });
        let ans = [Complex::new(-1.0, -2.0), Complex::new(-1.0, 2.0)];
        for (x, y) in res.iter().zip(ans.iter()) {
            assert!(Complex::norm(x - y) < EPS, "got {x}, expected {y}");
        }
    }

    #[test]
    fn test_pre_solve_cubic() {
        // x^3 - 7x - 6 = 0 => roots: -2, -1, 3.
        let mut res = pre_solve_cubic(-7.0, -6.0);
        res.sort_by(|x, y| x.re.partial_cmp(&y.re).unwrap());
        let ans = [
            Complex::from(-2.0),
            Complex::from(-1.0),
            Complex::from(3.0),
        ];
        for (x, y) in res.iter().zip(ans.iter()) {
            assert!(Complex::norm(x - y) < EPS, "got {x}, expected {y}");
        }
    }

    #[test]
    fn test_solve_cubic() {
        // x^3 - 3x^2 + 4 = 0 => roots: -1, 2, 2.
        let mut res = solve_cubic(-3.0, 0.0, 4.0);
        res.sort_by(|x, y| x.re.partial_cmp(&y.re).unwrap());
        let ans = [
            Complex::from(-1.0),
            Complex::from(2.0),
            Complex::from(2.0),
        ];
        for (x, y) in res.iter().zip(ans.iter()) {
            assert!(Complex::norm(x - y) < EPS, "got {x}, expected {y}");
        }
    }

    #[test]
    fn test_pre_solve_quartic() {
        // x^4 - 5x^2 + 4 = 0 => roots: -2, -1, 1, 2.
        let mut res = pre_solve_quartic(-5.0, 0.0, 4.0);
        res.sort_by(|x, y| x.re.partial_cmp(&y.re).unwrap());
        let ans = [
            Complex::from(-2.0),
            Complex::from(-1.0),
            Complex::from(1.0),
            Complex::from(2.0),
        ];
        for (x, y) in res.iter().zip(ans.iter()) {
            assert!(
                Complex::norm(x - y) < EPS_QUARTIC,
                "got {x}, expected {y}",
            );
        }
    }

    #[test]
    fn test_solve_quartic() {
        // x^4 + x^3 - 7x^2 - x + 6 = 0 => roots: -3, -1, 1, 2.
        let mut res = solve_quartic(1.0, -7.0, -1.0, 6.0);
        res.sort_by(|x, y| x.re.partial_cmp(&y.re).unwrap());
        let ans = [
            Complex::from(-3.0),
            Complex::from(-1.0),
            Complex::from(1.0),
            Complex::from(2.0),
        ];
        for (x, y) in res.iter().zip(ans.iter()) {
            assert!(
                Complex::norm(x - y) < EPS_QUARTIC,
                "got {x}, expected {y}",
            );
        }
    }

    #[test]
    fn test_solve_quartic_substitution_check() {
        // For each root r of x^4 + x^3 - 7x^2 - x + 6 = 0, verify substitution.
        let res = solve_quartic(1.0, -7.0, -1.0, 6.0);
        for r in &res {
            let val: Complex<f64> =
                r * r * r * r + r * r * r - 7.0 * r * r - r + 6.0;
            assert!(
                val.norm() < EPS,
                "Substitution check failed: f({r}) = {val}",
            );
        }
    }
}
