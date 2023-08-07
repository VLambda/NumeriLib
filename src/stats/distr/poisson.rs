use crate::special::{Functions, Gamma};

pub struct Poisson;

impl Poisson {
    /// Calculates the Probability Mass Function for the Poisson Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Poisson_distribution#Probability_mass_function" target="_blank">Wikipedia Probability Mass Function Poisson</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Poisson;
    ///
    /// let k = 7_f64;
    /// let lambda = 8_f64;
    ///
    /// let pmf = Poisson::pmf(k, lambda);
    ///
    /// assert_eq!(pmf, 0.13958653195059692_f64);
    /// ```
    /// <hr/>
    pub fn pmf(k: f64, lambda: f64) -> f64 {
        (lambda.powf(k) * (-lambda).exp()) / Functions::factorial(k)
    }
    /// Calculates the Cumulative Distribution Function (CDF) of the Poisson Distribution with the Q Regularized Gamma Function <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Poisson_distribution" target="_blank">Wikipedia Poisson Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Poisson;
    ///
    /// let k = 7_f64;
    /// let lambda = 8_f64;
    ///
    /// let cdf = Poisson::cdf(k, lambda);
    ///
    /// assert_eq!(cdf, 0.45296080949136563_f64);
    /// ```
    /// <hr/>
    pub fn cdf(k: f64, lambda: f64) -> f64 {
        Gamma::reggammac(k + 1_f64, lambda)
    }
}
