//! The mathematical constant tau.

extern crate core;

/// The mathematical constant [tau](http://tauday.com/),
/// where pi is defined as tau/2.
///
/// We provide it as `f64` for now.
///
/// ```rust{.example}
/// assert!(tau::TAU / 2.0 == core::f64::consts::PI);
/// ```
#[unstable = "still waiting for associated constants"]
pub const TAU: f64 = core::f64::consts::PI_2;
