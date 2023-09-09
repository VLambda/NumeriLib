/// A module containing functions to work with the Geometric distribution for trials.
pub struct GeometricTrials;

/// A module containing functions to work with the Geometric distribution for failures.
pub struct GeometricFailure;

impl GeometricTrials {
    /// Calculates the Probability Mass Function (PMF) of a given number of trials and a probability of success.
    ///
    /// The Geometric distribution models the probability of the number of trials needed to achieve the first success.
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    /// - `trials`: The number of trials.
    ///
    /// # Returns
    ///
    /// The value of the PMF at the given parameters.
    ///
    /// # Example #1:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 12_f64;
    ///
    /// let pmf = GeometricTrials::pmf(probability, trials);
    ///
    /// println!("PMF: {}", pmf);
    /// ```
    /// <hr/>
    pub fn pmf(probability: f64, trials: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && trials >= 1_f64 {
            (1_f64 - probability).powf(trials - 1_f64) * probability
        } else {
            0_f64
        }
    }

    /// Calculates the Cumulative Distribution Function (CDF) of a given number of trials and a probability of success.
    ///
    /// The Geometric distribution models the probability of the number of trials needed to achieve the first success.
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    /// - `trials`: The number of trials.
    ///
    /// # Returns
    ///
    /// The value of the CDF at the given parameters.
    ///
    /// # Example #1:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 12_f64;
    ///
    /// let cdf = GeometricTrials::cdf(probability, trials);
    ///
    /// println!("CDF: {}", cdf);
    /// ```
    /// <hr/>
    pub fn cdf(probability: f64, trials: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && trials >= 1_f64 {
            1_f64 - (1_f64 - probability).powf(trials.floor())
        } else {
            0_f64
        }
    }

    /// Calculates the mean of the Geometric Distribution for trials.
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    ///
    /// # Returns
    ///
    /// The mean of the distribution.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    ///
    /// let mean = GeometricTrials::mean(probability);
    ///
    /// println!("Mean: {}", mean);
    /// ```
    /// <hr/>
    pub fn mean(probability: f64) -> f64 {
        1_f64 / probability
    }

    /// Calculates the median of the Geometric Distribution for trials.
    ///
    /// The median is calculated using the formula: ceil(-1 / log2(1 - probability)).
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    ///
    /// # Returns
    ///
    /// The median of the distribution.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    ///
    /// let median = GeometricTrials::median(probability);
    ///
    /// println!("Median: {}", median);
    /// ```
    /// <hr/>
    pub fn median(probability: f64) -> f64 {
        (-1_f64 / (1_f64 - probability).log2()).ceil()
    }

    /// Calculates the variance of the Geometric Distribution for trials.
    ///
    /// The variance is calculated using the formula: (1 - probability) / probability^2.
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    ///
    /// # Returns
    ///
    /// The variance of the distribution.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    ///
    /// let variance = GeometricTrials::variance(probability);
    ///
    /// println!("Variance: {}", variance);
    /// ```
    /// <hr/>
    pub fn variance(probability: f64) -> f64 {
        (1_f64 - probability) / probability.powi(2)
    }

    /// Calculates the standard deviation of the Geometric Distribution for trials.
    ///
    /// The standard deviation is calculated as the square root of the variance.
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    ///
    /// # Returns
    ///
    /// The standard deviation of the distribution.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    ///
    /// let sd = GeometricTrials::sd(probability);
    ///
    /// println!("Standard Deviation: {}", sd);
    /// ```
    /// <hr/>
    pub fn sd(probability: f64) -> f64 {
        Self::variance(probability).sqrt()
    }

    /// Calculates the skewness of the Geometric Distribution for trials.
    ///
    /// The skewness is calculated using the formula: (2 - probability) / sqrt(1 - probability).
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    ///
    /// # Returns
    ///
    /// The skewness of the distribution.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    ///
    /// let skewness = GeometricTrials::skewness(probability);
    ///
    /// println!("Skewness: {}", skewness);
    /// ```
    /// <hr/>
    pub fn skewness(probability: f64) -> f64 {
        (2_f64 - probability) / (1_f64 - probability).sqrt()
    }

    /// Calculates the kurtosis of the Geometric Distribution for trials.
    ///
    /// The kurtosis is calculated using the formula: 6 + (probability^2 / (1 - probability)).
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    ///
    /// # Returns
    ///
    /// The kurtosis of the distribution.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    ///
    /// let kurtosis = GeometricTrials::kurtosis(probability);
    ///
    /// println!("Kurtosis: {}", kurtosis);
    /// ```
    /// <hr/>
    pub fn kurtosis(probability: f64) -> f64 {
        6_f64 + (probability.powi(2) / (1_f64 - probability))
    }
}

impl GeometricFailure {
    /// Calculates the Probability Mass Function (PMF) of a given number of failures and a probability of success.
    ///
    /// The Geometric distribution models the probability of the number of failures before the first success.
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    /// - `failures`: The number of failures.
    ///
    /// # Returns
    ///
    /// The value of the PMF at the given parameters.
    ///
    /// # Example #1:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricFailure;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 12_f64;
    ///
    /// let pmf = GeometricFailure::pmf(probability, failures);
    ///
    /// println!("PMF: {}", pmf);
    /// ```
    /// <hr/>
    pub fn pmf(probability: f64, failures: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && failures >= 0_f64 {
            (1_f64 - probability).powf(failures) * probability
        } else {
            0_f64
        }
    }

    /// Calculates the Cumulative Distribution Function (CDF) of a given number of failures and a probability of success.
    ///
    /// The Geometric distribution models the probability of the number of failures before the first success.
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    /// - `failures`: The number of failures.
    ///
    /// # Returns
    ///
    /// The value of the CDF at the given parameters.
    ///
    /// # Example #1:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricFailure;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 12_f64;
    ///
    /// let cdf = GeometricFailure::cdf(probability, failures);
    ///
    /// println!("CDF: {}", cdf);
    /// ```
    /// <hr/>
    pub fn cdf(probability: f64, failures: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && failures >= 1_f64 {
            1_f64 - (1_f64 - probability).powf(failures.floor() + 1_f64)
        } else {
            0_f64
        }
    }

    /// Calculates the mean of the Geometric Distribution for failures.
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    ///
    /// # Returns
    ///
    /// The mean of the distribution.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricFailure;
    ///
    /// let probability = 0.25_f64;
    ///
    /// let mean = GeometricFailure::mean(probability);
    ///
    /// println!("Mean: {}", mean);
    /// ```
    /// <hr/>
    pub fn mean(probability: f64) -> f64 {
        (1_f64 - probability) / probability
    }

    /// Calculates the median of the Geometric Distribution for failures.
    ///
    /// The median is calculated by subtracting 1 from the median of the corresponding GeometricTrials distribution.
    ///
    /// # Parameters
    ///
    /// - `probability`: The probability of success on a single trial.
    ///
    /// # Returns
    ///
    /// The median of the distribution.
    ///
    /// # Example:
    ///
    /// ```
    /// use mathematica::stats::distr::GeometricFailure;
    ///
    /// let probability = 0.25_f64;
    ///
    /// let median = GeometricFailure::median(probability);
    ///
    /// println!("Median: {}", median);
    /// ```
    /// <hr/>
    pub fn median(probability: f64) -> f64 {
        GeometricTrials::median(probability) - 1_f64
    }
}
