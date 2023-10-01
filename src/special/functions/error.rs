use crate::special::Probability;
use crate::Functions;
use std::f64::consts::FRAC_2_SQRT_PI;

/// Provides methods for calculating error functions and their inverses.
pub struct Error;

impl Error {
    /// Calculates the Error Function (erf) using the Maclaurin Series representation.
    ///
    /// The Error Function, erf(z), is defined as:
    /// erf(z) = (2 / sqrt(pi)) * ∫[0 to z] exp(-t^2) dt
    ///
    /// # Parameters
    ///
    /// - `z`: The value at which to calculate the Error Function.
    ///
    /// # Returns
    ///
    /// The value of the Error Function at the given `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Error;
    ///
    /// let bound = 4.0_f64;
    /// let erf = Error::erf(bound);
    ///
    /// println!("Error Function at {} is: {}", bound, erf);
    /// ```
    /// <hr/>
    pub fn erf(z: f64) -> f64 {
        let func = |x: f64| {
            ((-1_f64).powf(x) * z.powf(2_f64 * x + 1_f64))
                / ((2_f64 * x + 1_f64) * Probability::factorial(x))
        };

        let result = Functions::summation(0_f64, 99_f64, func);

        FRAC_2_SQRT_PI * result
    }

    /// Calculates the Complementary Error Function (erfc).
    ///
    /// The Complementary Error Function, erfc(z), is defined as:
    /// erfc(z) = 1 - erf(z)
    ///
    /// # Parameters
    ///
    /// - `z`: The value at which to calculate the Complementary Error Function.
    ///
    /// # Returns
    ///
    /// The value of the Complementary Error Function at the given `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Error;
    ///
    /// let z = 4.0_f64;
    /// let erfc = Error::erfc(z);
    ///
    /// println!("Complementary Error Function at {} is: {}", z, erfc);
    /// ```
    /// <hr/>
    pub fn erfc(z: f64) -> f64 {
        1_f64 - Error::erf(z)
    }

    /// Calculates the Inverse Error Function (inverf).
    ///
    /// The Inverse Error Function, inverf(x), returns the value `z` such that `erf(z) = x`.
    ///
    /// # Parameters
    ///
    /// - `x`: The value for which to calculate the Inverse Error Function.
    ///
    /// # Returns
    ///
    /// The value `z` such that `erf(z) = x`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Error;
    ///
    /// let x = 0.975;
    /// let inverf = Error::inverf(x);
    ///
    /// println!("Inverse Error Function at {} is: {}", x, inverf);
    /// ```
    /// <hr/>
    pub fn inverf(x: f64) -> f64 {
        let target = x;

        let func = |t: f64| Self::erf(t) - target;

        let guess = 0_f64;

        Functions::newmet(guess, func)
    }
}
