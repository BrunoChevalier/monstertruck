use monstertruck_geometry::prelude::*;
use monstertruck_solid::{FilletIntersectionCurve, ParameterCurveLinear};

use crate::{Curve, Surface};

impl TryFrom<Surface> for NurbsSurface<Vector4> {
    type Error = ();
    fn try_from(surface: Surface) -> std::result::Result<Self, ()> {
        match surface {
            Surface::Plane(plane) => Ok(NurbsSurface::from(BsplineSurface::from(plane))),
            Surface::BsplineSurface(bsp) => Ok(NurbsSurface::from(bsp)),
            Surface::NurbsSurface(ns) => Ok(ns),
            Surface::RevolutedCurve(_) | Surface::TSplineSurface(_) => Err(()),
        }
    }
}
// From<NurbsSurface<Vector4>> for Surface -- provided by derive_more::From

impl TryFrom<Curve> for NurbsCurve<Vector4> {
    type Error = ();
    fn try_from(curve: Curve) -> std::result::Result<Self, ()> {
        match curve {
            Curve::Line(line) => Ok(NurbsCurve::from(BsplineCurve::from(line))),
            Curve::BsplineCurve(bsp) => Ok(NurbsCurve::from(bsp)),
            Curve::NurbsCurve(nc) => Ok(nc),
            Curve::IntersectionCurve(ic) => {
                let range = ic.range_tuple();
                Ok(sample_to_nurbs(range, |t| ic.subs(t), 16))
            }
        }
    }
}
// From<NurbsCurve<Vector4>> for Curve -- provided by derive_more::From

impl From<ParameterCurveLinear> for Curve {
    fn from(c: ParameterCurveLinear) -> Self {
        let range = c.range_tuple();
        Curve::NurbsCurve(sample_to_nurbs(range, |t| c.subs(t), 16))
    }
}

impl From<FilletIntersectionCurve> for Curve {
    fn from(c: FilletIntersectionCurve) -> Self {
        let range = c.range_tuple();
        Curve::NurbsCurve(sample_to_nurbs(range, |t| c.subs(t), 16))
    }
}

/// Sample a parametric curve into a degree-1 NURBS polyline approximation.
fn sample_to_nurbs(
    range: (f64, f64),
    subs: impl Fn(f64) -> Point3,
    n: usize,
) -> NurbsCurve<Vector4> {
    let (t0, t1) = range;
    let pts: Vec<Point3> = (0..=n)
        .map(|i| subs(t0 + (t1 - t0) * (i as f64) / (n as f64)))
        .collect();
    let knots: Vec<f64> = (0..=n).map(|i| i as f64 / n as f64).collect();
    let knot_vec = KnotVector::from(
        std::iter::once(0.0)
            .chain(knots.iter().copied())
            .chain(std::iter::once(1.0))
            .collect::<Vec<_>>(),
    );
    let bsp = BsplineCurve::new(knot_vec, pts);
    NurbsCurve::from(bsp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use monstertruck_core::assert_near;

    /// Verifies that `sample_to_nurbs` produces a degree-3 curve
    /// and that the resulting curve interpolates the sampled points.
    #[test]
    fn sample_to_nurbs_produces_degree_3() {
        // A quarter circle in the XY plane: (cos(t), sin(t), 0) for t in [0, PI/2].
        let range = (0.0, std::f64::consts::FRAC_PI_2);
        let evaluate = |t: f64| Point3::new(t.cos(), t.sin(), 0.0);
        let n = 24;

        let nurbs = sample_to_nurbs(range, evaluate, n);

        // Must be degree 3.
        assert_eq!(nurbs.degree(), 3);

        // Must interpolate the endpoints.
        assert_near!(nurbs.subs(0.0), Point3::new(1.0, 0.0, 0.0));
        assert_near!(nurbs.subs(1.0), Point3::new(0.0, 1.0, 0.0));

        // Must approximate the midpoint well.
        let mid_t = std::f64::consts::FRAC_PI_4;
        let expected_mid = Point3::new(mid_t.cos(), mid_t.sin(), 0.0);
        let actual_mid = nurbs.subs(0.5);
        assert!(
            actual_mid.distance(expected_mid) < 0.01,
            "midpoint error too large: {actual_mid:?} vs {expected_mid:?}",
        );
    }
}
