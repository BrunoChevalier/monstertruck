//! Implementation of Newton method.

use std::ops::{Mul, Sub};

use crate::{cgmath64::*, tolerance::*};

/// The value and Jacobian corresponding to a parameter.
#[derive(Clone, Debug)]
pub struct CalcOutput<V, M> {
    /// The value of the function.
    pub value: V,
    /// The Jacobian of the function.
    pub derivation: M,
}

/// Jacobian of function.
pub trait Jacobian<V>: Mul<V, Output = V> + Sized {
    /// Inverts the Jacobian matrix, returning `None` if singular.
    fn invert(self) -> Option<Self>;
    /// Inverts the Jacobian with Levenberg-Marquardt damping applied to the diagonal.
    fn invert_damped(jacobian: Self, lambda: f64) -> Option<Self>;
}

impl Jacobian<f64> for f64 {
    #[inline(always)]
    fn invert(self) -> Option<Self> {
        match self.is_zero() {
            true => None,
            false => Some(1.0 / self),
        }
    }

    #[inline(always)]
    fn invert_damped(jacobian: Self, lambda: f64) -> Option<Self> {
        let damped = jacobian + lambda * jacobian.signum().max(1.0);
        match damped.abs() < f64::EPSILON {
            true => None,
            false => Some(1.0 / damped),
        }
    }
}

macro_rules! impl_jacobian {
    ($matrix: ty, $vector: ty) => {
        impl Jacobian<$vector> for $matrix {
            #[inline(always)]
            fn invert(self) -> Option<Self> {
                SquareMatrix::invert(&self)
            }

            #[inline(always)]
            fn invert_damped(jacobian: Self, lambda: f64) -> Option<Self> {
                let damped = jacobian + <$matrix>::from_value(lambda);
                SquareMatrix::invert(&damped)
            }
        }
    };
}

impl_jacobian!(Matrix2, Vector2);
impl_jacobian!(Matrix3, Vector3);
impl_jacobian!(Matrix4, Vector4);

/// Solve equation by Newton's method.
/// # Examples
/// ```
/// # fn main() -> anyhow::Result<()> {
/// use monstertruck_core::{newton::*, assert_near2};
///
/// let function = |x: f64| CalcOutput {
///     value: x * x - 2.0,
///     derivation: 2.0 * x,
/// };
/// let sqrt2 = solve(function, 1.0, 10).map_err(|e| anyhow::anyhow!("{e}"))?;
/// assert_near2!(sqrt2, f64::sqrt(2.0));
/// # Ok(())
/// # }
/// ```
pub fn solve<V, M>(
    function: impl Fn(V) -> CalcOutput<V, M>,
    mut hint: V,
    trials: usize,
) -> Result<V, NewtonLog<V>>
where
    V: Sub<Output = V> + Copy + Tolerance,
    M: Jacobian<V>,
{
    let mut log = NewtonLog::new(cfg!(debug_assertions), trials);
    for _ in 0..=trials {
        log.push(hint);
        let CalcOutput { value, derivation } = function(hint);
        let Some(inv) = derivation.invert() else {
            log.set_degenerate(true);
            return Err(log);
        };
        let next = hint - inv * value;
        if next.near(&hint) {
            return Ok(next);
        }
        hint = next;
    }
    Err(log)
}

/// Result of a robust Newton solve including fallback metadata.
#[derive(Clone, Debug)]
pub struct SolveResult<V> {
    /// The converged solution.
    pub value: V,
    /// Whether a fallback method was used.
    pub used_fallback: bool,
}

/// Solve equation robustly with Levenberg-Marquardt fallback for near-singular Jacobians.
///
/// First tries standard Newton (same as [`solve`]). On failure due to degenerate Jacobian,
/// retries with Levenberg-Marquardt damping.
pub fn solve_robust<V, M>(
    function: impl Fn(V) -> CalcOutput<V, M>,
    hint: V,
    trials: usize,
) -> Result<SolveResult<V>, NewtonLog<V>>
where
    V: Sub<Output = V> + Copy + Tolerance,
    M: Jacobian<V> + Copy,
{
    // Try standard Newton first.
    match solve(&function, hint, trials) {
        Ok(value) => Ok(SolveResult {
            value,
            used_fallback: false,
        }),
        Err(_) => {
            // Fall back to Levenberg-Marquardt damped Newton.
            levenberg_marquardt(&function, hint, trials)
        }
    }
}

/// Levenberg-Marquardt damped Newton solver.
///
/// Adds a damping factor to the Jacobian diagonal when inversion fails,
/// adapting the damping based on residual improvement.
fn levenberg_marquardt<V, M>(
    function: &impl Fn(V) -> CalcOutput<V, M>,
    mut hint: V,
    trials: usize,
) -> Result<SolveResult<V>, NewtonLog<V>>
where
    V: Sub<Output = V> + Copy + Tolerance,
    M: Jacobian<V> + Copy,
{
    let mut log = NewtonLog::new(cfg!(debug_assertions), trials);
    let mut lambda = 1e-4_f64;

    for _ in 0..=trials {
        log.push(hint);
        let CalcOutput { value, derivation } = function(hint);

        // Try undamped inversion first, fall back to damped inversion.
        let next = match derivation.invert() {
            Some(inv) => hint - inv * value,
            None => match M::invert_damped(derivation, lambda) {
                Some(inv) => {
                    let candidate = hint - inv * value;
                    lambda *= 0.5;
                    candidate
                }
                None => {
                    lambda *= 10.0;
                    continue;
                }
            },
        };

        if next.near(&hint) {
            log.set_used_fallback(true);
            return Ok(SolveResult {
                value: next,
                used_fallback: true,
            });
        }
        hint = next;
    }
    log.set_degenerate(true);
    log.set_used_fallback(true);
    Err(log)
}

/// Solve a 1D equation robustly with Newton, Levenberg-Marquardt, and bisection fallbacks.
///
/// Tries Newton first, then Levenberg-Marquardt with damping, then bisection
/// if a sign-changing bracket can be found near the hint.
pub fn solve_robust_1d(
    function: impl Fn(f64) -> CalcOutput<f64, f64>,
    hint: f64,
    trials: usize,
) -> Result<SolveResult<f64>, NewtonLog<f64>> {
    // Try standard Newton first.
    match solve(&function, hint, trials) {
        Ok(value) => Ok(SolveResult {
            value,
            used_fallback: false,
        }),
        Err(_) => {
            // Try Levenberg-Marquardt.
            match levenberg_marquardt_1d(&function, hint, trials) {
                Ok(result) => Ok(result),
                Err(_) => {
                    // Try bisection as last resort.
                    bisection_fallback_1d(&function, hint, trials)
                }
            }
        }
    }
}

/// Levenberg-Marquardt damped Newton for 1D scalar case.
fn levenberg_marquardt_1d(
    function: &impl Fn(f64) -> CalcOutput<f64, f64>,
    mut hint: f64,
    trials: usize,
) -> Result<SolveResult<f64>, NewtonLog<f64>> {
    let mut log = NewtonLog::new(cfg!(debug_assertions), trials);
    let mut lambda = 1e-4_f64;

    for _ in 0..=trials {
        log.push(hint);
        let CalcOutput { value, derivation } = function(hint);
        let damped = derivation + lambda * derivation.signum().max(1.0);
        if damped.abs() < f64::EPSILON {
            lambda *= 10.0;
            continue;
        }
        let inv = 1.0 / damped;
        let next = hint - inv * value;

        // Adapt damping based on residual.
        let next_residual = function(next).value.abs();
        let curr_residual = value.abs();
        if next_residual < curr_residual {
            lambda *= 0.5;
        } else {
            lambda *= 10.0;
        }

        if next.near(&hint) {
            return Ok(SolveResult {
                value: next,
                used_fallback: true,
            });
        }
        hint = next;
    }
    log.set_degenerate(true);
    Err(log)
}

/// Bisection fallback for 1D case.
///
/// Searches for a sign-changing bracket near the hint, then applies standard bisection.
fn bisection_fallback_1d(
    function: &impl Fn(f64) -> CalcOutput<f64, f64>,
    hint: f64,
    trials: usize,
) -> Result<SolveResult<f64>, NewtonLog<f64>> {
    let mut log = NewtonLog::new(cfg!(debug_assertions), trials);

    // Search for a bracket where f changes sign.
    let bracket = (1..=20)
        .map(|i| {
            let delta = 0.1 * (1.5_f64).powi(i);
            (hint - delta, hint + delta)
        })
        .find(|&(a, b)| {
            let fa = function(a).value;
            let fb = function(b).value;
            fa.is_finite() && fb.is_finite() && fa * fb < 0.0
        });

    let (mut a, mut b) = match bracket {
        Some(pair) => pair,
        None => {
            log.set_degenerate(true);
            return Err(log);
        }
    };

    // Standard bisection.
    for _ in 0..=trials {
        let mid = 0.5 * (a + b);
        log.push(mid);
        let f_mid = function(mid).value;
        if f_mid.abs() < TOLERANCE || (b - a).abs() < TOLERANCE {
            return Ok(SolveResult {
                value: mid,
                used_fallback: true,
            });
        }
        let f_a = function(a).value;
        if f_a * f_mid < 0.0 {
            b = mid;
        } else {
            a = mid;
        }
    }
    log.set_degenerate(true);
    Err(log)
}

mod newtonlog {
    use std::fmt::*;
    /// A structure that stores logs for debugging.
    #[derive(Clone, Debug)]
    pub struct NewtonLog<T> {
        log: Option<Vec<T>>,
        degenerate: bool,
        used_fallback: bool,
    }

    impl<T> NewtonLog<T> {
        /// Creates a new [`NewtonLog`].
        #[inline(always)]
        pub fn new(activate: bool, trials: usize) -> Self {
            match activate {
                true => NewtonLog {
                    log: Some(Vec::with_capacity(trials)),
                    degenerate: false,
                    used_fallback: false,
                },
                false => NewtonLog {
                    log: None,
                    degenerate: false,
                    used_fallback: false,
                },
            }
        }
        /// Returns `true` iff the Newton method terminates due to Jacobian degeneracy.
        #[inline(always)]
        pub fn degenerate(&self) -> bool {
            self.degenerate
        }
        /// Returns `true` if a fallback method was used during solving.
        #[inline(always)]
        pub fn used_fallback(&self) -> bool {
            self.used_fallback
        }
        #[inline(always)]
        pub(super) fn push(&mut self, log: T) {
            if let Some(vec) = &mut self.log {
                vec.push(log)
            }
        }
        #[inline(always)]
        pub(super) fn set_degenerate(&mut self, degenerate: bool) {
            self.degenerate = degenerate
        }
        /// Sets whether a fallback method was used.
        #[inline(always)]
        pub(super) fn set_used_fallback(&mut self, used_fallback: bool) {
            self.used_fallback = used_fallback
        }
    }

    impl<T: Debug> Display for NewtonLog<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self.degenerate {
                true => f.pad("Jacobian is dengenerate. ")?,
                false => f.pad("Newton method is not converges. ")?,
            }
            match &self.log {
                None => f.pad(
                    "If you want to see the Newton log, please re-run it with the debug build.",
                ),
                Some(vec) => {
                    f.pad("Newton Log:\n")?;
                    vec.iter()
                        .try_for_each(|log| f.write_fmt(format_args!("{log:?}\n")))
                }
            }
        }
    }
}
pub use newtonlog::NewtonLog;
