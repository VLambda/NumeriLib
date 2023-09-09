/// A module containing functions to work with Lists.
pub struct Generic;

impl Generic {
    /// Calculates the mean of a list of numbers.
    ///
    /// The mean is calculated as the sum of all numbers divided by the count of numbers.
    ///
    /// # Parameters
    ///
    /// - `list`: A slice of numbers for which the mean will be calculated.
    ///
    /// # Returns
    ///
    /// The calculated mean.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::Generic;
    ///
    /// let numbers = [3.0, 5.0, 8.0, 12.0, 15.0];
    /// let mean = Generic::mean(&numbers);
    ///
    /// println!("Mean: {}", mean);
    /// ```
    /// <hr/>
    pub fn mean(list: &[f64]) -> f64 {
        let sum: f64 = list.iter().sum();
        let count = list.len() as f64;

        sum / count
    }

    /// Calculates the sum of a list of numbers.
    ///
    /// The sum is calculated as the total of all numbers in the list.
    ///
    /// # Parameters
    ///
    /// - `list`: A slice of numbers to be summed.
    ///
    /// # Returns
    ///
    /// The calculated sum.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::Generic;
    ///
    /// let numbers = [3.0, 5.0, 8.0, 12.0, 15.0];
    /// let sum = Generic::sum(&numbers);
    ///
    /// println!("Sum: {}", sum);
    /// ```
    /// <hr/>
    pub fn sum(list: &[f64]) -> f64 {
        list.iter().sum()
    }

    /// Calculates the sum of squared values in a list of numbers.
    ///
    /// Each number in the list is squared, and then the sum of these squared values is calculated.
    ///
    /// # Parameters
    ///
    /// - `list`: A slice of numbers for which the sum of squared values will be calculated.
    ///
    /// # Returns
    ///
    /// The calculated sum of squared values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::Generic;
    ///
    /// let numbers = [3.0, 5.0, 8.0, 12.0, 15.0];
    /// let sum_squared = Generic::sum_squared(&numbers);
    ///
    /// println!("Sum of Squared Values: {}", sum_squared);
    /// ```
    /// <hr/>
    pub fn sum_squared(list: &[f64]) -> f64 {
        list.iter().map(|&x| x * x).sum()
    }

    /// Retrieves the number of elements in a list.
    ///
    /// This function returns the count of elements in the provided list.
    ///
    /// # Parameters
    ///
    /// - `list`: A slice of numbers for which the size will be determined.
    ///
    /// # Returns
    ///
    /// The size (count) of the list.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::Generic;
    ///
    /// let numbers = [3.0, 5.0, 8.0, 12.0, 15.0];
    /// let size = Generic::size(&numbers);
    ///
    /// println!("Size of List: {}", size);
    /// ```
    /// <hr/>
    pub fn size(list: &[f64]) -> usize {
        list.len()
    }

    /// Calculates the population variance of a list of numbers.
    ///
    /// The population variance is the average of the squared differences between each number
    /// and the mean of the entire population.
    ///
    /// # Parameters
    ///
    /// - `list`: A slice of numbers for which the population variance will be calculated.
    ///
    /// # Returns
    ///
    /// The calculated population variance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::Generic;
    ///
    /// let numbers = [3.0, 5.0, 8.0, 12.0, 15.0];
    /// let variance = Generic::population_variance(&numbers);
    ///
    /// println!("Population Variance: {}", variance);
    /// ```
    /// <hr/>
    pub fn population_variance(list: &[f64]) -> f64 {
        let mu = Self::mean(list);
        let squared_deviations_sum = list.iter().map(|x| (x - mu).powi(2)).sum::<f64>();
        squared_deviations_sum / Self::size(list) as f64
    }

    /// Calculates the sample variance of a list of numbers.
    ///
    /// The sample variance is an estimate of the variance within a sample from a larger population.
    ///
    /// # Parameters
    ///
    /// - `list`: A slice of numbers for which the sample variance will be calculated.
    ///
    /// # Returns
    ///
    /// The calculated sample variance.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::Generic;
    ///
    /// let numbers = [3.0, 5.0, 8.0, 12.0, 15.0];
    /// let sample_variance = Generic::sample_variance(&numbers);
    ///
    /// println!("Sample Variance: {}", sample_variance);
    /// ```
    /// <hr/>
    pub fn sample_variance(list: &[f64]) -> f64 {
        let mean = Self::mean(list);
        let squared_deviations_sum = list.iter().map(|x| (x - mean).powi(2)).sum::<f64>();
        squared_deviations_sum / (Self::size(list) - 1) as f64
    }

    /// Calculates the population standard deviation of a list of numbers.
    ///
    /// The population standard deviation is the square root of the population variance.
    ///
    /// # Parameters
    ///
    /// - `list`: A slice of numbers for which the population standard deviation will be calculated.
    ///
    /// # Returns
    ///
    /// The calculated population standard deviation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::Generic;
    ///
    /// let numbers = [3.0, 5.0, 8.0, 12.0, 15.0];
    /// let population_sd = Generic::population_sd(&numbers);
    ///
    /// println!("Population Standard Deviation: {}", population_sd);
    /// ```
    /// <hr/>
    pub fn population_sd(list: &[f64]) -> f64 {
        Self::population_variance(list).sqrt()
    }

    /// Calculates the sample standard deviation of a list of numbers.
    ///
    /// The sample standard deviation is calculated using the sample variance.
    ///
    /// # Parameters
    ///
    /// - `list`: A slice of numbers for which the sample standard deviation will be calculated.
    ///
    /// # Returns
    ///
    /// The calculated sample standard deviation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::Generic;
    ///
    /// let numbers = [3.0, 5.0, 8.0, 12.0, 15.0];
    /// let sd = Generic::sample_sd(&numbers);
    ///
    /// println!("Sample Standard Deviation: {}", sd);
    /// ```
    /// <hr/>
    pub fn sample_sd(list: &[f64]) -> f64 {
        Self::sample_variance(list).sqrt()
    }

    /// Calculates the five-number summary of a list of numbers.
    ///
    /// The five-number summary consists of the minimum, first quartile (Q1), median, third quartile (Q3),
    /// and maximum of the given list of numbers.
    ///
    /// # Parameters
    ///
    /// - `list`: A slice of numbers for which the five-number summary will be calculated.
    ///
    /// # Returns
    ///
    /// A vector containing the minimum, Q1, median, Q3, and maximum.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::stats::Generic;
    ///
    /// let numbers = [3.0, 5.0, 8.0, 12.0, 15.0, 19.0, 21.0, 25.0, 28.0, 31.0];
    /// let summary = Generic::five_number_summary(&numbers);
    ///
    /// println!("Five-Number Summary: {:?}", summary);
    /// ```
    /// <hr/>
    pub fn five_number_summary(list: &[f64]) -> Vec<f64> {
        let mut sorted_list = list.to_vec();
        sorted_list.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = sorted_list[0];
        let max = sorted_list[sorted_list.len() - 1];

        let median_index = (sorted_list.len() as f64 / 2.0).floor() as usize;
        let median = if sorted_list.len() % 2 == 0 {
            (sorted_list[median_index - 1] + sorted_list[median_index]) / 2.0
        } else {
            sorted_list[median_index]
        };

        let lower_half = &sorted_list[..median_index];
        let upper_half = &sorted_list[median_index + sorted_list.len() % 2..];

        let q1_index = (lower_half.len() as f64 / 2.0).floor() as usize;
        let q1 = if lower_half.len() % 2 == 0 {
            (lower_half[q1_index - 1] + lower_half[q1_index]) / 2.0
        } else {
            lower_half[q1_index]
        };

        let q3_index = (upper_half.len() as f64 / 2.0).floor() as usize;
        let q3 = if upper_half.len() % 2 == 0 {
            (upper_half[q3_index - 1] + upper_half[q3_index]) / 2.0
        } else {
            upper_half[q3_index]
        };

        vec![min, q1, median, q3, max]
    }
}
