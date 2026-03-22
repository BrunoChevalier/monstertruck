use super::*;
use monstertruck_traits::ParametricCurve as PcurveTrait;
use std::f64::consts::PI;

impl Revolution {
    fn new(origin: Point3, axis: Vector3) -> Self {
        Self {
            origin,
            axis: axis.normalize(),
        }
    }
    #[inline(always)]
    fn rotation_matrix(self, v: f64) -> Matrix3 {
        Matrix3::from_axis_angle(self.axis, Rad(v))
    }
    #[inline(always)]
    fn invert(&mut self) {
        self.axis *= -1.0;
    }
    #[inline(always)]
    fn inverse(mut self) -> Self {
        self.axis *= -1.0;
        self
    }
    #[inline(always)]
    fn contains(self, p: Point3) -> bool {
        (p - self.origin).cross(&self.axis).so_small()
    }
    #[inline(always)]
    fn proj_point(&self, p: Point3) -> Point2 {
        let r = p - self.origin;
        let z = r.dot(self.axis);
        let h = r - z * self.axis;
        Point2::new(z, h.magnitude())
    }
    #[inline(always)]
    fn proj_vector(&self, p: Point3, v: Vector3) -> Vector2 {
        let r = self.proj_point(p)[1];
        let vz = v.dot(self.axis);
        let vxy = v - vz * self.axis;
        let vq = (p - self.origin).dot(vxy) / r;
        Vector2::new(vz, vq)
    }
    #[inline(always)]
    fn proj_vector2(&self, p: Point3, v: Vector3, v2: Vector3) -> Vector2 {
        let r = self.proj_point(p)[1];
        let v2z = v2.dot(self.axis);
        let v2xy = v2 - v2z * self.axis;
        let vz = v.dot(self.axis);
        let vxy = v - vz * self.axis;
        let a = (vxy.dot(vxy) + (p - self.origin).dot(v2xy)) / r;
        let b = f64::powi((p - self.origin).dot(vxy), 2) / (r * r * r);
        Vector2::new(v2z, a - b)
    }
    #[inline(always)]
    fn proj_angle(&self, p: Point3, q: Point3) -> f64 {
        let (p, q) = (p - self.origin, q - self.origin);
        let hp = (p - p.dot(self.axis) * self.axis).normalize();
        let hq = (q - q.dot(self.axis) * self.axis).normalize();
        let t = f64::acos(f64::clamp(hp.dot(hq), -1.0, 1.0));
        match hp.cross(&hq).dot(self.axis) < 0.0 {
            false => t,
            true => 2.0 * PI - t,
        }
    }
}

impl<C: ParametricCurve3D> ParametricSurface for RevolutedCurve<C> {
    type Point = Point3;
    type Vector = Vector3;
    #[inline(always)]
    fn derivative_mn(&self, m: usize, n: usize, u: f64, v: f64) -> Vector3 {
        let center = match (m, n) {
            (0, 0) => self.origin().to_vec(),
            _ => Vector3::zero(),
        };
        let u_part = match m {
            0 => self.curve.evaluate(u) - self.origin(),
            _ => self.curve.der_n(m, u),
        };
        let v_part = from_axis_angle_derivation(n, self.axis(), Rad(v));
        v_part * u_part + center
    }
    #[inline(always)]
    fn evaluate(&self, u: f64, v: f64) -> Point3 {
        let mat = self.revolution.rotation_matrix(v);
        let (p, o) = (self.curve.evaluate(u), self.origin());
        o + mat * (p - o)
    }
    #[inline(always)]
    fn derivative_u(&self, u: f64, v: f64) -> Vector3 {
        self.revolution.rotation_matrix(v) * self.curve.der(u)
    }
    #[inline(always)]
    fn derivative_v(&self, u: f64, v: f64) -> Vector3 {
        let u_part = self.curve.evaluate(u) - self.origin();
        let v_part = from_axis_angle_derivation(1, self.axis(), Rad(v));
        v_part * u_part
    }
    #[inline(always)]
    fn derivative_uu(&self, u: f64, v: f64) -> Vector3 {
        self.revolution.rotation_matrix(v) * self.curve.der2(u)
    }
    #[inline(always)]
    fn derivative_vv(&self, u: f64, v: f64) -> Vector3 {
        let u_part = self.curve.evaluate(u) - self.origin();
        let v_part = from_axis_angle_derivation(2, self.axis(), Rad(v));
        v_part * u_part
    }
    #[inline(always)]
    fn derivative_uv(&self, u: f64, v: f64) -> Vector3 {
        let u_part = self.curve.der(u);
        let v_part = from_axis_angle_derivation(1, self.axis(), Rad(v));
        v_part * u_part
    }
    #[inline(always)]
    fn parameter_range(&self) -> (ParameterRange, ParameterRange) {
        (
            self.curve.parameter_range(),
            (Bound::Included(0.0), Bound::Excluded(2.0 * PI)),
        )
    }
    #[inline(always)]
    fn u_period(&self) -> Option<f64> {
        self.curve.period()
    }
    #[inline(always)]
    fn v_period(&self) -> Option<f64> {
        Some(2.0 * PI)
    }
}

impl<C: ParametricCurve3D + BoundedCurve> ParametricSurface3D for RevolutedCurve<C> {
    #[inline(always)]
    fn normal(&self, u: f64, v: f64) -> Vector3 {
        let (u0, u1) = self.curve.range_tuple();
        let (uder, vder) = if u.near(&u0) {
            let pt = self.curve.evaluate(u);
            let radius: Vector3 = self.axis().cross(&(pt - self.origin()));
            if radius.so_small() {
                let uder = self.curve.der(u);
                let cross: Vector3 = self.axis().cross(&uder);
                (uder, cross)
            } else {
                (self.derivative_u(u, v), self.derivative_v(u, v))
            }
        } else if u.near(&u1) {
            let pt = self.curve.evaluate(u);
            let radius: Vector3 = self.axis().cross(&(pt - self.origin()));
            if radius.so_small() {
                let uder = self.curve.der(u);
                let cross: Vector3 = uder.cross(&self.axis());
                (uder, cross)
            } else {
                (self.derivative_u(u, v), self.derivative_v(u, v))
            }
        } else {
            (self.derivative_u(u, v), self.derivative_v(u, v))
        };
        let cross: Vector3 = uder.cross(&vder);
        cross.normalize()
    }
}

impl<C: ParametricCurve3D + BoundedCurve> BoundedSurface for RevolutedCurve<C> {}

impl<C: Clone> Invertible for RevolutedCurve<C> {
    #[inline(always)]
    fn invert(&mut self) {
        self.revolution.invert()
    }
    #[inline(always)]
    fn inverse(&self) -> Self {
        RevolutedCurve {
            curve: self.curve.clone(),
            revolution: self.revolution.inverse(),
        }
    }
}

#[derive(Clone, Debug)]
struct ProjectedCurve<C> {
    curve: C,
    revolution: Revolution,
}

impl<C: ParametricCurve3D> PcurveTrait for ProjectedCurve<C> {
    type Point = Point2;
    type Vector = Vector2;
    #[inline(always)]
    fn derivative_n(&self, n: usize, t: f64) -> Self::Vector {
        match n {
            0 => self.evaluate(t).to_vec(),
            1 => self.derivative(t),
            2 => self.derivative_2(t),
            _ => unimplemented!(),
        }
    }
    #[inline(always)]
    fn evaluate(&self, t: f64) -> Self::Point {
        self.revolution.proj_point(self.curve.evaluate(t))
    }
    #[inline(always)]
    fn derivative(&self, t: f64) -> Self::Vector {
        self.revolution
            .proj_vector(self.curve.evaluate(t), self.curve.derivative(t))
    }
    #[inline(always)]
    fn derivative_2(&self, t: f64) -> Self::Vector {
        self.revolution.proj_vector2(
            self.curve.evaluate(t),
            self.curve.derivative(t),
            self.curve.derivative_2(t),
        )
    }
    #[inline(always)]
    fn parameter_range(&self) -> ParameterRange {
        self.curve.parameter_range()
    }
    #[inline(always)]
    fn period(&self) -> Option<f64> {
        self.curve.period()
    }
}

impl<C: ParametricCurve3D + BoundedCurve> BoundedCurve for ProjectedCurve<C> {}

impl<C: ParametricCurve3D + BoundedCurve> SearchParameter<D1> for ProjectedCurve<C> {
    type Point = Point2;
    fn search_parameter<H: Into<SearchParameterHint1D>>(
        &self,
        point: Self::Point,
        hint: H,
        trials: usize,
    ) -> Option<f64> {
        let hint = match hint.into() {
            SearchParameterHint1D::Parameter(t) => t,
            SearchParameterHint1D::Range(x, y) => {
                algo::curve::presearch(self, point, (x, y), PRESEARCH_DIVISION)
            }
            SearchParameterHint1D::None => {
                algo::curve::presearch(self, point, self.range_tuple(), PRESEARCH_DIVISION)
            }
        };
        algo::curve::search_parameter(self, point, hint, trials)
    }
}

impl<C: ParametricCurve3D + BoundedCurve> SearchNearestParameter<D1> for ProjectedCurve<C> {
    type Point = Point2;
    fn search_nearest_parameter<H: Into<SearchParameterHint1D>>(
        &self,
        point: Self::Point,
        hint: H,
        trials: usize,
    ) -> Option<f64> {
        let hint = match hint.into() {
            SearchParameterHint1D::Parameter(t) => t,
            SearchParameterHint1D::Range(x, y) => {
                algo::curve::presearch(self, point, (x, y), PRESEARCH_DIVISION)
            }
            SearchParameterHint1D::None => {
                algo::curve::presearch(self, point, self.range_tuple(), PRESEARCH_DIVISION)
            }
        };
        algo::curve::search_nearest_parameter(self, point, hint, trials)
    }
}

impl<C> RevolutedCurve<C> {
    /// Creates a surface by revoluting a curve.
    #[inline(always)]
    pub fn by_revolution(curve: C, origin: Point3, axis: Vector3) -> Self {
        RevolutedCurve {
            curve,
            revolution: Revolution::new(origin, axis),
        }
    }
    /// Returns the curve before revoluted.
    #[inline(always)]
    pub const fn entity_curve(&self) -> &C {
        &self.curve
    }
    /// Into the curve before revoluted.
    #[inline(always)]
    pub fn into_entity_curve(self) -> C {
        self.curve
    }
    /// Returns origin of revolution
    #[inline(always)]
    pub const fn origin(&self) -> Point3 {
        self.revolution.origin
    }
    /// Returns axis of revolution
    #[inline(always)]
    pub const fn axis(&self) -> Vector3 {
        self.revolution.axis
    }
}

impl<C: ParametricCurve3D + BoundedCurve> RevolutedCurve<C> {
    /// Returns true if the front point of the curve is on the axis of rotation.
    /// # Examples
    /// ```
    /// use monstertruck_geometry::prelude::*;
    /// let line = BsplineCurve::new(
    ///     KnotVector::bezier_knot(1),
    ///     vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0)],
    /// );
    /// let surface0 = RevolutedCurve::by_revolution(line.clone(), Point3::origin(), Vector3::unit_y());
    /// assert!(surface0.is_front_fixed());
    /// let surface1 = RevolutedCurve::by_revolution(line, Point3::new(1.0, 0.0, 0.0), Vector3::unit_y());
    /// assert!(!surface1.is_front_fixed());
    /// ```
    #[inline(always)]
    pub fn is_front_fixed(&self) -> bool {
        self.revolution.contains(self.curve.front())
    }
    /// Returns true if the back point of the curve is on the axis of rotation.
    /// # Examples
    /// ```
    /// use monstertruck_geometry::prelude::*;
    /// let line = BsplineCurve::new(
    ///     KnotVector::bezier_knot(1),
    ///     vec![Point3::new(0.0, 0.0, 1.0), Point3::new(0.0, 0.0, 0.0)],
    /// );
    /// let surface0 = RevolutedCurve::by_revolution(line.clone(), Point3::origin(), Vector3::unit_y());
    /// assert!(surface0.is_back_fixed());
    /// let surface1 = RevolutedCurve::by_revolution(line, Point3::new(1.0, 0.0, 0.0), Vector3::unit_y());
    /// assert!(!surface1.is_back_fixed());
    /// ```
    #[inline(always)]
    pub fn is_back_fixed(&self) -> bool {
        self.revolution.contains(self.curve.back())
    }
}

impl<C: ParametricCurve3D + BoundedCurve> SearchParameter<D2> for RevolutedCurve<C> {
    type Point = Point3;
    fn search_parameter<H: Into<SearchParameterHint2D>>(
        &self,
        point: Point3,
        hint: H,
        trials: usize,
    ) -> Option<(f64, f64)> {
        let (t0, t1) = self.curve.range_tuple();
        if self.is_front_fixed() && self.curve.front().near(&point) {
            match hint.into() {
                SearchParameterHint2D::Parameter(_, y) => Some((t0, y)),
                SearchParameterHint2D::Range((_, y), _) => Some((t0, y)),
                SearchParameterHint2D::None => Some((t0, 0.0)),
            }
        } else if self.is_back_fixed() && self.curve.back().near(&point) {
            match hint.into() {
                SearchParameterHint2D::Parameter(_, y) => Some((t1, y)),
                SearchParameterHint2D::Range(_, (_, y)) => Some((t1, y)),
                SearchParameterHint2D::None => Some((t1, 2.0 * PI)),
            }
        } else {
            let proj_curve = ProjectedCurve {
                curve: &self.curve,
                revolution: self.revolution,
            };
            let p = self.revolution.proj_point(point);
            let hint0 = match hint.into() {
                SearchParameterHint2D::Parameter(x, _) => SearchParameterHint1D::Parameter(x),
                SearchParameterHint2D::Range((x0, _), (x1, _)) => {
                    SearchParameterHint1D::Range(x0, x1)
                }
                SearchParameterHint2D::None => SearchParameterHint1D::None,
            };
            let t = proj_curve.search_parameter(p, hint0, trials)?;
            let p = self.curve.subs(t);
            let ang = self.revolution.proj_angle(p, point);
            match self.subs(t, ang).near(&point) {
                true => Some((t, ang)),
                false => None,
            }
        }
    }
}

impl<C: ParametricCurve3D + BoundedCurve> SearchNearestParameter<D2> for RevolutedCurve<C> {
    type Point = Point3;
    fn search_nearest_parameter<H: Into<SearchParameterHint2D>>(
        &self,
        point: Point3,
        hint: H,
        trials: usize,
    ) -> Option<(f64, f64)> {
        let (t0, t1) = self.curve.range_tuple();
        let on_axis = move |o: Point3, normal: Vector3| {
            let op = point - o;
            op.cross(&self.revolution.axis).so_small() && op.dot(normal) >= 0.0
        };
        if self.is_front_fixed() && on_axis(self.curve.front(), self.normal(t0, 0.0)) {
            match hint.into() {
                SearchParameterHint2D::Parameter(_, y) => Some((t0, y)),
                SearchParameterHint2D::Range((_, y), _) => Some((t0, y)),
                SearchParameterHint2D::None => Some((t0, 0.0)),
            }
        } else if self.is_back_fixed() && on_axis(self.curve.back(), self.normal(t1, 0.0)) {
            match hint.into() {
                SearchParameterHint2D::Parameter(_, y) => Some((t1, y)),
                SearchParameterHint2D::Range(_, (_, y)) => Some((t1, y)),
                SearchParameterHint2D::None => Some((t1, 2.0 * PI)),
            }
        } else {
            let proj_curve = ProjectedCurve {
                curve: &self.curve,
                revolution: self.revolution,
            };
            let p = self.revolution.proj_point(point);
            let hint0 = match hint.into() {
                SearchParameterHint2D::Parameter(x, _) => SearchParameterHint1D::Parameter(x),
                SearchParameterHint2D::Range((x0, _), (x1, _)) => {
                    SearchParameterHint1D::Range(x0, x1)
                }
                SearchParameterHint2D::None => SearchParameterHint1D::None,
            };
            let t = proj_curve.search_nearest_parameter(p, hint0, trials)?;
            let p = self.curve.subs(t);
            Some((t, self.revolution.proj_angle(p, point)))
        }
    }
}

fn sub_include<C0, C1>(
    surface: &RevolutedCurve<C0>,
    curve: &C1,
    knots: &[f64],
    degree: usize,
) -> bool
where
    C0: ParametricCurve3D + BoundedCurve,
    C1: ParametricCurve3D + BoundedCurve,
{
    let first = curve.subs(knots[0]);
    let mut hint = match surface.search_parameter(first, None, INCLUDE_CURVE_TRIALS) {
        Some(hint) => hint,
        None => return false,
    };
    knots
        .windows(2)
        .flat_map(move |knot| {
            (1..=degree).map(move |i| {
                let s = i as f64 / degree as f64;
                knot[0] * (1.0 - s) + knot[1] * s
            })
        })
        .all(move |t| {
            let pt = PcurveTrait::subs(curve, t);
            surface
                .search_parameter(pt, Some(hint), INCLUDE_CURVE_TRIALS)
                .or_else(|| surface.search_parameter(pt, None, INCLUDE_CURVE_TRIALS))
                .map(|res| hint = res)
                .is_some()
        })
}

impl IncludeCurve<BsplineCurve<Point3>> for RevolutedCurve<&BsplineCurve<Point3>> {
    fn include(&self, curve: &BsplineCurve<Point3>) -> bool {
        let knots = curve.knot_vec().to_single_multi().0;
        let degree = usize::max(2, usize::max(curve.degree(), self.curve.degree()));
        sub_include(self, curve, &knots, degree)
    }
}

impl IncludeCurve<BsplineCurve<Point3>> for RevolutedCurve<BsplineCurve<Point3>> {
    fn include(&self, curve: &BsplineCurve<Point3>) -> bool {
        let knots = curve.knot_vec().to_single_multi().0;
        let degree = usize::max(2, usize::max(curve.degree(), self.curve.degree()));
        sub_include(self, curve, &knots, degree)
    }
}

impl IncludeCurve<BsplineCurve<Point3>> for RevolutedCurve<&NurbsCurve<Vector4>> {
    fn include(&self, curve: &BsplineCurve<Point3>) -> bool {
        let knots = curve.knot_vec().to_single_multi().0;
        let degree = curve.degree() + usize::max(2, self.curve.degree());
        sub_include(self, curve, &knots, degree)
    }
}

impl IncludeCurve<BsplineCurve<Point3>> for RevolutedCurve<NurbsCurve<Vector4>> {
    fn include(&self, curve: &BsplineCurve<Point3>) -> bool {
        let knots = curve.knot_vec().to_single_multi().0;
        let degree = curve.degree() + usize::max(2, self.curve.degree());
        sub_include(self, curve, &knots, degree)
    }
}

impl IncludeCurve<NurbsCurve<Vector4>> for RevolutedCurve<&BsplineCurve<Point3>> {
    fn include(&self, curve: &NurbsCurve<Vector4>) -> bool {
        let knots = curve.knot_vec().to_single_multi().0;
        let degree = curve.degree() + usize::max(2, self.curve.degree());
        sub_include(self, curve, &knots, degree)
    }
}

impl IncludeCurve<NurbsCurve<Vector4>> for RevolutedCurve<BsplineCurve<Point3>> {
    fn include(&self, curve: &NurbsCurve<Vector4>) -> bool {
        let knots = curve.knot_vec().to_single_multi().0;
        let degree = curve.degree() + usize::max(2, self.curve.degree());
        sub_include(self, curve, &knots, degree)
    }
}

impl IncludeCurve<NurbsCurve<Vector4>> for RevolutedCurve<&NurbsCurve<Vector4>> {
    fn include(&self, curve: &NurbsCurve<Vector4>) -> bool {
        let knots = curve.knot_vec().to_single_multi().0;
        let degree = curve.degree() + usize::max(2, self.curve.degree());
        sub_include(self, curve, &knots, degree)
    }
}

impl IncludeCurve<NurbsCurve<Vector4>> for RevolutedCurve<NurbsCurve<Vector4>> {
    fn include(&self, curve: &NurbsCurve<Vector4>) -> bool {
        let knots = curve.knot_vec().to_single_multi().0;
        let degree = curve.degree() + usize::max(2, self.curve.degree());
        sub_include(self, curve, &knots, degree)
    }
}

impl<C> ParameterDivision2D for RevolutedCurve<C>
where
    C: ParametricCurve3D + ParameterDivision1D<Point = Point3>,
{
    fn parameter_division(
        &self,
        (urange, vrange): ((f64, f64), (f64, f64)),
        tol: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        let curve_division = self.curve.parameter_division(urange, tol);
        let max = curve_division
            .1
            .into_iter()
            .fold(0.0, |max2, pt| {
                let h = self.revolution.proj_point(pt)[1];
                f64::max(max2, h)
            })
            .sqrt();
        let acos = f64::acos(1.0 - tol / max);
        let div: usize = 1 + ((vrange.1 - vrange.0) / acos).floor() as usize;
        let circle_division = (0..=div)
            .map(|j| vrange.0 + (vrange.1 - vrange.0) * j as f64 / div as f64)
            .collect();
        (curve_division.0, circle_division)
    }
}

/// Cosine coefficients for the 9-point rational circle (4 quarter arcs, degree 2).
const CIRCLE_COS: [f64; 9] = [1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0, 1.0];
/// Sine coefficients for the 9-point rational circle.
const CIRCLE_SIN: [f64; 9] = [0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0];
/// Weights for the 9-point rational circle.
const CIRCLE_W: [f64; 9] = [
    1.0,
    std::f64::consts::FRAC_1_SQRT_2,
    1.0,
    std::f64::consts::FRAC_1_SQRT_2,
    1.0,
    std::f64::consts::FRAC_1_SQRT_2,
    1.0,
    std::f64::consts::FRAC_1_SQRT_2,
    1.0,
];

/// Builds the degree-2 knot vector for a full 2*PI revolution.
fn full_revolution_knot_vector() -> KnotVector {
    KnotVector::from(vec![
        0.0,
        0.0,
        0.0,
        PI / 2.0,
        PI / 2.0,
        PI,
        PI,
        3.0 * PI / 2.0,
        3.0 * PI / 2.0,
        2.0 * PI,
        2.0 * PI,
        2.0 * PI,
    ])
}

impl RevolutedCurve<NurbsCurve<Vector4>> {
    /// Converts this revolved surface to an exact [`NurbsSurface`] via rational
    /// circle arc tensor product.
    ///
    /// The profile curve becomes the u-direction and the revolution becomes the
    /// v-direction. A full 2*PI revolution is represented with 9 control points
    /// (degree 2) using the standard rational Bezier circle decomposition.
    ///
    /// Handles points at infinity (`w = 0`) correctly by working entirely in
    /// homogeneous coordinates.
    pub fn to_nurbs_surface(&self) -> NurbsSurface<Vector4> {
        let origin_vec = self.origin().to_vec();
        let axis = self.axis();
        let u_knots = self.curve.knot_vec().clone();

        let control_points: Vec<Vec<Vector4>> = self
            .curve
            .control_points()
            .iter()
            .map(|cp_hom| {
                // Decompose the 3D part of the homogeneous control point into
                // axis-parallel and perpendicular components relative to the
                // revolution origin. The origin contribution is scaled by w
                // (vanishes for w=0 points at infinity).
                let w_profile = cp_hom.w;
                let hxyz = Vector3::new(cp_hom.x, cp_hom.y, cp_hom.z);
                let hxyz_rel = hxyz - w_profile * origin_vec;
                let along = hxyz_rel.dot(axis);
                let radial = hxyz_rel - along * axis;
                let radius = radial.magnitude();

                // Fixed part: axis-parallel component plus origin contribution.
                let fixed = w_profile * origin_vec + along * axis;

                // Compute the radial and tangent unit vectors. When the point
                // lies on the axis (radius ~ 0), the rotated radial is zero
                // for all 9 circle positions.
                let (unit_r, unit_t) = if radius < 1.0e-14 {
                    (Vector3::zero(), Vector3::zero())
                } else {
                    (radial / radius, axis.cross(&(radial / radius)).normalize())
                };

                (0..9)
                    .map(|j| {
                        let rotated: Vector3 =
                            radius * (CIRCLE_COS[j] * unit_r + CIRCLE_SIN[j] * unit_t);
                        let h = fixed + rotated;
                        let wc = CIRCLE_W[j];
                        Vector4::new(h.x * wc, h.y * wc, h.z * wc, w_profile * wc)
                    })
                    .collect()
            })
            .collect();

        NurbsSurface::new(BsplineSurface::new(
            (u_knots, full_revolution_knot_vector()),
            control_points,
        ))
    }
}

impl RevolutedCurve<BsplineCurve<Point3>> {
    /// Converts this revolved surface to an exact [`NurbsSurface`] via rational
    /// circle arc tensor product.
    ///
    /// The [`BsplineCurve`] profile is first lifted to a [`NurbsCurve`] before
    /// performing the tensor product construction.
    pub fn to_nurbs_surface(&self) -> NurbsSurface<Vector4> {
        let nurbs_curve = NurbsCurve::from(self.curve.clone());
        RevolutedCurve::by_revolution(nurbs_curve, self.origin(), self.axis()).to_nurbs_surface()
    }
}

fn from_axis_angle_derivation(n: usize, axis: Vector3, angle: Rad<f64>) -> Matrix3 {
    let (s, c) = Rad::sin_cos(angle);
    let (s, c) = match n % 4 {
        0 => (s, c),
        1 => (c, -s),
        2 => (-s, -c),
        _ => (-c, s),
    };
    let _1subc = match n {
        0 => 1.0 - c,
        _ => -c,
    };

    #[allow(clippy::deprecated_cfg_attr)]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    Matrix3::new(
        _1subc * axis[0] * axis[0] + c,
        _1subc * axis[0] * axis[1] + s * axis[2],
        _1subc * axis[0] * axis[2] - s * axis[1],

        _1subc * axis[0] * axis[1] - s * axis[2],
        _1subc * axis[1] * axis[1] + c,
        _1subc * axis[1] * axis[2] + s * axis[0],

        _1subc * axis[0] * axis[2] + s * axis[1],
        _1subc * axis[1] * axis[2] - s * axis[0],
        _1subc * axis[2] * axis[2] + c,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use monstertruck_core::assert_near;

    /// Every point on the NURBS surface produced from revolving a line lies on the
    /// same surface of revolution (a cylinder). We verify this by checking that
    /// every evaluated point has the correct radius and height, and that the
    /// surface at the knot breakpoints matches exactly.
    #[test]
    fn to_nurbs_surface_line_around_y_axis() {
        let line = BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 1.0, 0.0)],
        );
        let revolved = RevolutedCurve::by_revolution(line, Point3::origin(), Vector3::unit_y());
        let nurbs = revolved.to_nurbs_surface();

        // At v-direction knot breakpoints the parameterization is exact.
        let exact_vs = [0.0, PI / 2.0, PI, 3.0 * PI / 2.0, 2.0 * PI];
        let n = 10;
        for i in 0..=n {
            let u = i as f64 / n as f64;
            for &v in &exact_vs {
                let expected = revolved.evaluate(u, v);
                let actual = nurbs.subs(u, v);
                assert_near!(
                    expected,
                    actual,
                    concat!("mismatch at knot (u={}, v={}): expected {:?}, got {:?}"),
                    u,
                    v,
                    expected,
                    actual,
                );
            }
        }

        // Every point must lie on the surface of revolution: radius = 1, y in [0, 1].
        for i in 0..=n {
            for j in 0..=n {
                let u = i as f64 / n as f64;
                let v = 2.0 * PI * j as f64 / n as f64;
                let pt = nurbs.subs(u, v);
                let r = (pt.x * pt.x + pt.z * pt.z).sqrt();
                assert!(
                    (r - 1.0).abs() < 1.0e-10,
                    "radius mismatch at (u={u}, v={v}): r={r}",
                );
            }
        }
    }

    /// Weighted profile curve (rational half-circle) revolved around X-axis
    /// produces a sphere; every evaluated point must have unit distance from origin.
    #[test]
    fn to_nurbs_surface_weighted_profile() {
        let knot_vec = KnotVector::bezier_knot(2);
        let control_points = vec![
            Vector4::new(1.0, 0.0, 0.0, 1.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(-1.0, 0.0, 0.0, 1.0),
        ];
        let profile = NurbsCurve::new(BsplineCurve::new(knot_vec, control_points));
        let revolved = RevolutedCurve::by_revolution(profile, Point3::origin(), Vector3::unit_x());
        let nurbs = revolved.to_nurbs_surface();

        // At v-direction knot breakpoints the parameterization is exact.
        let exact_vs = [0.0, PI / 2.0, PI, 3.0 * PI / 2.0, 2.0 * PI];
        let n = 10;
        for i in 0..=n {
            let u = i as f64 / n as f64;
            for &v in &exact_vs {
                let expected = revolved.evaluate(u, v);
                let actual = nurbs.subs(u, v);
                assert_near!(
                    expected,
                    actual,
                    concat!("mismatch at knot (u={}, v={}): expected {:?}, got {:?}"),
                    u,
                    v,
                    expected,
                    actual,
                );
            }
        }

        // Every point on the sphere should be at distance 1 from origin.
        for i in 0..=n {
            for j in 0..=n {
                let u = i as f64 / n as f64;
                let v = 2.0 * PI * j as f64 / n as f64;
                let pt = nurbs.subs(u, v);
                let dist = pt.to_vec().magnitude();
                assert!(
                    (dist - 1.0).abs() < 1.0e-10,
                    "distance from origin at (u={u}, v={v}): {dist}",
                );
            }
        }
    }

    /// [`BsplineCurve`] convenience conversion also produces geometrically exact results.
    #[test]
    fn to_nurbs_surface_bspline_convenience() {
        let line = BsplineCurve::new(
            KnotVector::bezier_knot(1),
            vec![Point3::new(2.0, 0.0, 0.0), Point3::new(2.0, 3.0, 0.0)],
        );
        let revolved = RevolutedCurve::by_revolution(line, Point3::origin(), Vector3::unit_y());
        let nurbs = revolved.to_nurbs_surface();

        // Every point must lie on the surface of revolution: radius = 2, y in [0, 3].
        let n = 8;
        for i in 0..=n {
            for j in 0..=n {
                let u = i as f64 / n as f64;
                let v = 2.0 * PI * j as f64 / n as f64;
                let pt = nurbs.subs(u, v);
                let r = (pt.x * pt.x + pt.z * pt.z).sqrt();
                assert!(
                    (r - 2.0).abs() < 1.0e-10,
                    "radius mismatch at (u={u}, v={v}): r={r}",
                );
            }
        }

        // At v-direction knot breakpoints, match exact.
        let exact_vs = [0.0, PI / 2.0, PI, 3.0 * PI / 2.0, 2.0 * PI];
        for i in 0..=n {
            let u = i as f64 / n as f64;
            for &v in &exact_vs {
                let expected = revolved.evaluate(u, v);
                let actual = nurbs.subs(u, v);
                assert_near!(
                    expected,
                    actual,
                    concat!("mismatch at knot (u={}, v={}): expected {:?}, got {:?}"),
                    u,
                    v,
                    expected,
                    actual,
                );
            }
        }
    }
}
