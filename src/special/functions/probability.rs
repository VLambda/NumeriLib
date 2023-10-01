use crate::special::Gamma;

/// Provides methods for performing probability-related calculations.
pub struct Probability;

impl Probability {
    /// Calculates the permutation of `r` items from a set of `n` items.
    ///
    /// A permutation is an ordered arrangement of items, and this function calculates the number
    /// of permutations possible given `r` items from a set of `n` items.
    ///
    /// # Parameters
    ///
    /// - `r`: The number of items to arrange (permutations).
    /// - `n`: The total number of items in the set.
    ///
    /// # Returns
    ///
    /// The number of permutations of `r` items from a set of `n` items.
    ///
    /// # Example 1:
    ///
    /// ```rust
    /// use numerilib::special::Probability;
    ///
    /// let n = 4.0;
    /// let r = 3.0;
    ///
    /// let npr = Probability::permutation(r, n);
    ///
    /// println!("Permutations({}, {}) = {}", r, n, npr);
    /// ```
    ///
    /// # Example 2:
    ///
    /// ```rust
    /// use numerilib::special::Probability;
    ///
    /// let n = 2.0;
    /// let r = 3.0;
    ///
    /// let npr = Probability::permutation(r, n);
    ///
    /// println!("Permutations({}, {}) = {}", r, n, npr);
    /// ```
    /// <hr/>
    pub fn permutation(n: f64, r: f64) -> f64 {
        if r <= n {
            let numerator = Self::factorial(n);
            let denominator = Self::factorial(n - r);
            numerator / denominator
        } else {
            0_f64
        }
    }

    /// Calculates the combination of `r` items from a set of `n` items.
    ///
    /// A combination is a selection of items without regard to the order. This function calculates
    /// the number of combinations possible given `r` items from a set of `n` items.
    ///
    /// # Parameters
    ///
    /// - `r`: The number of items to select (combinations).
    /// - `n`: The total number of items in the set.
    ///
    /// # Returns
    ///
    /// The number of combinations of `r` items from a set of `n` items.
    ///
    /// # Example 1:
    ///
    /// ```rust
    /// use numerilib::special::Probability;
    ///
    /// let n = 4.0;
    /// let r = 3.0;
    ///
    /// let ncr = Probability::combination(r, n);
    ///
    /// println!("Combinations({}, {}) = {}", r, n, ncr);
    /// ```
    ///
    /// # Example 2:
    ///
    /// ```rust
    /// use numerilib::special::Probability;
    ///
    /// let n = 2.0;
    /// let r = 3.0;
    ///
    /// let ncr = Probability::combination(r, n);
    ///
    /// println!("Combinations({}, {}) = {}", r, n, ncr);
    /// ```
    /// <hr/>
    pub fn combination(n: f64, r: f64) -> f64 {
        if r <= n {
            let numerator = Self::factorial(n);
            let denominator = Self::factorial(n - r);
            let denominator1 = Self::factorial(r);
            numerator / (denominator * denominator1)
        } else {
            0_f64
        }
    }

    /// Calculates a Factorial by using Lanczos's Gamma Function Approximation.
    ///
    /// # Parameters
    ///
    /// - `n`: The value for which the factorial is calculated.
    ///
    /// # Returns
    ///
    /// The factorial of the given value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Probability;
    ///
    /// fn main() {
    ///     let n = 6_f64;
    ///     let factorial = Probability::factorial(n);
    ///
    ///     println!("6! is: {}", factorial);
    /// }
    /// ```
    /// <hr/>
    pub fn factorial(n: f64) -> f64 {
        if n.floor() == n {
            (1..=n as u64).map(|i| i as f64).product()
        } else {
            Gamma::lanczos(n + 1_f64)
        }
    }

    /// Pochhammer's Function in Rust.
    ///
    /// # Parameters
    ///
    /// - `x`: The base value.
    /// - `n`: The exponent value.
    ///
    /// # Returns
    ///
    /// The value of Pochhammer's function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Probability;
    ///
    /// fn main() {
    ///     let x = 2_f64;
    ///     let n = 3_f64;
    ///     let poch = Probability::pochhammer(x, n);
    ///
    ///     println!("The Rising Factorial of 2^n n=3 is: {}", poch);
    /// }
    /// ```
    /// <hr/>
    pub fn pochhammer(x: f64, n: f64) -> f64 {
        Gamma::lanczos(x + n) / Gamma::lanczos(x)
    }

    /// Falling Factorials in Rust.
    ///
    /// # Parameters
    ///
    /// - `x`: The base of the falling factorial.
    /// - `n`: The number of falling factorial terms.
    ///
    /// # Returns
    ///
    /// The value of the falling factorial.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Probability;
    ///
    /// fn main() {
    ///     let x = 3_f64;
    ///     let n = 2_f64;
    ///
    ///     let fall = Probability::falling_factorial(x, n);
    ///
    ///     println!("The Falling Factorial of 3^n where n=2 is: {}", fall);
    /// }
    /// ```
    /// <hr/>                                       
    pub fn falling_factorial(x: f64, n: f64) -> f64 {
        Gamma::lanczos(x + 1_f64) / Gamma::lanczos(x - n + 1_f64)
    }
}
