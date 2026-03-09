#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use monstertruck_geometry::prelude::*;

/// Structured input for fuzzing `KnotVector` operations.
#[derive(Debug, Arbitrary)]
struct KnotVectorInput {
    /// Raw knot values (length will be clamped to 2..=30).
    knots: Vec<f64>,
    /// Degree for basis function evaluation.
    degree: u8,
    /// Parameter for evaluation.
    t: f64,
}

fuzz_target!(|input: KnotVectorInput| {
    // Filter out NaN/Inf knot values and clamp length.
    let knots: Vec<f64> = input
        .knots
        .iter()
        .copied()
        .filter(|v| v.is_finite())
        .take(30)
        .collect();

    if knots.len() < 2 {
        return;
    }

    // `KnotVector::from` sorts the input.
    let mut knot_vec = KnotVector::from(knots);

    let degree = (input.degree as usize) % knot_vec.len();

    // Exercise `try_bspline_basis_functions` -- should never panic.
    let _ = knot_vec.try_bspline_basis_functions(degree, 0, input.t);

    // Exercise knot insertion.
    if input.t.is_finite() {
        knot_vec.add_knot(input.t);
    }

    // Exercise multiplicity query.
    let _ = knot_vec.multiplicity(0);

    // Exercise range length.
    let _ = knot_vec.range_length();

    // Exercise normalization.
    let mut normalize_vec = knot_vec.clone();
    let _ = normalize_vec.try_normalize();

    // Exercise inversion.
    let mut invert_vec = knot_vec.clone();
    invert_vec.invert();

    // Exercise single-multi decomposition and round-trip.
    let (single_knots, mults) = knot_vec.to_single_multi();
    let _ = KnotVector::from_single_multi(single_knots, mults);
});
