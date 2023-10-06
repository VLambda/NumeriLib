use crate::special::{Gamma, Probability};

/// A module containing functions to work with the Poisson distribution.
pub struct Poisson;

impl Poisson {
    /// Calculates the Probability Mass Function for the Poisson Distribution.
    ///
    /// The Poisson distribution models the probability of a given number of events occurring in
    /// a fixed interval of time or space when events occur with a known constant mean rate and are independent.
    ///
    /// # Parameters
    ///
    /// - `k`: The number of events.
    /// - `lambda`: The mean rate of events.
    ///
    /// # Returns
    ///
    /// The value of the PMF at the given `k` and `lambda`.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Poisson;
    ///
    /// let k = 7_f64;
    /// let lambda = 8_f64;
    ///
    /// let pmf = Poisson::pmf(k, lambda);
    ///
    ///
    /// println!("PMF at k = {} and lambda = {}: {}", k, lambda, pmf);
    /// ```
    /// <hr/>
    pub fn pmf(k: f64, lambda: f64) -> f64 {
        (lambda.powf(k) * (-lambda).exp()) / Probability::factorial(k)
    }

    /// Calculates the Cumulative Distribution Function (CDF) of the Poisson Distribution with the Q Regularized Gamma Function.
    ///
    /// The CDF gives the probability that the number of events in the interval is less than or equal to a given value.
    ///
    /// # Parameters
    ///
    /// - `k`: The upper bound for the number of events.
    /// - `lambda`: The mean rate of events.
    ///
    /// # Returns
    ///
    /// The value of the CDF at the given `k` and `lambda`.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Poisson;
    ///
    /// let k = 7_f64;
    /// let lambda = 8_f64;
    ///
    /// let cdf = Poisson::cdf(k, lambda);
    ///
    /// println!("CDF at k = {} and lambda = {}: {}", k, lambda, cdf);
    /// ```
    /// <hr/>
    pub fn cdf(k: f64, lambda: f64) -> f64 {
        Gamma::reggammac(k + 1_f64, lambda)
    }

    /// Calculates the median of the Poisson Distribution.
    ///
    /// The median is the value that separates the distribution into two halves.
    ///
    /// # Parameters
    ///
    /// - `lambda`: The mean rate of events.
    ///
    /// # Returns
    ///
    /// The median of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Poisson;
    ///
    /// let lambda = 8_f64;
    ///
    /// let median = Poisson::median(lambda);
    ///
    /// println!("Median with lambda = {}: {}", lambda, median);
    /// ```
    /// <hr/>
    pub fn median(lambda: f64) -> f64 {
        (lambda + (1_f64 / 3_f64) - (1_f64 / (50_f64 * lambda))).floor()
    }

    /// Calculates the mode of the Poisson Distribution.
    ///
    /// The mode is the value that appears most frequently in the distribution.
    ///
    /// # Parameters
    ///
    /// - `lambda`: The mean rate of events.
    ///
    /// # Returns
    ///
    /// The mode of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Poisson;
    ///
    /// let lambda = 8_f64;
    ///
    /// let mode = Poisson::mode(lambda);
    ///
    /// println!("Mode with lambda = {}: {}", lambda, mode);
    /// ```
    /// <hr/>
    pub fn mode(lambda: f64) -> f64 {
        lambda.floor()
    }

    /// Calculates the standard deviation of the Poisson Distribution.
    ///
    /// The standard deviation measures the spread of the distribution.
    ///
    /// # Parameters
    ///
    /// - `lambda`: The mean rate of events.
    ///
    /// # Returns
    ///
    /// The standard deviation of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Poisson;
    ///
    /// let lambda = 8_f64;
    ///
    /// let sd = Poisson::sd(lambda);
    ///
    /// println!("Standard Deviation with lambda = {}: {}", lambda, sd);
    /// ```
    /// <hr/>
    pub fn sd(lambda: f64) -> f64 {
        lambda.sqrt()
    }

    /// Calculates the skewness of the Poisson Distribution.
    ///
    /// Skewness measures the asymmetry of the distribution.
    ///
    /// # Parameters
    ///
    /// - `lambda`: The mean rate of events.
    ///
    /// # Returns
    ///
    /// The skewness of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Poisson;
    ///
    /// let lambda = 8_f64;
    ///
    /// let skewness = Poisson::skewness(lambda);
    ///
    /// println!("Skewness with lambda = {}: {}", lambda, skewness);
    /// ```
    /// <hr/>
    pub fn skewness(lambda: f64) -> f64 {
        1_f64 / lambda.sqrt()
    }

    /// Calculates the kurtosis of the Poisson Distribution.
    ///
    /// Kurtosis measures the "tailedness" of the distribution.
    ///
    /// # Parameters
    ///
    /// - `lambda`: The mean rate of events.
    ///
    /// # Returns
    ///
    /// The kurtosis of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Poisson;
    ///
    /// let lambda = 8_f64;
    ///
    /// let kurtosis = Poisson::kurtosis(lambda);
    ///
    /// println!("Kurtosis with lambda = {}: {}", lambda, kurtosis);
    /// ```
    /// <hr/>
    pub fn kurtosis(lambda: f64) -> f64 {
        1_f64 / lambda
    }
}
