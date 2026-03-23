use crate::*;

// Since cgmath assumes a normalized view volume in OpenGL,
// this conversion is necessary to handle it with wgpu.
// cf https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera
// Constructed lazily because Matrix4::new is not const.
#[rustfmt::skip]
fn opengl_to_wgpu_matrix() -> Matrix4 {
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    )
}

impl Ray {
    /// Returns the origin of the ray
    #[inline(always)]
    pub const fn origin(&self) -> Point3 {
        self.origin
    }
    /// Returns the (normalized) direction of the ray
    #[inline(always)]
    pub const fn direction(&self) -> Vector3 {
        self.direction
    }
}

impl ProjectionMethod {
    /// Returns `ProjectionMethod::Perspective { fov }`.
    #[inline(always)]
    pub const fn perspective(fov: Rad<f64>) -> Self {
        Self::Perspective { fov }
    }
    /// Returns `ProjectionMethod::Parallel { screen_size }`.
    #[inline(always)]
    pub const fn parallel(screen_size: f64) -> Self {
        Self::Parallel { screen_size }
    }
}

impl Camera {
    /// Returns the position of camera,
    /// the forth column of the camera matrix.
    ///
    /// # Examples
    /// ```
    /// use monstertruck_gpu::*;
    /// use monstertruck_core::cgmath64::*;
    /// let mut camera = Camera::default();
    /// camera.matrix = Matrix4::from_translation(Vector3::new(1.0, 2.0, 3.0));
    /// assert_eq!(camera.position(), Point3::new(1.0, 2.0, 3.0));
    /// ```
    #[inline(always)]
    pub fn position(&self) -> Point3 {
        let col = self.matrix[3];
        Point3::new(col[0], col[1], col[2])
    }

    /// Returns the eye direction of camera.
    /// the inverse of the z-axis of the camera matrix.
    ///
    /// # Examples
    /// ```
    /// use std::f64::consts::PI;
    /// use monstertruck_gpu::*;
    /// use monstertruck_core::{cgmath64::*, tolerance::Tolerance};
    /// let mut camera = Camera::default();
    /// camera.matrix = Matrix4::from_axis_angle(
    ///     Vector3::new(1.0, 1.0, 1.0).normalize(),
    ///     Rad(2.0 * PI / 3.0),
    /// );
    /// assert!(camera.eye_direction().near(&-Vector3::unit_x()));
    /// ```
    #[inline(always)]
    pub fn eye_direction(&self) -> Vector3 {
        let col = self.matrix[2];
        -Vector3::new(col[0], col[1], col[2])
    }

    /// Returns the direction of the head vector, the y-axis of the camera matrix.
    /// # Examples
    /// ```
    /// use std::f64::consts::PI;
    /// use monstertruck_gpu::*;
    /// use monstertruck_core::{cgmath64::*, tolerance::Tolerance};
    /// let mut camera = Camera::default();
    /// camera.matrix = Matrix4::from_axis_angle(
    ///     Vector3::new(1.0, 1.0, 1.0).normalize(),
    ///     Rad(2.0 * PI / 3.0),
    /// );
    /// assert!(camera.head_direction().near(&Vector3::unit_z()));
    /// ```
    #[inline(always)]
    pub fn head_direction(&self) -> Vector3 {
        let col = self.matrix[1];
        Vector3::new(col[0], col[1], col[2])
    }

    /// Returns the projection matrix into the normalized view volume.
    /// # Arguments
    /// `aspect`: the aspect ratio, x-resolution / y-resolution.
    /// # Examples
    /// ```
    /// // perspective camera
    /// use std::f64::consts::PI;
    /// use monstertruck_core::{assert_near, cgmath64::*, tolerance::*};
    /// use monstertruck_gpu::*;
    ///
    /// let fov = PI / 4.0;
    /// let aspect = 1.2;
    /// let matrix = Matrix4::look_at_rh(
    ///     Point3::new(1.0, 1.0, 1.0),
    ///     Point3::origin(),
    ///     Vector3::new(0.0, 1.0, 0.0),
    /// );
    /// let camera = Camera {
    ///     matrix: matrix.invert().unwrap(),
    ///     method: ProjectionMethod::perspective(Rad(fov)),
    ///     near_clip: 0.1,
    ///     far_clip: 10.0,
    /// };
    ///
    /// // calculation by the ray-tracing
    /// let pt = Point3::new(-1.5, -1.4, -2.5);
    /// let vec = pt - camera.position();
    /// let far = 1.0 / (fov / 2.0).tan();
    /// let dir = camera.eye_direction();
    /// let y_axis = camera.head_direction();
    /// let x_axis = dir.cross(y_axis);
    /// let proj_length = dir.dot(vec);
    /// let h = (vec - proj_length * dir) * far / proj_length;
    /// let u = h.dot(x_axis) / aspect;
    /// let v = h.dot(y_axis);
    ///
    /// // check the answer
    /// let uv = camera.projection(aspect).transform_point(pt);
    /// assert_near!(u, uv[0]);
    /// assert_near!(v, uv[1]);
    /// ```
    /// ```
    /// // parallel camera
    /// use monstertruck_core::{assert_near, cgmath64::*, tolerance::*};
    /// use monstertruck_gpu::*;
    ///
    /// let size = 3.0;
    /// let aspect = 1.2;
    /// let matrix = Matrix4::look_at_rh(
    ///     Point3::new(1.0, 1.0, 1.0),
    ///     Point3::origin(),
    ///     Vector3::new(0.0, 1.0, 0.0),
    /// );
    /// let camera = Camera {
    ///     matrix: matrix.invert().unwrap(),
    ///     method: ProjectionMethod::parallel(size),
    ///     near_clip: 0.1,
    ///     far_clip: 10.0,
    /// };
    ///
    /// // calculation by the ray-tracing
    /// let pt = Point3::new(-1.5, -1.4, -2.5);
    /// let vec = pt - camera.position();
    /// let dir = camera.eye_direction();
    /// let y_axis = camera.head_direction();
    /// let x_axis = dir.cross(y_axis);
    /// let h = vec - vec.dot(dir) * dir;
    /// let u = h.dot(x_axis) / (size / 2.0) / aspect;
    /// let v = h.dot(y_axis) / (size / 2.0);
    ///
    /// // check the answer
    /// let uv = camera.projection(aspect).transform_point(pt);
    /// assert_near!(u, uv[0]);
    /// assert_near!(v, uv[1]);
    /// ```
    #[inline(always)]
    pub fn projection(&self, aspect: f64) -> Matrix4 {
        let (near, far) = (self.near_clip, self.far_clip);
        let normal_projection = match self.method {
            ProjectionMethod::Perspective { fov } => perspective(fov, aspect, near, far),
            ProjectionMethod::Parallel { screen_size } => {
                let y = screen_size / 2.0;
                let x = y * aspect;
                ortho(-x, x, -y, y, near, far)
            }
        };
        // SAFETY: camera matrix is a rigid-body transform (Euclidean group), always invertible.
        opengl_to_wgpu_matrix() * normal_projection * self.matrix.invert().unwrap()
    }

    #[inline(always)]
    fn camera_info(&self, aspect: f64) -> CameraInfo {
        CameraInfo {
            // SAFETY: f64-to-f32 cast is always valid for finite values in a camera matrix.
            camera_matrix: self.matrix.cast().unwrap().into(),
            // SAFETY: f64-to-f32 cast is always valid for finite values in a projection matrix.
            camera_projection: self.projection(aspect).cast().unwrap().into(),
        }
    }

    /// Creates a `UNIFORM` buffer of camera.
    ///
    /// The bind group provides [`Scene`] holds this uniform buffer.
    ///
    /// # Shader Example
    /// ```glsl
    /// layout(set = 0, binding = 0) uniform Camera {
    ///     mat4 camera_matrix;     // the camera matrix
    ///     mat4 camera_projection; // the projection into the normalized view volume
    /// };
    /// ```
    #[inline(always)]
    pub fn buffer(&self, aspect: f64, device: &Device) -> BufferHandler {
        BufferHandler::from_slice(&[self.camera_info(aspect)], device, BufferUsages::UNIFORM)
    }

    /// Returns the ray from camera with aspect-ratio = 1.0.
    ///
    /// # Examples
    /// ```
    /// // Perspective case
    /// use std::f64::consts::PI;
    /// use monstertruck_core::{assert_near, cgmath64::*, tolerance::Tolerance};
    /// use monstertruck_gpu::*;
    ///
    ///
    /// let matrix = Matrix4::look_at_rh(
    ///     Point3::new(1.0, 1.0, 1.0),
    ///     Point3::origin(),
    ///     Vector3::new(0.0, 1.0, 0.0),
    /// );
    /// let camera = Camera {
    ///     // depends on the difference of the style with cgmath,
    ///     // the matrix must be inverted
    ///     matrix: matrix.invert().unwrap(),
    ///     method: ProjectionMethod::perspective(Rad(PI / 4.0)),
    ///     near_clip: 0.1,
    ///     far_clip: 1.0,
    /// };
    ///
    /// // take a point in the 3D space
    /// let point = Point3::new(0.1, 0.15, 0.0);
    /// // project to the normalized view volume
    /// let uvz = camera.projection(1.0).transform_point(point);
    /// // coordinate on the screen
    /// let uv = Point2::new(uvz.x, uvz.y);
    ///
    /// let ray = camera.ray(uv);
    /// // the origin of the ray is camera position
    /// assert_near!(ray.origin(), camera.position());
    /// // the direction of the ray is the normalized vector of point - camera.position().
    /// assert_near!(ray.direction(), (point - camera.position()).normalize());
    /// ```
    /// ```
    /// // Parallel case
    /// use monstertruck_core::{assert_near, cgmath64::*, tolerance::*};
    /// use monstertruck_gpu::*;
    ///
    /// let matrix = Matrix4::look_at_rh(
    ///     Point3::new(1.0, 1.0, 1.0),
    ///     Point3::origin(),
    ///     Vector3::new(0.0, 1.0, 0.0),
    /// );
    /// let camera = Camera {
    ///     matrix: matrix.invert().unwrap(),
    ///     method: ProjectionMethod::parallel(3.0),
    ///     near_clip: 0.1,
    ///     far_clip: 10.0,
    /// };
    ///
    /// // take a point in the 3D space
    /// let point = Point3::new(0.1, 0.15, 0.0);
    /// // the projection of the point to the screen
    /// let projed = point
    ///     - camera.eye_direction() * camera.eye_direction().dot(point - camera.position());
    /// // project to the normalized view volume
    /// let uvz = camera.projection(1.0).transform_point(point);
    /// // coordinate on the screen
    /// let uv = Point2::new(uvz.x, uvz.y);
    ///
    /// let ray = camera.ray(uv);
    /// // the origin of the ray is the projection of the point.
    /// assert_near!(ray.origin(), projed);
    /// // the direction of the ray is eye direction.
    /// assert_near!(ray.direction(), camera.eye_direction());
    /// ```
    pub fn ray(&self, coord: Point2) -> Ray {
        match self.method {
            ProjectionMethod::Perspective { .. } => {
                let mat = self
                    .projection(1.0)
                    .invert()
                    .expect("non-invertible projection");
                let x = mat.transform_point(Point3::new(coord[0], coord[1], 0.5));
                let y = mat.transform_point(Point3::new(coord[0], coord[1], 1.0));
                Ray {
                    origin: self.position(),
                    direction: (y - x).normalize(),
                }
            }
            ProjectionMethod::Parallel { screen_size } => {
                let a = screen_size / 2.0;
                let col0 = self.matrix[0];
                let axis_x = Vector3::new(col0[0], col0[1], col0[2]) * a;
                let col1 = self.matrix[1];
                let axis_y = Vector3::new(col1[0], col1[1], col1[2]) * a;
                Ray {
                    origin: self.position() + coord[0] * axis_x + coord[1] * axis_y,
                    direction: self.eye_direction(),
                }
            }
        }
    }

    /// Creates a parallel projection camera that fits the point cloud perfectly into the view volume.
    /// # Examples
    /// ```
    /// use monstertruck_core::{assert_near, cgmath64::*, tolerance::*};
    /// use monstertruck_gpu::*;
    ///
    /// let points: &[Point3] = &[
    ///     (0.0, 0.0, 0.0).into(),
    ///     (1.0, 0.0, 0.0).into(),
    ///     (0.0, 1.0, 0.0).into(),
    ///     (0.0, 0.0, 1.0).into(),
    ///     (0.0, 1.0, 1.0).into(),
    ///     (1.0, 0.0, 1.0).into(),
    ///     (1.0, 1.0, 0.0).into(),
    ///     (1.0, 1.0, 1.0).into(),
    /// ];
    /// let direction = Matrix3::from_cols(
    ///     Vector3::new(1.0, 0.0, -1.0).normalize(),
    ///     Vector3::new(-1.0, 2.0, -1.0).normalize(),
    ///     Vector3::new(1.0, 1.0, 1.0).normalize(),
    /// );
    /// let aspect = 16.0 / 9.0;
    /// let near_clip = 0.1;
    ///
    /// let camera = Camera::parallel_view_fitting(
    ///     direction,
    ///     aspect,
    ///     near_clip,
    ///     points,
    /// );
    ///
    /// assert_eq!(camera.near_clip, near_clip);
    /// assert_near!(
    ///     camera.far_clip,
    ///     near_clip + f64::sqrt(3.0),
    /// );
    ///
    /// let projection = camera.projection(aspect);
    /// let bbox = points.iter().map(|p| {
    ///     let n = projection.transform_point(*p);
    ///     assert!(-1.0 <= n.x && n.x <= 1.0);
    ///     assert!(-1.0 <= n.y && n.y <= 1.0);
    ///     assert!(0.0 <= n.z && n.z <= 1.0);
    /// });
    /// ```
    pub fn parallel_view_fitting(
        direction: Matrix3,
        aspect: f64,
        near_clip: f64,
        points: &[Point3],
    ) -> Self {
        // SAFETY: direction is an orthonormal basis (rotation matrix), always invertible.
        let inv_dir = direction.invert().unwrap();
        let bbox = points
            .iter()
            .map(|p| inv_dir.transform_point(*p))
            .collect::<BoundingBox<_>>();
        let (center, diag) = (bbox.center(), bbox.diagonal());
        // Guard against degenerate point clouds where bbox has zero span.
        // Use a small fraction of the cloud's distance from origin as epsilon,
        // falling back to 1.0 when the cloud is at the origin.
        let scale = center.to_vec().magnitude().max(1.0);
        let eps = scale * 1e-10;
        let dx = diag[0].max(eps);
        let dy = diag[1].max(eps);
        let dz = diag[2].max(eps);
        let screen_size = f64::max(dx / aspect, dy);
        let position = Vector3::new(center[0], center[1], center[2] + dz / 2.0 + near_clip);
        Self {
            matrix: Matrix4::from(direction) * Matrix4::from_translation(position),
            method: ProjectionMethod::Parallel { screen_size },
            near_clip,
            far_clip: near_clip + dz,
        }
    }

    /// Creates a perspective projection camera that fits the point cloud perfectly into the view volume.
    /// # Examples
    /// ```
    /// use monstertruck_core::{assert_near, cgmath64::*, tolerance::*};
    /// use monstertruck_gpu::*;
    ///
    /// let points: &[Point3] = &[
    ///     (0.0, 0.0, 0.0).into(),
    ///     (1.0, 0.0, 0.0).into(),
    ///     (0.0, 1.0, 0.0).into(),
    ///     (0.0, 0.0, 1.0).into(),
    ///     (0.0, 1.0, 1.0).into(),
    ///     (1.0, 0.0, 1.0).into(),
    ///     (1.0, 1.0, 0.0).into(),
    ///     (1.0, 1.0, 1.0).into(),
    /// ];
    /// let direction = Matrix3::from_cols(
    ///     Vector3::new(1.0, 0.0, -1.0).normalize(),
    ///     Vector3::new(-1.0, 2.0, -1.0).normalize(),
    ///     Vector3::new(1.0, 1.0, 1.0).normalize(),
    /// );
    /// let aspect = 16.0 / 9.0;
    /// let fov = Rad(std::f64::consts::PI / 4.0);
    ///
    /// let camera = Camera::perspective_view_fitting(
    ///     direction,
    ///     aspect,
    ///     fov,
    ///     points,
    /// );
    ///
    /// assert_near!(
    ///     camera.near_clip,
    ///     camera.eye_direction().dot(points[7] - camera.position()),
    /// );
    /// assert_near!(
    ///     camera.far_clip,
    ///     camera.eye_direction().dot(points[0] - camera.position()),
    /// );
    ///
    /// let projection = camera.projection(aspect);
    /// let bbox = points.iter().map(|p| {
    ///     let n = projection.transform_point(*p);
    ///     assert!(-1.0 <= n.x && n.x <= 1.0);
    ///     assert!(-1.0 <= n.y && n.y <= 1.0);
    ///     assert!(0.0 <= n.z && n.z <= 1.0);
    /// });
    /// ```
    /// # Remarks
    /// Depending on the arrangement of the point cloud, near_clip may take a negative value.
    pub fn perspective_view_fitting(
        direction: Matrix3,
        aspect: f64,
        fov: Rad<f64>,
        points: &[Point3],
    ) -> Camera {
        // SAFETY: direction is an orthonormal basis (rotation matrix), always invertible.
        let inv_dir = direction.invert().unwrap();
        let tan = (fov / 2.0).tan();

        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;
        let mut z_min = f64::INFINITY;
        let mut z_max = f64::NEG_INFINITY;

        for p in points {
            let p = inv_dir.transform_point(*p);
            x_min = f64::min(x_min, p[0] - tan * p[2] * aspect);
            x_max = f64::max(x_max, p[0] + tan * p[2] * aspect);
            y_min = f64::min(y_min, p[1] - tan * p[2]);
            y_max = f64::max(y_max, p[1] + tan * p[2]);
            z_min = f64::min(z_min, p[2]);
            z_max = f64::max(z_max, p[2]);
        }

        let z_x = (x_max - x_min) / (2.0 * tan) / aspect;
        let z_y = (y_max - y_min) / (2.0 * tan);
        let mut cam_z = z_x.max(z_y);

        // Guard against degenerate point clouds where all points project to
        // the same x/y/z in camera space. Ensure the camera is placed behind
        // the cloud with a minimum separation so near_clip > 0 and
        // near_clip < far_clip.
        let scale = z_max.abs().max(z_min.abs()).max(1.0);
        let min_separation = scale * 1e-10;
        if cam_z <= z_max + min_separation {
            cam_z = z_max + min_separation;
        }
        let near_clip = cam_z - z_max;
        let far_clip_raw = cam_z - z_min;
        // Ensure far_clip > near_clip even when z_span == 0.
        let far_clip = if far_clip_raw <= near_clip {
            near_clip + min_separation
        } else {
            far_clip_raw
        };

        let position = Vector3::new((x_min + x_max) / 2.0, (y_min + y_max) / 2.0, cam_z);
        Camera {
            matrix: Matrix4::from(direction) * Matrix4::from_translation(position),
            method: ProjectionMethod::Perspective { fov },
            near_clip,
            far_clip,
        }
    }
}

impl Default for Camera {
    #[inline(always)]
    fn default() -> Self {
        Self {
            matrix: Matrix4::identity(),
            method: ProjectionMethod::perspective(Rad(std::f64::consts::PI / 4.0)),
            near_clip: 0.1,
            far_clip: 10.0,
        }
    }
}
