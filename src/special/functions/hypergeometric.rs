use crate::Functions;

/// Provides methods for calculating hypergeometric functions.
pub struct Hypergeometric;

impl Hypergeometric {
    /// Calculates the Gaussian hypergeometric function.
    ///
    /// The Gaussian hypergeometric function is a special function defined by a series involving
    /// the Pochhammer notation and factorials.
    ///
    /// # Parameters
    ///
    /// - `a`: The first parameter of the Gaussian hypergeometric function.
    /// - `b`: The second parameter of the Gaussian hypergeometric function.
    /// - `c`: The third parameter of the Gaussian hypergeometric function.
    /// - `z`: The value at which to calculate the function.
    ///
    /// # Returns
    ///
    /// The value of the Gaussian hypergeometric function at the given parameters `a`, `b`, `c`, and `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::special::Hypergeometric;
    ///
    /// let a = 1.0;
    /// let b = 2.0;
    /// let c = 3.0;
    /// let z = 0.5;
    ///
    /// let result = Hypergeometric::gaussian(a, b, c, z);
    ///
    /// println!("Gaussian Hypergeometric({}, {}, {}, {}) = {}", a, b, c, z, result);
    /// ```
    /// <hr/>
    pub fn gaussian(a: f64, b: f64, c: f64, z: f64) -> f64 {
        let mut result = 0_f64;

        for k in 0..70 {
            result += ((Functions::pochhammer(a, k as f64) * Functions::pochhammer(b, k as f64))
                / Functions::pochhammer(c, k as f64))
                * (z.powf(k as f64) / Functions::factorial(k as f64))
        }

        result
    }

    /// Calculates the Kummer hypergeometric function.
    ///
    /// The Kummer hypergeometric function is a special function defined by a series involving
    /// the Pochhammer notation and factorials.
    ///
    /// # Parameters
    ///
    /// - `a`: The first parameter of the Kummer hypergeometric function.
    /// - `b`: The second parameter of the Kummer hypergeometric function.
    /// - `z`: The value at which to calculate the function.
    ///
    /// # Returns
    ///
    /// The value of the Kummer hypergeometric function at the given parameters `a`, `b`, and `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::special::Hypergeometric;
    ///
    /// let a = 1.0;
    /// let b = 2.0;
    /// let z = 0.5;
    ///
    /// let result = Hypergeometric::kummer(a, b, z);
    ///
    /// println!("Kummer Hypergeometric({}, {}, {}) = {}", a, b, z, result);
    /// ```
    /// <hr/>
    pub fn kummer(a: f64, b: f64, z: f64) -> f64 {
        let mut result = 0_f64;

        for k in 0..99 {
            result += ((Functions::pochhammer(a, k as f64)) / Functions::pochhammer(b, k as f64))
                * (z.powf(k as f64) / Functions::factorial(k as f64))
        }

        result
    }

    /// Calculates the Whittaker hypergeometric function.
    ///
    /// The Whittaker hypergeometric function is a special function defined in terms of the Kummer hypergeometric function.
    ///
    /// # Parameters
    ///
    /// - `k`: The first parameter of the Whittaker hypergeometric function.
    /// - `m`: The second parameter of the Whittaker hypergeometric function.
    /// - `z`: The value at which to calculate the function.
    ///
    /// # Returns
    ///
    /// The value of the Whittaker hypergeometric function at the given parameters `k`, `m`, and `z`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use mathematica::special::Hypergeometric;
    ///
    /// let k = 1.0;
    /// let m = 2.0;
    /// let z = 0.5;
    ///
    /// let result = Hypergeometric::whittaker(k, m, z);
    ///
    /// println!("Whittaker Hypergeometric({}, {}, {}) = {}", k, m, z, result);
    /// ```
    /// <hr/>
    pub fn whittaker(k: f64, m: f64, z: f64) -> f64 {
        std::f64::consts::E.powf(-z / 2_f64)
            * z.powf(m + (1_f64 / 2_f64))
            * Self::kummer(m - k + (1_f64 / 2_f64), (2_f64 * m) + 1_f64, z)
    }
}
