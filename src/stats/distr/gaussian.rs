use crate::special::Error;

/// A module containing functions to work with the Gaussian (Normal) distribution.
pub struct Gaussian;

impl Gaussian {
    /// Calculates the Probability Density Function (PDF) on a Normal/Gaussian distribution.
    ///
    /// # Parameters
    ///
    /// - `x_value`: The value at which to evaluate the PDF.
    /// - `mean`: The mean (average) of the distribution.
    /// - `sd`: The standard deviation of the distribution.
    ///
    /// # Returns
    ///
    /// The calculated PDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::stats::distr::Gaussian;
    ///
    /// let x_value = 0.5;
    /// let mean = 0.0;
    /// let sd = 1.0;
    ///
    /// let normalpdf = Gaussian::pdf(x_value, mean, sd);
    ///
    /// println!("PDF at x = {}: {}", x_value, normalpdf);
    /// ```
    /// <hr/>
    pub fn pdf(
        x_value: impl Into<f64> + Copy,
        mean: impl Into<f64> + Copy,
        sd: impl Into<f64> + Copy,
    ) -> f64 {
        (1.0 / (sd.into() * (std::f64::consts::TAU).sqrt()))
            * (std::f64::consts::E)
                .powf((-1.0 / 2.0) * ((x_value.into() - mean.into()) / sd.into()).powi(2))
    }

    /// Calculates the Cumulative Distribution Function (CDF) on a Normal/Gaussian distribution.
    ///
    /// The CDF is calculated by converting the value to a z-score and using the Error function.
    ///
    /// # Parameters
    ///
    /// - `bound`: The upper bound of integration for the CDF.
    /// - `mean`: The mean (average) of the distribution.
    /// - `sd`: The standard deviation of the distribution.
    ///
    /// # Returns
    ///
    /// The calculated CDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::stats::distr::Gaussian;
    ///
    /// let bound = 1.96;
    /// let mean = 0.0;
    /// let sd = 1.0;
    ///
    /// let normalcdf = Gaussian::cdf(bound, mean, sd);
    ///
    /// println!("CDF at bound = {}: {}", bound, normalcdf);
    /// ```
    /// <hr/>
    pub fn cdf(bound: f64, mean: f64, sd: f64) -> f64 {
        let z = (bound - mean) / (sd * std::f64::consts::SQRT_2);

        let erfc = Error::erfc(z);

        erfc / 2_f64
    }

    /// Calculates a 2 tailed Cumulative Distribution Function (CDF) on a Normal/Gaussian distribution.
    ///
    /// The 2-tailed CDF is calculated by summing the CDF values for both tails.
    ///
    /// # Parameters
    ///
    /// - `lower`: The lower bound of integration for the CDF.
    /// - `upper`: The upper bound of integration for the CDF.
    /// - `mean`: The mean (average) of the distribution.
    /// - `sd`: The standard deviation of the distribution.
    ///
    /// # Returns
    ///
    /// The calculated 2-tailed CDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::stats::distr::Gaussian;
    ///
    /// let lower = -1.0;
    /// let upper = 1.0;
    /// let mean = 0.0;
    /// let sd = 1.0;
    ///
    /// let normalcdf = Gaussian::tailcdf(lower, upper, mean, sd);
    ///
    /// println!("2-tailed CDF between {} and {}: {}", lower, upper, normalcdf);
    /// ```
    /// <hr/>
    pub fn tailcdf(lower: f64, upper: f64, mean: f64, sd: f64) -> f64 {
        if lower > upper {
            return 0_f64;
        }
        Self::cdf(lower, mean, sd) - Self::cdf(upper, mean, sd)
    }

    /// Calculates the Inverse Normal (Quantile) function.
    ///
    /// This function is used to find the value that corresponds to a given cumulative probability.
    /// It recreates the InvNorm function in the Ti-83 & 84 calculators.
    ///
    /// # Parameters
    ///
    /// - `area`: The cumulative probability for which to find the quantile.
    /// - `mean`: The mean (average) of the distribution.
    /// - `sd`: The standard deviation of the distribution.
    /// - `tail`: The tail side for which to find the quantile ("Right" or "Left").
    ///
    /// # Returns
    ///
    /// - The calculated quantile value on success.
    /// - An error message on failure.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::stats::distr::Gaussian;
    ///
    /// let area = 0.975;
    /// let mean = 0.0;
    /// let sd = 1.0;
    /// let tail = "Left";
    ///
    /// let invnorm = Gaussian::inv(area, mean, sd, tail).unwrap();
    ///
    /// println!("Inverse normal quantile for area {}: {}", area, invnorm);
    /// ```
    /// <hr/>
    pub fn inv<T: AsRef<str> + Copy>(
        area: f64,
        mean: impl Into<f64> + Copy,
        sd: impl Into<f64> + Copy,
        tail: T,
    ) -> Result<f64, String> {
        let tail_str = tail.as_ref();

        let function: f64 =
            (std::f64::consts::SQRT_2 * sd.into()) * (Error::inverf(-2_f64 * area + 1_f64));

        let tail_val: f64 = if tail_str.eq_ignore_ascii_case("Right") {
            function + mean.into()
        } else if tail_str.eq_ignore_ascii_case("Left") {
            (-1_f64 * function) + mean.into()
        } else {
            return Err("Tail side Invalid, the only options are:\nRight\nleft".to_string());
        };

        let rounded_val =
            if (tail_val.ceil() - 1e-7) < tail_val || (tail_val.floor() + 1e-7) > tail_val {
                (tail_val * 1e+7).round() / 1e+7
            } else {
                tail_val
            };

        Ok(rounded_val)
    }

    /// Calculates the Median Absolute Deviation (MAD) on a Normal/Gaussian distribution.
    ///
    /// MAD is a robust measure of the variability of a univariate sample of quantitative data.
    ///
    /// # Parameters
    ///
    /// - `sigma`: The standard deviation multiplier.
    ///
    /// # Returns
    ///
    /// The calculated MAD value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::stats::distr::Gaussian;
    ///
    /// let sigma = 1.5;
    ///
    /// let mad = Gaussian::mad(sigma);
    ///
    /// println!("MAD for sigma {}: {}", sigma, mad);
    /// ```
    /// <hr/>
    pub fn mad(sigma: f64) -> f64 {
        Error::inverf(1_f64 / 2_f64) * sigma * std::f64::consts::SQRT_2
    }
}
