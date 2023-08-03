pub struct Geometric;

impl Geometric {
    /// Calculates the Probability Mass Function (PMF) of a given number of trials and a probability of success <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Geometric_distribution" target="_blank">Wikipedia Geometric Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1:
    ///
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 12_f64;
    ///
    /// let pmf = Geometric::pmf_trials(probability, trials);
    ///
    /// assert_eq!(pmf, 0.010558784008026123_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 0_f64;
    ///
    /// let pmf = Geometric::pmf_trials(probability, trials);
    ///     
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #3:
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0_f64;
    /// let trials = 12_f64;
    ///
    /// let pmf = Geometric::pmf_trials(probability, trials);
    ///
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    pub fn pmf_trials(probability: f64, trials: f64) -> f64 {
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
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 12_f64;
    ///
    /// let pmf = Geometric::cdf_trials(probability, trials);
    ///
    /// assert_eq!(pmf, 0.9683236479759216_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0.25_f64;
    /// let trials = 0_f64;
    ///
    /// let pmf = Geometric::cdf_trials(probability, trials);
    ///     
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #3:
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0_f64;
    /// let trials = 12_f64;
    ///
    /// let pmf = Geometric::cdf_trials(probability, trials);
    ///
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    pub fn cdf_trials(probability: f64, trials: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && trials >= 1_f64 {
            1_f64 - (1_f64 - probability).powf(trials.floor())
        } else {
            0_f64
        }
    }
    /// Calculates the Probability Mass Function (PMF) of a given number of failures and a probability of success <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Geometric_distribution" target="_blank">Wikipedia Geometric Distribution</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1:
    ///
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 12_f64;
    ///
    /// let pmf = Geometric::pmf_failure(probability, failures);
    ///
    /// assert_eq!(pmf, 0.007919088006019592_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 0_f64;
    ///
    /// let pmf = Geometric::pmf_failure(probability, failures);
    ///     
    /// assert_eq!(pmf, 0.25_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #3:
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0_f64;
    /// let failures = 12_f64;
    ///
    /// let pmf = Geometric::pmf_failure(probability, failures);
    ///
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    pub fn pmf_failure(probability: f64, failures: f64) -> f64 {
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
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 12_f64;
    ///
    /// let pmf = Geometric::cdf_failure(probability, failures);
    ///
    /// assert_eq!(pmf, 0.9762427359819412_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0.25_f64;
    /// let failures = 0_f64;
    ///
    /// let pmf = Geometric::cdf_failure(probability, failures);
    ///     
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #3:
    /// ```
    /// use ferrate::stats::distr::Geometric;
    ///
    /// let probability = 0_f64;
    /// let failures = 12_f64;
    ///
    /// let pmf = Geometric::cdf_failure(probability, failures);
    ///
    /// assert_eq!(pmf, 0_f64);
    /// ```
    /// <hr/>
    pub fn cdf_failure(probability: f64, failures: f64) -> f64 {
        if (0_f64..1_f64).contains(&probability) && failures >= 1_f64 {
            1_f64 - (1_f64 - probability).powf(failures.floor() + 1_f64)
        } else {
            0_f64
        }
    }
}
