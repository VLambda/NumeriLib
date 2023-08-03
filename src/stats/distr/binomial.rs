use crate::prob::Probability;
use crate::special::Beta;

pub struct Binomial;

impl Binomial {
    fn q(p: f64) -> f64 {
        1_f64 - p
    }
    /// The Probability Mass Function (PMF) of the Binomial Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Binomial_distribution#Probability_mass_function" target="_blank">Wikipedia Binomial PMF</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let trails = 10_f64;
    /// let probability = 0.25_f64;
    /// let success = 4_f64;
    ///
    /// let pmf = Binomial::pmf(trails, probability, success);
    ///
    /// assert_eq!(pmf, 0.14599800109621883_f64)
    /// ```
    /// <hr/>
    pub fn pmf(trails: f64, probability: f64, success: f64) -> f64 {
        Probability::combination(trails, success)
            * probability.powf(success)
            * Binomial::q(probability).powf(trails - success)
    }
    /// The Cumulative Density Function (CDF) for Binomial Distributions <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Binomial_distribution#Cumulative_distribution_function" target="_blank">Wikipedia Binomial CDF</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let trial = 10_f64;
    /// let probability = 0.25_f64;
    /// let success = 4_f64;
    ///
    /// let cdf = Binomial::cdf(trial, probability, success);
    ///
    /// assert_eq!(cdf, 0.9218730929013825_f64);
    /// ```
    /// <hr/>
    pub fn cdf(trails: f64, probability: f64, success: f64) -> f64 {
        Beta::regincbeta(trails - success, 1_f64 + success, Binomial::q(probability))
    }
}
