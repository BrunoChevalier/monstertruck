use monstertruck_core::{cgmath64::*, newton::*};

#[test]
fn solve_converges_for_well_conditioned_1d() {
    // Standard Newton on sqrt(2): f(x) = x^2 - 2.
    let function = |x: f64| CalcOutput {
        value: x * x - 2.0,
        derivation: 2.0 * x,
    };
    let result = solve(function, 1.0, 100);
    assert!(result.is_ok());
    let val = result.unwrap();
    assert!((val - f64::sqrt(2.0)).abs() < 1e-10);
}

#[test]
fn solve_robust_1d_converges_near_singular() {
    // f(x) = x^3 with hint at x=0 where f'(0)=0. Standard Newton fails due to zero derivative.
    let function = |x: f64| CalcOutput {
        value: x * x * x - 0.001,
        derivation: 3.0 * x * x,
    };
    // Standard Newton should fail starting from exactly zero (derivative is zero).
    let standard = solve(function, 0.0, 100);
    assert!(
        standard.is_err(),
        "Standard Newton should fail at x=0 where f'=0"
    );

    // Robust solver should converge.
    let robust = solve_robust_1d(function, 0.0, 300);
    assert!(robust.is_ok(), "solve_robust_1d should converge");
    let result = robust.unwrap();
    assert!(
        (result.value * result.value * result.value - 0.001).abs() < 1e-4,
        "Solution should satisfy f(x) ~ 0, got f({}) = {}",
        result.value,
        result.value * result.value * result.value - 0.001
    );
}

#[test]
fn solve_robust_2d_near_singular_matrix() {
    // 2D system with near-singular Jacobian.
    // f(v) = (v.x^2 + v.y^2 - 1, v.x - v.y) with a nearly singular Jacobian
    // by adding a small perturbation.
    let eps = 1e-12;
    let function = |v: Vector2| {
        let val = Vector2::new(v.x * v.x + v.y * v.y - 1.0, v.x - v.y);
        // Jacobian: [[2*v.x, 2*v.y], [1, -1]]
        // Make it near-singular when v.x ~ 0 and v.y ~ 0.
        let jac = Matrix2::new(2.0 * v.x + eps, 1.0, 2.0 * v.y + eps, -1.0);
        CalcOutput {
            value: val,
            derivation: jac,
        }
    };
    // Start from near-zero hint where Jacobian is near-singular.
    let result = solve_robust(function, Vector2::new(eps, eps), 200);
    assert!(
        result.is_ok(),
        "solve_robust should converge for 2D near-singular case"
    );
    let sol = result.unwrap();
    let residual = Vector2::new(
        sol.value.x * sol.value.x + sol.value.y * sol.value.y - 1.0,
        sol.value.x - sol.value.y,
    );
    assert!(
        residual.x.abs() < 1e-4 && residual.y.abs() < 1e-4,
        "Residual should be small, got {:?}",
        residual
    );
}

#[test]
fn solve_robust_reports_fallback_used() {
    // Near-singular 1D: f(x) = x^3 at x=0 where f'(0)=0. Standard Newton fails.
    let function = |x: f64| CalcOutput {
        value: x * x * x - 0.001,
        derivation: 3.0 * x * x,
    };
    let result = solve_robust_1d(function, 0.0, 300);
    assert!(result.is_ok());
    let sol = result.unwrap();
    assert!(sol.used_fallback, "Fallback should have been used");
}

#[test]
fn bisection_fallback_for_flat_derivative_1d() {
    // f(x) = x^5 - 1e-10: very flat derivative near zero.
    // Sign changes between negative and positive x values.
    let function = |x: f64| CalcOutput {
        value: x.powi(5) - 1e-10,
        derivation: 5.0 * x.powi(4),
    };
    let result = solve_robust_1d(function, 0.01, 300);
    assert!(result.is_ok(), "Bisection fallback should converge");
    let sol = result.unwrap();
    assert!(
        (sol.value.powi(5) - 1e-10).abs() < 1e-5,
        "Solution should satisfy f(x) ~ 0, got f({}) = {}",
        sol.value,
        sol.value.powi(5) - 1e-10
    );
}

#[test]
fn solve_robust_well_conditioned_no_fallback() {
    // Well-conditioned problem should converge without fallback.
    let function = |x: f64| CalcOutput {
        value: x * x - 4.0,
        derivation: 2.0 * x,
    };
    let result = solve_robust_1d(function, 1.5, 100);
    assert!(result.is_ok());
    let sol = result.unwrap();
    assert!((sol.value - 2.0).abs() < 1e-10);
    assert!(
        !sol.used_fallback,
        "Fallback should not be needed for well-conditioned problem"
    );
}
