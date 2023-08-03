use crate::special::Gamma;

pub struct Chi;

impl Chi {
    /// The Probability Density Function (PDF) for Chi Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Chi_distribution#Probability_density_function" target="_blank">Wikipedia Chi PDF</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::stats::distr::Chi;
    ///
    /// let x = 6_f64;
    /// let df = 5_f64;
    ///
    /// let pdf = Chi::pdf(x, df);
    ///
    /// assert_eq!(pdf, 0.09730437507302006);
    /// ```
    pub fn pdf(x: f64, df: f64) -> f64 {
        if x >= 0_f64 {
            let p1 = x.powf((df - 2_f64) / 2_f64);
            let p2 = std::f64::consts::E.powf(-x / 2_f64);
            let p3 = 2_f64.powf(df / 2_f64);
            let p4 = Gamma::lanczos(df / 2_f64);
            (p1 * p2) / (p3 * p4)
        } else {
            0_f64
        }
    }
    /// The Cumulative Density Function (CDF) Function for Chi Distributions <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Chi-squared_distribution#Cumulative_distribution_function" target="_blank">Wikipedia Chi CDF</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Chi;
    ///
    /// let l = 2_f64;
    /// let u = 5_f64;
    /// let df = 4_f64;
    ///
    /// let cdf = Chi::cdf(l, u, df);
    ///
    /// assert_eq!(cdf, 0.4484613871597444_f64);
    /// ```
    /// <hr/>
    pub fn cdf(l: f64, u: f64, df: f64) -> f64 {
        Gamma::reggamma(df / 2_f64, u / 2_f64) - Gamma::reggamma(df / 2_f64, l / 2_f64)
    }
}
