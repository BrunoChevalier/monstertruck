#![cfg(not(target_arch = "wasm32"))]

use monstertruck_core::{bounding_box::*, cgmath64::*, prop_assert_near, tolerance::*};
use monstertruck_gpu::*;
use proptest::prelude::*;
use std::f64::consts::PI;

proptest! {
    #[test]
    fn parallel_view_fitting(
        rot_axis in (-1.0f64..=1.0, 0.0..2.0 * PI),
        rot_angle in 0.0..2.0 * PI,
        aspect in 0.5f64..=10.0f64,
        near_clip in 0.01f64..=100.0,
        points in prop::array::uniform32(prop::array::uniform3(-100.0f64..=100.0f64)),
    ) {
        let z = rot_axis.0;
        let r = (1.0 - z * z).sqrt();
        let rot_axis = Vector3::new(r * f64::cos(rot_axis.1), r * f64::sin(rot_axis.1), z);
        let rot = Matrix3::from_axis_angle(rot_axis, Rad(rot_angle));
        let points: [Point3; 32] = points.map(|p| p.into());

        let same_plane = points.windows(4).all(|p| {
            Matrix3::from_cols(p[1] - p[0], p[2] - p[0], p[3] - p[0])
                .determinant()
                .so_small()
        });
        prop_assume!(!same_plane);

        let camera = Camera::parallel_view_fitting(rot, aspect, near_clip, &points);
        let nvolume = points
            .iter()
            .map(|p| camera.projection(aspect).transform_point(*p))
            .collect::<BoundingBox<Point3>>();

        let (min, max) = (nvolume.min(), nvolume.max());
        prop_assert_near!(min.x.min(min.y), -1.0);
        prop_assert_near!(max.x.max(max.y), 1.0);
        prop_assert_near!(min.z, 0.0);
        prop_assert_near!(max.z, 1.0);
    }

    #[test]
    fn perspective_view_fitting(
        rot_axis in (-1.0f64..=1.0, 0.0..2.0 * PI),
        rot_angle in 0.0..2.0 * PI,
        aspect in 0.5f64..=10.0f64,
        fov in 0.1f64..=3.05,
        points in prop::array::uniform32(prop::array::uniform3(-100.0f64..=100.0f64)),
    ) {
        let z = rot_axis.0;
        let r = (1.0 - z * z).sqrt();
        let rot_axis = Vector3::new(r * f64::cos(rot_axis.1), r * f64::sin(rot_axis.1), z);
        let rot = Matrix3::from_axis_angle(rot_axis, Rad(rot_angle));
        let points: [Point3; 32] = points.map(|p| p.into());

        let same_plane = points.windows(4).all(|p| {
            Matrix3::from_cols(p[1] - p[0], p[2] - p[0], p[3] - p[0])
                .determinant()
                .so_small()
        });
        prop_assume!(!same_plane);

        let camera = Camera::perspective_view_fitting(rot, aspect, Rad(fov), &points);
        prop_assume!(camera.near_clip > TOLERANCE);

        let nvolume = points
            .iter()
            .map(|p| camera.projection(aspect).transform_point(*p))
            .collect::<BoundingBox<Point3>>();

        let (min, max) = (nvolume.min(), nvolume.max());
        prop_assert_near!(min.x.min(min.y), -1.0);
        prop_assert_near!(max.x.max(max.y), 1.0);
        prop_assert_near!(min.z, 0.0);
        prop_assert_near!(max.z, 1.0);
    }
}

#[test]
fn parallel_view_fitting_all_zeros() {
    let rot = Matrix3::identity();
    let aspect = 1.0;
    let near_clip = 0.1;
    let points: Vec<Point3> = (0..32).map(|_| Point3::origin()).collect();
    let camera = Camera::parallel_view_fitting(rot, aspect, near_clip, &points);

    // Camera must have finite, non-NaN values.
    assert!(camera.near_clip.is_finite(), "near_clip must be finite");
    assert!(camera.far_clip.is_finite(), "far_clip must be finite");
    assert!(
        camera.near_clip < camera.far_clip,
        "near_clip ({}) must be < far_clip ({})",
        camera.near_clip,
        camera.far_clip
    );

    // screen_size must be strictly positive for a valid ortho projection.
    match camera.method {
        ProjectionMethod::Parallel { screen_size } => {
            assert!(
                screen_size > 0.0,
                "screen_size must be positive, got {}",
                screen_size
            );
        }
        _ => panic!("Expected Parallel projection method"),
    }
}

#[test]
fn parallel_view_fitting_all_identical() {
    let rot = Matrix3::identity();
    let aspect = 16.0 / 9.0;
    let near_clip = 0.5;
    let pt = Point3::new(5.0, 3.0, -2.0);
    let points: Vec<Point3> = (0..32).map(|_| pt).collect();
    let camera = Camera::parallel_view_fitting(rot, aspect, near_clip, &points);

    assert!(camera.near_clip.is_finite());
    assert!(camera.far_clip.is_finite());
    assert!(camera.near_clip < camera.far_clip);
    match camera.method {
        ProjectionMethod::Parallel { screen_size } => {
            assert!(
                screen_size > 0.0,
                "screen_size must be positive, got {}",
                screen_size
            );
        }
        _ => panic!("Expected Parallel projection method"),
    }
}

#[test]
fn perspective_view_fitting_all_zeros() {
    let rot = Matrix3::identity();
    let aspect = 1.0;
    let fov = Rad(PI / 4.0);
    let points: Vec<Point3> = (0..32).map(|_| Point3::origin()).collect();
    let camera = Camera::perspective_view_fitting(rot, aspect, fov, &points);

    assert!(camera.near_clip.is_finite(), "near_clip must be finite");
    assert!(camera.far_clip.is_finite(), "far_clip must be finite");
    assert!(
        camera.near_clip > 0.0,
        "near_clip ({}) must be > 0",
        camera.near_clip
    );
    assert!(
        camera.near_clip < camera.far_clip,
        "near_clip ({}) must be < far_clip ({})",
        camera.near_clip,
        camera.far_clip
    );
}

#[test]
fn perspective_view_fitting_all_identical() {
    let rot = Matrix3::identity();
    let aspect = 16.0 / 9.0;
    let fov = Rad(PI / 4.0);
    let pt = Point3::new(5.0, 3.0, -2.0);
    let points: Vec<Point3> = (0..32).map(|_| pt).collect();
    let camera = Camera::perspective_view_fitting(rot, aspect, fov, &points);

    assert!(camera.near_clip.is_finite(), "near_clip must be finite");
    assert!(camera.far_clip.is_finite(), "far_clip must be finite");
    assert!(
        camera.near_clip > 0.0,
        "near_clip ({}) must be > 0",
        camera.near_clip
    );
    assert!(
        camera.near_clip < camera.far_clip,
        "near_clip ({}) must be < far_clip ({})",
        camera.near_clip,
        camera.far_clip
    );
}
