use crate::special::Functions;

pub struct Gaussian;

impl Gaussian {
    /// This function recreates the InvNorm function in the Ti-83 & 84 calculators. <br>
    /// This is not to be confused with an Inverse Gaussian Distribution though. <br>
    /// This uses an approximation of the Inverse Error Function found at: <https://scistatcalc.blogspot.com/2013/09/numerical-estimate-of-inverse-error.html> <br>
    /// Learn more at: <https://www.statology.org/inverse-normal-distribution/>
    ///<hr/>
    /// # Example #1: Right Side Non-Standard Normal Distribution
    ///
    /// ```
    /// use vml::stats::distr::Gaussian;
    ///
    /// let area = 0.589255651;
    /// let mean = 42;
    /// let sd = 3.6;
    /// let tail = "Right";
    ///
    /// let invnorm = Gaussian::invnorm(area, mean, sd, tail).unwrap();
    /// assert_eq!(invnorm, 41.187729649603824);
    /// ```
    /// <hr/>
    /// # Example #2: Left Side Standard Normal Distribution
    ///
    /// ```
    /// use vml::stats::distr::Gaussian;
    ///
    /// let area = 0.975;
    /// let mean = 0;
    /// let sd = 1;
    /// let tail = "Left";
    ///
    /// let invnorm = Gaussian::invnorm(area, mean, sd, tail).unwrap();
    /// assert_eq!(invnorm, 1.9599639845401289);
    /// ```
    /// <hr/>
    ///
    pub fn invnorm<T: AsRef<str> + Copy>(area: f64, mean: impl Into<f64> + Copy, sd: impl Into<f64> + Copy, tail: T, ) -> Result<f64, String> {
        let tail_str = tail.as_ref();

        let function: f64 = (std::f64::consts::SQRT_2 * sd.into())
            * (Functions::inverf(-2_f64 * area + 1_f64));

        let tail_val: f64 = if tail_str.eq_ignore_ascii_case("Right") {
            function + mean.into()
        } else if tail_str.eq_ignore_ascii_case("Left") {
            (-1_f64 * function) + mean.into()
        } else {
            return Err("Tail side Invalid, the only options are:\nRight\nleft".to_string());
        };

        let rounded_val = if (tail_val.ceil() - 1e-7) < tail_val || (tail_val.floor() + 1e-7) > tail_val {
            (tail_val * 1e+7).round() / 1e+7
        } else {
            tail_val
        };

        Ok(rounded_val)
    }
    /// Calculates a Probability Density Function (PDF) on a Normal/Gaussian distribution <br>
    /// Learn more about Normal Distributions at: <https://wikipedia.org/wiki/Normal_distribution#Definitions>
    /// <hr/>
    ///
    /// # Example:
    ///
    /// ```
    /// use vml::stats::distr::Gaussian;
    ///
    /// let xvalue = 0.5;
    /// let mean = 0 as f64;
    /// let sd = 1 as f64;
    ///
    /// let normalpdf = Gaussian::normalpdf(xvalue,mean,sd);
    ///
    /// assert_eq!(normalpdf, 0.3520653267642995);
    /// ```
    /// <hr/>
    ///
    pub fn normalpdf(x_value: impl Into<f64> + Copy, mean: impl Into<f64> + Copy, sd: impl Into<f64> + Copy) -> f64 {
        (1.0 / (sd.into() * (std::f64::consts::TAU).sqrt()))
            * (std::f64::consts::E).powf((-1.0 / 2.0) * ((x_value.into() - mean.into()) / sd.into()).powi(2))
    }
    /// Calculates Cumulative Distribution Function (CDF) on a Normal/Gaussian distribution <br>
    /// Learn more about Normal Distributions at: <https://wikipedia.org/wiki/Normal_distribution#Definitions>
    /// <hr/>
    ///
    /// # Example:
    ///
    /// ```
    /// use vml::stats::distr::Gaussian;
    ///
    /// let lower = 45 as f64;
    /// let upper = 56 as f64;
    /// let mean = 42 as f64;
    /// let sd = 3.6;
    ///
    /// let normalcdf = Gaussian::normalcdf(lower, upper, mean, sd);
    ///
    /// assert_eq!(normalcdf, 0.20227802886072038);
    /// ```
    /// <hr/>
    ///
    pub fn normalcdf(lower: impl Into<f64> + Copy, upper: impl Into<f64> + Copy, mean: impl Into<f64> + Copy, sd: impl Into<f64> + Copy, ) -> f64 {
        let z1 = (lower.into() - mean.into()) / (sd.into() * std::f64::consts::SQRT_2);
        let z2 = (upper.into() - mean.into()) / (sd.into() * std::f64::consts::SQRT_2);

        let low_erf = Functions::lower_erf(z1);
        let up_erf = Functions::erf(z2);

        (low_erf + up_erf) / 2_f64
    }
}
