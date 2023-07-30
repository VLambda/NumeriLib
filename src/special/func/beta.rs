use crate::special::{Functions, Gamma};
use crate::extra::Extra;

const LIMIT: f64 = 1e-8;
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
    /// use vml::special::Beta;
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
    /// use vml::special::Beta;
    ///
    /// let z1 = 1_f64;
    /// let z2 = 2_f64;
    ///
    /// let beta = Beta::beta(z1, z2);
    ///
    /// assert_eq!(beta, 0.5);
    /// ```
    /// <hr/>
    pub fn beta(z1:f64, z2:f64) -> f64 {
        let eval = Beta::lnbeta(z1, z2).exp();
        Extra::round(eval)
    }
    /// This is the definition of the Incomplete Beta Function <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Beta_function#Incomplete_beta_function" target="_blank">Wikipedia Incomplete Beta Function</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use vml::special::Beta;
    ///
    /// let x = 1_f64 / 7_f64;
    /// let z1 = 0.5;
    /// let z2 = 3_f64;
    ///
    /// let incbeta = Beta::incbeta(x, z1, z2);
    ///
    /// assert_eq!(incbeta, 0.687021137333779)
    /// ```
    /// <hr/>
    pub fn incbeta(x: f64, z1: f64, z2: f64) -> f64 {
        let reg = Beta::regincbeta(z1, z2, x);
        let beta = Beta::beta(z1, z2);
        reg * beta
    }
    /// The Defintion of the Regularized Incomplete Beta Function <br>
    /// Learn more at: <a href="https://mathworld.wolfram.com/RegularizedBetaFunction.html" target="_blank">Wolfram MathWorld</a> <br>
    /// This was a Rust implementation of Codeplea's incbeta function, which was originally written in C, see it at: <a href="https://github.com/codeplea/incbeta" target="_blank">Codeplea's incbeta Github Repo</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use vml::special::Beta;
    ///
    /// let x = 0.8_f64;
    /// let z1 = 1_f64;
    /// let z2 = 2_f64;
    ///
    /// let regincbeta = Beta::regincbeta(z1, z2, x);
    ///
    /// assert_eq!(regincbeta, 0.96);
    /// ```
    /// <hr/>
    pub fn regincbeta(z1: f64, z2: f64, x: f64) -> f64 {
        if !(0_f64..=1_f64).contains(&x) {
            return f64::INFINITY;
        }

        /* The continued fraction converges nicely for x < (a+1)/(a+b+2) */
        if x > (z1 + 1_f64) / (z1 + z2 + 2_f64) {
            return 1_f64 - Beta::regincbeta(z2, z1, 1_f64 - x); /* Use the fact that beta is symmetrical. */
        }

        /* Find the first part before the continued fraction. */
        let lbeta = Beta::lnbeta(z1,z2);
        let front = (x.ln() * z1 + (1_f64 - x).ln() * z2 - lbeta).exp() / z1;

        /* Use Lentz's algorithm to evaluate the continued fraction. */
        let mut f = 1_f64;
        let mut c = 1_f64;
        let mut d = 0_f64;

        for i in 0..=200 {
            let m = i / 2;
            let n = m as f64;
            let numerator = if i == 0 {
                1_f64 /* First numerator is 1_f64. */
            } else if i % 2 == 0 {
                (n * (z2 - n) * x) / ((z1 + 2_f64 * n - 1_f64) * (z1 + 2_f64 * n)) /* Even term. */
            } else {
                -((z1 + n) * (z1 + z2 + n) * x) / ((z1 + 2_f64 * n) * (z1 + 2_f64 * n + 1_f64)) /* Odd term. */
            };

            /* Do an iteration of Lentz's algorithm. */
            d = 1_f64 + numerator * d;
            if d.abs() < EPSILON {
                d = EPSILON;
            }
            d = 1_f64 / d;

            c = 1_f64 + numerator / c;
            if c.abs() < EPSILON {
                c = EPSILON;
            }

            let cd = c * d;
            f *= cd;

            /* Check for stop. */
            if (1_f64 - cd).abs() < LIMIT {
                return Extra::round(front * (f - 1_f64));
            }
        }
        f64::INFINITY /* Needed more loops, did not converge. */
    }
    /// An approximation of the Inverse of the Regularized Incomplete Beta Function. <br>
    /// Learn more at: <a href="https://math.stackexchange.com/questions/2486667/can-anyone-explain-the-inverse-regularized-beta-function" target="_blank">Math Stack Exchange</a> <br>
    /// and: <a href="https://functions.wolfram.com/GammaBetaErf/InverseBetaRegularized/" target="_blank">Wolfram Math World Inverse Regularized Beta</a> <br>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use vml::special::Beta;
    ///
    /// let z1 = 1_f64;
    /// let z2 = 2_f64;
    /// let x = 0.96_f64;
    ///
    /// let inverse = Beta::invregincbeta(z1, z2, x);
    ///
    /// assert_eq!(inverse, 0.8_f64);
    /// ```
    pub fn invregincbeta(z1: f64, z2: f64, x: f64) -> f64 {
        if !(0_f64..=1_f64).contains(&x) {
            return f64::NAN;
        }

        let target_prob = x;
        let func = |t: f64| Beta::regincbeta(z1, z2, t) - target_prob;

        let mut guess = 0.5;

        if x < 0.1_f64 {
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