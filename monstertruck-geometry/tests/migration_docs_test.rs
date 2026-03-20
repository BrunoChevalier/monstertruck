//! Tests that migration documentation is present in doc comments.

/// Reads a source file and extracts the doc-comment block preceding a given
/// function signature.  Returns the concatenated doc lines.
fn extract_doc_comment(source: &str, fn_signature: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let sig_index = lines
        .iter()
        .position(|line| line.contains(fn_signature))
        .unwrap_or_else(|| panic!("function signature not found: {fn_signature}"));

    // Walk backwards from the signature to collect doc-comment lines and
    // attribute lines (skip attributes like `#[deprecated]`, `#[allow]`).
    let mut doc_lines = Vec::new();
    let mut i = sig_index;
    while i > 0 {
        i -= 1;
        let trimmed = lines[i].trim();
        if trimmed.starts_with("///") {
            doc_lines.push(trimmed);
        } else if trimmed.starts_with("#[") {
            // Skip attributes like #[deprecated], #[allow(...)].
            continue;
        } else {
            break;
        }
    }
    doc_lines.reverse();
    doc_lines.join("\n")
}

#[test]
fn try_skin_has_migration_section() {
    let source = include_str!("../src/nurbs/bspline_surface.rs");
    let doc = extract_doc_comment(source, "pub fn try_skin(");
    assert!(
        doc.contains("# Migration"),
        "try_skin doc comment must contain a '# Migration' section"
    );
    assert!(
        doc.contains("**Before**"),
        "try_skin migration section must show before example"
    );
    assert!(
        doc.contains("**After**"),
        "try_skin migration section must show after example"
    );
}

#[test]
fn try_sweep_rail_has_migration_section() {
    let source = include_str!("../src/nurbs/bspline_surface.rs");
    let doc = extract_doc_comment(source, "pub fn try_sweep_rail(");
    assert!(
        doc.contains("# Migration"),
        "try_sweep_rail doc comment must contain a '# Migration' section"
    );
    assert!(
        doc.contains("**Before**"),
        "try_sweep_rail migration section must show before example"
    );
    assert!(
        doc.contains("**After**"),
        "try_sweep_rail migration section must show after example"
    );
}

#[test]
fn try_birail1_has_migration_section() {
    let source = include_str!("../src/nurbs/bspline_surface.rs");
    let doc = extract_doc_comment(source, "pub fn try_birail1(");
    assert!(
        doc.contains("# Migration"),
        "try_birail1 doc comment must contain a '# Migration' section"
    );
    assert!(
        doc.contains("**Before**"),
        "try_birail1 migration section must show before example"
    );
    assert!(
        doc.contains("**After**"),
        "try_birail1 migration section must show after example"
    );
}

#[test]
fn try_birail2_has_migration_section() {
    let source = include_str!("../src/nurbs/bspline_surface.rs");
    let doc = extract_doc_comment(source, "pub fn try_birail2(");
    assert!(
        doc.contains("# Migration"),
        "try_birail2 doc comment must contain a '# Migration' section"
    );
    assert!(
        doc.contains("**Before**"),
        "try_birail2 migration section must show before example"
    );
    assert!(
        doc.contains("**After**"),
        "try_birail2 migration section must show after example"
    );
}

#[test]
fn try_gordon_has_migration_section() {
    let source = include_str!("../src/nurbs/bspline_surface.rs");
    let doc = extract_doc_comment(source, "pub fn try_gordon(");
    assert!(
        doc.contains("# Migration"),
        "try_gordon doc comment must contain a '# Migration' section"
    );
    assert!(
        doc.contains("**Before**"),
        "try_gordon migration section must show before example"
    );
    assert!(
        doc.contains("**After**"),
        "try_gordon migration section must show after example"
    );
}

#[test]
fn try_gordon_from_network_has_usage_example() {
    let source = include_str!("../src/nurbs/bspline_surface.rs");
    let doc = extract_doc_comment(source, "pub fn try_gordon_from_network(");
    assert!(
        doc.contains("# Example") || doc.contains("# Usage"),
        "try_gordon_from_network doc comment must contain a usage example section"
    );
    assert!(
        doc.contains("try_gordon_from_network"),
        "try_gordon_from_network doc comment must show usage of the function"
    );
}

#[test]
fn try_gordon_verified_has_usage_example() {
    let source = include_str!("../src/nurbs/bspline_surface.rs");
    let doc = extract_doc_comment(source, "pub fn try_gordon_verified(");
    assert!(
        doc.contains("# Example") || doc.contains("# Usage"),
        "try_gordon_verified doc comment must contain a usage example section"
    );
    assert!(
        doc.contains("try_gordon_verified"),
        "try_gordon_verified doc comment must show usage of the function"
    );
}

#[test]
fn lib_rs_has_migration_guide() {
    let source = include_str!("../src/lib.rs");
    assert!(
        source.contains("Migration Guide"),
        "lib.rs must contain a 'Migration Guide' section"
    );
    assert!(
        source.contains("try_skin"),
        "lib.rs migration guide must reference try_skin"
    );
    assert!(
        source.contains("try_sweep_rail"),
        "lib.rs migration guide must reference try_sweep_rail"
    );
    assert!(
        source.contains("try_birail1"),
        "lib.rs migration guide must reference try_birail1"
    );
    assert!(
        source.contains("try_birail2"),
        "lib.rs migration guide must reference try_birail2"
    );
    assert!(
        source.contains("try_gordon"),
        "lib.rs migration guide must reference try_gordon"
    );
    assert!(
        source.contains("try_gordon_from_network"),
        "lib.rs migration guide must reference try_gordon_from_network"
    );
    assert!(
        source.contains("try_gordon_verified"),
        "lib.rs migration guide must reference try_gordon_verified"
    );
}
