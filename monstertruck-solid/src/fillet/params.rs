use monstertruck_geometry::prelude::*;
use std::num::NonZeroUsize;

/// Controls how fillet faces relate to host faces.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FilletMode {
    /// Fillet face is a separate topological face (current behavior).
    #[default]
    KeepSeparateFace,
    /// Fillet face is a separate face annotated with G1/G2 continuity
    /// constraints at shared edges, enabling seamless tessellation.
    IntegrateVisual,
}

/// Controls how fillet surfaces extend beyond edge endpoints.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ExtendMode {
    /// Extend fillet surfaces beyond endpoints when possible (current behavior).
    #[default]
    Auto,
    /// Never extend fillet surfaces beyond endpoints.
    NoExtend,
}

/// Controls how fillet corners (where multiple fillets meet) are handled.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CornerMode {
    /// Default corner handling.
    #[default]
    Auto,
    /// Trim corners to a sharp intersection.
    Trim,
    /// Blend corners with a smooth transition.
    Blend,
}

/// Profile shape for fillet operations.
#[derive(Debug, Clone, Default)]
pub enum FilletProfile {
    /// Circular arc cross-section (traditional fillet).
    #[default]
    Round,
    /// Flat ruled surface (chamfer/bevel).
    ///
    /// Creates a flat cut between two adjacent faces, replacing the shared edge
    /// with a ruled surface. Unlike [`Round`](Self::Round), which creates a
    /// circular arc cross-section, `Chamfer` creates a straight-line transition.
    /// Use with [`FilletOptions::with_profile`].
    Chamfer,
    /// V-shaped ridge: two straight segments meeting at the transit point.
    Ridge,
    /// User-provided 2D profile curve. Domain [0,1], maps (0,0)→contact0,
    /// (1,0)→contact1, y-axis = displacement toward transit.
    Custom(Box<BsplineCurve<Point2>>),
}

/// Radius specification for fillet operations.
pub enum RadiusSpec {
    /// Constant radius along the entire edge/wire.
    Constant(f64),
    /// Variable radius as a function of normalized parameter `t` in `[0, 1]`.
    ///
    /// Supported for single-edge and wire fillets.
    /// For closed wires, endpoint continuity is required:
    /// `f(0.0)` must be near `f(1.0)`.
    Variable(Box<dyn Fn(f64) -> f64>),
    /// Per-edge radius. Length must match the edge count passed to [`fillet_edges`](super::fillet_edges).
    PerEdge(Vec<f64>),
}

impl std::fmt::Debug for RadiusSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(r) => f.debug_tuple("Constant").field(r).finish(),
            Self::Variable(_) => f.debug_tuple("Variable").field(&"<fn>").finish(),
            Self::PerEdge(v) => f.debug_tuple("PerEdge").field(v).finish(),
        }
    }
}

/// Options for fillet operations.
#[derive(Debug)]
pub struct FilletOptions {
    /// Radius specification.
    pub radius: RadiusSpec,
    /// Number of divisions for the rolling ball algorithm. Default: 5.
    pub divisions: NonZeroUsize,
    /// Profile shape. Default: [`FilletProfile::Round`].
    pub profile: FilletProfile,
    /// Fillet-to-host-face integration mode.
    pub mode: FilletMode,
    /// How fillet surfaces extend beyond edge endpoints.
    pub extend_mode: ExtendMode,
    /// How fillet corners are handled.
    pub corner_mode: CornerMode,
}

impl Default for FilletOptions {
    fn default() -> Self {
        Self {
            radius: RadiusSpec::Constant(0.1),
            divisions: NonZeroUsize::new(5).expect("5 is non-zero"),
            profile: FilletProfile::default(),
            mode: FilletMode::default(),
            extend_mode: ExtendMode::default(),
            corner_mode: CornerMode::default(),
        }
    }
}

impl FilletOptions {
    /// Creates options with a constant radius.
    pub fn constant(radius: f64) -> Self {
        Self {
            radius: RadiusSpec::Constant(radius),
            ..Default::default()
        }
    }

    /// Creates options with a variable radius function.
    pub fn variable(radius: impl Fn(f64) -> f64 + 'static) -> Self {
        Self {
            radius: RadiusSpec::Variable(Box::new(radius)),
            ..Default::default()
        }
    }

    /// Sets the fillet radius specification.
    pub fn with_radius(mut self, radius: RadiusSpec) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the division count used by the fillet algorithm.
    pub fn with_division(mut self, division: NonZeroUsize) -> Self {
        self.divisions = division;
        self
    }

    /// Sets the fillet profile.
    pub fn with_profile(mut self, profile: FilletProfile) -> Self {
        self.profile = profile;
        self
    }

    /// Sets the fillet-to-host-face integration mode.
    pub fn with_mode(mut self, mode: FilletMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets how fillet surfaces extend beyond edge endpoints.
    pub fn with_extend_mode(mut self, extend_mode: ExtendMode) -> Self {
        self.extend_mode = extend_mode;
        self
    }

    /// Sets how fillet corners are handled.
    pub fn with_corner_mode(mut self, corner_mode: CornerMode) -> Self {
        self.corner_mode = corner_mode;
        self
    }
}
