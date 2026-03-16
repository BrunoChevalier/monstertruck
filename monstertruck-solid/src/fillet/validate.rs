// Topology validation for fillet operations.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euler_poincare_check_exists_and_returns_bool() {
        // Verify the function exists and accepts a shell reference.
        // Build a minimal closed shell (cube) and check it.
        use monstertruck_geometry::prelude::*;
        use super::super::types::*;

        let p = [
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(0.0, 1.0, 1.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let v = Vertex::news(p);
        let line = |i: usize, j: usize| {
            let bsp = BsplineCurve::new(KnotVector::bezier_knot(1), vec![p[i], p[j]]);
            Edge::new(&v[i], &v[j], NurbsCurve::from(bsp).into())
        };
        let edge = [
            line(0, 1),
            line(1, 2),
            line(2, 3),
            line(3, 0),
            line(0, 4),
            line(1, 5),
            line(2, 6),
            line(3, 7),
            line(4, 5),
            line(5, 6),
            line(6, 7),
            line(7, 4),
        ];
        let plane = |i: usize, j: usize, k: usize, l: usize| {
            let control_points = vec![vec![p[i], p[l]], vec![p[j], p[k]]];
            let knot_vec = KnotVector::bezier_knot(1);
            let bsp = BsplineSurface::new((knot_vec.clone(), knot_vec), control_points);
            let wire: Wire = [i, j, k, l]
                .into_iter()
                .circular_tuple_windows()
                .map(|(a, b)| {
                    edge.iter()
                        .find_map(|e| {
                            if e.front() == &v[a] && e.back() == &v[b] {
                                Some(e.clone())
                            } else if e.back() == &v[a] && e.front() == &v[b] {
                                Some(e.inverse())
                            } else {
                                None
                            }
                        })
                        .unwrap()
                })
                .collect();
            Face::new(vec![wire], bsp.into())
        };
        let shell: Shell = [
            plane(0, 1, 2, 3),
            plane(1, 0, 4, 5),
            plane(2, 1, 5, 6),
            plane(3, 2, 6, 7),
            plane(0, 3, 7, 4),
            plane(5, 4, 7, 6),
        ]
        .into();

        assert!(euler_poincare_check(&shell));
        assert!(is_oriented_check(&shell));
    }
}
