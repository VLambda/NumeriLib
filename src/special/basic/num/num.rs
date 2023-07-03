pub struct Num;

impl Num {
    /// Uses the definition of a derivative to calculate the derivative of a function at a specific point of a given function. <br>
    /// Learn more about Derivatives and Differentiation at: <https://wikipedia.org/wiki/Derivative>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use vml::special::Num;
    ///
    /// let function = |x: f64| x.powi(2);
    /// let x = 2;
    ///
    /// let derivative = Num::derivative(function, x).unwrap();
    ///
    /// assert_eq!(derivative, 4);
    /// ```
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
    /// use vml::special::Num;
    ///
    /// let lower_bound = 0_f64;
    /// let upper_bound = 6_f64;
    /// let function = |x: f64| x.powi(2);
    ///
    /// let integral = Num::integral(lower_bound, upper_bound, function).unwrap();
    ///
    /// assert_eq!(integral, 72_f64)
    /// ```
    pub fn integral(lower_bound: f64, upper_bound: f64, f: fn(f64) -> f64) -> Result<f64, String> {
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

        let result = (h / 3.0) * sum;

        if result.is_infinite() || result.is_nan() {
            return Err("Function is Divergent".to_string());
        }

        if (result.floor() + 1e-6) > result || (result.ceil() - 1e-6) < result {
            let rounded_result = result.round();
            Ok(rounded_result)
        } else {
            Ok(result)
        }
    }
    /// Summations in Rust.
    /// Learn more at: <https://wikipedia.org/wiki/Summation>
    ///
    /// # Example #1: Function
    ///
    /// ```
    /// use vml::special::Num;
    ///
    /// let start = 0;
    /// let limit = 9;
    /// let function = |x: f64| 3_f64;
    ///
    /// let summation = Num::summation(start, limit, function);
    ///
    /// assert_eq!(summation, 30_f64);
    /// ```
    ///
    /// # Example #2: Function
    /// ```
    /// use vml::special::Num;
    ///
    /// let start = 4.5;
    /// let limit = 100;
    /// let function = |x: f64| 1_f64 / x;
    ///
    /// let summation = Num::summation(start, limit, function);
    ///
    /// assert_eq!(summation, 3.1040441843062854);
    /// ```
    pub fn summation(start: impl Into<f64> + Copy,limit: impl Into<f64> + Copy, f: fn(f64) -> f64) -> f64 {
        let start = start.into().round() as i64;
        let limit = limit.into().round() as i64;
        let mut result = 0.0;

        for i in start..=limit {
            result += f(i as f64);
        }

        result
    }
}