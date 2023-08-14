pub struct GeometricTrials;
pub struct GeometricFailure;

impl GeometricTrials {
    /// Calculates the Probability Mass Function (PMF) of a given number of trials and a probability of success <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Geometric_distribution" target="_blank">Wikipedia Geometric Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1:
    ///
    /// ```
    /// use ferrate::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 12_f64;
    ///
    /// let pmf = GeometricTrials::pmf(probability, trials);
    ///
    /// assert_eq!(pmf, 0.010558784008026123_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 0_f64;
    ///
    /// let pmf = GeometricTrials::pmf(probability, trials);
    ///     
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #3:
    /// ```
    /// use ferrate::stats::distr::GeometricTrials;
    ///
    /// let probability = 0_f64;
    /// let trials = 12_f64;
    ///
    /// let pmf = GeometricTrials::pmf(probability, trials);
    ///
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    pub fn pmf(probability: f64, trials: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && trials >= 1_f64 {
            (1_f64 - probability).powf(trials - 1_f64) * probability
        } else {
            0_f64
        }
    }
    /// Calculates the Cumulative Distribution Function (CDF) of a given number of trials and a probability of success <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Geometric_distribution" target="_blank">Wikipedia Geometric Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1:
    ///
    /// ```
    /// use ferrate::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 12_f64;
    ///
    /// let pmf = GeometricTrials::cdf(probability, trials);
    ///
    /// assert_eq!(pmf, 0.9683236479759216_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::stats::distr::GeometricTrials;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 0_f64;
    ///
    /// let pmf = GeometricTrials::cdf(probability, trials);
    ///     
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #3:
    /// ```
    /// use ferrate::stats::distr::GeometricTrials;
    ///
    /// let probability = 0_f64;
    /// let trials = 12_f64;
    ///
    /// let pmf = GeometricTrials::cdf(probability, trials);
    ///
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    pub fn cdf(probability: f64, trials: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && trials >= 1_f64 {
            1_f64 - (1_f64 - probability).powf(trials.floor())
        } else {
            0_f64
        }
    }

    pub fn mean(probability: f64) -> f64 {
        1_f64 / probability
    }

    pub fn median(probability: f64) -> f64 {
        (-1_f64 / (1_f64 - probability).log2()).ceil()
    }

    pub fn variance(probability: f64) -> f64 {
        (1_f64 - probability) / probability.powi(2)
    }

    pub fn sd(probability: f64) -> f64 {
        Self::variance(probability).sqrt()
    }

    pub fn skewness(probability: f64) -> f64 {
        (2_f64 - probability) / (1_f64 - probability).sqrt()
    }

    pub fn kurtosis(probability: f64) -> f64 {
        6_f64 + (probability.powi(2) / (1_f64 - probability))
    }
}

impl GeometricFailure {
    /// Calculates the Probability Mass Function (PMF) of a given number of failures and a probability of success <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Geometric_distribution" target="_blank">Wikipedia Geometric Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1:
    ///
    /// ```
    /// use ferrate::stats::distr::GeometricFailure;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 12_f64;
    ///
    /// let pmf = GeometricFailure::pmf(probability, failures);
    ///
    /// assert_eq!(pmf, 0.007919088006019592_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::stats::distr::GeometricFailure;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 0_f64;
    ///
    /// let pmf = GeometricFailure::pmf(probability, failures);
    ///     
    /// assert_eq!(pmf, 0.25_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #3:
    /// ```
    /// use ferrate::stats::distr::GeometricFailure;
    ///
    /// let probability = 0_f64;
    /// let failures = 12_f64;
    ///
    /// let pmf = GeometricFailure::pmf(probability, failures);
    ///
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    pub fn pmf(probability: f64, failures: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && failures >= 0_f64 {
            (1_f64 - probability).powf(failures) * probability
        } else {
            0_f64
        }
    }
    /// Calculates the Cumulative Distribution Function (CDF) of a given number of failures and a probability of success <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Geometric_distribution" target="_blank">Wikipedia Geometric Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1:
    ///
    /// ```
    /// use ferrate::stats::distr::GeometricFailure;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 12_f64;
    ///
    /// let pmf = GeometricFailure::cdf(probability, failures);
    ///
    /// assert_eq!(pmf, 0.9762427359819412_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::stats::distr::GeometricFailure;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 0_f64;
    ///
    /// let pmf = GeometricFailure::cdf(probability, failures);
    ///     
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #3:
    /// ```
    /// use ferrate::stats::distr::GeometricFailure;
    ///
    /// let probability = 0_f64;
    /// let failures = 12_f64;
    ///
    /// let pmf = GeometricFailure::cdf(probability, failures);
    ///
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    pub fn cdf(probability: f64, failures: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && failures >= 1_f64 {
            1_f64 - (1_f64 - probability).powf(failures.floor() + 1_f64)
        } else {
            0_f64
        }
    }

    pub fn mean(probability: f64) -> f64 {
        (1_f64 - probability) / probability
    }

    pub fn median(probability: f64) -> f64 {
        GeometricTrials::median(probability) - 1_f64
    }
}
