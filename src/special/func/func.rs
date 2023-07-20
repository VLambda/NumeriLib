use crate::special::Gamma;

pub struct Functions;

impl Functions {
    /// A simple implementation of the Error Function that is used to Calculate The NormalCDF Function. <br>
    /// Learn more about the Error Function at: <https://wikipedia.org/wiki/Error_function>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use vml::special::Functions;
    ///
    /// let bound = 4_f64;
    /// let low_erf = Functions::lower_erf(bound);
    ///
    /// assert_eq!(low_erf, -0.9999999845827403);
    /// ```
    /// <hr/>
    ///
    pub fn lower_erf(bound: f64) -> f64 {
        let f = |x: f64| (std::f64::consts::E).powf(-1_f64 * (x.powi(2)));

        let error_function: f64 = (2.0 / std::f64::consts::PI.sqrt()) * Functions::integral(bound, 0_f64, f).unwrap();
        error_function
    }
    /// A traditional implementation of the standard Error Function. <br>
    /// Learn more about the Error Function at: <https://wikipedia.org/wiki/Error_function>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use vml::special::Functions;
    ///
    /// let bound = 4_f64;
    /// let low_erf = Functions::erf(bound);
    ///
    /// assert_eq!(low_erf, 0.9999999845827289);
    /// ```
    /// <hr/>
    ///
    pub fn erf(bound: f64) -> f64 {
        let f = |x: f64| (std::f64::consts::E).powf(-1_f64 * (x.powi(2)));

        let error_function: f64 = (2.0 / std::f64::consts::PI.sqrt()) * Functions::integral(0_f64, bound, f).unwrap();
        error_function
    }
    /// A Rust implementation of the this approximation by Alijah Ahmed at: <https://scistatcalc.blogspot.com/2013/09/numerical-estimate-of-inverse-error.html>
    ///
    /// # Example
    ///
    /// ```
    /// use vml::special::Functions;
    ///
    /// let x = 0.975;
    /// let inverf = Functions::inverf(x);
    ///
    /// assert_eq!(inverf, 1.584911068059619);
    /// ```
    /// <hr/>
    ///
    pub fn inverf(x: f64) -> f64 {
        let w = -((1.0 - x) * (1.0 + x)).ln();
        let mut p;

        if w < 5.0 {
            let w_minus_2_5 = w - 2.5;
            p = 2.81022636e-08;
            p = 3.43273939e-07 + p * w_minus_2_5;
            p = -3.5233877e-06 + p * w_minus_2_5;
            p = -4.39150654e-06 + p * w_minus_2_5;
            p = 0.00021858087 + p * w_minus_2_5;
            p = -0.00125372503 + p * w_minus_2_5;
            p = -0.00417768164 + p * w_minus_2_5;
            p = 0.246640727 + p * w_minus_2_5;
            p = 1.50140941 + p * w_minus_2_5;
        } else {
            let sqrt_w_minus_3 = (w).sqrt() - 3.0;
            p = -0.000200214257;
            p = 0.000100950558 + p * sqrt_w_minus_3;
            p = 0.00134934322 + p * sqrt_w_minus_3;
            p = -0.00367342844 + p * sqrt_w_minus_3;
            p = 0.00573950773 + p * sqrt_w_minus_3;
            p = -0.0076224613 + p * sqrt_w_minus_3;
            p = 0.00943887047 + p * sqrt_w_minus_3;
            p = 1.00167406 + p * sqrt_w_minus_3;
            p = 2.83297682 + p * sqrt_w_minus_3;
        }

        let res_ra = p * x;

        let fx = Functions::erf(res_ra) - x;
        let df = (2.0 / (std::f64::consts::PI).sqrt()) * (-res_ra * res_ra).exp();
        let d2f = -2.0 * res_ra * df;

        res_ra - (2.0 * fx * df) / ((2.0 * df * df) - (fx * d2f))
    }
    /// Uses the definition of a derivative to calculate the derivative of a function at a specific point of a given function. <br>
    /// Learn more about Derivatives and Differentiation at: <https://wikipedia.org/wiki/Derivative>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use vml::special::Functions;
    ///
    /// let function = |x: f64| x.powi(2);
    /// let x = 2;
    ///
    /// let derivative = Functions::derivative(function, x).unwrap();
    ///
    /// assert_eq!(derivative, 4);
    /// ```
    /// <hr/>
    ///
    pub fn derivative(f: fn(f64) -> f64, x: impl Into<f64> + Copy) -> Result<i64, f64> {
        let h = 1e-7;
        let definition = ((f(x.into() + h)) - f(x.into())) / h;

        if definition.fract().abs() < 1e-7 {
            Ok(definition.round() as i64)
        } else {
            Err(definition)
        }
    }
    /// Uses Simpson's 1/3rd Rule to calculate the integral.<br>
    /// Learn more about Integrals at: <https://wikipedia.org/wiki/Integral> <br>
    /// Learn more about Simpson's 1/3rd Rule at: <https://wikipedia.org/wiki/Simpson's_rule> <br>
    /// <hr/>
    ///
    /// # Example
    /// ```
    /// use vml::special::Functions;
    ///
    /// let lower_bound = 0_f64;
    /// let upper_bound = 6_f64;
    /// let function = |x: f64| x.powi(2);
    ///
    /// let integral = Functions::integral(lower_bound, upper_bound, function).unwrap();
    ///
    /// assert_eq!(integral, 72_f64)
    /// ```
    /// <hr/>
    ///
    pub fn integral<F: Fn(f64) -> f64>(lower_bound: f64, upper_bound: f64, f: F) -> Result<f64, String> {
        let lower = lower_bound;
        let upper = upper_bound;

        let n = 100000;
        let h = (upper - lower) / n as f64;

        let mut sum = f(lower) + f(upper);

        for i in 1..n {
            let x = lower + i as f64 * h;
            sum += if i % 2 == 0 {
                2.0 * f(x)
            } else {
                4.0 * f(x)
            };
        }

        let r1 = (h / 3.0) * sum;

        if r1.is_infinite() || r1.is_nan() {
            return Err("Function is Divergent".to_string());
        }

        let r2 = (r1 * 1e+6).round();

        if ((r1 * 1e+6).floor() + 1e-6) > r2 || ((r1 * 1e+6).ceil() - 1e-6) < r2 {
            Ok(r2 / 1e+6)
        } else {
            Ok(r1)
        }
    }
    /// Summations in Rust. <br>
    /// Learn more at: <https://wikipedia.org/wiki/Summation>
    /// <hr/>
    ///
    /// # Example #1: Constant
    ///
    /// ```
    /// use vml::special::Functions;
    ///
    /// let start = 0;
    /// let limit = 9;
    /// let function = |x: f64| 3_f64;
    ///
    /// let summation = Functions::summation(start, limit, function);
    ///
    /// assert_eq!(summation, 30_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2: Function
    /// ```
    /// use vml::special::Functions;
    ///
    /// let start = 4.5;
    /// let limit = 100;
    /// let function = |x: f64| 1_f64 / x;
    ///
    /// let summation = Functions::summation(start, limit, function);
    ///
    /// assert_eq!(summation, 3.1040441843062854);
    /// ```
    /// <hr/>
    ///
    pub fn summation(start: impl Into<f64> + Copy,limit: impl Into<f64> + Copy, f: fn(f64) -> f64) -> f64 {
        let start = start.into().round() as i64;
        let limit = limit.into().round() as i64;
        let mut result = 0.0;

        for i in start..=limit {
            result += f(i as f64);
        }

        result
    }
    /// Calculates a Factorial by using Lanczos's Gamma Function Approximation. <br>
    /// Learn more at: <https://wikipedia.org/wiki/Factorial>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use vml::special::Functions;
    ///
    /// let n = 6_f64;
    /// let factorial = Functions::factorial(n);
    /// assert_eq!(factorial, 720_f64);
    /// ```
    /// <hr/>
    ///
    pub fn factorial(n: f64) -> f64 {
        Gamma::lanczos(n + 1_f64)
    }
    /// Calculates the product of a function. a.k.a capital Pi Notation. <br>
    /// Learn more at: <https://wikipedia.org/wiki/Product_(mathematics)#Product_of_a_sequence>
    /// <hr/>
    ///
    /// # Example #1: Constant
    ///
    /// ```
    /// use vml::special::Functions;
    ///
    /// let start = 2_f64;
    /// let limit = 7_f64;
    /// let f = |x: f64| 3_f64;
    ///
    /// let product_series = Functions::product(start, limit, f);
    /// assert_eq!(product_series, 729_f64);
    /// ```
    /// <hr/>
    ///
    /// # Example #2: Function
    ///
    /// ```
    /// use vml::special::Functions;
    ///
    /// let start = 1_f64;
    /// let limit = 6_f64;
    /// let f = |x: f64| x.powi(2);
    ///
    /// let product_series = Functions::product(start, limit, f);
    /// assert_eq!(product_series, 518400_f64);
    /// ```
    /// <hr/>
    ///
    pub fn product<F: Fn(f64) -> f64>(start: f64, limit: f64, f: F) -> f64 {
        let mut result = 1.0;
        let start = start.round() as i64;
        let limit = limit.round() as i64;

        for i in start..=limit {
            result *= f(i as f64);
        }
        result
    }
}