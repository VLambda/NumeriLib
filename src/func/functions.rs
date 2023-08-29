use crate::extra::Extra;
use crate::func::integration::Integration;
use crate::special::Gamma;

pub struct Functions;

impl Functions {
    /// Uses the definition of a derivative to calculate the derivative of a function at a specific point of a given function. <br>
    /// Learn more about Derivatives and Differentiation at: <a href="https://wikipedia.org/wiki/Derivative" target="_blank">Wikipedia Derivative</a> <br>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use ferrate::Functions;
    ///     
    /// fn main() {
    ///     let function = |x: f64| x.powi(2);
    ///     let x = 2;
    ///
    ///     let derivative = Functions::derivative(function, x);
    ///
    ///     println!("The Derivative of x^2 as x=2 is: {}", derivative);
    /// }
    /// ```
    /// <hr/>
    ///
    pub fn derivative<F: Fn(f64) -> f64>(f: F, x: impl Into<f64> + Copy) -> f64 {
        let h = 1e-7;
        let definition = ((f(x.into() + h)) - f(x.into())) / h;

        if definition.fract().abs() < Extra::EPSILON1 {
            definition.round()
        } else {
            definition
        }
    }
    /// The Right Endpoint method to calculate a definite integral.<br>
    /// Learn more about Integrals at: <a href="https://wikipedia.org/wiki/Integral" target="_blank">Wikipedia Integral</a> <br>
    /// Learn more about Simpson's 1/3rd Rule at: <a href="https://wikipedia.org/wiki/Riemann_sum" target="_blank">Wikipedia Riemann Sum</a> <br>
    /// <hr/>
    ///
    /// # Example
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::right_riemann(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn right_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::right_riemann(function, lower_limit, upper_limit, interval_size)
    }
    /// The Left Endpoint method to calculate a definite integral.<br>
    /// Learn more about Integrals at: <a href="https://wikipedia.org/wiki/Integral" target="_blank">Wikipedia Integral</a> <br>
    /// Learn more about Simpson's 1/3rd Rule at: <a href="https://wikipedia.org/wiki/Riemann_sum" target="_blank">Wikipedia Riemann Sum</a> <br>
    /// <hr/>
    ///
    /// # Example
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::left_riemann(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn left_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::left_riemann(function, lower_limit, upper_limit, interval_size)
    }
    /// The Left Endpoint method to calculate a definite integral.<br>
    /// Learn more about Integrals at: <a href="https://wikipedia.org/wiki/Integral" target="_blank">Wikipedia Integral</a> <br>
    /// Learn more about Simpson's 1/3rd Rule at: <a href="https://wikipedia.org/wiki/Riemann_sum" target="_blank">Wikipedia Riemann Sum</a> <br>
    /// <hr/>
    ///
    /// # Example
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::midpoint_riemann(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn midpoint_riemann<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::midpoint_riemann(function, lower_limit, upper_limit, interval_size)
    }
    /// The Trapezoid method to calculate a definite integral.<br>
    /// Learn more about Integrals at: <a href="https://wikipedia.org/wiki/Integral" target="_blank">Wikipedia Integral</a> <br>
    /// Learn more about Simpson's 1/3rd Rule at: <a href="https://wikipedia.org/wiki/Trapezoidal_rule" target="_blank">Wikipedia Trapezoid</a> <br>
    /// <hr/>
    ///
    /// # Example
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::trapezoid(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn trapezoid<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::trapezoid(function, lower_limit, upper_limit, interval_size)
    }
    /// Uses Simpson's Rule to calculate a definite integral.<br>
    /// Learn more about Integrals at: <a href="https://wikipedia.org/wiki/Integral" target="_blank">Wikipedia Integral</a> <br>
    /// Learn more about Simpson's 1/3rd Rule at: <a href="https://wikipedia.org/wiki/Simpson's_rule" target="_blank">Wikipedia Simpson's Rule</a> <br>
    /// <hr/>
    ///
    /// # Example
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let lower_bound = 0_f64;
    ///     let upper_bound = 6_f64;
    ///     let interval_size = 1_f64;
    ///     let function = |x: f64| x.powi(2);
    ///
    ///     let integral = Functions::simpson(function, lower_bound, upper_bound, interval_size);
    ///
    ///     println!("The Integral of x^2 at [0,6] is: {}", integral)
    /// }
    /// ```
    /// <hr/>
    pub fn simpson<F: Fn(f64) -> f64>(
        function: F,
        lower_limit: f64,
        upper_limit: f64,
        interval_size: f64,
    ) -> f64 {
        Integration::simpson(function, lower_limit, upper_limit, interval_size)
    }
    /// Calculates a Factorial by using Lanczos's Gamma Function Approximation. <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Factorial" target="_blank">Wikipedia Factorials</a> <br>
    /// <hr/>
    ///
    /// # Example:
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let n = 6_f64;
    ///     let factorial = Functions::factorial(n);
    ///     
    ///     println!("6! is: {}", factorial);
    /// }
    /// ```
    /// <hr/>
    ///
    pub fn factorial(n: f64) -> f64 {
        if n.floor() == n {
            (1..=n as u64).map(|i| i as f64).product()
        } else {
            Gamma::lanczos(n + 1_f64)
        }
    }
    /// Summations in Rust. <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Summation" target="_blank">Wikipedia Summation</a> <br>
    /// <hr/>
    ///
    /// # Example #1: Constant
    ///
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let start = 0;
    ///     let limit = 9;
    ///     let function = |x: f64| 3_f64;
    ///
    ///     let summation = Functions::summation(start, limit, function);
    ///
    ///     println!("The summation of the constant 3 from [0,9] is: {}", summation);
    /// }
    /// ```
    /// <hr/>
    ///
    /// # Example #2: Function
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let start = 4.5;
    ///     let limit = 100;
    ///     let function = |x: f64| 1_f64 / x;
    ///
    ///     let summation = Functions::summation(start, limit, function);
    ///
    ///     println!("The summation of the function 1/x from [4.5,100] is: {}", summation);
    /// }
    /// ```
    /// <hr/>
    pub fn summation(
        start: impl Into<f64> + Copy,
        limit: impl Into<f64> + Copy,
        f: fn(f64) -> f64,
    ) -> f64 {
        let start = start.into().round() as i64;
        let limit = limit.into().round() as i64;
        let mut result = 0.0;

        for i in start..=limit {
            result += f(i as f64);
        }

        result
    }
    /// Calculates the product of a function. a.k.a Capital Pi Notation. <br>
    /// Learn more at: <a href="https://wikipedia.org/wiki/Product_(mathematics)#Product_of_a_sequence" target="_blank">Wikipedia Capital Pi Notation</a> <br>
    /// <hr/>
    ///
    /// # Example #1: Constant
    ///
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let start = 2_f64;
    ///     let limit = 7_f64;
    ///     let f = |x: f64| 3_f64;
    ///
    ///     let product_series = Functions::product(start, limit, f);
    ///     
    ///     println!("The Product Series of the Constant 3 from [2,7] is: {}", product_series);
    /// }
    /// ```
    /// <hr/>
    ///
    /// # Example #2: Function
    ///
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let start = 1_f64;
    ///     let limit = 6_f64;
    ///     let f = |x: f64| x.powi(2);
    ///
    ///     let product_series = Functions::product(start, limit, f);
    ///     
    ///     println!("The product series of the function x^2 from [1,6] is: {}", product_series);
    /// }
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
    /// A rust implementation of the Newton–Raphson method for finding roots.
    /// Learn more at: <a href="https://wikipedia.org/wiki/Newton's_method" target="_blank">Wikipedia Newton–Raphson method</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let x = 1.5_f64;
    ///     let function = |x: f64| x.powi(2) - 2_f64;
    ///     let newton = Functions::newmet(x, function);
    ///
    ///     println!("Using Newton's Method we can approximate the root of x^2-2 with a guess of 1.5 as: {}", newton);
    /// }
    /// ```
    /// <hr/>
    pub fn newmet<F: Fn(f64) -> f64>(guess: f64, func: F) -> f64 {
        let mut iteration = 0;
        let mut result = guess;
        while iteration < 200 {
            let value = func(result);
            let derivative_value = Functions::derivative(&func, result);

            if derivative_value.abs() < Extra::EPSILON2 {
                break;
            }

            result -= value / derivative_value;

            if value.abs() < Extra::EPSILON2 {
                break;
            }

            iteration += 1;
        }

        result
    }
    /// Pochhammer's Function in Rust <br>
    /// Learn more at: <a href="https://en.wikipedia.org/wiki/Falling_and_rising_factorials" target="_blank">Wikipedia Falling and Rising Factorials</a> <br>
    /// <hr/>
    ///
    /// # Example:
    ///
    /// ```
    /// use ferrate::Functions;
    ///
    /// fn main() {
    ///     let x = 2_f64;
    ///     let n = 3_f64;
    ///     let poch = Functions::pochhammer(x, n);
    ///
    ///     println!("The Rising Factorial of 2^n n=3 is: {}", poch);
    /// }
    /// ```
    /// <hr/>
    pub fn pochhammer(x: f64, n: f64) -> f64 {
        Gamma::lanczos(x + n) / Gamma::lanczos(x)
    }
    /// Falling Factorials in Rust <br>
    /// Learn more at: <a href="https://en.wikipedia.org/wiki/Falling_and_rising_factorials" target="_blank">Wikipedia Falling and Rising Factorials</a> <br>
    /// <hr/>
    ///
    ///
    /// # Example:
    ///
    /// ```                                       
    /// use ferrate::Functions;          
    ///
    /// fn main() {                                          
    ///     let x = 3_f64;                            
    ///     let n = 2_f64;                            
    ///                                           
    ///     let fall = Functions::fallfactorial(x, n);   
    ///                                           
    ///     println!("The Falling Factorial of 3^n where n=2 is: {}", fall);
    /// }               
    /// ```                                       
    pub fn fallfactorial(x: f64, n: f64) -> f64 {
        Gamma::lanczos(x + 1_f64) / Gamma::lanczos(x - n + 1_f64)
    }
}
