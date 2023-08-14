use crate::special::Gamma;
use crate::Functions;

const EPSILON: f64 = 1e-15;

pub struct Beta;

impl Beta {
    /// Calculates the Log Beta Function by using the Gamma Function definition of the Function <br>
    /// Learn more about the Log Beta Function at: <a href="https://math.stackexchange.com/questions/3922187/what-is-the-log-of-the-beta-function-how-can-it-be-simplified" target="_blank">Math Stack Exchange</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::special::Beta;
    ///
    /// let z1 = 1_f64;
    /// let z2 = 2_f64;
    /// let lnbeta = Beta::lnbeta(z1, z2);
    ///
    /// assert_eq!(lnbeta, -0.6931471805616405);
    /// ```
    /// <hr/>
    pub fn lnbeta(z1: f64, z2: f64) -> f64 {
        Gamma::lanczosln(z1) + Gamma::lanczosln(z2) - Gamma::lanczosln(z1 + z2)
    }
    /// Raises the Log Beta Function to the power of e <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Beta_function" target="_blank">Wikipedia Beta Function</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::special::Beta;
    ///
    /// let z1 = 8_f64;
    /// let z2 = 7_f64;
    ///
    /// let beta = Beta::beta(z1, z2);
    ///
    /// assert_eq!(beta, 4.162504162405661e-5);
    /// ```
    /// <hr/>
    pub fn beta(z1: f64, z2: f64) -> f64 {
        Beta::lnbeta(z1, z2).exp()
    }
    /// This is the definition of the Incomplete Beta Function <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Beta_function#Incomplete_beta_function" target="_blank">Wikipedia Incomplete Beta Function</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::special::Beta;
    ///
    /// let x = 1_f64 / 7_f64;
    /// let z1 = 0.5_f64;
    /// let z2 = 3_f64;
    ///
    /// let incbeta = Beta::incbeta(x, z1, z2);
    ///
    /// assert_eq!(incbeta, 0.6870211373344728_f64)
    /// ```
    /// <hr/>
    pub fn incbeta(x: f64, z1: f64, z2: f64) -> f64 {
        let reg = Beta::regincbeta(z1, z2, x);
        let beta = Beta::beta(z1, z2);
        reg * beta
    }
    /// The Series Definition of the Regularized Incomplete Beta Function <br>
    /// Learn more at: <a href="https://mathworld.wolfram.com/RegularizedBetaFunction.html" target="_blank">Wolfram MathWorld</a> <br>
    /// This was a Rust implementation of this paper, see it at: <a href="http://www.phys.uri.edu/nigh/NumRec/bookfpdf/f6-4.pdf" target="_blank">University of Rhode Island</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::special::Beta;
    ///
    /// let x = 1_f64 / 7_f64;
    /// let z1 = 1_f64 / 2_f64;
    /// let z2 = 3_f64;
    ///
    /// let regincbeta = Beta::regincbeta(z1, z2, x);
    ///
    /// assert_eq!(regincbeta, 0.6440823162530317);
    /// ```
    /// <hr/>
    pub fn regincbeta(z1: f64, z2: f64, x: f64) -> f64 {
        if !(0_f64..=1_f64).contains(&x) {
            return f64::INFINITY;
        }

        let mut result = 0_f64;

        let y = (x.powf(z1) * (1_f64 - x).powf(z2)) / (z1 * Beta::beta(z1, z2));

        for i in 0..99 {
            result += (Beta::beta(z1 + 1_f64, i as f64 + 1_f64)
                / Beta::beta(z1 + z2, i as f64 + 1_f64))
                * x.powf(i as f64 + 1_f64);
        }

        (result + 1_f64) * y
    }
    /// An approximation of the Inverse of the Regularized Incomplete Beta Function. <br>
    /// Learn more at: <a href="https://math.stackexchange.com/questions/2486667/can-anyone-explain-the-inverse-regularized-beta-function" target="_blank">Math Stack Exchange</a> <br>
    /// and: <a href="https://functions.wolfram.com/GammaBetaErf/InverseBetaRegularized/" target="_blank">Wolfram Math World Inverse Regularized Beta</a> <br>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use ferrate::special::Beta;
    ///
    /// let z1 = 1_f64;
    /// let z2 = 2_f64;
    /// let x = 0.590401_f64;
    ///
    /// let inverse = Beta::invregincbeta(z1, z2, x);
    ///
    /// assert_eq!(inverse, 0.3600007812492397_f64);
    /// ```
    pub fn invregincbeta(z1: f64, z2: f64, x: f64) -> f64 {
        if !(0_f64..=1_f64).contains(&x) {
            return f64::NAN;
        }

        let target_prob = x;
        let func = |t: f64| Beta::regincbeta(z1, z2, t) - target_prob;

        let mut guess = 0.5;

        if x < 1e-1 {
            let mut low = 0.0;
            let mut high = 1.0;
            while high - low > EPSILON {
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
