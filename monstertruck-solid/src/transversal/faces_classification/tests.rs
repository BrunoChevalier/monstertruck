use super::{
    super::{divide_face, loops_store},
    *,
};
use monstertruck_geometry::prelude::*;
use monstertruck_meshing::prelude::*;
use monstertruck_topology::{Vertex, shell::ShellCondition};
const TOL: f64 = 0.05;

type AlternativeIntersection = crate::alternative::Alternative<
    NurbsCurve<Vector4>,
    IntersectionCurve<PolylineCurve<Point3>, AlternativeSurface, AlternativeSurface>,
>;
type AlternativeSurface = crate::alternative::Alternative<BsplineSurface<Point3>, Plane>;

fn parabola_surfaces() -> (AlternativeSurface, AlternativeSurface) {
    // define surfaces
    #[rustfmt::skip]
	let ctrl0 = vec![
		vec![Point3::new(-1.0, -1.0, 3.0), Point3::new(-1.0, 0.0, -1.0), Point3::new(-1.0, 1.0, 3.0)],
		vec![Point3::new(0.0, -1.0, -1.0), Point3::new(0.0, 0.0, -5.0), Point3::new(0.0, 1.0, -1.0)],
		vec![Point3::new(1.0, -1.0, 3.0), Point3::new(1.0, 0.0, -1.0), Point3::new(1.0, 1.0, 3.0)],
	];
    #[rustfmt::skip]
	let ctrl1 = vec![
		vec![Point3::new(-1.0, -1.0, -3.0), Point3::new(-1.0, 0.0, 1.0), Point3::new(-1.0, 1.0, -3.0)],
		vec![Point3::new(0.0, -1.0, 1.0), Point3::new(0.0, 0.0, 5.0), Point3::new(0.0, 1.0, 1.0)],
		vec![Point3::new(1.0, -1.0, -3.0), Point3::new(1.0, 0.0, 1.0), Point3::new(1.0, 1.0, -3.0)],
	];
    (
        BsplineSurface::new(
            (KnotVector::bezier_knot(2), KnotVector::bezier_knot(2)),
            ctrl0,
        )
        .into(),
        BsplineSurface::new(
            (KnotVector::bezier_knot(2), KnotVector::bezier_knot(2)),
            ctrl1,
        )
        .into(),
    )
}

#[test]
fn independent_intersection() {
    // prepare geoetries
    let arc00: AlternativeIntersection = NurbsCurve::new(BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Vector4::new(1.0, 0.0, 1.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 1.0, 1.0),
        ],
    ))
    .into();
    let arc01: AlternativeIntersection = NurbsCurve::new(BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Vector4::new(-1.0, 0.0, 1.0, 1.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(1.0, 0.0, 1.0, 1.0),
        ],
    ))
    .into();
    let arc10: AlternativeIntersection = NurbsCurve::new(BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Vector4::new(1.0, 0.0, -1.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, -1.0, 1.0),
        ],
    ))
    .into();
    let arc11: AlternativeIntersection = NurbsCurve::new(BsplineCurve::new(
        KnotVector::bezier_knot(2),
        vec![
            Vector4::new(-1.0, 0.0, -1.0, 1.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(1.0, 0.0, -1.0, 1.0),
        ],
    ))
    .into();
    let (surface0, surface1) = parabola_surfaces();
    let plane0: AlternativeSurface = Plane::new(
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
    )
    .into();
    let plane1: AlternativeSurface = Plane::new(
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(1.0, 0.0, -1.0),
        Point3::new(0.0, 1.0, -1.0),
    )
    .into();

    // prepare topologies
    let v00 = Vertex::new(Point3::new(1.0, 0.0, 1.0));
    let v01 = Vertex::new(Point3::new(-1.0, 0.0, 1.0));
    let v10 = Vertex::new(Point3::new(1.0, 0.0, -1.0));
    let v11 = Vertex::new(Point3::new(-1.0, 0.0, -1.0));
    let wire0: Wire<_, _> = vec![Edge::new(&v00, &v01, arc00), Edge::new(&v01, &v00, arc01)].into();
    let wire1: Wire<_, _> = vec![Edge::new(&v10, &v11, arc10), Edge::new(&v11, &v10, arc11)].into();
    let shell0: Shell<_, _, _> = vec![
        Face::new(vec![wire0.clone()], plane0),
        Face::new(vec![wire0], surface0).inverse(),
    ]
    .into();
    assert_eq!(shell0.shell_condition(), ShellCondition::Closed);
    let shell1: Shell<_, _, _> = vec![
        Face::new(vec![wire1.clone()], plane1).inverse(),
        Face::new(vec![wire1], surface1),
    ]
    .into();
    assert_eq!(shell1.shell_condition(), ShellCondition::Closed);
    let poly_shell0 = shell0.triangulation(TOL);
    let poly_shell1 = shell1.triangulation(TOL);

    let loops_store::LoopsStoreQuadruple {
        geom_loops_store0: loops_store0,
        geom_loops_store1: loops_store1,
        ..
    } = loops_store::create_loops_stores(&shell0, &poly_shell0, &shell1, &poly_shell1).unwrap();
    let mut cls0 = divide_face::divide_faces(&shell0, &loops_store0, TOL).unwrap();
    cls0.integrate_by_component();
    let mut cls1 = divide_face::divide_faces(&shell1, &loops_store1, TOL).unwrap();
    cls1.integrate_by_component();

    let [mut and, mut or, _] = cls0.and_or_unknown();
    let [and1, or1, _] = cls1.and_or_unknown();
    and.extend(and1);
    or.extend(or1);

    assert_eq!(and.len(), 2);
    assert_eq!(or.len(), 4);
    assert_eq!(and.shell_condition(), ShellCondition::Closed);
    assert_eq!(or.shell_condition(), ShellCondition::Closed);
}

/// `integrate_by_component` must not panic when an unknown component has
/// empty boundary wires. When two unknown faces share the same edges but
/// opposite orientations, they form a closed sub-shell whose
/// `extract_boundaries` returns empty. The old code panics on
/// `boundary[0][0]`; the new code must skip such components gracefully.
#[test]
fn integrate_by_component_empty_boundary_no_panic() {
    use super::super::loops_store::ShapesOpStatus;

    let v0 = Vertex::new(Point3::new(0.0, 0.0, 0.0));
    let v1 = Vertex::new(Point3::new(1.0, 0.0, 0.0));

    let make_line = |a: &Vertex<Point3>, b: &Vertex<Point3>| {
        let pa = a.point();
        let pb = b.point();
        NurbsCurve::new(BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![
                Vector4::new(pa.x, pa.y, pa.z, 1.0),
                Vector4::new(pb.x, pb.y, pb.z, 1.0),
            ],
        ))
    };

    let e01: Edge<_, _> = Edge::new(&v0, &v1, make_line(&v0, &v1));
    let e10: Edge<_, _> = Edge::new(&v1, &v0, make_line(&v1, &v0));

    let plane_up: AlternativeSurface = Plane::new(
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    )
    .into();
    let plane_dn: AlternativeSurface = Plane::new(
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(1.0, 0.0, -1.0),
        Point3::new(0.0, 1.0, -1.0),
    )
    .into();

    // Two faces that share the same two edges, forming a closed component.
    let wire_a: Wire<_, _> = vec![e01.clone(), e10.clone()].into();
    let wire_b: Wire<_, _> = vec![e01, e10].into();
    let face_a = Face::new(vec![wire_a], plane_up.clone());
    let face_b = Face::new(vec![wire_b], plane_dn);

    // We also need an And face with boundary edges so that the `and_boundary`
    // is non-empty, triggering the code path that accesses `boundary[0][0]`.
    let v2 = Vertex::new(Point3::new(2.0, 0.0, 0.0));
    let v3 = Vertex::new(Point3::new(3.0, 0.0, 0.0));
    let e23: Edge<_, _> = Edge::new(&v2, &v3, make_line(&v2, &v3));
    let e32: Edge<_, _> = Edge::new(&v3, &v2, make_line(&v3, &v2));
    let and_wire: Wire<_, _> = vec![e23, e32].into();
    let and_face = Face::new(vec![and_wire], plane_up.clone());

    let mut cls = FacesClassification::default();
    cls.push(and_face, ShapesOpStatus::And);
    let id_a = face_a.id();
    let id_b = face_b.id();
    cls.push(face_a, ShapesOpStatus::Unknown);
    cls.push(face_b, ShapesOpStatus::Unknown);

    // This must NOT panic. The old code panics because the unknown component
    // has empty boundaries but the and boundary is non-empty, causing
    // `boundary[0][0].id()` to be evaluated.
    cls.integrate_by_component();

    // Both unknown faces should remain Unknown since their component has
    // no boundary edges to match against.
    let [_and, _or, unknown] = cls.and_or_unknown();
    assert_eq!(unknown.len(), 2);
    let unknown_ids: Vec<_> = unknown.iter().map(|f| f.id()).collect();
    assert!(unknown_ids.contains(&id_a));
    assert!(unknown_ids.contains(&id_b));
}

/// `integrate_by_component` uses majority-edge scoring: when a component
/// shares edges with both the And and Or boundaries, it picks the side
/// with the higher count.
#[test]
fn integrate_by_component_majority_edge_scoring() {
    use super::super::loops_store::ShapesOpStatus;

    // Build a triangular wire from 3 vertices and 3 edges.
    let v0 = Vertex::new(Point3::new(0.0, 0.0, 0.0));
    let v1 = Vertex::new(Point3::new(1.0, 0.0, 0.0));
    let v2 = Vertex::new(Point3::new(0.5, 1.0, 0.0));

    let make_edge = |a: &Vertex<Point3>, b: &Vertex<Point3>| {
        let pa = a.point();
        let pb = b.point();
        let curve = NurbsCurve::new(BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![
                Vector4::new(pa.x, pa.y, pa.z, 1.0),
                Vector4::new(pb.x, pb.y, pb.z, 1.0),
            ],
        ));
        Edge::new(a, b, curve)
    };

    let e01 = make_edge(&v0, &v1);
    let e12 = make_edge(&v1, &v2);
    let e20 = make_edge(&v2, &v0);

    let plane0: AlternativeSurface = Plane::new(
        Point3::origin(),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    )
    .into();
    let plane1: AlternativeSurface = Plane::new(
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
    )
    .into();

    // And-face uses edge e01.
    let and_wire: Wire<_, _> = vec![e01.clone(), make_edge(&v1, &v0)].into();
    let and_face = Face::new(vec![and_wire], plane0.clone());

    // Or-face uses edge e12.
    let or_wire: Wire<_, _> = vec![e12.clone(), make_edge(&v2, &v1)].into();
    let or_face = Face::new(vec![or_wire], plane1.clone());

    // Unknown-face uses edges e01, e12, e20 (shares 1 edge with And, 1 with Or).
    // Tie: and_matches == or_matches == 1. Should remain Unknown because
    // the rule is: and_matches > 0 && and_matches >= or_matches -> And.
    // With a tie of 1:1, and_matches >= or_matches is true -> And.
    let unknown_wire: Wire<_, _> = vec![e01, e12, e20].into();
    let unknown_face = Face::new(vec![unknown_wire], plane0);

    let mut cls = FacesClassification::default();
    cls.push(and_face, ShapesOpStatus::And);
    cls.push(or_face, ShapesOpStatus::Or);
    let unknown_id = unknown_face.id();
    cls.push(unknown_face, ShapesOpStatus::Unknown);

    cls.integrate_by_component();

    // The unknown face shares 1 edge with And and 1 with Or.
    // With the majority rule (and_matches >= or_matches), it should become And.
    let [and, _or, unknown] = cls.and_or_unknown();
    assert!(unknown.is_empty(), "Face should have been classified.");
    assert!(
        and.iter().any(|f| f.id() == unknown_id),
        "Tied face should be classified as And (and_matches >= or_matches)."
    );
}
