/*  I gotta give credit where credit is due. I took the Lanczos Approximation from
   Axect's Lanczos repo at: https://github.com/Axect/Lanczos.git
*/

use crate::Functions;

const G: f64 = 5f64;

const LG5N7: [f64; 7] = [
    1.000000000189712,
    76.18009172948503,
    -86.50532032927205,
    24.01409824118972,
    -1.2317395783752254,
    0.0012086577526594748,
    -0.00000539702438713199,
];

pub struct Gamma;

impl Gamma {
    /// Stirling's Approximation for the Gamma Function (Also Factorials) <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Stirling's_approximation" target="_blank">Wikipedia Stirling's Approximation</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::special::Gamma;
    ///
    /// let n = 2_f64;
    /// let stirling = Gamma::stirling(n);
    ///
    /// assert_eq!(stirling, 1.9190043514889832_f64);
    /// ```
    /// <hr/>
    pub fn stirling(n: f64) -> f64 {
        (std::f64::consts::TAU * (n)).sqrt() * ((n) / std::f64::consts::E).powf(n)
    }
    /// <hr/>
    ///
    /// # !!! I gotta give credit where credit is due. I took the approximation from Axect's repo at: <https://github.com/Axect/Lanczos.git> !!! <br>
    ///
    /// <hr/>
    /// This is an approximation of the Log Gamma Function using Lanczos Approximation. <br>
    /// Learn more about the Log Gamma Function at: <a href="https://wikipedia.org/wiki/Gamma_function#The_log-gamma_function" target="_blank">Wikipedia Log Gamma</a> <br>
    /// Learn more about Lanczos Approximation at: <a href="https://wikipedia.org/wiki/Lanczos_approximation" target="_blank">Wikipedia Lanczos Approximation</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::special::Gamma;
    ///
    /// let n = 6_f64;
    /// let lanczos_ln = Gamma::lanczosln(n);
    ///
    /// assert_eq!(lanczos_ln, 4.787491742764145);
    /// ```
    /// <hr/>
    pub fn lanczosln(z: f64) -> f64 {
        let z = z - 1f64;
        let base = z + G + 0.5;

        let s: f64 = LG5N7
            .iter()
            .skip(1)
            .enumerate()
            .fold(LG5N7[0], |acc, (i, &val)| acc + val / (z + (i + 1) as f64));

        (2f64 * std::f64::consts::PI).sqrt().ln() + s.ln() - base + base.ln() * (z + 0.5)
    }
    /// Uses the Lanczos Approximation for the Log Gamma Function and raises it to the power of e <br>
    /// Learn More about the Gamma Function at: <a href="https://wikipedia.org/wiki/Gamma_function" target="_blank">Wikipedia Gamma Function</a> <br>
    /// Learn more about Lanczos Approximation at: <a href="https://wikipedia.org/wiki/Lanczos_approximation" target="_blank">Wikipedia Lanczos Approximation</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::special::Gamma;
    ///
    /// let n = 6_f64;
    /// let lanczos = Gamma::lanczos(n);
    ///
    /// assert_eq!(lanczos, 120_f64);
    /// ```
    /// <hr/>
    pub fn lanczos(z: f64) -> f64 {
        let exp = Gamma::lanczosln(z).exp();

        if ((exp * 1e+6).ceil() - 1e-6) < (exp * 1e+6)
            || ((exp * 1e+6).floor() + 1e+6) > (exp * 1e+6)
        {
            ((exp * 1e+6).round()) / 1e+6
        } else {
            exp
        }
    }
    /// An implementation of the Lower Incomplete Gamma Function <br>
    /// learn more at: <a href="https://www.mathworks.com/help/symbolic/sym.igamma.html?s_tid=doc_ta#bt6_4j4-4" target="_blank">MatLab IGamma</a> <br>
    /// and: <a href="https://www.boost.org/doc/libs/1_35_0/libs/math/doc/sf_and_dist/html/math_toolkit/special/sf_gamma/igamma.html" target="_blank">Boost Incomplete Gamma Functions</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use ferrate::special::Gamma;
    ///
    /// let bound = 3_f64;
    /// let x = 1_f64;
    ///
    /// let gamma = Gamma::incgamma(bound, x);
    ///
    /// assert_eq!(gamma, 0.16060279414278839_f64);
    ///
    /// ```
    /// <hr/>
    pub fn incgamma(bound: f64, x: f64) -> f64 {
        let mut result = 0_f64;

        for i in 0..99 {
            result += (((-1_f64).powi(i) * (x).powi(i))
                / ((bound + i as f64) * Functions::factorial(i as f64)))
                * x.powf(bound);
        }

        result
    }
    /// An implementation of the Upper Incomplete Gamma Function <br>
    /// Learn more at: <a href="https://www.mathworks.com/help/symbolic/sym.igamma.html?s_tid=doc_ta#bt6_4j4-4" target="_blank">MatLab IGamma</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use ferrate::special::Gamma;
    ///
    /// let a = 3_f64;
    /// let x = 1_f64;
    ///
    /// let gamma = Gamma::incgammac(a, x);
    ///
    /// assert_eq!(gamma, 1.8393972058572117_f64);
    /// ```
    /// <hr/>
    pub fn incgammac(bound: f64, x: f64) -> f64 {
        Gamma::lanczos(bound) - Gamma::incgamma(bound, x)
    }
    /// An implementation of the Regularized Incomplete Gamma Function <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Incomplete_gamma_function#Regularized_gamma_functions_and_Poisson_random_variables" target="_blank">Wikipedia Regularized Gamma Functions</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use ferrate::special::Gamma;
    ///
    /// let bound = 5_f64;
    /// let x = 2_f64;
    ///
    /// let reggamma = Gamma::reggamma(bound, x);
    ///
    /// assert_eq!(reggamma, 0.052653017343711174_f64);
    /// ```
    /// <hr/>
    pub fn reggamma(bound: f64, x: f64) -> f64 {
        Gamma::incgamma(bound, x) / Gamma::lanczos(bound)
    }
    /// The CDF for the Poisson Random Variables <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Incomplete_gamma_function#Regularized_gamma_functions_and_Poisson_random_variables" target="_blank">Wikipedia Poisson Random Variables</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use ferrate::special::Gamma;
    ///
    /// let bound = 5_f64;
    /// let x = 2_f64;
    ///
    /// let reggammac = Gamma::reggammac(bound, x);
    ///
    /// assert_eq!(reggammac, 0.9473469826562888_f64);
    /// ```
    /// <hr/>
    pub fn reggammac(bound: f64, x: f64) -> f64 {
        1_f64 - Gamma::reggamma(bound, x)
    }
}
