use crate::special::Gamma;
use crate::Functions;

pub struct Polygamma;

impl Polygamma {
    /// The Defintion of the Digamma function in Rust <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Digamma_function" target="_blank">Wikipedia Digamma Function</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::special::Polygamma;
    ///
    /// let z = 2_f64;
    /// let digamma = Polygamma::digamma(z);
    ///
    /// assert_eq!(digamma, 0.42278438084235914);
    /// ```
    pub fn digamma(z: f64) -> f64 {
        let p1 = |t: f64| Gamma::lanczosln(t).exp();
        let dt = Functions::derivative(p1, z);
        dt / Gamma::lanczosln(z).exp()
    }
    /// The Polygamma Functions in Rust <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Polygamma_function" target="_blank">Wikipedia Polygamma Functions</a> <br>
    ///
    ///
    /// # Example #1:
    ///
    /// ```
    /// use ferrate::special::Polygamma;
    ///
    /// let degree = 4;
    /// let z = 5_f64;
    ///
    /// let polygamma = Polygamma::polygamma(degree, z);
    ///
    /// assert_eq!(polygamma, -0.014063191342111519_f64)
    /// ```
    /// <hr/>
    ///
    /// # Example 2:
    ///
    /// ```
    /// use ferrate::special::Polygamma;
    ///
    /// let degree = -1;
    /// let z = 5_f64;
    ///
    /// let polygamma = Polygamma::polygamma(degree, z);
    ///
    /// assert_eq!(polygamma, 3.1780538303306463_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example 3:
    ///
    /// ```
    /// use ferrate::special::Polygamma;
    ///
    /// let degree = 0;
    /// let z = 5_f64;
    ///
    /// let polygamma = Polygamma::polygamma(degree, z);
    ///
    /// assert_eq!(polygamma, 1.5061177964312848_f64);
    /// ```
    /// <hr/>
    pub fn polygamma(degree: i32, z: f64) -> f64 {
        if degree == -1 {
            Gamma::lanczosln(z)
        } else if degree == 0 {
            return Polygamma::digamma(z);
        } else {
            let mut result = 0_f64;

            for i in 0..99999 {
                result += (-1_f64).powi(degree + 1)
                    * Functions::factorial(degree as f64)
                    * (1_f64 / (z + i as f64).powi(degree + 1));
            }

            result
        }
    }
}
