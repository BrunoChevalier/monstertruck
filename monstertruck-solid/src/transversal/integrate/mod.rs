use crate::alternative::Alternative;
use crate::healing::RobustSplitClosedEdgesAndFaces;

use super::*;
use monstertruck_geometry::prelude::*;
use monstertruck_meshing::prelude::*;
use monstertruck_topology::{
    compress::{CompressedEdge, CompressedEdgeIndex, CompressedShell},
    errors::Error as TopologyError,
    shell::ShellCondition,
    *,
};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use thiserror::Error;

/// Only solids consisting of faces whose surface is implemented this trait can be used for set operations.
pub trait ShapeOpsSurface:
    ParametricSurface3D
    + ParameterDivision2D
    + SearchParameter<D2, Point = Point3>
    + SearchNearestParameter<D2, Point = Point3>
    + Clone
    + Invertible
    + Send
    + Sync
{
}
impl<S> ShapeOpsSurface for S where
    S: ParametricSurface3D
        + ParameterDivision2D
        + SearchParameter<D2, Point = Point3>
        + SearchNearestParameter<D2, Point = Point3>
        + Clone
        + Invertible
        + Send
        + Sync
{
}

/// Only solids consisting of edges whose curve is implemented this trait can be used for set operations.
pub trait ShapeOpsCurve<S: ShapeOpsSurface>:
    ParametricCurve3D
    + ParameterDivision1D<Point = Point3>
    + Cut
    + Clone
    + TryFrom<ParameterCurve<Line<Point2>, S>>
    + Invertible
    + From<IntersectionCurve<BsplineCurve<Point3>, S, S>>
    + SearchParameter<D1, Point = Point3>
    + SearchNearestParameter<D1, Point = Point3>
    + Send
    + Sync
{
}
impl<C, S: ShapeOpsSurface> ShapeOpsCurve<S> for C where
    C: ParametricCurve3D
        + ParameterDivision1D<Point = Point3>
        + Cut
        + Clone
        + TryFrom<ParameterCurve<Line<Point2>, S>>
        + Invertible
        + From<IntersectionCurve<BsplineCurve<Point3>, S, S>>
        + SearchParameter<D1, Point = Point3>
        + SearchNearestParameter<D1, Point = Point3>
        + Send
        + Sync
{
}

/// Errors for boolean shape operations.
#[derive(Debug, Error)]
pub enum ShapeOpsError {
    /// `tol` was not positive enough for robust meshing and projection.
    #[error("`tol` must be at least `TOLERANCE`.")]
    InvalidTolerance,
    /// Building intersection loops failed.
    #[error("failed to build intersection loops: {source}")]
    CreateLoopsStoreFailed {
        /// Detailed loop-store creation error.
        #[source]
        source: loops_store::CreateLoopsStoreError,
    },
    /// Face division failed for one shell.
    #[error("failed to divide faces for shell {shell_index}.")]
    DivideFacesFailed {
        /// 0 for the first shell, 1 for the second shell.
        shell_index: usize,
    },
    /// Unknown face classification failed for one shell.
    #[error("failed to classify unknown faces for shell {shell_index}.")]
    UnknownClassificationFailed {
        /// 0 for the first shell, 1 for the second shell.
        shell_index: usize,
    },
    /// Converting temporary intersection curves back to target curves failed.
    #[error("failed to convert temporary shell for `{operation}`.")]
    AltShellConversionFailed {
        /// `and` or `or`.
        operation: &'static str,
    },
    /// The generated shell failed manifold checks before solid construction.
    #[error(transparent)]
    InvalidOutputShellCondition(Box<InvalidOutputShellConditionData>),
    /// The output has no boundary shells.
    #[error("invalid output shell for `{operation}`: no boundary shells.")]
    EmptyOutputShell {
        /// Boolean operation name.
        operation: &'static str,
    },
    /// The generated shell is topologically invalid.
    #[error("invalid output shell for `{operation}`: {source}.")]
    InvalidOutputShell {
        /// Boolean operation name.
        operation: &'static str,
        /// Topology validation error.
        #[source]
        source: TopologyError,
    },
}

/// Diagnostic data for invalid output shell conditions.
#[derive(Debug, Error)]
#[error(
    "invalid output shell for `{operation}` at index {shell_index}: empty={empty}, connected={connected}, condition={condition:?}, boundary_loops={boundary_loops}, first_boundary_len={first_boundary_len:?}, first_boundary_front={first_boundary_front:?}, first_boundary_back={first_boundary_back:?}, singular_vertices={singular_vertices}, first_singular={first_singular:?}."
)]
pub struct InvalidOutputShellConditionData {
    /// Boolean operation name.
    pub operation: &'static str,
    /// Boundary shell index.
    pub shell_index: usize,
    /// Whether shell has no faces.
    pub empty: bool,
    /// Whether shell is topologically connected.
    pub connected: bool,
    /// Evaluated shell condition.
    pub condition: ShellCondition,
    /// Count of extracted open boundary wires.
    pub boundary_loops: usize,
    /// Number of edges in first open boundary wire.
    pub first_boundary_len: Option<usize>,
    /// Front point of first open boundary wire.
    pub first_boundary_front: Option<Point3>,
    /// Back point of first open boundary wire.
    pub first_boundary_back: Option<Point3>,
    /// Number of singular vertices.
    pub singular_vertices: usize,
    /// First singular vertex point if present.
    pub first_singular: Option<Point3>,
}

type ShapeOpsResult<T> = std::result::Result<T, ShapeOpsError>;

type AltCurveShell<C, S> =
    Shell<Point3, Alternative<C, IntersectionCurve<PolylineCurve<Point3>, S, S>>, S>;

fn classify_inside_with_polyshell(
    poly_shell: &Shell<Point3, PolylineCurve<Point3>, Option<PolygonMesh>>,
    pt: Point3,
) -> Option<bool> {
    let offsets = [
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.613, -0.271, 0.149),
        Vector3::new(-0.347, 0.509, -0.221),
        Vector3::new(0.193, 0.401, 0.577),
        Vector3::new(-0.433, -0.127, 0.389),
    ];
    let (inside_votes, outside_votes) = offsets
        .into_iter()
        .map(|offset| hash::take_one_unit(pt + offset))
        .try_fold((0usize, 0usize), |(inside, outside), dir| {
            let count = poly_shell.iter().try_fold(0isize, |count, face| {
                let poly = face.surface()?;
                Some(count + poly.signed_crossing_faces(pt, dir))
            })?;
            match count == 0 {
                true => Some((inside, outside + 1)),
                false => Some((inside + 1, outside)),
            }
        })?;
    Some(inside_votes > outside_votes)
}

fn sample_points_on_face<C, S>(face: &Face<Point3, C, S>) -> Option<Vec<Point3>> {
    let wire = face.absolute_boundaries().first()?;
    let vertices: Vec<_> = wire.vertex_iter().map(|v| v.point()).collect();
    let (sum, count) = vertices
        .iter()
        .fold((Vector3::new(0.0, 0.0, 0.0), 0usize), |(sum, count), pt| {
            (sum + pt.to_vec(), count + 1)
        });
    if count == 0 {
        return None;
    }
    let centroid = Point3::from_vec(sum / count as f64);
    Some(
        std::iter::once(centroid)
            .chain(vertices.into_iter().map(|v| centroid.midpoint(v)))
            .collect(),
    )
}

fn classify_unknown_face<C, S>(
    poly_shell: &Shell<Point3, PolylineCurve<Point3>, Option<PolygonMesh>>,
    face: &Face<Point3, C, S>,
) -> Option<bool> {
    let points = sample_points_on_face(face)?;
    let (inside, outside) =
        points
            .into_iter()
            .try_fold((0usize, 0usize), |(inside, outside), pt| {
                if classify_inside_with_polyshell(poly_shell, pt)? {
                    Some((inside + 1, outside))
                } else {
                    Some((inside, outside + 1))
                }
            })?;
    Some(inside >= outside)
}

fn altshell_to_shell<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    altshell: &AltCurveShell<C, S>,
    tol: f64,
) -> Option<Shell<Point3, C, S>> {
    altshell.try_mapped(
        |p| Some(*p),
        |c| match c {
            Alternative::FirstType(c) => Some(c.clone()),
            Alternative::SecondType(ic) => {
                let bsp = BsplineCurve::quadratic_approximation(ic, ic.range_tuple(), tol, 100)?;
                Some(
                    IntersectionCurve::new(ic.surface0().clone(), ic.surface1().clone(), bsp)
                        .into(),
                )
            }
        },
        |s| Some(s.clone()),
    )
}

/// Merge geometrically coincident vertices and edges in a compressed shell.
///
/// Boolean face division produces separate edge instances for shell0 and
/// shell1 along the same intersection curve. After the AND/OR faces are
/// combined, these duplicate edges prevent the shell from being closed.
/// This function welds vertices within `tol` and deduplicates edges that
/// connect the same (welded) vertex pair.
fn weld_compressed_shell<C: Clone, S: Clone>(
    mut cshell: CompressedShell<Point3, C, S>,
    tol: f64,
) -> CompressedShell<Point3, C, S> {
    let n = cshell.vertices.len();
    if n == 0 {
        return cshell;
    }
    // Build a canonical vertex mapping via greedy spatial merge.
    let tol2 = tol * tol;
    let mut canonical: Vec<usize> = (0..n).collect();
    // For efficiency, sort vertex indices by x-coordinate for sweep-based merging.
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by(|&a, &b| {
        cshell.vertices[a][0]
            .partial_cmp(&cshell.vertices[b][0])
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    order.iter().enumerate().for_each(|(i, &vi)| {
        if canonical[vi] != vi {
            return;
        }
        order[i + 1..].iter().for_each(|&vj| {
            if cshell.vertices[vj][0] - cshell.vertices[vi][0] > tol {
                return;
            }
            if canonical[vj] != vj {
                return;
            }
            let d2 = (cshell.vertices[vi] - cshell.vertices[vj]).magnitude2();
            if d2 < tol2 {
                canonical[vj] = vi;
            }
        });
    });
    // Flatten transitive chains.
    (0..n).for_each(|i| {
        let mut root = canonical[i];
        while canonical[root] != root {
            root = canonical[root];
        }
        canonical[i] = root;
    });
    // Build compacted vertex list and remapping.
    let mut new_index: Vec<usize> = vec![usize::MAX; n];
    let mut new_vertices = Vec::new();
    (0..n).for_each(|i| {
        let root = canonical[i];
        if new_index[root] == usize::MAX {
            new_index[root] = new_vertices.len();
            new_vertices.push(cshell.vertices[root]);
        }
        new_index[i] = new_index[root];
    });
    // Remap edge vertex references.
    cshell.edges.iter_mut().for_each(|edge| {
        edge.vertices.0 = new_index[edge.vertices.0];
        edge.vertices.1 = new_index[edge.vertices.1];
    });
    cshell.vertices = new_vertices;
    // Deduplicate edges that connect the same vertex pair (regardless of direction).
    let edge_count = cshell.edges.len();
    let mut edge_canonical: Vec<usize> = (0..edge_count).collect();
    let mut edge_flip: Vec<bool> = vec![false; edge_count];
    let mut seen: HashMap<(usize, usize), (usize, bool)> = HashMap::default();
    (0..edge_count).for_each(|i| {
        let (a, b) = cshell.edges[i].vertices;
        if a == b {
            return;
        }
        let key = if a < b { (a, b) } else { (b, a) };
        let flipped = a != key.0;
        if let Some(&(canon, canon_flipped)) = seen.get(&key) {
            edge_canonical[i] = canon;
            edge_flip[i] = flipped != canon_flipped;
        } else {
            seen.insert(key, (i, flipped));
        }
    });
    // Remap face boundary edge references.
    cshell.faces.iter_mut().for_each(|face| {
        face.boundaries.iter_mut().for_each(|wire| {
            wire.iter_mut().for_each(|cei| {
                let orig = cei.index;
                let canon = edge_canonical[orig];
                if canon != orig {
                    cei.index = canon;
                    if edge_flip[orig] {
                        cei.orientation = !cei.orientation;
                    }
                }
            });
        });
    });
    // Compact edges: remove unused edges and remap indices.
    let mut used: Vec<bool> = vec![false; edge_count];
    cshell.faces.iter().for_each(|face| {
        face.boundaries.iter().for_each(|wire| {
            wire.iter().for_each(|cei| {
                used[cei.index] = true;
            });
        });
    });
    let mut edge_remap: Vec<usize> = vec![0; edge_count];
    let mut new_edges = Vec::new();
    (0..edge_count).for_each(|i| {
        if used[i] {
            edge_remap[i] = new_edges.len();
            new_edges.push(cshell.edges[i].clone());
        }
    });
    cshell.faces.iter_mut().for_each(|face| {
        face.boundaries.iter_mut().for_each(|wire| {
            wire.iter_mut().for_each(|cei| {
                cei.index = edge_remap[cei.index];
            });
        });
    });
    cshell.edges = new_edges;
    cshell
}

/// Split edges at intermediate vertices that lie on the edge curve.
///
/// After vertex welding, some edges may pass through vertices that are now
/// coincident with points on the edge. This happens when face division
/// introduces split points on one face's boundary that are not propagated
/// to the same edge on an adjacent face. Splitting these edges allows faces
/// from different source shells to share sub-edges, producing a connected
/// shell.
fn split_edges_at_intermediate_vertices<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    mut cshell: CompressedShell<Point3, C, S>,
    tol: f64,
) -> CompressedShell<Point3, C, S> {
    let nv = cshell.vertices.len();
    let ne = cshell.edges.len();
    if nv == 0 || ne == 0 {
        return cshell;
    }
    let tol2 = tol * tol;
    // For each edge, find intermediate vertices that lie on the curve.
    let edge_splits: Vec<Vec<(f64, usize)>> = (0..ne)
        .map(|ei| {
            let (va, vb) = cshell.edges[ei].vertices;
            if va == vb {
                return Vec::new();
            }
            let pa = cshell.vertices[va];
            let pb = cshell.vertices[vb];
            let edge_len2 = (pb - pa).magnitude2();
            if edge_len2 < tol2 {
                return Vec::new();
            }
            let curve = &cshell.edges[ei].curve;
            let (t0, t1) = curve.range_tuple();
            (0..nv)
                .filter(|&vi| vi != va && vi != vb)
                .filter_map(|vi| {
                    let pv = cshell.vertices[vi];
                    let da2 = (pv - pa).magnitude2();
                    let db2 = (pv - pb).magnitude2();
                    if da2 > edge_len2 + tol2 || db2 > edge_len2 + tol2 {
                        return None;
                    }
                    // Use fewer trials for this proximity check -- full
                    // Newton convergence is not needed, just a rough
                    // nearest-parameter estimate.
                    let t = curve.search_nearest_parameter(pv, Some((t0 + t1) * 0.5), 10)?;
                    let margin = (t1 - t0) * 0.01;
                    if t <= t0 + margin || t >= t1 - margin {
                        return None;
                    }
                    let cp = curve.subs(t);
                    if cp.distance2(pv) < tol2 {
                        Some((t, vi))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .map(|mut splits| {
            splits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            splits.dedup_by_key(|s| s.1);
            splits
        })
        .collect();
    if edge_splits.iter().all(|s| s.is_empty()) {
        return cshell;
    }
    // Build new edges from splits.
    let mut new_edges: Vec<CompressedEdge<C>> = Vec::new();
    let edge_replacement: Vec<Vec<usize>> = (0..ne)
        .map(|ei| {
            if edge_splits[ei].is_empty() {
                let new_idx = new_edges.len();
                new_edges.push(cshell.edges[ei].clone());
                vec![new_idx]
            } else {
                let (va, vb) = cshell.edges[ei].vertices;
                let splits = &edge_splits[ei];
                let mut result = Vec::with_capacity(splits.len() + 1);
                let mut remaining_curve = cshell.edges[ei].curve.clone();
                let mut prev_vertex = va;
                for &(_t, vi) in splits {
                    let (rt0, rt1) = remaining_curve.range_tuple();
                    if let Some(split_t) = remaining_curve.search_nearest_parameter(
                        cshell.vertices[vi],
                        Some((rt0 + rt1) * 0.5),
                        10,
                    ) {
                        let margin = (rt1 - rt0) * 0.01;
                        if split_t > rt0 + margin && split_t < rt1 - margin {
                            let tail = remaining_curve.cut(split_t);
                            let new_idx = new_edges.len();
                            new_edges.push(CompressedEdge {
                                vertices: (prev_vertex, vi),
                                curve: remaining_curve,
                            });
                            result.push(new_idx);
                            remaining_curve = tail;
                            prev_vertex = vi;
                        }
                    }
                }
                let new_idx = new_edges.len();
                new_edges.push(CompressedEdge {
                    vertices: (prev_vertex, vb),
                    curve: remaining_curve,
                });
                result.push(new_idx);
                result
            }
        })
        .collect();
    // Remap face boundary edge references.
    cshell.faces.iter_mut().for_each(|face| {
        face.boundaries.iter_mut().for_each(|wire| {
            let new_wire: Vec<CompressedEdgeIndex> = wire
                .iter()
                .flat_map(|cei| {
                    let replacement = &edge_replacement[cei.index];
                    let sub: Vec<CompressedEdgeIndex> = replacement
                        .iter()
                        .map(|&new_idx| CompressedEdgeIndex {
                            index: new_idx,
                            orientation: cei.orientation,
                        })
                        .collect();
                    // When the original edge reference is inverted, the sub-edges
                    // must appear in reverse order to maintain wire continuity.
                    if cei.orientation {
                        sub
                    } else {
                        sub.into_iter().rev().collect()
                    }
                })
                .collect();
            *wire = new_wire;
        });
    });
    cshell.edges = new_edges;
    // Deduplicate edges that now share the same vertex pair.
    let final_edge_count = cshell.edges.len();
    let mut edge_canonical: Vec<usize> = (0..final_edge_count).collect();
    let mut edge_flip: Vec<bool> = vec![false; final_edge_count];
    let mut seen: HashMap<(usize, usize), (usize, bool)> = HashMap::default();
    (0..final_edge_count).for_each(|i| {
        let (a, b) = cshell.edges[i].vertices;
        if a == b {
            return;
        }
        let key = if a < b { (a, b) } else { (b, a) };
        let flipped = a != key.0;
        if let Some(&(canon, canon_flipped)) = seen.get(&key) {
            edge_canonical[i] = canon;
            edge_flip[i] = flipped != canon_flipped;
        } else {
            seen.insert(key, (i, flipped));
        }
    });
    cshell.faces.iter_mut().for_each(|face| {
        face.boundaries.iter_mut().for_each(|wire| {
            wire.iter_mut().for_each(|cei| {
                let orig = cei.index;
                let canon = edge_canonical[orig];
                if canon != orig {
                    cei.index = canon;
                    if edge_flip[orig] {
                        cei.orientation = !cei.orientation;
                    }
                }
            });
        });
    });
    // Compact: remove unused edges.
    let mut used: Vec<bool> = vec![false; final_edge_count];
    cshell.faces.iter().for_each(|face| {
        face.boundaries.iter().for_each(|wire| {
            wire.iter().for_each(|cei| {
                used[cei.index] = true;
            });
        });
    });
    let mut edge_remap: Vec<usize> = vec![0; final_edge_count];
    let mut compacted_edges = Vec::new();
    (0..final_edge_count).for_each(|i| {
        if used[i] {
            edge_remap[i] = compacted_edges.len();
            compacted_edges.push(cshell.edges[i].clone());
        }
    });
    cshell.faces.iter_mut().for_each(|face| {
        face.boundaries.iter_mut().for_each(|wire| {
            wire.iter_mut().for_each(|cei| {
                cei.index = edge_remap[cei.index];
            });
        });
    });
    cshell.edges = compacted_edges;
    cshell
}

fn heal_shell_if_needed<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    shell: Shell<Point3, C, S>,
    tol: f64,
) -> Option<Shell<Point3, C, S>> {
    // Stage 0: If already valid, return immediately.
    if shell.shell_condition() == ShellCondition::Closed && shell.singular_vertices().is_empty() {
        return Some(shell);
    }

    let original_quality = shell_quality(&shell);
    let debug_heal = std::env::var("MT_BOOL_DEBUG_HEAL").is_ok();

    // Stage 0.5: Weld coincident vertices/edges from different source shells.
    // Boolean face division creates separate edge instances along intersection
    // curves for each input shell. Welding merges these so the combined shell
    // can become closed.
    let weld_tol = f64::max(tol, 100.0 * TOLERANCE);
    let welded_compressed = weld_compressed_shell(shell.compress(), weld_tol);
    // Split edges that pass through intermediate welded vertices so that
    // faces from different source shells can share sub-edges.
    let welded_compressed = split_edges_at_intermediate_vertices(welded_compressed, weld_tol);
    let welded = Shell::extract(welded_compressed).ok();

    if let Some(ref w) = welded {
        let q = shell_quality(w);
        if debug_heal {
            eprintln!("debug heal stage0.5-weld quality={q:?} original={original_quality:?}");
        }
        if q <= original_quality && q.0 == 0 && q.2 == 0 {
            // Already closed with no singular vertices -- best outcome.
            return welded;
        }
    }

    // Use the best shell so far as the base for further healing.
    let (base_shell, base_quality) = match &welded {
        Some(w) if shell_quality(w) <= original_quality => (w.clone(), shell_quality(w)),
        _ => (shell.clone(), original_quality),
    };

    // Stage 1: Compress + robust heal + extract.
    let mut compressed = base_shell.clone().compress();
    compressed.robust_split_closed_edges_and_faces(tol);
    let healed = Shell::extract(compressed).ok();

    if let Some(ref h) = healed {
        let q = shell_quality(h);
        if debug_heal {
            eprintln!("debug heal stage1 quality={q:?} original={base_quality:?}");
        }
        if q <= base_quality {
            return healed;
        }
    }

    // Stage 2: Compress without heal + extract (in case healing made it worse).
    let compressed_no_heal = base_shell.clone().compress();
    let unhealed = Shell::extract(compressed_no_heal).ok();

    if let Some(ref u) = unhealed {
        let q = shell_quality(u);
        if debug_heal {
            eprintln!("debug heal stage2 quality={q:?} original={base_quality:?}");
        }
        if q <= base_quality {
            return unhealed;
        }
    }

    // Stage 3: Pick the best candidate among all options.
    let candidates: Vec<Shell<Point3, C, S>> = [healed, unhealed, welded]
        .into_iter()
        .flatten()
        .chain(std::iter::once(shell))
        .collect();
    if debug_heal {
        eprintln!("debug heal stage3 candidates={}", candidates.len());
    }
    candidates.into_iter().min_by_key(|s| shell_quality(s))
}

fn shell_condition_rank(condition: ShellCondition) -> usize {
    match condition {
        ShellCondition::Closed => 0,
        ShellCondition::Oriented => 1,
        ShellCondition::Regular => 2,
        ShellCondition::Irregular => 3,
    }
}

fn shell_quality<C, S>(shell: &Shell<Point3, C, S>) -> (usize, usize, usize) {
    (
        shell_condition_rank(shell.shell_condition()),
        shell.extract_boundaries().len(),
        shell.singular_vertices().len(),
    )
}

fn try_cap_shell_with_existing_surfaces<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    shell: Shell<Point3, C, S>,
    _tol: f64,
) -> Shell<Point3, C, S> {
    // Early exit: already closed shells don't need capping.
    if shell.shell_condition() == ShellCondition::Closed {
        return shell;
    }
    let debug_cap = std::env::var("MT_BOOL_DEBUG_CAP").is_ok();
    let mut capped = shell;
    let boundaries = capped.extract_boundaries();
    boundaries.into_iter().for_each(|wire| {
        let edge_ids: Vec<_> = wire.edge_iter().map(|edge| edge.id()).collect();
        let mut candidate_surfaces = Vec::new();
        capped.iter().for_each(|face| {
            let face_edge_ids: HashSet<_> = face.edge_iter().map(|edge| edge.id()).collect();
            if edge_ids
                .iter()
                .any(|edge_id| face_edge_ids.contains(edge_id))
            {
                candidate_surfaces.push(face.surface().clone());
            }
        });
        let current_quality = shell_quality(&capped);
        let mut candidate_count = 0usize;
        let best = candidate_surfaces
            .into_iter()
            .flat_map(|surface| {
                [wire.clone(), wire.inverse()]
                    .into_iter()
                    .filter_map(move |boundary| Face::try_new(vec![boundary], surface.clone()).ok())
            })
            .inspect(|_| candidate_count += 1)
            .map(|face| {
                let mut candidate = capped.clone();
                candidate.push(face.clone());
                (shell_quality(&candidate), face)
            })
            .min_by_key(|(quality, _)| *quality)
            .filter(|(quality, _)| *quality < current_quality)
            .map(|(_, face)| face);
        if debug_cap {
            eprintln!(
                "debug cap boundary_len={} current={:?} candidates={} picked={}",
                wire.len(),
                current_quality,
                candidate_count,
                best.is_some(),
            );
        }
        if let Some(face) = best {
            capped.push(face);
        }
    });
    capped
}

fn process_one_pair_of_shells<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    shell0: &Shell<Point3, C, S>,
    shell1: &Shell<Point3, C, S>,
    tol: f64,
) -> ShapeOpsResult<[Shell<Point3, C, S>; 2]> {
    type ShellQualityScore = (usize, usize, usize, usize);
    type BooleanQualityScore = (ShellQualityScore, ShellQualityScore);

    let debug_bool = std::env::var("MT_BOOL_DEBUG_COUNTS").is_ok();
    // Operation tolerance must be at least the global geometric coincidence threshold.
    if tol < TOLERANCE {
        return Err(ShapeOpsError::InvalidTolerance);
    }

    // Diagnostic: detect coincident faces on original shell geometry.
    // This is logging-only -- it does NOT feed into classification.
    let debug_coincident = std::env::var("MT_BOOL_DEBUG_COINCIDENT").is_ok();
    if debug_coincident {
        let coincident_pairs = edge_cases::detect_coincident_faces(shell0, shell1, tol);
        if !coincident_pairs.is_empty() {
            eprintln!(
                "debug coincident_pairs count={} pairs={:?}",
                coincident_pairs.len(),
                coincident_pairs,
            );
        }
    }

    // Triangulation tolerance: 25% of operation tol, floored at 2x `TOLERANCE` for mesh stability.
    let poly_tol = f64::max(tol * 0.25, 2.0 * TOLERANCE);
    let poly_shell0 = shell0.triangulation(poly_tol);
    let poly_shell1 = shell1.triangulation(poly_tol);
    let altshell0: AltCurveShell<C, S> =
        shell0.mapped(|x| *x, |c| Alternative::FirstType(c.clone()), Clone::clone);
    let altshell1: AltCurveShell<C, S> =
        shell1.mapped(|x| *x, |c| Alternative::FirstType(c.clone()), Clone::clone);

    let loops_store::LoopsStoreQuadruple {
        geom_loops_store0: loops_store0,
        geom_loops_store1: loops_store1,
        ..
    } = loops_store::create_loops_stores_with_tolerance(
        &altshell0,
        &poly_shell0,
        &altshell1,
        &poly_shell1,
        poly_tol,
    )
    .map_err(|source| ShapeOpsError::CreateLoopsStoreFailed { source })?;

    let mut cls0 = divide_face::divide_faces(&altshell0, &loops_store0, tol)
        .ok_or(ShapeOpsError::DivideFacesFailed { shell_index: 0 })?;
    cls0.integrate_by_component();

    let mut cls1 = divide_face::divide_faces(&altshell1, &loops_store1, tol)
        .ok_or(ShapeOpsError::DivideFacesFailed { shell_index: 1 })?;
    cls1.integrate_by_component();

    let [and0, or0, unknown0] = cls0.and_or_unknown();
    if debug_bool {
        eprintln!(
            "debug class0 and={} or={} unknown={}",
            and0.len(),
            or0.len(),
            unknown0.len()
        );
    }
    let mut unknown_faces = Vec::new();
    let mut shell0_classify_failures = 0usize;
    for face in unknown0.into_iter() {
        match classify_unknown_face(&poly_shell1, &face) {
            Some(is_inside) => {
                unknown_faces.push((face.clone(), is_inside));
            }
            None => {
                // Classification failed for this face. Use a conservative default
                // (false = outside = OR) rather than failing the entire boolean.
                // For AND operations this is safe: an incorrectly-classified face
                // will be rejected by the shell quality optimizer.
                shell0_classify_failures += 1;
                unknown_faces.push((face.clone(), false));
            }
        }
    }
    if debug_bool && shell0_classify_failures > 0 {
        eprintln!(
            "debug classify shell0: {shell0_classify_failures} faces fell back to default (outside)"
        );
    }

    let [and1, or1, unknown1] = cls1.and_or_unknown();
    if debug_bool {
        eprintln!(
            "debug class1 and={} or={} unknown={}",
            and1.len(),
            or1.len(),
            unknown1.len()
        );
    }
    let mut shell1_classify_failures = 0usize;
    for face in unknown1.into_iter() {
        match classify_unknown_face(&poly_shell0, &face) {
            Some(is_inside) => {
                unknown_faces.push((face.clone(), is_inside));
            }
            None => {
                shell1_classify_failures += 1;
                unknown_faces.push((face.clone(), false));
            }
        }
    }
    if debug_bool && shell1_classify_failures > 0 {
        eprintln!(
            "debug classify shell1: {shell1_classify_failures} faces fell back to default (outside)"
        );
    }

    // Pre-convert known AND/OR faces from `AltCurve` to regular curves once.
    // This avoids re-running expensive `quadratic_approximation` on intersection
    // curves during every `evaluate` call in the hill-climbing loop.
    let mut known_and_alt: Shell<Point3, _, S> = Shell::default();
    and0.into_iter().for_each(|f| known_and_alt.push(f));
    and1.into_iter().for_each(|f| known_and_alt.push(f));
    let mut known_or_alt: Shell<Point3, _, S> = Shell::default();
    or0.into_iter().for_each(|f| known_or_alt.push(f));
    or1.into_iter().for_each(|f| known_or_alt.push(f));
    let known_and_shell = altshell_to_shell(&known_and_alt, tol)
        .ok_or(ShapeOpsError::AltShellConversionFailed { operation: "and" })?;
    let known_or_shell = altshell_to_shell(&known_or_alt, tol)
        .ok_or(ShapeOpsError::AltShellConversionFailed { operation: "and" })?;
    // Pre-convert unknown faces (cheap: they have no intersection curves).
    let unknown_converted: Vec<(Face<Point3, C, S>, bool)> = unknown_faces
        .iter()
        .map(|(face, is_and)| {
            let mut single: Shell<Point3, _, S> = Shell::default();
            single.push(face.clone());
            altshell_to_shell(&single, tol).map(|s| {
                // SAFETY: single-face shell always has exactly one face.
                let converted_face = s.into_iter().next().unwrap();
                (converted_face, *is_and)
            })
        })
        .collect::<Option<Vec<_>>>()
        .ok_or(ShapeOpsError::AltShellConversionFailed { operation: "and" })?;
    let build_raw_shells = |assignments: &[bool]| -> [Shell<Point3, C, S>; 2] {
        let mut and_shell = known_and_shell.clone();
        let mut or_shell = known_or_shell.clone();
        unknown_converted
            .iter()
            .zip(assignments.iter().copied())
            .for_each(|((face, _), is_and)| {
                if is_and {
                    and_shell.push(face.clone());
                } else {
                    or_shell.push(face.clone());
                }
            });
        [and_shell, or_shell]
    };
    let build_shells = |assignments: &[bool]| -> Option<[Shell<Point3, C, S>; 2]> {
        let [and_shell, or_shell] = build_raw_shells(assignments);
        let and_shell = heal_shell_if_needed(and_shell, tol)?;
        let or_shell = heal_shell_if_needed(or_shell, tol)?;
        Some([and_shell, or_shell])
    };
    let score = |shell: &Shell<Point3, C, S>| -> ShellQualityScore {
        (
            usize::from(shell.is_empty()),
            shell_condition_rank(shell.shell_condition()),
            shell.extract_boundaries().len(),
            shell.singular_vertices().len(),
        )
    };
    let evaluate = |assignments: &[bool]| -> BooleanQualityScore {
        let [and_shell, or_shell] = build_raw_shells(assignments);
        (score(&and_shell), score(&or_shell))
    };
    let mut assignments: Vec<bool> = unknown_converted
        .iter()
        .map(|(_, is_and)| *is_and)
        .collect();
    let mut best_score = evaluate(&assignments);
    let exact_unknown = std::env::var("MT_BOOL_EXACT_UNKNOWN").is_ok();
    if exact_unknown && unknown_converted.len() <= 12 {
        let total = 1usize << unknown_converted.len();
        let mut best_assignments = assignments.clone();
        (0..total).for_each(|mask| {
            let candidate: Vec<bool> = (0..unknown_converted.len())
                .map(|i| ((mask >> i) & 1) == 1)
                .collect();
            let candidate_score = evaluate(&candidate);
            if candidate_score < best_score {
                best_score = candidate_score;
                best_assignments = candidate;
            }
        });
        assignments = best_assignments;
    } else if unknown_converted.len() <= 24 {
        let mut improved = true;
        while improved {
            improved = false;
            (0..assignments.len()).for_each(|index| {
                let mut candidate = assignments.clone();
                candidate[index] = !candidate[index];
                let candidate_score = evaluate(&candidate);
                if candidate_score < best_score {
                    assignments = candidate;
                    best_score = candidate_score;
                    improved = true;
                }
            });
        }
    }
    let [and_shell, or_shell] = build_shells(&assignments)
        .ok_or(ShapeOpsError::AltShellConversionFailed { operation: "and" })?;
    if debug_bool {
        let and_count =
            known_and_shell.len() + assignments.iter().copied().filter(|is_and| *is_and).count();
        let or_count =
            known_or_shell.len() + assignments.len() - (and_count - known_and_shell.len());
        eprintln!(
            "debug class-final and={} or={} score_and={:?} score_or={:?}",
            and_count, or_count, best_score.0, best_score.1,
        );
    }
    if debug_bool {
        eprintln!(
            "debug shell-final and_faces={} and_condition={:?} and_boundary={} and_singular={} | or_faces={} or_condition={:?} or_boundary={} or_singular={}",
            and_shell.len(),
            and_shell.shell_condition(),
            and_shell.extract_boundaries().len(),
            and_shell.singular_vertices().len(),
            or_shell.len(),
            or_shell.shell_condition(),
            or_shell.extract_boundaries().len(),
            or_shell.singular_vertices().len(),
        );
    }

    Ok([and_shell, or_shell])
}

fn try_build_solid<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    operation: &'static str,
    boundaries: Vec<Shell<Point3, C, S>>,
    tol: f64,
) -> ShapeOpsResult<Solid<Point3, C, S>> {
    let boundaries: Vec<_> = boundaries
        .into_iter()
        .map(|shell| try_cap_shell_with_existing_surfaces(shell, tol))
        .collect();
    if boundaries.is_empty() {
        return Err(ShapeOpsError::EmptyOutputShell { operation });
    }
    let is_valid = |shell: &Shell<Point3, C, S>| {
        !shell.is_empty()
            && shell.is_connected()
            && shell.shell_condition() == ShellCondition::Closed
            && shell.singular_vertices().is_empty()
    };
    let valid_boundaries: Vec<_> = boundaries
        .iter()
        .filter(|shell| is_valid(shell))
        .cloned()
        .collect();
    if valid_boundaries.is_empty() {
        let (shell_index, shell) = boundaries
            .iter()
            .enumerate()
            .find(|(_, shell)| !is_valid(shell))
            .ok_or(ShapeOpsError::EmptyOutputShell { operation })?;
        let boundary_loops = shell.extract_boundaries();
        if std::env::var("MT_BOOL_DEBUG_BOUNDARY").is_ok() {
            boundary_loops.iter().enumerate().for_each(|(index, wire)| {
                let points: Vec<_> = wire.vertex_iter().map(|vertex| vertex.point()).collect();
                eprintln!(
                    "debug boundary loop index={index} len={} points={points:?}",
                    wire.len()
                );
            });
        }
        let first_boundary = boundary_loops.first();
        return Err(ShapeOpsError::InvalidOutputShellCondition(Box::new(
            InvalidOutputShellConditionData {
                operation,
                shell_index,
                empty: shell.is_empty(),
                connected: shell.is_connected(),
                condition: shell.shell_condition(),
                boundary_loops: boundary_loops.len(),
                first_boundary_len: first_boundary.map(|wire| wire.len()),
                first_boundary_front: first_boundary
                    .and_then(Wire::front_vertex)
                    .map(Vertex::point),
                first_boundary_back: first_boundary
                    .and_then(Wire::back_vertex)
                    .map(Vertex::point),
                singular_vertices: shell.singular_vertices().len(),
                first_singular: shell.singular_vertices().first().map(Vertex::point),
            },
        )));
    }
    let output_boundaries = if valid_boundaries.len() < boundaries.len() {
        if std::env::var("MT_BOOL_DEBUG_COMPONENTS").is_ok() {
            boundaries
                .iter()
                .enumerate()
                .filter(|(_, shell)| !is_valid(shell))
                .for_each(|(i, shell)| {
                    eprintln!(
                        "debug build_solid dropping shell[{i}] condition={:?} boundary={} singular={}",
                        shell.shell_condition(),
                        shell.extract_boundaries().len(),
                        shell.singular_vertices().len(),
                    );
                });
        }
        valid_boundaries
    } else {
        boundaries
    };
    Solid::try_new(output_boundaries)
        .map_err(|source| ShapeOpsError::InvalidOutputShell { operation, source })
}

/// AND operation between two solids.
pub fn and<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    solid0: &Solid<Point3, C, S>,
    solid1: &Solid<Point3, C, S>,
    tol: f64,
) -> ShapeOpsResult<Solid<Point3, C, S>> {
    let debug_components = std::env::var("MT_BOOL_DEBUG_COMPONENTS").is_ok();
    let mut iter0 = solid0.boundaries().iter();
    let mut iter1 = solid1.boundaries().iter();
    let shell0 = iter0
        .next()
        .ok_or(ShapeOpsError::EmptyOutputShell { operation: "and" })?;
    let shell1 = iter1
        .next()
        .ok_or(ShapeOpsError::EmptyOutputShell { operation: "and" })?;

    let [mut and_shell, _] = process_one_pair_of_shells(shell0, shell1, tol)?;
    for shell in iter0 {
        let [res, _] = process_one_pair_of_shells(&and_shell, shell, tol)?;
        and_shell = res;
    }
    for shell in iter1 {
        let [res, _] = process_one_pair_of_shells(&and_shell, shell, tol)?;
        and_shell = res;
    }

    let boundaries = {
        let comps = and_shell.connected_components();
        if comps.is_empty() {
            vec![and_shell.clone()]
        } else {
            comps
        }
    };
    if debug_components {
        boundaries.iter().enumerate().for_each(|(i, shell)| {
            eprintln!(
                "debug and component[{i}] faces={} condition={:?} boundary={} singular={}",
                shell.len(),
                shell.shell_condition(),
                shell.extract_boundaries().len(),
                shell.singular_vertices().len(),
            );
        });
    }
    try_build_solid("and", boundaries, tol)
}

/// OR operation between two solids.
pub fn or<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    solid0: &Solid<Point3, C, S>,
    solid1: &Solid<Point3, C, S>,
    tol: f64,
) -> ShapeOpsResult<Solid<Point3, C, S>> {
    let debug_components = std::env::var("MT_BOOL_DEBUG_COMPONENTS").is_ok();
    let mut iter0 = solid0.boundaries().iter();
    let mut iter1 = solid1.boundaries().iter();
    let shell0 = iter0
        .next()
        .ok_or(ShapeOpsError::EmptyOutputShell { operation: "or" })?;
    let shell1 = iter1
        .next()
        .ok_or(ShapeOpsError::EmptyOutputShell { operation: "or" })?;

    let [_, mut or_shell] = process_one_pair_of_shells(shell0, shell1, tol)?;
    for shell in iter0 {
        let [_, res] = process_one_pair_of_shells(&or_shell, shell, tol)?;
        or_shell = res;
    }
    for shell in iter1 {
        let [_, res] = process_one_pair_of_shells(&or_shell, shell, tol)?;
        or_shell = res;
    }

    let boundaries = {
        let comps = or_shell.connected_components();
        if comps.is_empty() {
            vec![or_shell.clone()]
        } else {
            comps
        }
    };
    if debug_components {
        boundaries.iter().enumerate().for_each(|(i, shell)| {
            eprintln!(
                "debug or component[{i}] faces={} condition={:?} boundary={} singular={}",
                shell.len(),
                shell.shell_condition(),
                shell.extract_boundaries().len(),
                shell.singular_vertices().len(),
            );
        });
    }
    try_build_solid("or", boundaries, tol)
}

/// Difference: the region inside `solid0` but outside `solid1`.
pub fn difference<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    solid0: &Solid<Point3, C, S>,
    solid1: &Solid<Point3, C, S>,
    tol: f64,
) -> ShapeOpsResult<Solid<Point3, C, S>> {
    let mut neg = solid1.clone();
    neg.not();
    and(solid0, &neg, tol)
}

/// Symmetric difference (XOR): the region inside exactly one of the solids.
pub fn symmetric_difference<C: ShapeOpsCurve<S>, S: ShapeOpsSurface>(
    solid0: &Solid<Point3, C, S>,
    solid1: &Solid<Point3, C, S>,
    tol: f64,
) -> ShapeOpsResult<Solid<Point3, C, S>> {
    let d0 = difference(solid0, solid1, tol)?;
    let d1 = difference(solid1, solid0, tol)?;
    or(&d0, &d1, tol)
}

#[cfg(test)]
mod tests;
