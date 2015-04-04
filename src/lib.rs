//! The mathematical constant tau.

#![feature(core)]

extern crate core;

/// The mathematical constant [tau](http://tauday.com/),
/// where pi is defined as tau/2.
///
/// We provide it as `f64` for now.
///
/// ```rust{.example}
/// use tau::TAU;
/// assert_eq!(TAU.cos(), 1.0);
/// ```
#[unstable = "still waiting for associated constants"]
pub const TAU: f64 = core::f64::consts::PI_2;

#[cfg(test)]
mod test {
    extern crate core;
    use super::TAU;

    #[test]
    fn pi_is_a_derived_constant() {
        assert_eq!(core::f64::consts::PI, TAU / 2.0);
    }
}
