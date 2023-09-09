use crate::Functions;

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
    /// use mathematica::special::Probability;
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
    /// use mathematica::special::Probability;
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
            let numerator = Functions::factorial(n);
            let denominator = Functions::factorial(n - r);
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
    /// use mathematica::special::Probability;
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
    /// use mathematica::special::Probability;
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
            let numerator = Functions::factorial(n);
            let denominator = Functions::factorial(n - r);
            let denominator1 = Functions::factorial(r);
            numerator / (denominator * denominator1)
        } else {
            0_f64
        }
    }
}
