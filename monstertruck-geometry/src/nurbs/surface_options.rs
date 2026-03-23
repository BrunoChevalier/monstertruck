/// Frame rule for sweep operations -- controls how the profile is oriented along the rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FrameRule {
    /// Tangent-aligned framing (rotate profile to match rail tangent at each section).
    #[default]
    TangentAligned,
    /// Fixed-up framing (project profile using a fixed up-vector).
    FixedUp,
}

/// Options for single-rail sweep surface construction.
///
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::SweepRailOptions;
/// let mut opts = SweepRailOptions::default();
/// opts.n_sections = 5;
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SweepRailOptions {
    /// Number of sections to sample along the rail.
    pub n_sections: usize,
    /// How the profile frame is oriented along the rail.
    pub frame_rule: FrameRule,
}

impl Default for SweepRailOptions {
    fn default() -> Self {
        Self {
            n_sections: 10,
            frame_rule: FrameRule::TangentAligned,
        }
    }
}

/// Options for single-profile birail surface construction.
///
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::Birail1Options;
/// let mut opts = Birail1Options::default();
/// opts.n_sections = 5;
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Birail1Options {
    /// Number of sections to sample along the rails.
    pub n_sections: usize,
}

impl Default for Birail1Options {
    fn default() -> Self {
        Self { n_sections: 10 }
    }
}

/// Options for dual-profile birail surface construction.
///
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::Birail2Options;
/// let mut opts = Birail2Options::default();
/// opts.n_sections = 5;
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Birail2Options {
    /// Number of sections to sample along the rails.
    pub n_sections: usize,
}

impl Default for Birail2Options {
    fn default() -> Self {
        Self { n_sections: 10 }
    }
}

/// Options for Gordon surface construction.
///
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::GordonOptions;
/// let opts = GordonOptions::default();
/// assert!(opts.grid_tolerance > 0.0);
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct GordonOptions {
    /// Tolerance for grid point validation in [`crate::nurbs::BsplineSurface::try_gordon_verified`].
    /// Points within this distance of the expected curve position are snapped.
    /// Defaults to [`SNAP_TOLERANCE`](monstertruck_core::tolerance_constants::SNAP_TOLERANCE).
    pub grid_tolerance: f64,
}

impl Default for GordonOptions {
    fn default() -> Self {
        Self {
            grid_tolerance: monstertruck_core::tolerance_constants::SNAP_TOLERANCE,
        }
    }
}

/// Options for skin (loft) surface construction.
///
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::SkinOptions;
/// let mut opts = SkinOptions::default();
/// opts.v_degree = 3;
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SkinOptions {
    /// Polynomial degree in the v-direction (loft direction).
    ///
    /// - `1` (default): linear interpolation between sections (degree-1 v-knot vector).
    /// - `2` or `3`: higher-order B-spline interpolation through the section curves.
    ///
    /// Clamped to `1..=min(n-1, requested)` where `n` = number of sections.
    pub v_degree: usize,
}

impl Default for SkinOptions {
    fn default() -> Self {
        Self { v_degree: 1 }
    }
}

/// Options for ruled surface construction between two boundary curves.
///
/// Currently a marker struct. Future versions may add fields to control
/// v-direction parameterization.
///
/// # Examples
///
/// ```
/// use monstertruck_geometry::nurbs::surface_options::RuledSurfaceOptions;
/// let opts = RuledSurfaceOptions::default();
/// ```
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct RuledSurfaceOptions {}
