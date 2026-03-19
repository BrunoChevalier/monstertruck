//! Criterion benchmarks for the font profile pipeline.
//!
//! Measures throughput for single glyph profiling, multi-character text
//! profiling at various lengths (1, 10, 100, 1000 chars), stress corpus
//! pathological geometry, and the full glyph-to-solid pipeline.
//!
//! Run with:
//! ```bash
//! cargo bench -p monstertruck-modeling --features font
//! ```

use criterion::{Criterion, black_box, criterion_group, criterion_main, measurement::WallTime};
use monstertruck_modeling::*;
use std::time::Duration;

#[path = "../test-fixtures/stress-corpus/mod.rs"]
mod stress_corpus;

/// Bundled DejaVu Sans font fixture.
const FONT_BYTES: &[u8] = include_bytes!("../test-fixtures/DejaVuSans.ttf");

fn face() -> ttf_parser::Face<'static> {
    ttf_parser::Face::parse(FONT_BYTES, 0).expect("valid TTF")
}

fn default_opts() -> text::TextOptions {
    text::TextOptions::default()
}

/// Generates a string of approximately `target_len` characters by repeating a
/// base phrase.
fn make_text(target_len: usize) -> String {
    let base = "The quick brown fox jumps over the lazy dog. ";
    base.chars().cycle().take(target_len).collect()
}

// ---------------------------------------------------------------------------
// Single glyph benchmarks
// ---------------------------------------------------------------------------

fn bench_glyph_profile(c: &mut Criterion) {
    let f = face();
    let opts = default_opts();

    let mut group = c.benchmark_group("glyph_profile");

    // Simple glyph: 'l' (1 contour, no holes).
    let glyph_l = f.glyph_index('l').expect("glyph for 'l'");
    group.bench_function("simple_l", |b| {
        b.iter(|| black_box(text::glyph_profile(&f, black_box(glyph_l), &opts)))
    });

    // Complex glyph: 'B' (3 contours with holes).
    let glyph_b = f.glyph_index('B').expect("glyph for 'B'");
    group.bench_function("complex_B", |b| {
        b.iter(|| black_box(text::glyph_profile(&f, black_box(glyph_b), &opts)))
    });

    // Complex glyph: '@' (nested contours).
    let glyph_at = f.glyph_index('@').expect("glyph for '@'");
    group.bench_function("complex_at", |b| {
        b.iter(|| black_box(text::glyph_profile(&f, black_box(glyph_at), &opts)))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Text string benchmarks (FONT-04 core requirement)
// ---------------------------------------------------------------------------

fn bench_text_profile(c: &mut Criterion) {
    let f = face();
    let opts = default_opts();

    let mut group = c.benchmark_group("text_profile");

    // 1 character.
    let text_1 = "H";
    group.bench_function("1_char", |b| {
        b.iter(|| black_box(text::text_profile(&f, black_box(text_1), &opts)))
    });

    // 10 characters.
    let text_10 = "HelloWorld";
    group.bench_function("10_chars", |b| {
        b.iter(|| black_box(text::text_profile(&f, black_box(text_10), &opts)))
    });

    // 100 characters.
    let text_100 = make_text(100);
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("100_chars", |b| {
        b.iter(|| black_box(text::text_profile(&f, black_box(text_100.as_str()), &opts)))
    });

    // 1000 characters.
    let text_1000 = make_text(1000);
    group.sample_size(50);
    group.bench_function("1000_chars", |b| {
        b.iter(|| black_box(text::text_profile(&f, black_box(text_1000.as_str()), &opts)))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Stress corpus benchmarks
// ---------------------------------------------------------------------------

fn bench_stress_corpus(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress_corpus");
    group.measurement_time(Duration::from_secs(10));

    let fixtures = stress_corpus::all_fixtures();

    for (name, wires) in &fixtures {
        // Only benchmark fixtures that produce valid wires for normalization.
        let test_wires = wires.clone();
        if profile::attach_plane_normalized::<Curve, Surface>(test_wires).is_ok() {
            let bench_name = *name;
            group.bench_function(bench_name, |b| {
                b.iter(|| {
                    let w = wires.clone();
                    black_box(profile::attach_plane_normalized::<Curve, Surface>(w))
                })
            });
        }
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Full pipeline benchmarks
// ---------------------------------------------------------------------------

fn bench_full_pipeline(c: &mut Criterion) {
    let f = face();
    let opts = default_opts();

    let mut group = c.benchmark_group("full_pipeline");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    // Glyph 'O' -> solid.
    let glyph_o = f.glyph_index('O').expect("glyph for 'O'");
    group.bench_function(
        "glyph_to_solid",
        |b: &mut criterion::Bencher<'_, WallTime>| {
            b.iter(|| {
                let wires =
                    text::glyph_profile(&f, black_box(glyph_o), &opts).expect("glyph_profile");
                black_box(profile::solid_from_planar_profile::<Curve, Surface>(
                    wires,
                    Vector3::new(0.0, 0.0, 1.0),
                ))
            })
        },
    );

    // 100-char text -> wires.
    let text_100 = make_text(100);
    group.bench_function(
        "text_to_wires_100",
        |b: &mut criterion::Bencher<'_, WallTime>| {
            b.iter(|| black_box(text::text_profile(&f, black_box(text_100.as_str()), &opts)))
        },
    );

    group.finish();
}

criterion_group!(glyph_benches, bench_glyph_profile);
criterion_group!(text_benches, bench_text_profile);
criterion_group!(stress_benches, bench_stress_corpus);
criterion_group!(pipeline_benches, bench_full_pipeline);
criterion_main!(
    glyph_benches,
    text_benches,
    stress_benches,
    pipeline_benches
);
