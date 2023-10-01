use crate::special::Beta;
use crate::Functions;
use std::sync::atomic::Ordering::SeqCst;

/// A module containing functions to work with the Student's T distribution.
pub struct Student;

impl Student {
    /// Calculates the Probability Density Function (PDF) of the Student's T Distribution.
    ///
    /// The Student's T distribution describes the distribution of the ratio of a standard normal variable
    /// to the square root of a scaled chi-squared variable. It is commonly used in hypothesis testing
    /// when the sample size is small and the population variance is unknown.
    ///
    /// # Parameters
    ///
    /// - `x`: The value at which to evaluate the PDF.
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The value of the PDF at the given `x`.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Student;
    ///
    /// fn main() {
    ///     let x = 0.975;
    ///     let df = 6_f64;
    ///
    ///     let tpdf = Student::pdf(x, df);
    ///
    ///     println!("PDF at x = {}: {}", x, tpdf);
    /// }
    /// ```
    /// <hr/>
    pub fn pdf(x: f64, df: f64) -> f64 {
        let p1 = 1_f64 + (x.powi(2) / df);
        let p2 = -((df + 1_f64) / 2_f64);
        let p3 = 1_f64 / (df.sqrt() * Beta::beta(0.5, df / 2_f64));
        p3 * p1.powf(p2)
    }

    fn beta_cdf(bound: f64, df: f64) -> f64 {
        if df.is_sign_negative() {
            return f64::NAN;
        }

        let limit = df / (bound.powi(2) + df);
        let p1 = Beta::regincbeta(df / 2_f64, 1_f64 / 2_f64, limit);
        p1 / 2_f64
    }

    /// Calculates the Cumulative Density Function (CDF) of the Student's T Distribution.
    ///
    /// The CDF gives the probability that a random variable following the Student's T distribution
    /// is less than or equal to a given value.
    ///
    /// # Parameters
    ///
    /// - `bound`: The upper bound value for which to calculate the CDF.
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The value of the CDF at the given `bound`.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Student;
    ///
    /// fn main() {
    ///     let bound = 1_f64;
    ///     let df = 6_f64;
    ///
    ///     let tcdf = Student::cdf(bound, df);
    ///
    ///     println!("CDF at bound = {}: {}", bound, tcdf);
    /// }
    /// ```
    /// <hr/>
    pub fn cdf(bound: f64, df: f64) -> f64 {
        if df.is_sign_negative() {
            return f64::NAN;
        }

        if bound <= 0_f64 {
            Self::beta_cdf(bound, df)
        } else {
            1_f64 - Self::beta_cdf(-bound, df)
        }
    }

    /// Calculates the two-tailed Cumulative Density Function (CDF) of the Student's T Distribution.
    ///
    /// This function calculates the probability that a random variable following the Student's T distribution
    /// falls between two given values.
    ///
    /// # Parameters
    ///
    /// - `lower`: The lower bound value.
    /// - `upper`: The upper bound value.
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The probability that the random variable falls between `lower` and `upper`.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Student;
    ///
    /// fn main() {
    ///     let lower = 1_f64;
    ///     let upper = 1.96;
    ///     let df = 6_f64;
    ///
    ///     let tcdf = Student::tailcdf(lower, upper, df);
    ///
    ///     println!("Two-tailed CDF between {} and {}: {}", lower, upper, tcdf);
    /// }
    /// ```
    /// <hr/>
    pub fn tailcdf(lower: f64, upper: f64, df: f64) -> f64 {
        let (bound_low, bound_high) = if lower < upper {
            (lower, upper)
        } else {
            (upper, lower)
        };

        let p1 = Self::cdf(bound_low, df);
        let p2 = Self::cdf(bound_high, df);
        p2 - p1
    }

    fn upper_inv(area: f64, df: f64) -> f64 {
        let p1 = df.sqrt();
        let z1 = df / 2_f64;
        let z2 = 0.5_f64;
        let x = 2_f64 * (1_f64 - area);
        let p2 = Beta::invregincbeta(z1, z2, x);
        let p3 = 1_f64 / p2;
        let p4 = p3 - 1_f64;
        let p5 = p4.sqrt();
        p1 * p5
    }

    /// Calculates the Inverse of the two-tailed Cumulative Density Function (CDF), also known as InvT.
    ///
    /// The inverse of the two-tailed CDF gives the value at which the cumulative probability falls within
    /// a specified range for a random variable following the Student's T distribution.
    ///
    /// # Parameters
    ///
    /// - `area`: The desired cumulative probability range (0.0 to 1.0).
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The value `x` such that the cumulative probability of the Student's T distribution falls
    /// between `-x` and `x` is equal to the specified `area`.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Student;
    ///
    /// fn main() {
    ///     let area = 0.025_f64;
    ///     let df = 63_f64;
    ///
    ///     let inverse_t = Student::inv(area, df);
    ///
    ///     println!("Inverse of two-tailed CDF with area {}: {}", area, inverse_t);
    /// }
    /// ```
    /// <hr/>
    pub fn inv(area: f64, df: f64) -> f64 {
        if (0_f64..0.5_f64).contains(&area) {
            -Self::upper_inv(1_f64 - area, df)
        } else if area == 0.5_f64 {
            0_f64
        } else if (0.5_f64..1_f64).contains(&area) {
            Self::upper_inv(area, df)
        } else if area <= 0_f64 {
            f64::NEG_INFINITY
        } else {
            f64::INFINITY
        }
    }

    /// Calculates the variance of the Student's T Distribution.
    ///
    /// The variance is a measure of the dispersion of the distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The variance of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Student;
    ///
    /// fn main() {
    ///     let df = 6_f64;
    ///
    ///     let variance = Student::variance(df);
    ///
    ///     println!("Variance: {}", variance);
    /// }
    /// ```
    /// <hr/>
    pub fn variance(df: f64) -> f64 {
        if df > 2_f64 {
            df / (df - 2_f64)
        } else if (1_f64..2_f64).contains(&df) {
            f64::INFINITY
        } else {
            f64::NAN
        }
    }

    /// Calculates the standard deviation of the Student's T Distribution.
    ///
    /// The standard deviation is the square root of the variance and measures the spread of the distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The standard deviation of the distribution.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Student;
    ///
    /// fn main() {
    ///     let df = 6_f64;
    ///
    ///     let sd = Student::sd(df);
    ///
    ///     println!("Standard Deviation: {}", sd);
    /// }
    /// ```
    /// <hr/>
    pub fn sd(df: f64) -> f64 {
        Self::variance(df).sqrt()
    }

    /// Calculates the kurtosis of the Student's T Distribution.
    ///
    /// Kurtosis measures the "tailedness" of the distribution.
    ///
    /// # Parameters
    ///
    /// - `df`: The degrees of freedom parameter.
    ///
    /// # Returns
    ///
    /// The kurtosis of the distribution. Returns NaN if `df` is not greater than 4.
    ///
    /// # Example
    ///
    /// ```
    /// use numerilib::stats::distr::Student;
    ///
    /// fn main() {
    ///     let df = 6_f64;
    ///
    ///     let kurtosis = Student::kurtosis(df);
    ///
    ///     println!("Kurtosis: {}", kurtosis);
    /// }
    /// ```
    /// <hr/>
    pub fn kurtosis(df: f64) -> f64 {
        if df > 4_f64 {
            6_f64 / (df - 4_f64)
        } else if (2_f64..4_f64).contains(&df) {
            f64::INFINITY
        } else {
            f64::NAN
        }
    }
}
