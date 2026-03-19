//! Detection and handling logic for degenerate boolean operation cases.
//!
//! Provides utilities to detect tangent faces, coincident faces, and
//! pole-degenerate surfaces before or during boolean operations.
//! These are diagnostic utilities available for future integration
//! into the boolean pipeline.

use monstertruck_geometry::prelude::*;
use monstertruck_topology::*;

/// A pair of face indices from two shells that share a tangent relationship.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TangentFacePair {
    /// Face index in the first shell.
    pub face0: usize,
    /// Face index in the second shell.
    pub face1: usize,
}

/// A pair of face indices from two shells that share a coincident relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoincidentFacePair {
    /// Face index in the first shell.
    pub face0: usize,
    /// Face index in the second shell.
    pub face1: usize,
    /// Whether the normals agree in direction.
    pub normals_agree: bool,
}

/// Number of sample points along each parameter axis for face comparison.
const SAMPLE_GRID_SIZE: usize = 3;

/// Detects pairs of faces across two shells that are tangent.
///
/// Tangent faces touch but their intersection curve degenerates to a point
/// or has zero length. Detection samples surface normals at a grid of points
/// and checks if they are parallel (or anti-parallel) while the surfaces
/// are within tolerance.
#[allow(dead_code)]
pub fn detect_tangent_faces<C, S>(
    shell0: &Shell<Point3, C, S>,
    shell1: &Shell<Point3, C, S>,
    tol: f64,
) -> Vec<TangentFacePair>
where
    S: ParametricSurface3D + Clone,
{
    (0..shell0.len())
        .flat_map(|i| (0..shell1.len()).map(move |j| (i, j)))
        .filter(|&(i, j)| {
            let s0 = shell0[i].surface();
            let s1 = shell1[j].surface();
            is_tangent_pair(&s0, &s1, tol)
        })
        .map(|(face0, face1)| TangentFacePair { face0, face1 })
        .collect()
}

/// Detects pairs of faces across two shells that are coincident.
///
/// Coincident faces share the same surface geometry within tolerance.
/// The function also determines whether their normals agree in direction,
/// which affects classification for boolean operations.
pub fn detect_coincident_faces<C, S>(
    shell0: &Shell<Point3, C, S>,
    shell1: &Shell<Point3, C, S>,
    tol: f64,
) -> Vec<CoincidentFacePair>
where
    S: ParametricSurface3D + Clone,
{
    (0..shell0.len())
        .flat_map(|i| (0..shell1.len()).map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            // Skip self-comparison: a face is trivially coincident with itself.
            if shell0[i].id() == shell1[j].id() {
                return None;
            }
            let s0 = shell0[i].surface();
            let s1 = shell1[j].surface();
            is_coincident_pair(&s0, &s1, tol).map(|normals_agree| CoincidentFacePair {
                face0: i,
                face1: j,
                normals_agree,
            })
        })
        .collect()
}

/// Checks whether a surface has pole degeneration.
///
/// A pole-degenerate surface has an entire edge of the parameter domain
/// mapping to a single 3D point (e.g., the poles of a sphere).
/// Returns `true` if any edge of the parameter domain is degenerate.
#[allow(dead_code)]
pub fn is_pole_degenerate<S: ParametricSurface3D>(surface: &S, tol: f64) -> bool {
    let (Some(u_range), Some(v_range)) = surface.try_range_tuple() else {
        return false;
    };
    let (u0, u1) = u_range;
    let (v0, v1) = v_range;

    // Check the four corners of the parameter domain.
    let corners = [
        surface.subs(u0, v0),
        surface.subs(u1, v0),
        surface.subs(u1, v1),
        surface.subs(u0, v1),
    ];

    let tol_sq = tol * tol;

    // Check if adjacent corners map to the same 3D point.
    // If so, the edge between them is degenerate.
    (0..4).any(|i| {
        let j = (i + 1) % 4;
        corners[i].distance2(corners[j]) < tol_sq
    })
}

/// Handles a degenerate intersection result by classifying tangent and
/// coincident face pairs. Returns the number of faces that were handled.
#[allow(dead_code)]
pub fn handle_degenerate_intersection<C, S>(
    shell0: &Shell<Point3, C, S>,
    shell1: &Shell<Point3, C, S>,
    tol: f64,
) -> (Vec<TangentFacePair>, Vec<CoincidentFacePair>)
where
    S: ParametricSurface3D + Clone,
{
    let tangent = detect_tangent_faces(shell0, shell1, tol);
    let coincident = detect_coincident_faces(shell0, shell1, tol);
    (tangent, coincident)
}

/// Checks if two surfaces are tangent at sampled points.
fn is_tangent_pair<S: ParametricSurface3D>(s0: &S, s1: &S, tol: f64) -> bool {
    let (Some(u_range0), Some(v_range0)) = s0.try_range_tuple() else {
        return false;
    };
    let (Some(u_range1), Some(v_range1)) = s1.try_range_tuple() else {
        return false;
    };
    let range0 = (u_range0, v_range0);
    let range1 = (u_range1, v_range1);
    let tol_sq = tol * tol;
    let grid = sample_grid(range0);

    // All sample points on s0 must be close to s1 and have parallel normals.
    grid.iter().all(|&(u, v)| {
        let p0 = s0.subs(u, v);
        let n0 = s0.normal(u, v);

        // Find a nearby point on s1 by sampling its parameter domain.
        let grid1 = sample_grid(range1);
        grid1.iter().any(|&(u1, v1)| {
            let p1 = s1.subs(u1, v1);
            if p0.distance2(p1) > tol_sq {
                return false;
            }
            let n1 = s1.normal(u1, v1);
            // Normals must be parallel (cross product near zero).
            n0.cross(&n1).so_small()
        })
    })
}

/// Checks if two surfaces are coincident at sampled points.
/// Returns `Some(normals_agree)` if coincident, `None` otherwise.
fn is_coincident_pair<S: ParametricSurface3D>(s0: &S, s1: &S, tol: f64) -> Option<bool> {
    let (Some(u_range0), Some(v_range0)) = s0.try_range_tuple() else {
        return None;
    };
    let (Some(u_range1), Some(v_range1)) = s1.try_range_tuple() else {
        return None;
    };
    let range0 = (u_range0, v_range0);
    let range1 = (u_range1, v_range1);
    let tol_sq = tol * tol;
    let grid0 = sample_grid(range0);
    let grid1 = sample_grid(range1);

    let mut all_close = true;
    let mut normals_agree_count = 0usize;
    let mut normals_disagree_count = 0usize;

    grid0.iter().for_each(|&(u, v)| {
        let p0 = s0.subs(u, v);
        let n0 = s0.normal(u, v);

        let any_match = grid1.iter().any(|&(u1, v1)| {
            let p1 = s1.subs(u1, v1);
            if p0.distance2(p1) > tol_sq {
                return false;
            }
            let n1 = s1.normal(u1, v1);
            if !n0.cross(&n1).so_small() {
                return false;
            }
            if n0.dot(n1) > 0.0 {
                normals_agree_count += 1;
            } else {
                normals_disagree_count += 1;
            }
            true
        });
        if !any_match {
            all_close = false;
        }
    });

    if !all_close {
        return None;
    }

    Some(normals_agree_count >= normals_disagree_count)
}

/// Generates a grid of parameter values for sampling a surface.
fn sample_grid(range: ((f64, f64), (f64, f64))) -> Vec<(f64, f64)> {
    let ((u0, u1), (v0, v1)) = range;
    (0..SAMPLE_GRID_SIZE)
        .flat_map(|i| {
            let u = u0 + (u1 - u0) * (i as f64 + 0.5) / SAMPLE_GRID_SIZE as f64;
            (0..SAMPLE_GRID_SIZE).map(move |j| {
                let v = v0 + (v1 - v0) * (j as f64 + 0.5) / SAMPLE_GRID_SIZE as f64;
                (u, v)
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use monstertruck_geometry::specifieds::Plane;

    #[test]
    fn test_is_pole_degenerate_plane() {
        // A plane is not pole-degenerate.
        let plane = Plane::new(
            Point3::origin(),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        );
        assert!(!is_pole_degenerate(&plane, 0.05));
    }

    #[test]
    fn test_sample_grid_produces_correct_count() {
        let grid = sample_grid(((0.0, 1.0), (0.0, 1.0)));
        assert_eq!(grid.len(), SAMPLE_GRID_SIZE * SAMPLE_GRID_SIZE);
    }

    #[test]
    fn test_sample_grid_values_in_range() {
        let grid = sample_grid(((0.0, 1.0), (2.0, 3.0)));
        grid.iter().for_each(|&(u, v)| {
            assert!((0.0..=1.0).contains(&u), "u out of range: {u}");
            assert!((2.0..=3.0).contains(&v), "v out of range: {v}");
        });
    }
}
