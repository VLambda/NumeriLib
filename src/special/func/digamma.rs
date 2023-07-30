use crate::special::{Functions, Gamma};

pub struct Digamma;

impl Digamma {
    /// The Defintion of the Digamma function in Rust <br>
    /// Learn more at: <a href="https://en.wikipedia.org/wiki/Digamma_function" target="_blank">Wikipedia Digamma Function</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use vml::special::Digamma;
    ///
    /// let z = 2_f64;
    /// let digamma = Digamma::digamma(z);
    ///
    /// assert_eq!(digamma, 0.42278438084235914);
    /// ```
    pub fn digamma(z: f64) -> f64 {
        let p1 = |t: f64| Gamma::lanczosln(t).exp();
        let dt = Functions::derivative(p1, z);
        dt / Gamma::lanczosln(z).exp()
    }
}