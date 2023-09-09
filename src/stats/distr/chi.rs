use crate::special::Gamma;

/// A module containing functions to work with the Chi distribution.
pub struct Chi;

impl Chi {
    /// Calculates the Probability Density Function (PDF) of the Chi distribution.
    ///
    /// The Chi distribution describes the distribution of the magnitude of the square root of a
    /// sum of squares of independent standard normal random variables.
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
    /// use mathematica::stats::distr::Chi;
    ///
    /// let x = 2.5;
    /// let df = 4.0;
    ///
    /// let pdf = Chi::pdf(x, df);
    ///
    /// println!("PDF at x = {}: {}", x, pdf);
    /// ```
    /// <hr/>
    pub fn pdf(x: f64, df: f64) -> f64 {
        if x >= 0_f64 {
            let p1 = x.powf(df - 1_f64) * std::f64::consts::E.powf(-(x.powi(2) / 2_f64));
            let p2 = 2_f64.powf((df / 2_f64) - 1_f64) * Gamma::lanczos(df / 2_f64);
            p1 / p2
        } else {
            0_f64
        }
    }

    /// Calculates the Cumulative Density Function (CDF) of the Chi distribution.
    ///
    /// # Parameters
    ///
    /// - `x`: The value at which to evaluate the CDF.
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated CDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi;
    ///
    /// let x = 2.5;
    /// let df = 4.0;
    ///
    /// let cdf = Chi::cdf(x, df);
    ///
    /// println!("CDF at x = {}: {}", x, cdf);
    /// ```
    /// <hr/>
    pub fn cdf(x: f64, df: f64) -> f64 {
        Gamma::reggamma(df / 2_f64, x.powi(2) / 2_f64)
    }

    /// Calculates the mean of the Chi distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The calculated mean.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Chi;
    ///
    /// let df = 5.0;
    ///
    /// let mean = Chi::mean(df);
    ///
    /// println!("Mean for df = {}: {}", df, mean);
    /// ```
    /// <hr/>
    pub fn mean(df: f64) -> f64 {
        std::f64::consts::SQRT_2
            * (Gamma::lanczos((df + 1_f64) / 2_f64) / Gamma::lanczos(df / 2_f64))
    }

    /// Calculates the median of the Chi distribution.
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
    /// use mathematica::stats::distr::Chi;
    ///
    /// let df = 5.0;
    ///
    /// let median = Chi::median(df);
    ///
    /// println!("Median for df = {}: {}", df, median);
    /// ```
    /// <hr/>
    pub fn median(df: f64) -> f64 {
        (df * (1_f64 - (2_f64 / (9_f64 * df))).powi(3)).sqrt()
    }

    /// Calculates the mode of the Chi distribution.
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
    /// use mathematica::stats::distr::Chi;
    ///
    /// let df = 4.0;
    ///
    /// let mode = Chi::mode(df);
    ///
    /// println!("Mode for df = {}: {}", df, mode);
    /// ```
    /// <hr/>
    pub fn mode(df: f64) -> f64 {
        if df < 1_f64 {
            return 0_f64;
        }
        (df - 1_f64).sqrt()
    }

    /// Calculates the variance of the Chi distribution.
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
    /// use mathematica::stats::distr::Chi;
    ///
    /// let df = 4.0;
    ///
    /// let variance = Chi::variance(df);
    ///
    /// println!("Variance for df = {}: {}", df, variance);
    /// ```
    /// <hr/>
    pub fn variance(df: f64) -> f64 {
        df - Self::mean(df).powi(2)
    }

    /// Calculates the standard deviation of the Chi distribution.
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
    /// use mathematica::stats::distr::Chi;
    ///
    /// let df = 4.0;
    ///
    /// let sd = Chi::sd(df);
    ///
    /// println!("Standard Deviation for df = {}: {}", df, sd);
    /// ```
    /// <hr/>
    pub fn sd(df: f64) -> f64 {
        Self::variance(df).sqrt()
    }

    /// Calculates the skewness of the Chi distribution.
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
    /// use mathematica::stats::distr::Chi;
    ///
    /// let df = 4.0;
    ///
    /// let skewness = Chi::skewness(df);
    ///
    /// println!("Skewness for df = {}: {}", df, skewness);
    /// ```
    /// <hr/>
    pub fn skewness(df: f64) -> f64 {
        let mu = Self::mean(df);
        let omega = Self::sd(df);
        (mu / omega.powi(3)) * (1_f64 - 2_f64 * omega.powi(2))
    }

    /// Calculates the kurtosis of the Chi distribution.
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
    /// use mathematica::stats::distr::Chi;
    ///
    /// let df = 4.0;
    ///
    /// let kurtosis = Chi::kurtosis(df);
    ///
    /// println!("Kurtosis for df = {}: {}", df, kurtosis);
    /// ```
    /// <hr/>
    pub fn kurtosis(df: f64) -> f64 {
        let sd = Self::sd(df);
        let skewness = Self::skewness(df);
        let mu = Self::mean(df);
        (2_f64 / sd.powi(2)) * (1_f64 - mu * sd * skewness - sd.powi(2))
    }
}
