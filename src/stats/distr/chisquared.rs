use crate::special::Gamma;

/// A module containing functions to work with the Chi-squared distribution.
pub struct Chi2;

impl Chi2 {
    /// Calculates the Probability Density Function (PDF) of the Chi-squared distribution.
    ///
    /// The Chi-squared distribution describes the distribution of the sum of the squares of
    /// independent standard normal random variables.
    ///
    /// # Parameters
    ///
    /// - `x`: The value at which to evaluate the PDF.
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated PDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi2;
    ///
    /// let x = 6.0;
    /// let df = 5.0;
    ///
    /// let pdf = Chi2::pdf(x, df);
    ///
    /// println!("PDF at x = {}: {}", x, pdf);
    /// ```
    /// <hr/>
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

    /// Calculates the Cumulative Density Function (CDF) of the Chi-squared distribution.
    ///
    /// # Parameters
    ///
    /// - `bound`: The upper bound of integration for the CDF.
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated CDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi2;
    ///
    /// let bound = 2.0;
    /// let df = 4.0;
    ///
    /// let cdf = Chi2::cdf(bound, df);
    ///
    /// println!("CDF at bound = {}: {}", bound, cdf);
    /// ```
    /// <hr/>
    pub fn cdf(bound: f64, df: f64) -> f64 {
        Gamma::reggamma(df / 2_f64, bound / 2_f64)
    }

    /// Calculates a 2 Tailed Cumulative Density Function (CDF) of the Chi-squared distribution.
    ///
    /// # Parameters
    ///
    /// - `lower`: The lower bound of integration for the CDF.
    /// - `upper`: The upper bound of integration for the CDF.
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated 2-tailed CDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi2;
    ///
    /// let lower = 2.0;
    /// let upper = 5.0;
    /// let df = 4.0;
    ///
    /// let cdf = Chi2::tailcdf(lower, upper, df);
    ///
    /// println!("2-tailed CDF between {} and {}: {}", lower, upper, cdf);
    /// ```
    /// <hr/>
    pub fn tailcdf(lower: f64, upper: f64, df: f64) -> f64 {
        Self::cdf(upper, df) - Self::cdf(lower, df)
    }

    /// Calculates the median of the Chi-squared distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated median.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi2;
    ///
    /// let df = 5.0;
    ///
    /// let median = Chi2::median(df);
    ///
    /// println!("Median for df = {}: {}", df, median);
    /// ```
    /// <hr/>
    pub fn median(df: f64) -> f64 {
        let p1 = 1_f64 - 2_f64 / (9_f64 * df);
        df * p1.powi(3)
    }

    /// Calculates the mode of the Chi-squared distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated mode.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi2;
    ///
    /// let df = 4.0;
    ///
    /// let mode = Chi2::mode(df);
    ///
    /// println!("Mode for df = {}: {}", df, mode);
    /// ```
    /// <hr/>
    pub fn mode(df: f64) -> f64 {
        if df == 0_f64 {
            return 0_f64;
        }

        df - 2_f64
    }

    /// Calculates the variance of the Chi-squared distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated variance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi2;
    ///
    /// let df = 4.0;
    ///
    /// let variance = Chi2::variance(df);
    ///
    /// println!("Variance for df = {}: {}", df, variance);
    /// ```
    /// <hr/>
    pub fn variance(df: f64) -> f64 {
        2_f64 * df
    }

    /// Calculates the standard deviation of the Chi-squared distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated standard deviation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi2;
    ///
    /// let df = 4.0;
    ///
    /// let sd = Chi2::sd(df);
    ///
    /// println!("Standard Deviation for df = {}: {}", df, sd);
    /// ```
    /// <hr/>
    pub fn sd(df: f64) -> f64 {
        Chi2::variance(df).sqrt()
    }

    /// Calculates the skewness of the Chi-squared distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated skewness.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi2;
    ///
    /// let df = 4.0;
    ///
    /// let skewness = Chi2::skewness(df);
    ///
    /// println!("Skewness for df = {}: {}", df, skewness);
    /// ```
    /// <hr/>
    pub fn skewness(df: f64) -> f64 {
        (8_f64 / df).sqrt()
    }

    /// Calculates the kurtosis of the Chi-squared distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated kurtosis.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi2;
    ///
    /// let df = 4.0;
    ///
    /// let kurtosis = Chi2::kurtosis(df);
    ///
    /// println!("Kurtosis for df = {}: {}", df, kurtosis);
    /// ```
    /// <hr/>
    pub fn kurtosis(df: f64) -> f64 {
        12_f64 / df
    }
}
