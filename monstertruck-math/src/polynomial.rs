//! Solvers for polynomial equations up to degree 4.
//!
//! Ported from `matext4cgmath::solver`. All functions return roots as
//! [`Complex<F>`] arrays and apply Newton refinement for numerical precision.

use crate::traits::BaseFloat;
use num_complex::Complex;

/// Solve x^2 + ax + b = 0.
pub fn solve_quadratic<F: BaseFloat>(_a: F, _b: F) -> [Complex<F>; 2] {
    todo!()
}

/// Solve x^3 + px + q = 0 (depressed cubic).
pub fn pre_solve_cubic<F: BaseFloat>(_p: F, _q: F) -> [Complex<F>; 3] {
    todo!()
}

/// Solve x^3 + ax^2 + bx + c = 0.
pub fn solve_cubic<F: BaseFloat>(_a: F, _b: F, _c: F) -> [Complex<F>; 3] {
    todo!()
}

/// Solve x^4 + px^2 + qx + r = 0 (depressed quartic).
pub fn pre_solve_quartic<F: BaseFloat>(_p: F, _q: F, _r: F) -> [Complex<F>; 4] {
    todo!()
}

/// Solve x^4 + ax^3 + bx^2 + cx + d = 0.
pub fn solve_quartic<F: BaseFloat>(_a: F, _b: F, _c: F, _d: F) -> [Complex<F>; 4] {
    todo!()
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
        res.sort_by(|x, y| x.re.partial_cmp(&y.re).unwrap().then(x.im.partial_cmp(&y.im).unwrap()));
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
        let ans = [Complex::from(-2.0), Complex::from(-1.0), Complex::from(3.0)];
        for (x, y) in res.iter().zip(ans.iter()) {
            assert!(Complex::norm(x - y) < EPS, "got {x}, expected {y}");
        }
    }

    #[test]
    fn test_solve_cubic() {
        // x^3 - 3x^2 + 4 = 0 => roots: -1, 2, 2.
        let mut res = solve_cubic(-3.0, 0.0, 4.0);
        res.sort_by(|x, y| x.re.partial_cmp(&y.re).unwrap());
        let ans = [Complex::from(-1.0), Complex::from(2.0), Complex::from(2.0)];
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
            assert!(Complex::norm(x - y) < EPS_QUARTIC, "got {x}, expected {y}");
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
            assert!(Complex::norm(x - y) < EPS_QUARTIC, "got {x}, expected {y}");
        }
    }

    #[test]
    fn test_solve_quartic_substitution_check() {
        // For each root r of x^4 + x^3 - 7x^2 - x + 6 = 0, verify substitution.
        let res = solve_quartic(1.0, -7.0, -1.0, 6.0);
        for r in &res {
            let val: Complex<f64> = r * r * r * r + r * r * r - 7.0 * r * r - r + 6.0;
            assert!(
                val.norm() < EPS,
                "Substitution check failed: f({r}) = {val}",
            );
        }
    }
}
