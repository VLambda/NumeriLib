use crate::special::Beta;
use crate::special::Probability;
use crate::Functions;

/// A module containing functions to work with the Binomial distribution.
pub struct Binomial;

impl Binomial {
    fn q(p: f64) -> f64 {
        1_f64 - p
    }

    /// Calculates the Probability Mass Function (PMF) of the Binomial distribution.
    ///
    /// The Binomial distribution models the number of successes in a fixed number of independent Bernoulli trials.
    ///
    /// # Parameters
    ///
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    /// - `success`: The number of successful outcomes.
    ///
    /// # Returns
    ///
    /// The calculated PMF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let trials = 10.0;
    /// let probability = 0.25;
    /// let success = 4.0;
    ///
    /// let pmf = Binomial::pmf(trials, probability, success);
    ///
    /// println!("PMF at {} successes: {}", success, pmf);
    /// ```
    /// <hr/>
    pub fn pmf(trails: f64, probability: f64, success: f64) -> f64 {
        Probability::combination(success, trails)
            * probability.powf(success)
            * Binomial::q(probability).powf(trails - success)
    }

    /// Calculates the Cumulative Density Function (CDF) of the Binomial distribution.
    ///
    /// # Parameters
    ///
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    /// - `success`: The number of successful outcomes.
    ///
    /// # Returns
    ///
    /// The calculated CDF.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let trials = 10.0;
    /// let probability = 0.25;
    /// let success = 4.0;
    ///
    /// let cdf = Binomial::cdf(trials, probability, success);
    ///
    /// println!("CDF at {} successes: {}", success, cdf);
    /// ```
    /// <hr/>
    pub fn cdf(trails: f64, probability: f64, success: f64) -> f64 {
        Beta::regincbeta(trails - success, 1_f64 + success, Binomial::q(probability))
    }

    /// Calculates the Inverse CDF of the Binomial distribution.
    ///
    /// # Parameters
    ///
    /// - `area`: The desired cumulative probability.
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    ///
    /// # Returns
    ///
    /// The calculated quantile value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let area = 0.51;
    /// let trials = 500.0;
    /// let probability = 0.6;
    ///
    /// let quantile = Binomial::inv(area, trials, probability);
    ///
    /// println!("Quantile for probability {}: {}", area, quantile);
    /// ```
    /// <hr/>
    pub fn inv(area: f64, trials: f64, probability: f64) -> f64 {
        let func = |t: f64| Binomial::cdf(trials, probability, t) - area;
        let guess = trials * probability;

        let new = Functions::newmet(guess, func);

        new.ceil()
    }

    /// Calculates the mean of a Binomial Distribution.
    ///
    /// The mean of a binomial distribution is given by `trials * probability`.
    ///
    /// # Parameters
    ///
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    ///
    /// # Returns
    ///
    /// The calculated mean.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let trials = 8.0;
    /// let probability = 0.125;
    ///
    /// let mean = Binomial::mean(trials, probability);
    ///
    /// println!("Mean: {}", mean);
    /// ```
    /// <hr/>
    pub fn mean(trials: f64, probability: f64) -> f64 {
        trials * probability
    }

    /// Calculates the median of a Binomial Distribution.
    ///
    /// The median of a binomial distribution is estimated as the rounded value of the mean.
    ///
    /// # Parameters
    ///
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    ///
    /// # Returns
    ///
    /// The calculated median.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let trials = 8.0;
    /// let probability = 0.125;
    ///
    /// let median = Binomial::median(trials, probability);
    ///
    /// println!("Median: {}", median);
    /// ```
    /// <hr/>
    pub fn median(trials: f64, probability: f64) -> f64 {
        Self::mean(trials, probability).round()
    }

    /// Calculates the mode of a Binomial Distribution.
    ///
    /// The mode of a binomial distribution is given by `(trials + 1) * probability` rounded down.
    ///
    /// # Parameters
    ///
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    ///
    /// # Returns
    ///
    /// The calculated mode.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let trials = 8.0;
    /// let probability = 0.125;
    ///
    /// let mode = Binomial::mode(trials, probability);
    ///
    /// println!("Mode: {}", mode);
    /// ```
    /// <hr/>
    pub fn mode(trials: f64, probability: f64) -> f64 {
        ((trials + 1_f64) * probability).floor()
    }

    /// Calculates the variance of a Binomial Distribution.
    ///
    /// The variance of a binomial distribution is given by `trials * probability * q(probability)`.
    ///
    /// # Parameters
    ///
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    ///
    /// # Returns
    ///
    /// The calculated variance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let trials = 8.0;
    /// let probability = 0.125;
    ///
    /// let variance = Binomial::variance(trials, probability);
    ///
    /// println!("Variance: {}", variance);
    /// ```
    /// <hr/>
    pub fn variance(trials: f64, probability: f64) -> f64 {
        trials * probability * Binomial::q(probability)
    }

    /// Calculates the standard deviation of a Binomial Distribution.
    ///
    /// The standard deviation of a binomial distribution is the square root of the variance.
    ///
    /// # Parameters
    ///
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    ///
    /// # Returns
    ///
    /// The calculated standard deviation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let trials = 8.0;
    /// let probability = 0.125;
    ///
    /// let sd = Binomial::sd(trials, probability);
    ///
    /// println!("Standard Deviation: {}", sd);
    /// ```
    /// <hr/>
    pub fn sd(trials: f64, probability: f64) -> f64 {
        Binomial::variance(trials, probability).sqrt()
    }

    /// Calculates the skewness of a Binomial Distribution.
    ///
    /// The skewness of a binomial distribution is given by `(q(probability) - probability) / (trials * probability * q(probability))`.
    ///
    /// # Parameters
    ///
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    ///
    /// # Returns
    ///
    /// The calculated skewness.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let trials = 8.0;
    /// let probability = 0.125;
    ///
    /// let skewness = Binomial::skewness(trials, probability);
    ///
    /// println!("Skewness: {}", skewness);
    /// ```
    /// <hr/>
    pub fn skewness(trials: f64, probability: f64) -> f64 {
        let p1 = Binomial::q(probability) - probability;
        let p2 = (trials * probability * Binomial::q(probability)).sqrt();
        p1 / p2
    }

    /// Calculates the kurtosis of a Binomial Distribution.
    ///
    /// The kurtosis of a binomial distribution is given by `(1 - 6 * probability * q(probability)) / (trials * probability * q(probability))`.
    ///
    /// # Parameters
    ///
    /// - `trials`: The number of trials.
    /// - `probability`: The probability of success in a single trial.
    ///
    /// # Returns
    ///
    /// The calculated kurtosis.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::distr::Binomial;
    ///
    /// let trials = 8.0;
    /// let probability = 0.125;
    ///
    /// let kurtosis = Binomial::kurtosis(trials, probability);
    ///
    /// println!("Kurtosis: {}", kurtosis);
    /// ```
    /// <hr/>
    pub fn kurtosis(trials: f64, probability: f64) -> f64 {
        let p1 = 1_f64 - 6_f64 * probability * Binomial::q(probability);
        let p2 = trials * probability * Binomial::q(probability);
        p1 / p2
    }
}
