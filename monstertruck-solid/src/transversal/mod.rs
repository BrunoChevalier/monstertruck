mod divide_face;
pub(crate) mod edge_cases;
mod faces_classification;
mod integrate;
mod intersection_curve;
mod loops_store;
mod polyline_construction;
pub use integrate::{
    ShapeOpsCurve, ShapeOpsError, ShapeOpsSurface, and, difference, or, symmetric_difference,
};
