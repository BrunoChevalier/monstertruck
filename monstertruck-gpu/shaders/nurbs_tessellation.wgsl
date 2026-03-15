// NURBS Surface Tessellation Compute Shader
//
// Evaluates a NURBS surface at a grid of (u,v) parameter points, producing
// tessellated vertices and surface normals.
//
// **Degree constraint**: WGSL does not support dynamic array sizes in functions.
// `MAX_DEGREE` defines the maximum supported NURBS degree. All basis function
// arrays are declared as `array<f32, 9>` (i.e. MAX_DEGREE + 1). Surfaces with
// degree > MAX_DEGREE will produce incorrect results. The host code MUST
// validate `degree <= MAX_DEGREE` before dispatch.

// Override constant -- the host can specialise this at pipeline creation time.
override MAX_DEGREE: u32 = 8u;

// Parameters passed from host code.
struct ControlParams {
    degree_u: u32,
    degree_v: u32,
    num_cp_u: u32,
    num_cp_v: u32,
    num_knots_u: u32,
    num_knots_v: u32,
    grid_u: u32,
    grid_v: u32,
    tolerance: f32,
    // Padding to align to 16 bytes.
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

// --- Bind group 0: inputs ---

@group(0) @binding(0) var<storage, read> control_points: array<vec4<f32>>;
@group(0) @binding(1) var<storage, read> knots_u: array<f32>;
@group(0) @binding(2) var<storage, read> knots_v: array<f32>;
@group(0) @binding(3) var<uniform> params: ControlParams;

// --- Bind group 1: outputs ---

@group(1) @binding(0) var<storage, read_write> output_vertices: array<vec4<f32>>;
@group(1) @binding(1) var<storage, read_write> output_normals: array<vec4<f32>>;

// Find the knot span index for parameter `t` using binary search.
// `n` is the number of basis functions minus one (num_cp - 1).
fn find_span(n: u32, degree: u32, t: f32, knot_offset: u32, is_u: bool) -> u32 {
    // Load a knot value from the appropriate knot vector.
    var knot_val: f32;

    // Clamp to valid range.
    var tt = t;
    var lo: u32 = degree;
    var hi: u32 = n + 1u;

    // Load boundary knots.
    var knot_lo: f32;
    var knot_hi: f32;
    if is_u {
        knot_lo = knots_u[lo];
        knot_hi = knots_u[hi];
    } else {
        knot_lo = knots_v[lo];
        knot_hi = knots_v[hi];
    }

    if tt >= knot_hi {
        return n;
    }
    if tt <= knot_lo {
        return degree;
    }

    // Binary search.
    var mid: u32;
    for (var iter: u32 = 0u; iter < 64u; iter++) {
        if hi - lo <= 1u {
            break;
        }
        mid = (lo + hi) / 2u;
        if is_u {
            knot_val = knots_u[mid];
        } else {
            knot_val = knots_v[mid];
        }
        if tt < knot_val {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    return lo;
}

// Compute the non-vanishing B-spline basis functions at parameter `t`.
// Returns an array of size 9 (MAX_DEGREE + 1). Only indices 0..degree are
// valid; remaining entries are zero.
fn basis_funs(span: u32, degree: u32, t: f32, is_u: bool) -> array<f32, 9> {
    var N: array<f32, 9>;
    var left: array<f32, 9>;
    var right: array<f32, 9>;

    // Initialise to zero.
    for (var z: u32 = 0u; z < 9u; z++) {
        N[z] = 0.0;
        left[z] = 0.0;
        right[z] = 0.0;
    }

    N[0] = 1.0;

    for (var j: u32 = 1u; j <= degree; j++) {
        // Load knot values.
        var knot_span_plus_j: f32;
        var knot_span_minus_j_plus_1: f32;
        if is_u {
            knot_span_plus_j = knots_u[span + j];
            knot_span_minus_j_plus_1 = knots_u[span + 1u - j];
        } else {
            knot_span_plus_j = knots_v[span + j];
            knot_span_minus_j_plus_1 = knots_v[span + 1u - j];
        }
        left[j] = t - knot_span_minus_j_plus_1;
        right[j] = knot_span_plus_j - t;

        var saved: f32 = 0.0;
        for (var r: u32 = 0u; r < j; r++) {
            let temp = N[r] / (right[r + 1u] + left[j - r]);
            N[r] = saved + right[r + 1u] * temp;
            saved = left[j - r] * temp;
        }
        N[j] = saved;
    }

    return N;
}

// Evaluate the NURBS surface point at parameters (u, v).
// Returns a `vec4<f32>` where xyz is the Cartesian point and w is 1.0.
fn surface_point(u: f32, v: f32) -> vec4<f32> {
    let degree_u = params.degree_u;
    let degree_v = params.degree_v;
    let n_u = params.num_cp_u - 1u;
    let n_v = params.num_cp_v - 1u;

    let span_u = find_span(n_u, degree_u, u, 0u, true);
    let span_v = find_span(n_v, degree_v, v, 0u, false);

    let Nu = basis_funs(span_u, degree_u, u, true);
    let Nv = basis_funs(span_v, degree_v, v, false);

    var point = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    for (var l: u32 = 0u; l <= degree_v; l++) {
        var temp = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        let row = span_v - degree_v + l;
        for (var k: u32 = 0u; k <= degree_u; k++) {
            let col = span_u - degree_u + k;
            let idx = row * params.num_cp_u + col;
            let cp = control_points[idx];
            temp += Nu[k] * cp;
        }
        point += Nv[l] * temp;
    }

    // Rational projection: divide by weight.
    let w = point.w;
    if abs(w) > 1e-10 {
        return vec4<f32>(point.xyz / w, 1.0);
    }
    return vec4<f32>(point.xyz, 1.0);
}

// Compute the surface normal at (u, v) via finite differences.
fn surface_normal(u: f32, v: f32) -> vec4<f32> {
    let eps = 1e-5;

    // Compute partial derivative with respect to u.
    let u_lo = max(u - eps, 0.0);
    let u_hi = min(u + eps, 1.0);
    let du = u_hi - u_lo;
    let pu_lo = surface_point(u_lo, v);
    let pu_hi = surface_point(u_hi, v);
    let dPdu = (pu_hi.xyz - pu_lo.xyz) / du;

    // Compute partial derivative with respect to v.
    let v_lo = max(v - eps, 0.0);
    let v_hi = min(v + eps, 1.0);
    let dv = v_hi - v_lo;
    let pv_lo = surface_point(u, v_lo);
    let pv_hi = surface_point(u, v_hi);
    let dPdv = (pv_hi.xyz - pv_lo.xyz) / dv;

    // Normal is the cross product of the partial derivatives.
    var normal = cross(dPdu, dPdv);
    let len = length(normal);
    if len > 1e-10 {
        normal = normal / len;
    }
    return vec4<f32>(normal, 0.0);
}

// Main compute entry point. Each thread evaluates one (u,v) parameter pair.
@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let iu = global_id.x;
    let iv = global_id.y;

    if iu >= params.grid_u || iv >= params.grid_v {
        return;
    }

    // Map grid indices to parameter values in [0, 1].
    var u: f32;
    var v: f32;
    if params.grid_u > 1u {
        u = f32(iu) / f32(params.grid_u - 1u);
    } else {
        u = 0.0;
    }
    if params.grid_v > 1u {
        v = f32(iv) / f32(params.grid_v - 1u);
    } else {
        v = 0.0;
    }

    let idx = iv * params.grid_u + iu;
    output_vertices[idx] = surface_point(u, v);
    output_normals[idx] = surface_normal(u, v);
}
