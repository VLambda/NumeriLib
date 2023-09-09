use crate::Functions;

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
    /// use mathematica::special::Error;
    ///
    /// let bound = 4.0_f64;
    /// let erf = Error::erf(bound);
    ///
    /// println!("Error Function at {} is: {}", bound, erf);
    /// ```
    /// <hr/>
    pub fn erf(z: f64) -> f64 {
        let mut result = 0_f64;

        for i in 0..99 {
            result += std::f64::consts::FRAC_2_SQRT_PI
                * (((-1_f64).powf(i as f64) * z.powf(2_f64 * i as f64 + 1_f64))
                    / ((2_f64 * i as f64 + 1_f64) * Functions::factorial(i as f64)))
        }

        result
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
    /// use mathematica::special::Error;
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
    /// use mathematica::special::Error;
    ///
    /// let x = 0.975;
    /// let inverf = Error::inverf(x);
    ///
    /// println!("Inverse Error Function at {} is: {}", x, inverf);
    /// ```
    /// <hr/>
    pub fn inverf(x: f64) -> f64 {
        let w = -((1.0 - x) * (1.0 + x)).ln();
        let mut p;

        if w < 5.0 {
            let w_minus_2_5 = w - 2.5;
            p = 2.81022636e-08;
            p = 3.43273939e-07 + p * w_minus_2_5;
            p = -3.5233877e-06 + p * w_minus_2_5;
            p = -4.39150654e-06 + p * w_minus_2_5;
            p = 0.00021858087 + p * w_minus_2_5;
            p = -0.00125372503 + p * w_minus_2_5;
            p = -0.00417768164 + p * w_minus_2_5;
            p = 0.246640727 + p * w_minus_2_5;
            p = 1.50140941 + p * w_minus_2_5;
        } else {
            let sqrt_w_minus_3 = (w).sqrt() - 3.0;
            p = -0.000200214257;
            p = 0.000100950558 + p * sqrt_w_minus_3;
            p = 0.00134934322 + p * sqrt_w_minus_3;
            p = -0.00367342844 + p * sqrt_w_minus_3;
            p = 0.00573950773 + p * sqrt_w_minus_3;
            p = -0.0076224613 + p * sqrt_w_minus_3;
            p = 0.00943887047 + p * sqrt_w_minus_3;
            p = 1.00167406 + p * sqrt_w_minus_3;
            p = 2.83297682 + p * sqrt_w_minus_3;
        }

        let res_ra = p * x;

        let fx = Self::erf(res_ra) - x;
        let df = (2.0 / (std::f64::consts::PI).sqrt()) * (-res_ra * res_ra).exp();
        let d2f = -2.0 * res_ra * df;

        res_ra - (2.0 * fx * df) / ((2.0 * df * df) - (fx * d2f))
    }
}
