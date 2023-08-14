use crate::special::Gamma;

pub struct Chi2;

impl Chi2 {
    /// The Probability Density Function (PDF) for Chi Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Chi_distribution#Probability_density_function" target="_blank">Wikipedia Chi PDF</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::stats::distr::Chi2;
    ///
    /// let x = 6_f64;
    /// let df = 5_f64;
    ///
    /// let pdf = Chi2::pdf(x, df);
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
    /// use ferrate::stats::distr::Chi2;
    ///
    /// let bound = 2_f64;
    /// let df = 4_f64;
    ///
    /// let cdf = Chi2::cdf(bound, df);
    ///
    /// assert_eq!(cdf, 0.2642411176571154_f64);
    /// ```
    /// <hr/>
    pub fn cdf(bound: f64, df: f64) -> f64 {
        Gamma::reggamma(df / 2_f64, bound / 2_f64)
    }
    /// The 2 Tailed Cumulative Density Function (CDF) Function for Chi Distributions <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Chi-squared_distribution#Cumulative_distribution_function" target="_blank">Wikipedia Chi CDF</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Chi2;
    ///
    /// let l = 2_f64;
    /// let u = 5_f64;
    /// let df = 4_f64;
    ///
    /// let cdf = Chi2::tailcdf(l, u, df);
    ///
    /// assert_eq!(cdf, 0.4484613871592377_f64);
    /// ```
    /// <hr/>
    pub fn tailcdf(lower: f64, upper: f64, df: f64) -> f64 {
        Self::cdf(upper, df) - Self::cdf(lower, df)
    }

    pub fn median(df: f64) -> f64 {
        let p1 = 1_f64 - 2_f64 / (9_f64 * df);
        df * p1.powi(3)
    }

    pub fn mode(df: f64) -> f64 {
        if df == 0_f64 {
            return 0_f64;
        }

        df - 2_f64
    }

    pub fn variance(df: f64) -> f64 {
        2_f64 * df
    }

    pub fn sd(df: f64) -> f64 {
        Chi2::variance(df).sqrt()
    }

    pub fn skewness(df: f64) -> f64 {
        (8_f64 / df).sqrt()
    }

    pub fn kurtosis(df: f64) -> f64 {
        12_f64 / df
    }
}
