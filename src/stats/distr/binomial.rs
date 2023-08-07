use crate::special::Probability;
use crate::special::{Beta, Functions};

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
    /// assert_eq!(pmf, 0.1459980010986328_f64)
    /// ```
    /// <hr/>
    pub fn pmf(trails: f64, probability: f64, success: f64) -> f64 {
        Probability::combination(success, trails)
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
    /// assert_eq!(cdf, 0.9218730912280808_f64);
    /// ```
    /// <hr/>
    pub fn cdf(trails: f64, probability: f64, success: f64) -> f64 {
        Beta::regincbeta(trails - success, 1_f64 + success, Binomial::q(probability))
    }
    /// Calculates the InverseCDF of a Binomial Distribution <br>
    /// Learn more at: <a href="https://www.mathworks.com/help/stats/binoinv.html" target="_blank">MatLab InverseCDF Binomial Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let area = 0.51_f64;
    /// let trials = 500_f64;
    /// let probability = 0.6_f64;
    ///
    /// let inv = Binomial::inv(area, trials, probability);
    ///
    /// assert_eq!(inv, 300_f64);
    /// ```
    /// <hr/>
    pub fn inv(area: f64, trials: f64, probability: f64) -> f64 {
        let func = |t: f64| Binomial::cdf(trials, probability, t) - area;
        let guess = trials * probability;

        let new = Functions::newmet(guess, func);

        new.ceil()
    }
    /// Calculates the mean of a Binomial Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Binomial_distribution" target="_blank">Wikipedia Binomial Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let trials = 8_f64;
    /// let probability = 0.125_f64;
    ///
    /// let mean = Binomial::mean(trials, probability);
    ///
    /// assert_eq!(mean, 1_f64)
    /// ```
    /// <hr/>
    pub fn mean(trials: f64, probability: f64) -> f64 {
        trials * probability
    }
    /// Calculates the median of a Binomial Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Binomial_distribution" target="_blank">Wikipedia Binomial Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let trials = 8_f64;
    /// let probability = 0.125_f64;
    ///
    /// let median = Binomial::median(trials, probability);
    ///
    /// assert_eq!(median, 1_f64)
    /// ```
    /// <hr/>
    pub fn median(trials: f64, probability: f64) -> f64 {
        Self::mean(trials, probability).round()
    }
    /// Calculates the mode of a Binomial Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Binomial_distribution" target="_blank">Wikipedia Binomial Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let trials = 8_f64;
    /// let probability = 0.125_f64;
    ///
    /// let mode = Binomial::mode(trials, probability);
    ///
    /// assert_eq!(mode, 1_f64)
    /// ```
    /// <hr/>
    pub fn mode(trials: f64, probability: f64) -> f64 {
        ((trials + 1_f64) * probability).floor()
    }
    /// Calculates the variance of a Binomial Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Binomial_distribution" target="_blank">Wikipedia Binomial Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let trials = 8_f64;
    /// let probability = 0.125_f64;
    ///
    /// let variance = Binomial::variance(trials, probability);
    ///
    /// assert_eq!(variance, 0.875_f64)
    /// ```
    /// <hr/>
    pub fn variance(trials: f64, probability: f64) -> f64 {
        trials * probability * Binomial::q(probability)
    }
    /// Calculates the standard deviation of a Binomial Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Binomial_distribution" target="_blank">Wikipedia Binomial Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let trials = 8_f64;
    /// let probability = 0.125_f64;
    ///
    /// let variance = Binomial::variance(trials, probability);
    ///
    /// assert_eq!(variance, 0.875_f64)
    /// ```
    /// <hr/>
    pub fn sd(trials: f64, probability: f64) -> f64 {
        Binomial::variance(trials, probability).sqrt()
    }
    /// Calculates the mean of a Binomial Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Binomial_distribution" target="_blank">Wikipedia Binomial Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let trials = 8_f64;
    /// let probability = 0.125_f64;
    ///
    /// let skewness = Binomial::skewness(trials, probability);
    ///
    /// assert_eq!(skewness, 0.8017837257372732_f64)
    /// ```
    /// <hr/>
    pub fn skewness(trials: f64, probability: f64) -> f64 {
        let p1 = Binomial::q(probability) - probability;
        let p2 = (trials * probability * Binomial::q(probability)).sqrt();
        p1 / p2
    }
    /// Calculates the kurtosis of a Binomial Distribution <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Binomial_distribution" target="_blank">Wikipedia Binomial Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::stats::distr::Binomial;
    ///
    /// let trials = 8_f64;
    /// let probability = 0.125_f64;
    ///
    /// let kurtosis = Binomial::kurtosis(trials, probability);
    ///
    /// assert_eq!(kurtosis, 0.39285714285714285_f64)
    /// ```
    /// <hr/>
    pub fn kurtosis(trials: f64, probability: f64) -> f64 {
        let p1 = 1_f64 - 6_f64 * probability * Binomial::q(probability);
        let p2 = trials * probability * Binomial::q(probability);
        p1 / p2
    }
}
