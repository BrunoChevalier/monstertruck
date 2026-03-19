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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
/// Currently a marker struct. Future versions may add tolerance fields
/// for controlling intersection point matching.
#[derive(Debug, Clone, Default)]
pub struct GordonOptions {}

/// Options for skin surface construction.
///
/// Currently a marker struct. Future versions may add fields to control
/// v-direction degree or parameterization strategy.
#[derive(Debug, Clone, Default)]
pub struct SkinOptions {}
