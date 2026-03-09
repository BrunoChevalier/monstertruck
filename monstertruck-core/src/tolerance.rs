use crate::cgmath64::*;
use cgmath::AbsDiffEq;
use std::fmt::Debug;

/// general tolerance
pub const TOLERANCE: f64 = 1.0e-6;

/// general tolerance of square order
pub const TOLERANCE2: f64 = TOLERANCE * TOLERANCE;

/// Defines a tolerance in the whole package
pub trait Tolerance: AbsDiffEq<Epsilon = f64> + Debug {
    /// The "distance" is less than `TOLERANCE`.
    fn near(&self, other: &Self) -> bool {
        self.abs_diff_eq(other, TOLERANCE)
    }

    /// The "distance" is less than `TOLERANCR2`.
    fn near2(&self, other: &Self) -> bool {
        self.abs_diff_eq(other, TOLERANCE2)
    }
}

impl<T: AbsDiffEq<Epsilon = f64> + Debug> Tolerance for T {}

/// Asserts that `left.near(&right)` (using `Tolerance`).
#[macro_export]
macro_rules! assert_near {
    ($left: expr, $right: expr $(,)?) => {{
        let (left, right) = ($left, $right);
        assert!(
            $crate::tolerance::Tolerance::near(&left, &right),
            "assertion failed: `left` is near `right`\nleft: {left:?},\nright: {right:?}",
        )
    }};
    ($left: expr, $right: expr, $($arg: tt)+) => {{
        let (left, right) = ($left, $right);
        assert!(
            $crate::tolerance::Tolerance::near(&left, &right),
            "assertion failed: `left` is near `right`\nleft: {left:?},\nright: {right:?}: {}",
            format_args!($($arg)+),
        )
    }};
}

/// Similar to `assert_near!`, but returns a test failure instead of panicking if the condition fails.
#[macro_export]
macro_rules! prop_assert_near {
    ($left: expr, $right: expr $(,)?) => {{
        let (left, right) = ($left, $right);
        prop_assert!(
            $crate::tolerance::Tolerance::near(&left, &right),
            "assertion failed: `left` is near `right`\nleft: {left:?},\nright: {right:?}",
        )
    }};
    ($left: expr, $right: expr, $($arg: tt)+) => {{
        let (left, right) = ($left, $right);
        prop_assert!(
            $crate::tolerance::Tolerance::near(&left, &right),
            "assertion failed: `left` is near `right`\nleft: {left:?}, right: {right:?}: {}",
            format_args!($($arg)+),
        )
    }};
}

#[test]
#[should_panic]
fn assert_near_without_msg() {
    assert_near!(1.0, 2.0)
}

#[test]
#[should_panic]
fn assert_near_with_msg() {
    assert_near!(1.0, 2.0, "{}", "test OK")
}

/// Asserts that `left.near2(&right)` (using `Tolerance`).
#[macro_export]
macro_rules! assert_near2 {
    ($left: expr, $right: expr $(,)?) => {{
        let (left, right) = ($left, $right);
        assert!(
            $crate::tolerance::Tolerance::near2(&left, &right),
            "assertion failed: `left` is near `right`\nleft: {left:?},\nright: {right:?}",
        )
    }};
    ($left: expr, $right: expr, $($arg: tt)+) => {{
        let (left, right) = ($left, $right);
        assert!(
            $crate::tolerance::Tolerance::near2(&left, &right),
            "assertion failed: `left` is near `right`\nleft: {left:?},\nright: {right:?}: {}",
            format_args!($($arg)+),
        )
    }};
}

/// Similar to `assert_near2!`, but returns a test failure instead of panicking if the condition fails.
#[macro_export]
macro_rules! prop_assert_near2 {
    ($left: expr, $right: expr $(,)?) => {{
        let (left, right) = ($left, $right);
        prop_assert!(
            $crate::tolerance::Tolerance::near2(&left, &right),
            "assertion failed: `left` is near `right`\nleft: {left:?},\nright: {right:?}",
        )
    }};
    ($left: expr, $right: expr, $($arg: tt)+) => {
        let (left, right) = ($left, $right);
        prop_assert!(
            $crate::tolerance::Tolerance::near2(&left, &right),
            "assertion failed: `left` is near `right`\nleft: {left:?},\nright: {right:?}: {}",
            format_args!($($arg)+),
        )
    };
}

#[test]
#[should_panic]
fn assert_near2_without_msg() {
    assert_near2!(1.0, 2.0)
}

#[test]
#[should_panic]
fn assert_near2_with_msg() {
    assert_near2!(1.0, 2.0, "{}", "test OK")
}

/// The structs defined the origin. `f64`, `Vector`, and so on.
pub trait Origin: Tolerance + Zero {
    /// near origin
    #[inline(always)]
    fn so_small(&self) -> bool {
        self.near(&Self::zero())
    }

    /// near origin in square order
    #[inline(always)]
    fn so_small2(&self) -> bool {
        self.near2(&Self::zero())
    }
}

impl<T: Tolerance + Zero> Origin for T {}

/// Per-operation tolerance tracking to prevent error accumulation across chained operations.
#[derive(Clone, Debug)]
pub struct OperationTolerance {
    /// Base tolerance for individual operations.
    base: f64,
    /// Accumulated error from prior operations in the pipeline.
    accumulated_error: f64,
    /// Number of operations that have contributed to accumulated error.
    operation_count: usize,
    /// Tag of the most recent operation.
    last_operation: Option<&'static str>,
}

impl OperationTolerance {
    /// Creates a new tolerance tracker with the given base tolerance and zero accumulated error.
    pub fn new(base: f64) -> Self {
        Self {
            base,
            accumulated_error: 0.0,
            operation_count: 0,
            last_operation: None,
        }
    }

    /// Creates a new tolerance tracker from the global [`TOLERANCE`] constant.
    pub fn from_global() -> Self {
        Self::new(TOLERANCE)
    }

    /// Returns a new [`OperationTolerance`] with accumulated error increased by `local_error`
    /// and operation count incremented.
    pub fn after_operation(&self, operation: &'static str, local_error: f64) -> Self {
        Self {
            base: self.base,
            accumulated_error: self.accumulated_error + local_error,
            operation_count: self.operation_count + 1,
            last_operation: Some(operation),
        }
    }

    /// Returns the effective tolerance: `base + accumulated_error`.
    pub fn effective_tolerance(&self) -> f64 {
        self.base + self.accumulated_error
    }

    /// Returns `true` when `accumulated_error` is less than `base`.
    pub fn within_budget(&self) -> bool {
        self.accumulated_error < self.base
    }

    /// Returns the base tolerance.
    pub fn base(&self) -> f64 {
        self.base
    }

    /// Returns the accumulated error from prior operations.
    pub fn accumulated_error(&self) -> f64 {
        self.accumulated_error
    }

    /// Returns the number of operations that have contributed to accumulated error.
    pub fn operation_count(&self) -> usize {
        self.operation_count
    }

    /// Returns the tag of the most recent operation, if any.
    pub fn last_operation(&self) -> Option<&'static str> {
        self.last_operation
    }
}
