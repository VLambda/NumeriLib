use crate::special::Beta;

/// A module containing functions to work with the Fisher-Snedecor (F) distribution.
pub struct Fisher;

impl Fisher {
    /// Calculates the Probability Density Function (PDF) of the Fisher-Snedecor distribution.
    ///
    /// The Fisher-Snedecor distribution describes the distribution of the ratio of two independent
    /// chi-squared random variables, each divided by their respective degrees of freedom.
    ///
    /// # Parameters
    ///
    /// - `x`: The value at which to evaluate the PDF.
    /// - `d1`: The degrees of freedom of the first variable.
    /// - `d2`: The degrees of freedom of the second variable.
    ///
    /// # Returns
    ///
    /// The calculated PDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Fisher;
    ///
    /// let x = 7.0;
    /// let d1 = 6.0;
    /// let d2 = 5.0;
    ///
    /// let pdf = Fisher::pdf(x, d1, d2);
    ///
    /// println!("PDF at x = {}: {}", x, pdf);
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

    /// Calculates the Cumulative Density Function (CDF) of the Fisher-Snedecor distribution.
    ///
    /// # Parameters
    ///
    /// - `bound`: The upper bound of integration for the CDF.
    /// - `d1`: The degrees of freedom of the first variable.
    /// - `d2`: The degrees of freedom of the second variable.
    ///
    /// # Returns
    ///
    /// The calculated CDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Fisher;
    ///
    /// let bound = 8.0;
    /// let d1 = 4.0;
    /// let d2 = 8.0;
    ///
    /// let cdf = Fisher::cdf(bound, d1, d2);
    ///
    /// println!("CDF at bound = {}: {}", bound, cdf);
    /// ```
    /// <hr/>
    pub fn cdf(bound: f64, d1: f64, d2: f64) -> f64 {
        let limit = (d1 * bound) / (d1 * bound + d2);
        Beta::regincbeta(d1 / 2_f64, d2 / 2_f64, limit)
    }

    /// Calculates a 2 Tailed CDF for the Fisher-Snedecor distribution.
    ///
    /// # Parameters
    ///
    /// - `lower`: The lower bound of integration for the CDF.
    /// - `upper`: The upper bound of integration for the CDF.
    /// - `d1`: The degrees of freedom of the first variable.
    /// - `d2`: The degrees of freedom of the second variable.
    ///
    /// # Returns
    ///
    /// The calculated 2-tailed CDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Fisher;
    ///
    /// let lower = 8.0;
    /// let upper = 9.0;
    /// let d1 = 4.0;
    /// let d2 = 8.0;
    ///
    /// let cdf = Fisher::tailcdf(lower, upper, d1, d2);
    ///
    /// println!("2-tailed CDF between {} and {}: {}", lower, upper, cdf);
    /// ```
    /// <hr/>
    pub fn tailcdf(lower: f64, upper: f64, d1: f64, d2: f64) -> f64 {
        Self::cdf(upper, d1, d2) - Self::cdf(lower, d1, d2)
    }

    /// Calculates the mean of the Fisher-Snedecor distribution.
    ///
    /// # Parameters
    ///
    /// - `d2`: The degrees of freedom of the second variable.
    ///
    /// # Returns
    ///
    /// The calculated mean.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Fisher;
    ///
    /// let d2 = 8.0;
    ///
    /// let mean = Fisher::mean(d2);
    ///
    /// println!("Mean for d2 = {}: {}", d2, mean);
    /// ```
    /// <hr/>
    pub fn mean(d2: f64) -> f64 {
        if d2 < 3_f64 {
            return f64::NAN;
        }
        d2 / (d2 - 2_f64)
    }

    /// Calculates the mode of the Fisher-Snedecor distribution.
    ///
    /// # Parameters
    ///
    /// - `d1`: The degrees of freedom of the first variable.
    /// - `d2`: The degrees of freedom of the second variable.
    ///
    /// # Returns
    ///
    /// The calculated mode.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Fisher;
    ///
    /// let d1 = 4.0;
    /// let d2 = 8.0;
    ///
    /// let mode = Fisher::mode(d1, d2);
    ///
    /// println!("Mode for d1 = {} and d2 = {}: {}", d1, d2, mode);
    /// ```
    /// <hr/>
    pub fn mode(d1: f64, d2: f64) -> f64 {
        if d1 < 3_f64 {
            return f64::NAN;
        }
        ((d1 - 2_f64) / d1) * (d2 / (d2 + 2_f64))
    }

    /// Calculates the variance of the Fisher-Snedecor distribution.
    ///
    /// # Parameters
    ///
    /// - `d1`: The degrees of freedom of the first variable.
    /// - `d2`: The degrees of freedom of the second variable.
    ///
    /// # Returns
    ///
    /// The calculated variance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Fisher;
    ///
    /// let d1 = 4.0;
    /// let d2 = 8.0;
    ///
    /// let variance = Fisher::variance(d1, d2);
    ///
    /// println!("Variance for d1 = {} and d2 = {}: {}", d1, d2, variance);
    /// ```
    /// <hr/>
    pub fn variance(d1: f64, d2: f64) -> f64 {
        if d2 < 5_f64 {
            return f64::NAN;
        }
        (2_f64 * d2.powi(2) * (d1 + d2 - 2_f64)) / (d1 * (d2 - 2_f64).powi(2) * (d2 - 4_f64))
    }

    /// Calculates the standard deviation of the Fisher-Snedecor distribution.
    ///
    /// # Parameters
    ///
    /// - `d1`: The degrees of freedom of the first variable.
    /// - `d2`: The degrees of freedom of the second variable.
    ///
    /// # Returns
    ///
    /// The calculated standard deviation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Fisher;
    ///
    /// let d1 = 4.0;
    /// let d2 = 8.0;
    ///
    /// let sd = Fisher::sd(d1, d2);
    ///
    /// println!("Standard Deviation for d1 = {} and d2 = {}: {}", d1, d2, sd);
    /// ```
    /// <hr/>
    pub fn sd(d1: f64, d2: f64) -> f64 {
        Self::variance(d1, d2).sqrt()
    }

    /// Calculates the skewness of the Fisher-Snedecor distribution.
    ///
    /// # Parameters
    ///
    /// - `d1`: The degrees of freedom of the first variable.
    /// - `d2`: The degrees of freedom of the second variable.
    ///
    /// # Returns
    ///
    /// The calculated skewness.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Fisher;
    ///
    /// let d1 = 4.0;
    /// let d2 = 8.0;
    ///
    /// let skewness = Fisher::skewness(d1, d2);
    ///
    /// println!("Skewness for d1 = {} and d2 = {}: {}", d1, d2, skewness);
    /// ```
    /// <hr/>
    pub fn skewness(d1: f64, d2: f64) -> f64 {
        if d2 < 7_f64 {
            return f64::NAN;
        }
        ((2_f64 * d1 + d2 - 2_f64) * (8_f64 * (d2 - 4_f64)).sqrt())
            / ((d2 - 6_f64) * (d1 * (d1 + d2 - 2_f64)).sqrt())
    }
}
