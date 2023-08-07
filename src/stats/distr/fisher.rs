use crate::special::Beta;

pub struct Fisher;

impl Fisher {
    /// The Probability Density Function (PDF) of the Fisher–Snedecor distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/F-distribution" target="_blank">Wikipedia F-Distributions</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Fisher;
    ///
    /// let x = 7_f64;
    /// let d1 = 6_f64;
    /// let d2 = 5_f64;
    ///
    /// let pdf = Fisher::pdf(x, d1, d2);
    ///
    /// assert_eq!(pdf, 0.007408447276742485_f64);
    /// ```
    /// <hr/>
    pub fn pdf(x: f64, d1: f64, d2: f64) -> f64 {
        let p1 = (d1 * x).powf(d1) * d2.powf(d2);
        let p2 = (d1 * x + d2).powf(d1 + d2);
        let p3 = p1 / p2;
        let p4 = p3.sqrt();
        let p5 = x * Beta::beta(d1 / 2_f64, d2 / 2_f64);
        p4 / p5
    }
    /// Calculates the Cumulative Density Function (CDF) of the Fisher–Snedecor distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/F-distribution" target="_blank">Wikipedia F-Distributions</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Fisher;
    ///
    /// let bound = 8_f64;
    /// let d1 = 4_f64;
    /// let d2 = 8_f64;
    ///
    /// let cdf = Fisher::cdf(bound, d1, d2);
    ///
    /// assert_eq!(cdf, 0.9932797799053232_f64);
    /// ```
    pub fn cdf(bound: f64, d1: f64, d2: f64) -> f64 {
        let limit = (d1 * bound) / (d1 * bound + d2);
        Beta::regincbeta(d1 / 2_f64, d2 / 2_f64, limit)
    }
    /// Calculates a 2 Tailed CDF for the Fisher–Snedecor distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/F-distribution" target="_blank">Wikipedia F-Distributions</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Fisher;
    ///
    /// let lower = 8_f64;
    /// let upper = 9_f64;
    /// let d1 = 4_f64;
    /// let d2 = 8_f64;
    ///
    /// let cdf = Fisher::tailcdf(lower, upper, d1, d2);
    ///
    /// assert_eq!(cdf, 0.002049230226804699_f64);
    pub fn tailcdf(lower: f64, upper: f64, d1: f64, d2: f64) -> f64 {
        Self::cdf(upper, d1, d2) - Self::cdf(lower, d1, d2)
    }

    pub fn mean(d2: f64) -> f64 {
        if d2 < 2 {
            f64::NAN
        }
        d2 / (d2 - 2_f64)
    }
}
