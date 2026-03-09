use monstertruck_modeling::*;
use monstertruck_step::save::*;

macro_rules! dir ( () => { concat!(env!("CARGO_MANIFEST_DIR"), "/../resources/shape/") });

const SOLID_JSONS: &[&str] = &[
    concat!(dir!(), "bottle.json"),
    concat!(dir!(), "punched-cube.json"),
    concat!(dir!(), "torus-punched-cube.json"),
    concat!(dir!(), "cube-in-cube.json"),
    concat!(dir!(), "punched-cube-shapeops.json"),
];

#[test]
fn parse_solid() {
    for json_file in SOLID_JSONS.iter() {
        let json = std::fs::read(json_file).unwrap();
        let solid: CompressedSolid = serde_json::from_reader(json.as_slice()).unwrap();
        let step_string =
            CompleteStepDisplay::new(StepModel::from(&solid), Default::default()).to_string();
        ruststep::parser::parse(&step_string).unwrap_or_else(|e| {
            panic!(
                "failed to parse step from {json_file}\n[Error Message]\n{e}[STEP file]\n{step_string}"
            )
        });
    }
}

#[test]
fn parse_shell() {
    for json_file in SOLID_JSONS.iter() {
        let json = std::fs::read(json_file).unwrap();
        let mut solid: CompressedSolid = serde_json::from_reader(json.as_slice()).unwrap();
        let shell = solid.boundaries.pop().unwrap();
        let step_string =
            CompleteStepDisplay::new(StepModel::from(&shell), Default::default()).to_string();
        ruststep::parser::parse(&step_string).unwrap_or_else(|e| {
            panic!(
                "failed to parse step from {json_file}\n[Error Message]\n{e}[STEP file]\n{step_string}"
            )
        });
    }
}

#[test]
fn parse_boolean_result_solid() {
    let json_file = concat!(dir!(), "punched-cube-shapeops.json");
    let json = std::fs::read(json_file).unwrap();
    let solid: CompressedSolid = serde_json::from_reader(json.as_slice()).unwrap();
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&solid), Default::default()).to_string();
    ruststep::parser::parse(&step_string).unwrap_or_else(|e| {
        panic!(
            "failed to parse step from {json_file}\n[Error Message]\n{e}[STEP file]\n{step_string}"
        )
    });
    // Verify that entity IDs in the STEP output are unique.
    // The surface1 index bug causes surface1 entities to be written at surface0's
    // starting index, producing duplicate entity IDs.
    let mut entity_ids: Vec<&str> = step_string
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                trimmed.split_once(' ').map(|(id, _)| id)
            } else {
                None
            }
        })
        .collect();
    let total = entity_ids.len();
    entity_ids.sort_unstable();
    entity_ids.dedup();
    assert_eq!(
        entity_ids.len(),
        total,
        "STEP output has duplicate entity IDs (expected {total} unique, got {})",
        entity_ids.len()
    );
}

#[test]
fn boolean_step_round_trip() {
    // Load a boolean AND result (punched-cube-shapeops) and verify the full
    // export -> parse round-trip with content validation.
    let json_file = concat!(dir!(), "punched-cube-shapeops.json");
    let json = std::fs::read(json_file).unwrap();
    let solid: CompressedSolid = serde_json::from_reader(json.as_slice()).unwrap();
    let step_string =
        CompleteStepDisplay::new(StepModel::from(&solid), Default::default()).to_string();

    // Parse the STEP output.
    ruststep::parser::parse(&step_string).unwrap_or_else(|e| {
        panic!("failed to parse STEP from boolean result\n[Error Message]\n{e}")
    });

    // Verify expected STEP entities for boolean-result geometry.
    assert!(
        step_string.contains("INTERSECTION_CURVE"),
        "boolean result should contain INTERSECTION_CURVE entities"
    );
    assert!(
        step_string.contains("B_SPLINE_CURVE_WITH_KNOTS"),
        "boolean result should contain B_SPLINE_CURVE_WITH_KNOTS entities"
    );
    assert!(
        step_string.contains("PLANE"),
        "boolean result should contain PLANE entities"
    );
    assert!(
        step_string.contains("MANIFOLD_SOLID_BREP"),
        "boolean result should contain MANIFOLD_SOLID_BREP entity"
    );
    assert!(
        step_string.contains("CLOSED_SHELL"),
        "boolean result should contain CLOSED_SHELL entity"
    );

    // Verify entity ID uniqueness (catches the surface1 index bug).
    let mut entity_ids: Vec<&str> = step_string
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                trimmed.split_once(' ').map(|(id, _)| id)
            } else {
                None
            }
        })
        .collect();
    let total = entity_ids.len();
    entity_ids.sort_unstable();
    entity_ids.dedup();
    assert_eq!(
        entity_ids.len(),
        total,
        "boolean result STEP has duplicate entity IDs"
    );

    // Verify INTERSECTION_CURVE count matches expectations.
    let ic_count = step_string
        .lines()
        .filter(|l| l.contains("INTERSECTION_CURVE"))
        .count();
    assert!(
        ic_count > 0,
        "expected at least one INTERSECTION_CURVE entity"
    );
}

#[test]
fn parse_solids() {
    let solids: Vec<CompressedSolid> = SOLID_JSONS
        .iter()
        .map(|json_file| {
            let json = std::fs::read(json_file).unwrap();
            serde_json::from_reader(json.as_slice()).unwrap()
        })
        .collect();
    let step_string =
        CompleteStepDisplay::new(StepModels::from_iter(&solids), Default::default()).to_string();
    ruststep::parser::parse(&step_string).unwrap_or_else(|e| {
        panic!("failed to parse step\n[Error Message]\n{e}[STEP file]\n{step_string}")
    });
}
