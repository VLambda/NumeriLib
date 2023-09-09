use crate::special::Probability;

/// A module containing functions to work with the Hypergeometric distribution.
pub struct Hypergeometric;

impl Hypergeometric {
    /// Calculates the Probability Mass Function for the Hypergeometric Distribution.
    ///
    /// The Hypergeometric distribution models the probability of drawing a certain number of successes
    /// in a sample without replacement from a finite population containing both successes and failures.
    ///
    /// # Parameters
    ///
    /// - `population`: The total population size.
    /// - `success`: The number of successful items in the population.
    /// - `draws`: The number of items drawn in the sample.
    /// - `observed`: The number of successful items observed in the drawn sample.
    ///
    /// # Returns
    ///
    /// The value of the PMF at the given parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::stats::distr::Hypergeometric;
    ///
    /// fn main() {
    ///     let population = 100_f64;
    ///     let success = 20_f64;
    ///     let draws = 10_f64;
    ///     let observed = 4_f64;
    ///
    ///     let pmf = Hypergeometric::pmf(population, success, draws, observed);
    ///
    ///     println!("PMF: {}", pmf);
    /// }
    /// ```
    /// <hr/>
    pub fn pmf(population: f64, success: f64, draws: f64, observed: f64) -> f64 {
        (Probability::combination(success, observed)
            * Probability::combination(population - success, draws - observed))
            / Probability::combination(population, draws)
    }

    /// Calculates the Lower Cumulative Distribution Function (CDF) of the Hypergeometric Distribution.
    ///
    /// The Lower CDF gives the probability that the number of successful items in the drawn sample is less than
    /// or equal to a given number.
    ///
    /// # Parameters
    ///
    /// - `population`: The total population size.
    /// - `success`: The number of successful items in the population.
    /// - `draws`: The number of items drawn in the sample.
    /// - `observed`: The number of successful items observed in the drawn sample.
    ///
    /// # Returns
    ///
    /// The value of the Lower CDF at the given parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::stats::distr::Hypergeometric;
    ///
    /// fn main() {
    ///     let population = 100_f64;
    ///     let success = 20_f64;
    ///     let draws = 10_f64;
    ///     let observed = 4_f64;
    ///
    ///     let cdf = Hypergeometric::lcdf(population, success, draws, observed);
    ///
    ///     println!("CDF: {}", cdf);
    /// }
    /// ```
    /// <hr/>
    pub fn lcdf(population: f64, success: f64, draws: f64, observed: f64) -> f64 {
        let mut result = 0_f64;
        let x = observed as i32;

        for i in 0..x + 1 {
            result += Self::pmf(population, success, draws, i as f64)
        }

        result
    }

    /// Calculates the Upper Cumulative Distribution Function (CDF) of the Hypergeometric Distribution.
    ///
    /// The Upper CDF gives the probability that the number of successful items in the drawn sample is less than
    /// or equal to a given number.
    ///
    /// # Parameters
    ///
    /// - `population`: The total population size.
    /// - `success`: The number of successful items in the population.
    /// - `draws`: The number of items drawn in the sample.
    /// - `observed`: The number of successful items observed in the drawn sample.
    ///
    /// # Returns
    ///
    /// The value of the Upper CDF at the given parameters.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::stats::distr::Hypergeometric;
    ///
    /// fn main() {
    ///     let population = 100_f64;
    ///     let success = 20_f64;
    ///     let draws = 10_f64;
    ///     let observed = 4_f64;
    ///
    ///     let cdf = Hypergeometric::ucdf(population, success, draws, observed);
    ///
    ///     println!("CDF: {}", cdf);
    /// }
    /// ```
    /// <hr/>
    pub fn ucdf(population: f64, success: f64, draws: f64, observed: f64) -> f64 {
        let mut result = 0_f64;
        let x = observed as i32;
        let m = success as i32;

        for i in x..m + 1 {
            result += Self::pmf(population, success, draws, i as f64)
        }

        result
    }

    /// Calculates the mean of the Hypergeometric Distribution.
    ///
    /// The mean is the average number of successful items in the drawn sample.
    ///
    /// # Parameters
    ///
    /// - `population`: The total population size.
    /// - `success`: The number of successful items in the population.
    /// - `draws`: The number of items drawn in the sample.
    ///
    /// # Returns
    ///
    /// The mean of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::stats::distr::Hypergeometric;
    ///
    /// fn main() {
    ///     let population = 100_f64;
    ///     let success = 20_f64;
    ///     let draws = 10_f64;
    ///
    ///     let mean = Hypergeometric::mean(population, success, draws);
    ///
    ///     println!("Mean: {}", mean);
    /// }
    /// ```
    /// <hr/>
    pub fn mean(population: f64, success: f64, draws: f64) -> f64 {
        draws * (success / population)
    }

    /// Calculates the mode of the Hypergeometric Distribution.
    ///
    /// The mode is the value that appears most frequently in the distribution.
    ///
    /// # Parameters
    ///
    /// - `population`: The total population size.
    /// - `success`: The number of successful items in the population.
    /// - `draws`: The number of items drawn in the sample.
    ///
    /// # Returns
    ///
    /// The mode of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::stats::distr::Hypergeometric;
    ///
    /// fn main() {
    ///     let population = 100_f64;
    ///     let success = 20_f64;
    ///     let draws = 10_f64;
    ///
    ///     let mode = Hypergeometric::mode(population, success, draws);
    ///
    ///     println!("Mode: {}", mode);
    /// }
    /// ```
    /// <hr/>
    pub fn mode(population: f64, success: f64, draws: f64) -> f64 {
        (((draws + 1_f64) * (success + 1_f64)) / (population + 2_f64)).floor()
    }

    /// Calculates the variance of the Hypergeometric Distribution.
    ///
    /// The variance measures the spread of the distribution.
    ///
    /// # Parameters
    ///
    /// - `population`: The total population size.
    /// - `success`: The number of successful items in the population.
    /// - `draws`: The number of items drawn in the sample.
    ///
    /// # Returns
    ///
    /// The variance of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::stats::distr::Hypergeometric;
    ///
    /// fn main() {
    ///     let population = 100_f64;
    ///     let success = 20_f64;
    ///     let draws = 10_f64;
    ///
    ///     let variance = Hypergeometric::variance(population, success, draws);
    ///
    ///     println!("Variance: {}", variance);
    /// }
    /// ```
    /// <hr/>
    pub fn variance(population: f64, success: f64, draws: f64) -> f64 {
        draws
            * (success / population)
            * ((population - success) / population)
            * ((population - draws) / (population - 1_f64))
    }

    /// Calculates the standard deviation of the Hypergeometric Distribution.
    ///
    /// The standard deviation measures the spread of the distribution.
    ///
    /// # Parameters
    ///
    /// - `population`: The total population size.
    /// - `success`: The number of successful items in the population.
    /// - `draws`: The number of items drawn in the sample.
    ///
    /// # Returns
    ///
    /// The standard deviation of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::stats::distr::Hypergeometric;
    ///
    /// fn main() {
    ///     let population = 100_f64;
    ///     let success = 20_f64;
    ///     let draws = 10_f64;
    ///
    ///     let sd = Hypergeometric::sd(population, success, draws);
    ///
    ///     println!("Standard Deviation: {}", sd);
    /// }
    /// ```
    /// <hr/>
    pub fn sd(population: f64, success: f64, draws: f64) -> f64 {
        Self::variance(population, success, draws).sqrt()
    }

    /// Calculates the skewness of the Hypergeometric Distribution.
    ///
    /// The skewness measures the asymmetry of the distribution.
    ///
    /// # Parameters
    ///
    /// - `population`: The total population size.
    /// - `success`: The number of successful items in the population.
    /// - `draws`: The number of items drawn in the sample.
    ///
    /// # Returns
    ///
    /// The skewness of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::stats::distr::Hypergeometric;
    ///
    /// fn main() {
    ///     let population = 100_f64;
    ///     let success = 20_f64;
    ///     let draws = 10_f64;
    ///
    ///     let skewness = Hypergeometric::skewness(population, success, draws);
    ///
    ///     println!("Skewness: {}", skewness);
    /// }
    /// ```
    /// <hr/>
    pub fn skewness(population: f64, success: f64, draws: f64) -> f64 {
        ((population - (2_f64 * success))
            * (population - 1_f64).sqrt()
            * (population - (2_f64 * draws)))
            / ((draws * population * (population - success) * (population - draws)).sqrt()
                * (population - 2_f64))
    }

    /// Calculates the kurtosis of the Hypergeometric Distribution.
    ///
    /// The kurtosis measures the "tailedness" of the distribution.
    ///
    /// # Parameters
    ///
    /// - `population`: The total population size.
    /// - `success`: The number of successful items in the population.
    /// - `draws`: The number of items drawn in the sample.
    ///
    /// # Returns
    ///
    /// The kurtosis of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use mathematica::stats::distr::Hypergeometric;
    ///
    /// fn main() {
    ///     let population = 100_f64;
    ///     let success = 20_f64;
    ///     let draws = 10_f64;
    ///
    ///     let kurtosis = Hypergeometric::kurtosis(population, success, draws);
    ///
    ///     println!("Kurtosis: {}", kurtosis);
    /// }
    /// ```
    /// <hr/>
    pub fn kurtosis(population: f64, success: f64, draws: f64) -> f64 {
        let p1 = 1_f64
            / (draws
                * success
                * (population - success)
                * (population - draws)
                * (population - 2_f64)
                * (population - 3_f64));
        let p2 = (population - 1_f64)
            * population.powi(2)
            * (population * (population + 1_f64)
                - (6_f64 * success) * (population - success)
                - (6_f64 * draws) * (population - draws))
            + (6_f64 * draws * success)
                * (population - success)
                * (population - draws)
                * ((5_f64 * population) - 6_f64);
        p1 * p2
    }
}
