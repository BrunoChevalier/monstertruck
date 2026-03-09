#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use monstertruck_geometry::prelude::*;

/// Structured input for fuzzing NURBS curve evaluation.
#[derive(Debug, Arbitrary)]
struct NurbsEvalInput {
    /// Degree of the B-spline (will be clamped to 1..=5).
    degree: u8,
    /// Number of control points (will be clamped to degree+1..=20).
    num_points: u8,
    /// Raw control point coordinates as flat f64 triples.
    coords: Vec<(f64, f64, f64)>,
    /// Parameter at which to evaluate.
    t: f64,
}

fuzz_target!(|input: NurbsEvalInput| {
    let degree = (input.degree % 5).max(1) as usize;
    let min_points = degree + 1;
    let num_points = ((input.num_points as usize) % 16).max(min_points).min(20);

    if input.coords.len() < num_points {
        return;
    }

    // Filter out NaN/Inf control point values.
    let control_points: Vec<Vector4> = input.coords[..num_points]
        .iter()
        .map(|&(x, y, z)| {
            if !x.is_finite() || !y.is_finite() || !z.is_finite() {
                return None;
            }
            Some(Vector4::new(x, y, z, 1.0))
        })
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default();

    if control_points.len() != num_points {
        return;
    }

    let division = num_points - degree;
    let knot_vec = KnotVector::uniform_knot(degree, division);

    // Use `try_new` to avoid panicking on invalid configurations.
    let curve = match BsplineCurve::try_new(knot_vec, control_points) {
        Ok(c) => c,
        Err(_) => return,
    };

    // Evaluate the curve at the given parameter.
    let _ = curve.subs(input.t);
    let _ = curve.der(input.t);
    let _ = curve.der2(input.t);
});
