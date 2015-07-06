//! The mathematical constant tau.

// Temporarily commented out because unstable.
//use std::f64::consts::PI_2;

/// The mathematical constant [tau](http://tauday.com/),
/// where pi is defined as tau/2.
///
/// We provide it as `f64` for now.
///
/// ```rust{.example}
/// use tau::TAU;
/// assert_eq!(TAU.cos(), 1.0);
/// ```
pub const TAU: f64 = 6.28318530717958647692528676655900576f64;
// Temporarily copied from https://doc.rust-lang.org/stable/std/f64/consts/constant.PI_2.html because unsable.
//pub const TAU: f64 = PI_2;

#[cfg(test)]
mod test {
    use super::TAU;
    use std::f64::consts::PI;

    #[test]
    fn pi_is_a_derived_constant() {
        assert_eq!(PI, TAU / 2.0);
    }
}
