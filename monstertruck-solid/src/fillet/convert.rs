use monstertruck_core::tolerance_constants::SNAP_TOLERANCE;
use monstertruck_geometry::prelude::*;
use monstertruck_traits::{BoundedCurve, ParametricCurve, ParametricSurface};

use super::error::FilletError;
use super::types::{self, Curve, ParameterCurveLinear};

type InternalShell = types::Shell;
const CURVE_SAMPLE_COUNT: usize = 24;
const SURFACE_SAMPLE_COUNT: usize = 64;

/// Intersection curve type used internally by fillet operations.
pub type FilletIntersectionCurve =
    IntersectionCurve<ParameterCurveLinear, Box<NurbsSurface<Vector4>>, Box<NurbsSurface<Vector4>>>;

/// Surface types that can participate in fillet operations.
///
/// Automatically implemented for any type satisfying the bounds.
pub trait FilletableSurface:
    Clone
    + ParametricSurface<Point = Point3>
    + TryInto<NurbsSurface<Vector4>>
    + From<NurbsSurface<Vector4>>
{
    /// Converts this surface to a NURBS surface used by internal fillet logic.
    fn to_nurbs_surface(&self) -> Option<NurbsSurface<Vector4>> {
        self.clone()
            .try_into()
            .ok()
            .or_else(|| sample_surface_to_nurbs(self, SURFACE_SAMPLE_COUNT))
    }
}

impl<T> FilletableSurface for T where
    T: Clone
        + ParametricSurface<Point = Point3>
        + TryInto<NurbsSurface<Vector4>>
        + From<NurbsSurface<Vector4>>
{
}

/// Curve types that can participate in fillet operations.
///
/// Automatically implemented for any type satisfying the bounds.
pub trait FilletableCurve:
    Clone
    + ParametricCurve<Point = Point3>
    + BoundedCurve
    + TryInto<NurbsCurve<Vector4>>
    + From<NurbsCurve<Vector4>>
    + From<ParameterCurveLinear>
    + From<FilletIntersectionCurve>
{
    /// Converts this curve to a NURBS curve used by internal fillet logic.
    fn to_nurbs_curve(&self) -> NurbsCurve<Vector4> {
        self.clone().try_into().ok().unwrap_or_else(|| {
            sample_curve_to_nurbs(self.range_tuple(), |t| self.evaluate(t), CURVE_SAMPLE_COUNT)
        })
    }
}

impl<T> FilletableCurve for T where
    T: Clone
        + ParametricCurve<Point = Point3>
        + BoundedCurve
        + TryInto<NurbsCurve<Vector4>>
        + From<NurbsCurve<Vector4>>
        + From<ParameterCurveLinear>
        + From<FilletIntersectionCurve>
{
}

/// Snaps the first and last control points of a clamped NURBS curve to
/// match the given target positions exactly. For clamped knot vectors
/// (produced by [`KnotVector::uniform_knot`]), the first/last control points
/// directly determine the curve's start/end positions.
pub(super) fn snap_curve_endpoints(
    curve: &mut NurbsCurve<Vector4>,
    front: Point3,
    back: Point3,
) {
    let n = curve.control_points().len();
    if n == 0 {
        return;
    }
    let w0 = curve.control_points()[0][3];
    *curve.control_point_mut(0) = Vector4::new(front.x * w0, front.y * w0, front.z * w0, w0);
    if n > 1 {
        let wn = curve.control_points()[n - 1][3];
        *curve.control_point_mut(n - 1) =
            Vector4::new(back.x * wn, back.y * wn, back.z * wn, wn);
    }
}

pub(super) fn sample_curve_to_nurbs(
    range: (f64, f64),
    evaluate: impl Fn(f64) -> Point3,
    sample_count: usize,
) -> NurbsCurve<Vector4> {
    let (t0, t1) = range;
    let n_points = sample_count + 1;
    let knot = KnotVector::uniform_knot(3, n_points - 3);
    let param_points: Vec<(f64, Point3)> = (0..n_points)
        .map(|i| {
            let u = i as f64 / (n_points - 1) as f64;
            let t = t0 + (t1 - t0) * u;
            (u, evaluate(t))
        })
        .collect();
    let front = evaluate(t0);
    let back = evaluate(t1);
    match BsplineCurve::try_interpolate(knot, param_points) {
        Ok(bsp) => {
            let mut nc = NurbsCurve::from(bsp);
            snap_curve_endpoints(&mut nc, front, back);
            nc
        }
        Err(_) => {
            // Degree-1 fallback.
            let points: Vec<Point3> = (0..=sample_count)
                .map(|i| t0 + (t1 - t0) * (i as f64) / (sample_count as f64))
                .map(&evaluate)
                .collect();
            let knot_vector = KnotVector::uniform_knot(1, sample_count);
            NurbsCurve::from(BsplineCurve::new(knot_vector, points))
        }
    }
}

/// Computes the Greville abscissae for a knot vector of given degree.
/// These are the optimal parameter values for B-spline interpolation.
fn greville_abscissae(knots: &KnotVector, degree: usize) -> Vec<f64> {
    let n = knots.len() - degree - 1;
    (0..n)
        .map(|i| (1..=degree).map(|j| knots[i + j]).sum::<f64>() / degree as f64)
        .collect()
}

fn sample_surface_to_nurbs<S: ParametricSurface<Point = Point3>>(
    surface: &S,
    sample_count: usize,
) -> Option<NurbsSurface<Vector4>> {
    let (u_range, v_range) = surface.try_range_tuple();
    let ((u0, u1), (v0, v1)) = u_range.zip(v_range)?;

    if let Some(result) = try_degree3_surface(surface, sample_count, (u0, u1), (v0, v1)) {
        Some(result)
    } else {
        // Degree-1 fallback.
        let control_points: Vec<Vec<Point3>> = (0..=sample_count)
            .map(|iu| {
                let u = u0 + (u1 - u0) * (iu as f64) / (sample_count as f64);
                (0..=sample_count)
                    .map(|iv| {
                        let v = v0 + (v1 - v0) * (iv as f64) / (sample_count as f64);
                        surface.evaluate(u, v)
                    })
                    .collect()
            })
            .collect();
        let u_knot = KnotVector::uniform_knot(1, sample_count);
        let v_knot = KnotVector::uniform_knot(1, sample_count);
        Some(NurbsSurface::from(BsplineSurface::new(
            (u_knot, v_knot),
            control_points,
        )))
    }
}

/// Attempts degree-3 tensor product interpolation for a surface.
/// Returns `None` if any interpolation step fails.
fn try_degree3_surface<S: ParametricSurface<Point = Point3>>(
    surface: &S,
    sample_count: usize,
    u_range: (f64, f64),
    v_range: (f64, f64),
) -> Option<NurbsSurface<Vector4>> {
    let (u0, u1) = u_range;
    let (v0, v1) = v_range;
    let n_points = sample_count + 1;
    let u_knot = KnotVector::uniform_knot(3, n_points - 3);
    let v_knot = KnotVector::uniform_knot(3, n_points - 3);

    let u_grev = greville_abscissae(&u_knot, 3);
    let v_grev = greville_abscissae(&v_knot, 3);

    // Map Greville abscissae to surface domain.
    let u_params: Vec<f64> = u_grev.iter().map(|&g| u0 + (u1 - u0) * g).collect();
    let v_params: Vec<f64> = v_grev.iter().map(|&g| v0 + (v1 - v0) * g).collect();

    // Sample the surface at the tensor product grid.
    let surface_points: Vec<Vec<Point3>> = u_params
        .iter()
        .map(|&u| v_params.iter().map(|&v| surface.evaluate(u, v)).collect())
        .collect();

    // First pass -- interpolate each row (v-direction).
    let row_curves: Vec<BsplineCurve<Point3>> = surface_points
        .iter()
        .map(|row| {
            let params: Vec<(f64, Point3)> =
                v_grev.iter().copied().zip(row.iter().copied()).collect();
            BsplineCurve::try_interpolate(v_knot.clone(), params)
        })
        .collect::<std::result::Result<_, _>>()
        .ok()?;

    // Collect intermediate control points (one row per u-sample).
    let intermediate: Vec<Vec<Point3>> = row_curves
        .iter()
        .map(|c| c.control_points().to_vec())
        .collect();

    // Second pass -- interpolate each column (u-direction).
    let col_cps: Vec<Vec<Point3>> = (0..n_points)
        .map(|j| {
            let params: Vec<(f64, Point3)> = u_grev
                .iter()
                .copied()
                .zip(intermediate.iter().map(|row| row[j]))
                .collect();
            BsplineCurve::try_interpolate(u_knot.clone(), params)
                .map(|c| c.control_points().to_vec())
        })
        .collect::<std::result::Result<_, _>>()
        .ok()?;

    // Transpose from [V][U] to [U][V] for [`BsplineSurface`].
    let control_points: Vec<Vec<Point3>> = (0..n_points)
        .map(|i| (0..n_points).map(|j| col_cps[j][i]).collect())
        .collect();

    Some(NurbsSurface::from(BsplineSurface::new(
        (u_knot, v_knot),
        control_points,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use monstertruck_core::assert_near;

    /// Verifies that `sample_curve_to_nurbs` produces a degree-3 curve
    /// and that the resulting curve passes through the sampled points.
    #[test]
    fn sample_curve_to_nurbs_produces_degree_3() {
        // A quarter circle in the XY plane: (cos(t), sin(t), 0) for t in [0, PI/2].
        let range = (0.0, std::f64::consts::FRAC_PI_2);
        let evaluate = |t: f64| Point3::new(t.cos(), t.sin(), 0.0);
        let sample_count = 24;

        let nurbs = sample_curve_to_nurbs(range, evaluate, sample_count);

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

    /// Verifies that `sample_surface_to_nurbs` produces a degree-3 surface
    /// in both parametric directions.
    #[test]
    fn sample_surface_to_nurbs_produces_degree_3() {
        // A simple parametric surface: S(u, v) = (u, v, u*v) on [0, 1] x [0, 1].
        // We use a BsplineSurface to provide `try_range_tuple`.
        let n = 5;
        let u_knot = KnotVector::uniform_knot(1, n);
        let v_knot = KnotVector::uniform_knot(1, n);
        let control_points: Vec<Vec<Point3>> = (0..=n)
            .map(|i| {
                let u = i as f64 / n as f64;
                (0..=n)
                    .map(|j| {
                        let v = j as f64 / n as f64;
                        Point3::new(u, v, u * v)
                    })
                    .collect()
            })
            .collect();
        let surface = BsplineSurface::new((u_knot, v_knot), control_points);

        let nurbs = sample_surface_to_nurbs(&surface, 24).expect("should produce a surface");

        // Must be degree 3 in both directions.
        let (u_deg, v_deg) = nurbs.degrees();
        assert_eq!(u_deg, 3, "u-direction should be degree 3");
        assert_eq!(v_deg, 3, "v-direction should be degree 3");
    }
}

// TryFrom for extracting NurbsCurve from internal Curve type.
impl TryFrom<Curve> for NurbsCurve<Vector4> {
    type Error = ();
    fn try_from(curve: Curve) -> std::result::Result<Self, ()> {
        match curve {
            Curve::NurbsCurve(c) => Ok(c),
            _ => Err(()),
        }
    }
}

/// Convert an external shell to internal fillet types.
///
/// Returns the internal shell and the internal `EdgeId`s corresponding to
/// the selected external edges (matched by endpoint positions).
pub(super) fn convert_shell_in<C: FilletableCurve, S: FilletableSurface>(
    shell: &monstertruck_topology::Shell<Point3, C, S>,
    edges: &[monstertruck_topology::Edge<Point3, C>],
) -> std::result::Result<(InternalShell, Vec<types::EdgeId>), FilletError> {
    // Collect endpoint pairs for requested edges (front, back).
    let edge_endpoints: Vec<(Point3, Point3)> = edges
        .iter()
        .map(|e| (e.absolute_front().point(), e.absolute_back().point()))
        .collect();

    let internal_shell: InternalShell = shell
        .try_mapped(
            |p| Some(*p),
            |c| Some(Curve::NurbsCurve(c.to_nurbs_curve())),
            |s| s.to_nurbs_surface(),
        )
        .ok_or(FilletError::UnsupportedGeometry {
            context: "failed to convert shell curves or surfaces to NURBS",
        })?;

    // Snap curve endpoints to vertex positions after conversion.
    // `Edge::set_curve` takes `&self` (interior mutability via `Arc<RwLock>`),
    // so immutable `edge_iter()` is sufficient.
    for edge in internal_shell.edge_iter() {
        let front = edge.absolute_front().point();
        let back = edge.absolute_back().point();
        let mut curve = edge.curve();
        if let Curve::NurbsCurve(ref mut nc) = curve {
            snap_curve_endpoints(nc, front, back);
            edge.set_curve(curve);
        }
    }

    // Match external edges to internal edges by endpoint positions.
    let internal_edge_ids: Vec<types::EdgeId> = edge_endpoints
        .iter()
        .map(|(ext_front, ext_back)| {
            internal_shell
                .edge_iter()
                .find(|ie| {
                    let f = ie.absolute_front().point();
                    let b = ie.absolute_back().point();
                    (f.abs_diff_eq(ext_front, SNAP_TOLERANCE)
                        && b.abs_diff_eq(ext_back, SNAP_TOLERANCE))
                        || (f.abs_diff_eq(ext_back, SNAP_TOLERANCE)
                            && b.abs_diff_eq(ext_front, SNAP_TOLERANCE))
                })
                .map(|ie| ie.id())
                .ok_or(FilletError::EdgeNotFound)
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok((internal_shell, internal_edge_ids))
}

/// Convert an internal fillet shell back to external types.
pub(super) fn convert_shell_out<C: FilletableCurve, S: FilletableSurface>(
    shell: &InternalShell,
) -> std::result::Result<monstertruck_topology::Shell<Point3, C, S>, FilletError> {
    // Snap internal curves to vertex positions before converting out.
    // `Edge::set_curve` takes `&self`, so we can mutate through immutable
    // shell reference.
    for edge in shell.edge_iter() {
        let front = edge.absolute_front().point();
        let back = edge.absolute_back().point();
        let mut curve = edge.curve();
        if let Curve::NurbsCurve(ref mut nc) = curve {
            snap_curve_endpoints(nc, front, back);
            edge.set_curve(curve);
        }
    }
    shell
        .try_mapped(
            |p| Some(*p),
            |c| {
                Some(match c {
                    Curve::NurbsCurve(nc) => C::from(nc.clone()),
                    Curve::ParameterCurve(pc) => C::from(pc.clone()),
                    Curve::IntersectionCurve(ic) => C::from(ic.clone()),
                })
            },
            |s| Some(S::from(s.clone())),
        )
        .ok_or(FilletError::UnsupportedGeometry {
            context: "failed to convert internal shell back to external types",
        })
}
