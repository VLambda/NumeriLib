use crate::special::Functions;

pub struct Error;

impl Error {
    /// Uses the Maclaurin Series representation of the Error Function <br>
    /// Learn more about the Error Function at: <a href="https://wikipedia.org/wiki/Error_function" target="_blank">Wikipedia Error Function</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::special::Error;
    ///
    /// let bound = 4_f64;
    /// let erf = Error::erf(bound);
    ///
    /// assert_eq!(erf, 0.9999999845946841);
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
    /// The Definition of the Complementary Error Function <br>
    /// Learn moare at: <a href="https://wikipedia.org/wiki/Error_function#Complementary_error_function" target="_blank">Wikipedia Complementary Error Function</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::special::Error;
    ///
    /// let z = 4_f64;
    ///
    /// let erfc = Error::erfc(z);
    ///
    /// assert_eq!(erfc, 0.000000015405315911820594_f64);
    /// ```
    /// <hr/>
    pub fn erfc(z: f64) -> f64 {
        1_f64 - Error::erf(z)
    }
    /// A Rust implementation of the Inverse Error Function <br>
    /// Uses this approximation by Alijah Ahmed at: <a href="https://scistatcalc.blogspot.com/2013/09/numerical-estimate-of-inverse-error.html" target="_blank">Inverse Error Function Approximation</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use ferrate::special::Error;
    ///
    /// let x = 0.975;
    /// let inverf = Error::inverf(x);
    ///
    /// assert_eq!(inverf, 1.5849110680594818);
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
