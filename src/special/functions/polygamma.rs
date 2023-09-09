use crate::special::Gamma;
use crate::Functions;

/// Provides methods for calculating polygamma functions.
pub struct Polygamma;

impl Polygamma {
    /// Calculates the digamma function.
    ///
    /// The digamma function, denoted as ψ(z), is the derivative of the natural logarithm of the gamma function.
    ///
    /// # Parameters
    ///
    /// - `z`: The value at which to calculate the digamma function.
    ///
    /// # Returns
    ///
    /// The value of the digamma function at the given parameter `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::special::Polygamma;
    ///
    /// let z = 2.0;
    /// let digamma = Polygamma::digamma(z);
    ///
    /// println!("Digamma({}) = {}", z, digamma);
    /// ```
    /// <hr/>
    pub fn digamma(z: f64) -> f64 {
        let p1 = |t: f64| Gamma::lanczosln(t).exp();
        let dt = Functions::derivative(p1, z);
        dt / Gamma::lanczosln(z).exp()
    }

    /// Calculates the polygamma function of a given degree.
    ///
    /// The polygamma function, denoted as ψ^(n)(z), is the n-th derivative of the digamma function.
    ///
    /// # Parameters
    ///
    /// - `degree`: The degree of the polygamma function. Use -1 for the digamma function and 0 for the first derivative (digamma function).
    /// - `z`: The value at which to calculate the polygamma function.
    ///
    /// # Returns
    ///
    /// The value of the polygamma function of the specified degree at the given parameter `z`.
    ///
    /// # Example 1 (Digamma):
    ///
    /// ```rust
    /// use mathematica::special::Polygamma;
    ///
    /// let degree = 0; // 0 corresponds to the digamma function
    /// let z = 5.0;
    ///
    /// let digamma = Polygamma::polygamma(degree, z);
    ///
    /// println!("Digamma({}) = {}", z, digamma);
    /// ```
    ///
    /// # Example 2:
    ///
    /// ```rust
    /// use mathematica::special::Polygamma;
    ///
    /// let degree = 1;
    /// let z = 5.0;
    ///
    /// let polygamma = Polygamma::polygamma(degree, z);
    ///
    /// println!("Polygamma({},{}) = {}", degree, z, polygamma);
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
