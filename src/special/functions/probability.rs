use crate::special::Functions;

pub struct Probability;

impl Probability {
    /// Implementation of Permutation in rust <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Permutation" target="_blank">Wikipedia Permutation</a> <br>
    /// <hr/>
    ///
    /// # Example #1:
    ///
    /// ```
    /// use ferrate::special::Probability;
    ///
    /// let n = 4_f64;
    /// let r = 3_f64;
    ///
    /// let npr = Probability::permutation(r, n);
    ///
    /// assert_eq!(npr, 24_f64);
    /// ```
    ///
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::special::Probability;
    ///
    /// let n = 2_f64;
    /// let r = 3_f64;
    ///
    /// let npr = Probability::permutation(r, n);
    ///
    /// assert_eq!(npr, 0_f64);
    /// ```
    /// <hr/>
    pub fn permutation(r: f64, n: f64) -> f64 {
        if r <= n {
            let numerator = Functions::factorial(n);
            let denominator = Functions::factorial(n - r);
            numerator / denominator
        } else {
            0_f64
        }
    }
    /// An implementation of Combination in Rust <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Combination" target="_blank">Wikipedia Combination</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example #1:
    ///
    /// ```
    /// use ferrate::special::Probability;
    ///
    /// let n = 4_f64;
    /// let r = 3_f64;
    ///
    /// let ncr = Probability::combination(r, n);
    ///
    /// assert_eq!(ncr, 4_f64);
    /// ```
    ///
    /// <hr/>
    ///
    /// # Example #2:
    ///
    /// ```
    /// use ferrate::special::Probability;
    ///
    /// let n = 2_f64;
    /// let r = 3_f64;
    ///
    /// let ncr = Probability::combination(r, n);
    ///
    /// assert_eq!(ncr, 0_f64);
    /// ```
    /// <hr/>
    pub fn combination(r: f64, n: f64) -> f64 {
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
