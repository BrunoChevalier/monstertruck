use std::fmt;

/// Diagnostic information about curve network incompatibilities.
#[derive(Debug, Clone, PartialEq)]
pub enum CurveNetworkDiagnostic {
    /// Too few curves provided.
    InsufficientCurves {
        /// Minimum required count.
        required: usize,
        /// Actual count provided.
        got: usize,
    },
    /// Too few sections requested.
    InsufficientSections {
        /// Minimum required count.
        required: usize,
        /// Actual count provided.
        got: usize,
    },
    /// Curve endpoints do not meet at expected intersection points.
    EndpointMismatch {
        /// Index of the problematic curve.
        curve_index: usize,
        /// Expected endpoint coordinates.
        expected: [f64; 3],
        /// Actual endpoint coordinates.
        actual: [f64; 3],
        /// Euclidean distance between expected and actual.
        distance: f64,
    },
    /// Curves have incompatible parameter domains.
    DomainMismatch {
        /// Index of the first curve.
        curve_a: usize,
        /// Index of the second curve.
        curve_b: usize,
        /// Parameter range of the first curve.
        range_a: (f64, f64),
        /// Parameter range of the second curve.
        range_b: (f64, f64),
    },
    /// Grid dimension mismatch for Gordon surface.
    GridDimensionMismatch {
        /// Expected number of rows.
        expected_rows: usize,
        /// Expected number of columns.
        expected_cols: usize,
        /// Actual number of rows.
        actual_rows: usize,
        /// Actual number of columns.
        actual_cols: usize,
    },
    /// Compatibility normalization failed.
    CompatNormalizationFailed {
        /// Description of the failure.
        reason: String,
    },
    /// Degenerate geometry (e.g., zero-length chord).
    DegenerateGeometry {
        /// Description of the degenerate condition.
        description: String,
    },
    /// Intersection count mismatch between curve families.
    IntersectionCountMismatch {
        /// Index of the u-curve.
        u_curve_index: usize,
        /// Index of the v-curve.
        v_curve_index: usize,
        /// Number of intersections found.
        found: usize,
        /// Number expected (1 for a well-formed Gordon network).
        expected: usize,
    },
    /// A caller-supplied grid point does not lie on both corresponding curves.
    GridPointNotOnCurve {
        /// Row index (u-curve index) of the failing point.
        row: usize,
        /// Column index (v-curve index) of the failing point.
        col: usize,
        /// Distance from the point to the nearest position on the u-curve.
        u_distance: f64,
        /// Distance from the point to the nearest position on the v-curve.
        v_distance: f64,
        /// Tolerance that was exceeded.
        tolerance: f64,
    },
}

impl fmt::Display for CurveNetworkDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientCurves { required, got } => {
                write!(
                    f,
                    "insufficient curves: need at least {required}, got {got}"
                )
            }
            Self::InsufficientSections { required, got } => {
                write!(
                    f,
                    "insufficient sections: need at least {required}, got {got}"
                )
            }
            Self::EndpointMismatch {
                curve_index,
                expected,
                actual,
                distance,
            } => write!(
                f,
                "endpoint mismatch on curve {curve_index}: expected {expected:?}, got {actual:?} (distance {distance:.6})"
            ),
            Self::DomainMismatch {
                curve_a,
                curve_b,
                range_a,
                range_b,
            } => write!(
                f,
                "domain mismatch: curve {curve_a} has range {range_a:?}, curve {curve_b} has range {range_b:?}"
            ),
            Self::GridDimensionMismatch {
                expected_rows,
                expected_cols,
                actual_rows,
                actual_cols,
            } => write!(
                f,
                "grid dimension mismatch: expected {expected_rows}x{expected_cols}, got {actual_rows}x{actual_cols}"
            ),
            Self::CompatNormalizationFailed { reason } => {
                write!(f, "compatibility normalization failed: {reason}")
            }
            Self::DegenerateGeometry { description } => {
                write!(f, "degenerate geometry: {description}")
            }
            Self::IntersectionCountMismatch {
                u_curve_index,
                v_curve_index,
                found,
                expected,
            } => write!(
                f,
                "intersection count mismatch at u[{u_curve_index}] x v[{v_curve_index}]: found {found}, expected {expected}"
            ),
            Self::GridPointNotOnCurve {
                row,
                col,
                u_distance,
                v_distance,
                tolerance,
            } => write!(
                f,
                "grid point [{row}][{col}] not on curves: u-distance={u_distance:.6}, v-distance={v_distance:.6}, tolerance={tolerance:.6}"
            ),
        }
    }
}

impl std::error::Error for CurveNetworkDiagnostic {}
