/// A module containing functions to work with the Cauchy distribution.
pub struct Cauchy;

impl Cauchy {
    /// Calculates the Probability Density Function (PDF) of the Cauchy distribution.
    ///
    /// The Cauchy distribution describes a symmetric distribution with heavy tails. It has no finite
    /// mean or variance.
    ///
    /// # Parameters
    ///
    /// - `location`: The location parameter (median) of the distribution.
    /// - `scale`: The scale parameter of the distribution.
    /// - `x`: The value at which to evaluate the PDF.
    ///
    /// # Returns
    ///
    /// The calculated PDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::stats::distr::Cauchy;
    ///
    /// let location = 0.0;
    /// let scale = 1.0;
    /// let x = 2.5;
    ///
    /// let pdf = Cauchy::pdf(location, scale, x);
    ///
    /// println!("PDF at x = {}: {}", x, pdf);
    /// ```
    /// <hr/>
    pub fn pdf(location: f64, scale: f64, x: f64) -> f64 {
        1_f64 / (std::f64::consts::PI * scale * ((x - location) / scale).powi(2))
    }

    /// Calculates the Cumulative Density Function (CDF) of the Cauchy distribution.
    ///
    /// # Parameters
    ///
    /// - `location`: The location parameter (median) of the distribution.
    /// - `scale`: The scale parameter of the distribution.
    /// - `x`: The value at which to evaluate the CDF.
    ///
    /// # Returns
    ///
    /// The calculated CDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::stats::distr::Cauchy;
    ///
    /// let location = 0.0;
    /// let scale = 1.0;
    /// let x = 2.5;
    ///
    /// let cdf = Cauchy::cdf(location, scale, x);
    ///
    /// println!("CDF at x = {}: {}", x, cdf);
    /// ```
    /// <hr/>
    pub fn cdf(location: f64, scale: f64, x: f64) -> f64 {
        std::f64::consts::FRAC_1_PI * ((x - location) / scale).atan() + (1_f64 / 2_f64)
    }

    /// Calculates the Inverse Cauchy function.
    ///
    /// This function is used to find the value that corresponds to a given cumulative probability.
    ///
    /// # Parameters
    ///
    /// - `location`: The location parameter (median) of the distribution.
    /// - `scale`: The scale parameter of the distribution.
    /// - `probability`: The cumulative probability for which to find the quantile.
    ///
    /// # Returns
    ///
    /// The calculated quantile value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::stats::distr::Cauchy;
    ///
    /// let location = 0.0;
    /// let scale = 1.0;
    /// let probability = 0.75;
    ///
    /// let quantile = Cauchy::inv(location, scale, probability);
    ///
    /// println!("Quantile for probability {}: {}", probability, quantile);
    /// ```
    /// <hr/>
    pub fn inv(location: f64, scale: f64, probability: f64) -> f64 {
        location + scale * (std::f64::consts::PI * (probability - (1_f64 / 2_f64))).tan()
    }
}
