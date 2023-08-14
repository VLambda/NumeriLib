use crate::special::Error;

pub struct Gaussian;

impl Gaussian {
    /// Calculates a Probability Density Function (PDF) on a Normal/Gaussian distribution <br>
    /// Learn more about Normal Distributions at: <a href="https://wikipedia.org/wiki/Normal_distribution#Definitions" target="_blank">Wikipedia normal.md PDF</a> <br>
    /// <hr/>
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Gaussian;
    ///
    /// let xvalue = 0.5;
    /// let mean = 0 as f64;
    /// let sd = 1 as f64;
    ///
    /// let normalpdf = Gaussian::pdf(xvalue,mean,sd);
    ///
    /// assert_eq!(normalpdf, 0.3520653267642995);
    /// ```
    /// <hr/>
    ///
    pub fn pdf(
        x_value: impl Into<f64> + Copy,
        mean: impl Into<f64> + Copy,
        sd: impl Into<f64> + Copy,
    ) -> f64 {
        (1.0 / (sd.into() * (std::f64::consts::TAU).sqrt()))
            * (std::f64::consts::E)
                .powf((-1.0 / 2.0) * ((x_value.into() - mean.into()) / sd.into()).powi(2))
    }
    /// Calculates Cumulative Distribution Function (CDF) on a Normal/Gaussian distribution <br>
    /// Learn more about Normal Distributions at: <a href="https://wikipedia.org/wiki/Normal_distribution#Definitions" target="_blank">Wikipedia normal.md CDF</a> <br>
    /// <hr/>
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Gaussian;
    ///
    /// let bound = 1.96_f64 ;
    /// let mean = 0_f64 ;
    /// let sd = 1_f64;
    ///
    /// let normalcdf = Gaussian::cdf(bound, mean, sd);
    ///
    /// assert_eq!(normalcdf, 0.024997895148220428_f64);
    /// ```
    /// <hr/>
    pub fn cdf(bound: f64, mean: f64, sd: f64) -> f64 {
        let z = (bound - mean) / (sd * std::f64::consts::SQRT_2);

        let erfc = Error::erfc(z);

        erfc / 2_f64
    }
    /// Calculates a 2 tailed Cumulative Distribution Function (CDF) on a Normal/Gaussian distribution <br>
    /// Learn more about Normal Distributions at: <a href="https://wikipedia.org/wiki/Normal_distribution#Definitions" target="_blank">Wikipedia normal.md CDF</a> <br>
    /// <hr/>
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Gaussian;
    ///
    /// let lower = 45_f64 ;
    /// let upper = 56_f64 ;
    /// let mean = 42_f64 ;
    /// let sd = 3.6_f64;
    ///
    /// let normalcdf = Gaussian::tailcdf(lower, upper, mean, sd);
    ///
    /// assert_eq!(normalcdf, 0.2023787330665709_f64);
    /// ```
    /// <hr/>
    pub fn tailcdf(lower: f64, upper: f64, mean: f64, sd: f64) -> f64 {
        Self::cdf(upper, mean, sd) + Self::cdf(lower, mean, sd)
    }
    /// This function recreates the InvNorm function in the Ti-83 & 84 calculators. <br>
    /// This is not to be confused with an Inverse gaussian Distribution though. <br>
    /// Learn more at: <a href="https://www.statology.org/inverse-normal-distribution/" target="_blank">Statology InvNorm</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1: Right Side Non-Standard Normal Distribution
    ///
    /// ```
    /// use ferrate::stats::distr::Gaussian;
    ///
    /// let area = 0.589255651;
    /// let mean = 42;
    /// let sd = 3.6;
    /// let tail = "Right";
    ///
    /// let invnorm = Gaussian::inv(area, mean, sd, tail).unwrap();
    /// assert_eq!(invnorm, 41.18772964960383);
    /// ```
    /// <hr/>
    ///
    /// # Example #2: Left Side Standard Normal Distribution
    ///
    /// ```
    /// use ferrate::stats::distr::Gaussian;
    ///
    /// let area = 0.975;
    /// let mean = 0;
    /// let sd = 1;
    /// let tail = "Left";
    ///
    /// let invnorm = Gaussian::inv(area, mean, sd, tail).unwrap();
    /// assert_eq!(invnorm, 1.9599639845400547);
    /// ```
    /// <hr/>
    ///
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

    pub fn mad(omega: f64) -> f64 {
        Error::inverf(1_f64 / 2_f64) * omega * std::f64::consts::SQRT_2
    }
}
