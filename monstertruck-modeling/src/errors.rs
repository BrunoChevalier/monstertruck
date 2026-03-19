use thiserror::Error;

/// Modeling errors.
#[derive(Debug, PartialEq, Eq, Error)]
pub enum Error {
    /// Wrapper of topological error.
    #[error(transparent)]
    FromTopology(#[from] monstertruck_topology::errors::Error),
    /// Tried to attach a plane to a wire that was not on one plane.
    /// cf. [`builder::try_attach_plane`](../builder/fn.try_attach_plane.html)
    #[error("cannot attach a plane to a wire that is not on one plane.")]
    WireNotInOnePlane,
    /// Tried to create homotopy for two wires with different numbers of edges.
    /// cf. [`builder::try_wire_homotopy`](../builder/fn.try_wire_homotopy.html)
    #[error("The wires must contain the same number of edges to create a homotopy.")]
    NotSameNumberOfEdges,
    /// One or more wires are not closed.
    #[error("all wires must be closed for profile construction.")]
    OpenWire,
    /// Ambiguous nesting: a loop is not clearly inside or outside another.
    #[error("ambiguous nesting between loops; loops may overlap or touch.")]
    AmbiguousNesting,
    /// No outer loop found among the provided wires.
    #[error("no outer loop found; at least one wire must have positive signed area.")]
    NoOuterLoop,
    /// Too few rails provided for a multi-rail sweep (need >= 2).
    #[error("multi-rail sweep requires at least {required} rails, got {got}.")]
    InsufficientRails {
        /// Minimum number of rails required.
        required: usize,
        /// Number of rails actually provided.
        got: usize,
    },
    /// Too few sections for surface construction.
    #[error("surface construction requires at least {required} sections, got {got}.")]
    InsufficientSections {
        /// Minimum number of sections required.
        required: usize,
        /// Number of sections actually provided.
        got: usize,
    },
    /// Surface construction algorithm failed (e.g., degenerate geometry, incompatible curves).
    #[error("surface construction failed: {reason}")]
    SurfaceConstructionFailed {
        /// Description of the failure.
        reason: String,
    },
    /// Curve grid dimensions mismatch for [`gordon`](crate::builder::try_gordon) surface.
    #[error(
        "gordon surface requires matching grid dimensions: expected {expected_rows}x{expected_cols}, got {actual_rows}x{actual_cols}."
    )]
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
}

#[test]
fn print_messages() {
    use std::io::Write;
    writeln!(
        &mut std::io::stderr(),
        "****** test of the expressions of error messages ******\n"
    )
    .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "{}\n",
        Error::FromTopology(monstertruck_topology::errors::Error::SameVertex)
    )
    .unwrap();
    writeln!(&mut std::io::stderr(), "{}\n", Error::WireNotInOnePlane).unwrap();
    writeln!(&mut std::io::stderr(), "{}\n", Error::OpenWire).unwrap();
    writeln!(&mut std::io::stderr(), "{}\n", Error::AmbiguousNesting).unwrap();
    writeln!(&mut std::io::stderr(), "{}\n", Error::NoOuterLoop).unwrap();
    writeln!(
        &mut std::io::stderr(),
        "{}\n",
        Error::InsufficientRails {
            required: 2,
            got: 1
        }
    )
    .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "{}\n",
        Error::InsufficientSections {
            required: 2,
            got: 1
        }
    )
    .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "{}\n",
        Error::SurfaceConstructionFailed {
            reason: "test".into()
        }
    )
    .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "{}\n",
        Error::GridDimensionMismatch {
            expected_rows: 2,
            expected_cols: 3,
            actual_rows: 1,
            actual_cols: 2,
        }
    )
    .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "*******************************************************"
    )
    .unwrap();
}
