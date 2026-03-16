/// Verifies that the polynomial solver module is re-exported through cgmath64.
use monstertruck_core::cgmath64::polynomial;

#[test]
fn polynomial_solve_quartic_accessible() {
    // x^4 + 0x^3 + 0x^2 + 0x - 1 = 0 => roots are +/-1 (real) and +/-i.
    let roots = polynomial::solve_quartic(0.0, 0.0, 0.0, -1.0);
    assert_eq!(roots.len(), 4);
    // At least two roots should be real (im ~= 0).
    let real_count = roots.iter().filter(|z| z.im.abs() < 1e-10).count();
    assert_eq!(real_count, 2);
}

#[test]
fn polynomial_pre_solve_cubic_accessible() {
    // t^3 + pt + q = 0 with p=0, q=-1 => t=1 is a root.
    let roots = polynomial::pre_solve_cubic(0.0, -1.0);
    assert_eq!(roots.len(), 3);
    let has_one =
        roots.iter().any(|z| z.im.abs() < 1e-10 && (z.re - 1.0).abs() < 1e-10);
    assert!(has_one);
}
