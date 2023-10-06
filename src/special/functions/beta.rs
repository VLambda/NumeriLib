use crate::extra::Extra;
use crate::special::Gamma;
use crate::Functions;

/// A module containing functions to work with the Beta Functions.
pub struct Beta;

impl Beta {
    /// Calculates the natural logarithm of the Beta function using the definition of the Beta function.
    ///
    /// The Beta function, B(z1, z2), is defined as B(z1, z2) = Γ(z1) * Γ(z2) / Γ(z1 + z2), where Γ represents the gamma function.
    ///
    /// # Parameters
    ///
    /// - `z1`: The first parameter of the Beta function.
    /// - `z2`: The second parameter of the Beta function.
    ///
    /// # Returns
    ///
    /// The natural logarithm of the Beta function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Beta;
    ///
    /// let z1 = 1_f64;
    /// let z2 = 2_f64;
    /// let lnbeta = Beta::lnbeta(z1, z2);
    ///
    /// println!("Natural Logarithm of Beta({}, {}) is: {}", z1, z2, lnbeta);
    /// ```
    /// <hr/>
    pub fn lnbeta(z1: f64, z2: f64) -> f64 {
        Gamma::lanczosln(z1) + Gamma::lanczosln(z2) - Gamma::lanczosln(z1 + z2)
    }

    /// Calculates the Beta function using the natural logarithm of the Beta function.
    ///
    /// The Beta function, B(z1, z2), is defined as B(z1, z2) = Γ(z1) * Γ(z2) / Γ(z1 + z2), where Γ represents the gamma function.
    ///
    /// # Parameters
    ///
    /// - `z1`: The first parameter of the Beta function.
    /// - `z2`: The second parameter of the Beta function.
    ///
    /// # Returns
    ///
    /// The value of the Beta function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Beta;
    ///
    /// let z1 = 8_f64;
    /// let z2 = 7_f64;
    /// let beta = Beta::beta(z1, z2);
    ///
    /// println!("Beta({}, {}) is: {}", z1, z2, beta);
    /// ```
    /// <hr/>
    pub fn beta(z1: f64, z2: f64) -> f64 {
        Self::lnbeta(z1, z2).exp()
    }

    /// Calculates the Incomplete Beta function (I_x(z1, z2)).
    ///
    /// # Parameters
    ///
    /// - `x`: The upper limit of integration.
    /// - `z1`: The first parameter of the Beta function.
    /// - `z2`: The second parameter of the Beta function.
    ///
    /// # Returns
    ///
    /// The value of the regularized incomplete Beta function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Beta;
    ///
    /// let x = 0.2_f64;
    /// let z1 = 2.0_f64;
    /// let z2 = 3.0_f64;
    /// let incbeta = Beta::incbeta(x, z1, z2);
    ///
    /// println!("Incomplete Beta({}, {}, {}) is: {}", x, z1, z2, incbeta);
    /// ```
    /// <hr/>
    pub fn incbeta(z1: f64, z2: f64, x: f64) -> f64 {
        let reg = Self::regincbeta(z1, z2, x);
        let beta = Self::beta(z1, z2);
        reg * beta
    }

    /// Calculates the regularized incomplete Beta function using a series definition.
    ///
    /// The regularized incomplete Beta function, I_x(z1, z2), is defined as (1/B(z1, z2)) * ∫[0 to x] t^(z1-1) * (1-t)^(z2-1) dt, where B represents the Beta function.
    ///
    /// # Parameters
    ///
    /// - `z1`: The first parameter of the Beta function.
    /// - `z2`: The second parameter of the Beta function.
    /// - `x`: The upper limit of integration.
    ///
    /// # Returns
    ///
    /// The value of the regularized incomplete Beta function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Beta;
    ///
    /// let x = 1_f64 / 7_f64;
    /// let z1 = 1_f64 / 2_f64;
    /// let z2 = 3_f64;
    /// let regincbeta = Beta::regincbeta(z1, z2, x);
    ///
    /// println!("Regularized Incomplete Beta({}, {}, {}) is: {}", z1, z2, x, regincbeta);
    /// ```
    /// <hr/>
    pub fn regincbeta(z1: f64, z2: f64, x: f64) -> f64 {
        if !(0_f64..=1_f64).contains(&x) {
            return f64::INFINITY;
        }

        let y = (x.powf(z1) * (1_f64 - x).powf(z2)) / (z1 * Beta::beta(z1, z2));

        let func = |i: f64| {
            (Self::beta(z1 + 1_f64, i + 1_f64) / Self::beta(z1 + z2, i + 1_f64)) * x.powf(i + 1_f64)
        };

        let result = Functions::summation(0_f64, 99_f64, func);

        (result + 1_f64) * y
    }

    /// Approximates the inverse of the regularized incomplete Beta function.
    ///
    /// The regularized incomplete Beta function, I_x(z1, z2), is defined as (1/B(z1, z2)) * ∫[0 to x] t^(z1-1) * (1-t)^(z2-1) dt, where B represents the Beta function.
    ///
    /// # Parameters
    ///
    /// - `z1`: The first parameter of the Beta function.
    /// - `z2`: The second parameter of the Beta function.
    /// - `x`: The target probability, typically a value between 0 and 1.
    ///
    /// # Returns
    ///
    /// An approximation of the inverse of the regularized incomplete Beta function.
    ///
    /// # Example
    ///
    /// ```rust
    /// use numerilib::special::Beta;
    ///
    /// let z1 = 1_f64;
    /// let z2 = 2_f64;
    /// let x = 0.590401_f64;
    /// let inverse = Beta::invregincbeta(z1, z2, x);
    ///
    /// println!("Inverse Regularized Incomplete Beta({}, {}, {}) is: {}", z1, z2, x, inverse);
    /// ```
    /// <hr/>
    pub fn invregincbeta(z1: f64, z2: f64, x: f64) -> f64 {
        if !(0_f64..1_f64).contains(&x) {
            return f64::NAN;
        }

        let target_prob = x;
        let func = |t: f64| Beta::regincbeta(z1, z2, t) - target_prob;

        let mut guess = 0.5;

        if x - 1e-1 <= 1e-6 {
            let mut low = 0.0;
            let mut high = 1.0;
            while high - low > Extra::EPSILON2 {
                if func(guess) < 0.0 {
                    low = guess;
                } else {
                    high = guess;
                }

                guess = (low + high) / 2.0;
            }
        }

        Functions::newmet(guess, func)
    }
}
