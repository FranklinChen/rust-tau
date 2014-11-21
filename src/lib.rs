//! The mathematical constant tau.

use std::num::Float;

/// The mathematical constant [tau](http://tauday.com/),
/// where pi is defined as tau/2.
///
/// ```rust{.example}
/// use std::num::Float;
/// let tau64: f64 = tau::tau();
/// assert!(tau64/2.0f64 == Float::pi())
/// ```
#[inline]
pub fn tau<T: Float>() -> T {
    Float::two_pi()
}
