use crate::special::Probability;
use crate::Functions;

const G: f64 = 5f64;

const LG5N7: [f64; 7] = [
    1.000000000189712,
    76.18009172948503,
    -86.50532032927205,
    24.01409824118972,
    -1.2317395783752254,
    0.0012086577526594748,
    -0.00000539702438713199,
];

/// Provides methods for calculating the Gamma function and related functions.
pub struct Gamma;

impl Gamma {
    /// Calculates Stirling's approximation for the Gamma function.
    ///
    /// Stirling's approximation is an approximation for the Gamma function and factorials.
    ///
    /// # Parameters
    ///
    /// - `n`: The value for which to calculate Stirling's approximation.
    ///
    /// # Returns
    ///
    /// The calculated value of Stirling's approximation for the Gamma function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Gamma;
    ///
    /// let n = 2_f64;
    /// let stirling = Gamma::stirling(n);
    ///
    /// println!("Stirling's Approximation for {} is: {}", n, stirling);
    /// ```
    /// <hr/>
    pub fn stirling(n: f64) -> f64 {
        (std::f64::consts::TAU * (n)).sqrt() * ((n) / std::f64::consts::E).powf(n)
    }

    /// Calculates the natural logarithm of the Gamma function using Lanczos approximation.
    ///
    /// The Lanczos approximation provides an efficient way to compute the logarithm of the Gamma function.
    ///
    /// # Parameters
    ///
    /// - `z`: The value for which to calculate the logarithm of the Gamma function.
    ///
    /// # Returns
    ///
    /// The natural logarithm of the Gamma function at the given `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Gamma;
    ///
    /// let n = 6_f64;
    /// let lanczos_ln = Gamma::lanczosln(n);
    ///
    /// println!("Lanczos Logarithm of Gamma for {} is: {}", n, lanczos_ln);
    /// ```
    /// <hr/>
    pub fn lanczosln(z: f64) -> f64 {
        let z = z - 1f64;
        let base = z + G + 0.5;

        let s: f64 = LG5N7
            .iter()
            .skip(1)
            .enumerate()
            .fold(LG5N7[0], |acc, (i, &val)| acc + val / (z + (i + 1) as f64));

        (std::f64::consts::TAU).sqrt().ln() + s.ln() - base + base.ln() * (z + 0.5)
    }

    /// Calculates the Gamma function using Lanczos approximation.
    ///
    /// The Lanczos approximation provides an efficient way to compute the Gamma function.
    ///
    /// # Parameters
    ///
    /// - `z`: The value for which to calculate the Gamma function.
    ///
    /// # Returns
    ///
    /// The calculated value of the Gamma function at the given `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Gamma;
    ///
    /// let n = 6_f64;
    /// let lanczos = Gamma::lanczos(n);
    ///
    /// println!("Lanczos Gamma for {} is: {}", n, lanczos);
    /// ```
    /// <hr/>
    pub fn lanczos(z: f64) -> f64 {
        Self::lanczosln(z).exp()
    }

    /// Calculates the incomplete gamma function.
    ///
    /// The incomplete gamma function represents the integral of the Gamma probability density function
    /// from 0 to `x`, with shape parameter `bound`.
    ///
    /// # Parameters
    ///
    /// - `bound`: The shape parameter of the incomplete gamma function.
    /// - `x`: The upper bound of integration.
    ///
    /// # Returns
    ///
    /// The value of the incomplete gamma function at the given `bound` and `x`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Gamma;
    ///
    /// let bound = 3_f64;
    /// let x = 1_f64;
    ///
    /// let gamma = Gamma::incgamma(bound, x);
    ///
    /// println!("Incomplete Gamma at ({}, {}) is: {}", bound, x, gamma);
    /// ```
    /// <hr/>
    pub fn incgamma(bound: f64, x: f64) -> f64 {
        let func = |z: f64| {
            ((-1_f64).powf(z) * x.powf(bound) * x.powf(z))
                / ((bound + z) * Probability::factorial(z))
        };

        Functions::summation(0_f64, 99_f64, func)
    }

    /// Calculates the complementary incomplete gamma function.
    ///
    /// The complementary incomplete gamma function represents the integral of the Gamma probability density function
    /// from `x` to infinity, with shape parameter `bound`.
    ///
    /// # Parameters
    ///
    /// - `bound`: The shape parameter of the complementary incomplete gamma function.
    /// - `x`: The lower bound of integration.
    ///
    /// # Returns
    ///
    /// The value of the complementary incomplete gamma function at the given `bound` and `x`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Gamma;
    ///
    /// let a = 3_f64;
    /// let x = 1_f64;
    ///
    /// let gamma = Gamma::incgammac(a, x);
    ///
    /// println!("Complementary Incomplete Gamma at ({}, {}) is: {}", a, x, gamma);
    /// ```
    /// <hr/>
    pub fn incgammac(bound: f64, x: f64) -> f64 {
        Self::lanczos(bound) - Self::incgamma(bound, x)
    }

    /// Calculates the regularized incomplete gamma function.
    ///
    /// The regularized incomplete gamma function is the ratio of the incomplete gamma function
    /// to the complete gamma function.
    ///
    /// # Parameters
    ///
    /// - `bound`: The shape parameter of the regularized incomplete gamma function.
    /// - `x`: The upper bound of integration.
    ///
    /// # Returns
    ///
    /// The value of the regularized incomplete gamma function at the given `bound` and `x`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Gamma;
    ///
    /// let bound = 5_f64;
    /// let x = 2_f64;
    ///
    /// let reggamma = Gamma::reggamma(bound, x);
    ///
    /// println!("Regularized Incomplete Gamma at ({}, {}) is: {}", bound, x, reggamma);
    /// ```
    /// <hr/>
    pub fn reggamma(bound: f64, x: f64) -> f64 {
        Self::incgamma(bound, x) / Self::lanczos(bound)
    }

    /// Calculates the cumulative distribution function (CDF) for Poisson random variables.
    ///
    /// The CDF for Poisson random variables gives the probability that a Poisson-distributed random variable
    /// is less than or equal to `x`, with the parameter `bound`.
    ///
    /// # Parameters
    ///
    /// - `bound`: The parameter of the Poisson distribution.
    /// - `x`: The upper bound for the CDF.
    ///
    /// # Returns
    ///
    /// The probability that a Poisson-distributed random variable is less than or equal to `x`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Gamma;
    ///
    /// let bound = 5_f64;
    /// let x = 2_f64;
    ///
    /// let reggammac = Gamma::reggammac(bound, x);
    ///
    /// println!("Poisson CDF at ({}, {}) is: {}", bound, x, reggammac);
    /// ```
    /// <hr/>
    pub fn reggammac(bound: f64, x: f64) -> f64 {
        1_f64 - Self::reggamma(bound, x)
    }
}
